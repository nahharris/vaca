use std::env;
use std::fs;

fn main() {
    // Minimal CLI for now:
    // `vaca <file>` parses the file as EDN and prints the AST.
    //
    // The crate is primarily intended to be used as an SDK; this binary is a
    // small convenience for debugging the frontend.
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: vaca <file.vaca>");
        std::process::exit(2);
    };

    let input = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            std::process::exit(2);
        }
    };

    match vaca::parse(&input) {
        Ok(nodes) => {
            println!("{:#?}", nodes);
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
