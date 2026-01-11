use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write, stdout},
};

use jlox::scanner::Scanner;

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
        if let Err(e) = run_prompt() {
            eprintln!("Error starting repl {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    if let Err(e) = run(&input) {
        eprintln!("Error executing program {e}");
        std::process::exit(65);
    };
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        println!("> ");
        stdout().flush()?;

        let mut buffer = String::new();
        let bytes_read = handle.read_line(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        if let Err(e) = run(&buffer) {
            eprintln!("Error executing program {e}");
        };
    }
    Ok(())
}

fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;

    for token in tokens.iter() {
        println!("{}", token);
    }
    Ok(())
}
