use std::cell::RefCell;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::rc::Rc;
use std::thread;

use vaca::bezerro::{eval, register_builtins, Env, EvalError, Value};
use vaca::ErrorKind;

fn main() {
    match env::args().nth(1).as_deref() {
        None => run_repl(),
        Some(path) => run_file(path),
    }
}

fn make_global_env() -> Rc<RefCell<Env>> {
    let env = Rc::new(RefCell::new(Env::new()));
    register_builtins(&mut env.borrow_mut());
    env
}

fn run_file(path: &str) {
    let input = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            std::process::exit(2);
        }
    };

    // Run user code on a larger stack so deep recursion doesn't crash the process before we can
    // return a proper EvalError::StackOverflow.
    let source_dir = std::path::Path::new(path).parent().map(|p| p.to_path_buf());
    let result: Result<Option<String>, String> = thread::Builder::new()
        .name("vaca-eval".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let forms = match vaca::parse(&input) {
                Ok(nodes) => nodes,
                Err(err) => return Err(err.to_string()),
            };

            let env = make_global_env();
            if let Some(dir) = source_dir {
                env.borrow_mut().set_source_dir(dir);
            }
            let mut last = Value::Nil;
            for form in &forms {
                match eval(form, &env) {
                    Ok(v) => last = v,
                    Err(e) => return Err(e.to_string()),
                }
            }
            Ok((!matches!(last, Value::Nil)).then(|| last.to_string()))
        })
        .unwrap_or_else(|e| {
            eprintln!("failed to start eval thread: {e}");
            std::process::exit(1);
        })
        .join()
        .unwrap_or_else(|_| {
            eprintln!("evaluation panicked");
            std::process::exit(1);
        });

    match result {
        Ok(output) => {
            // Script mode doesn't implicitly print results; keep this quiet unless non-nil.
            if let Some(s) = output {
                println!("{s}");
            }
        }
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    let env = make_global_env();
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        if buffer.is_empty() {
            print!("vaca> ");
        } else {
            print!("...> ");
        }
        if io::stdout().flush().is_err() {
            break;
        }

        let mut line = String::new();
        let n = match stdin.read_line(&mut line) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("read error: {err}");
                break;
            }
        };
        if n == 0 {
            break; // EOF
        }

        buffer.push_str(&line);

        let forms = match vaca::parse(&buffer) {
            Ok(nodes) => nodes,
            Err(err) => {
                if is_incomplete(&err.kind) {
                    continue;
                }
                eprintln!("{err}");
                buffer.clear();
                continue;
            }
        };

        let mut last = Value::Nil;
        for form in &forms {
            match eval(form, &env) {
                Ok(v) => last = v,
                Err(EvalError::Custom(msg)) => {
                    eprintln!("{msg}");
                    last = Value::Nil;
                    break;
                }
                Err(e) => {
                    eprintln!("{e}");
                    last = Value::Nil;
                    break;
                }
            }
        }

        if !matches!(last, Value::Nil) {
            println!("{last}");
        }
        buffer.clear();
    }
}

fn is_incomplete(kind: &ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::UnexpectedEof
            | ErrorKind::UnterminatedString
            | ErrorKind::UnterminatedCollection { .. }
    )
}
