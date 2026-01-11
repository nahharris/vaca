use std::cell::RefCell;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::rc::Rc;

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

    let forms = match vaca::parse(&input) {
        Ok(nodes) => nodes,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let env = make_global_env();
    let mut last = Value::Nil;
    for form in &forms {
        match eval(form, &env) {
            Ok(v) => last = v,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }

    // Script mode doesn't implicitly print results; keep this quiet unless non-nil.
    if !matches!(last, Value::Nil) {
        println!("{last}");
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
