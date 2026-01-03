use std::{env, error::Error};

mod repl;
use crate::repl::start_repl;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        eprintln!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        if let Err(e) = run_file(&args[0]) {
            eprintln!("Error running file {}", e);
            std::process::exit(1);
        }
    } else {
        if let Err(e) = start_repl() {
            eprintln!("Error starting repl {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

