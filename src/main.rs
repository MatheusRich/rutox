use std::{env, fs, process};
mod exitcodes;
mod parser;
mod rutox_error;
mod scanner;
use scanner::Scanner;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rutox [script]");
            process::exit(exitcodes::USAGE);
        }
    }
}

fn run_file(path: &str) {
    let file_content = fs::read_to_string(path).unwrap_or_else(|err| {
        println!("Error while opening {}: {}", path, err);
        process::exit(exitcodes::IOERR);
    });

    match run(file_content) {
        Ok(_) => {}
        Err(err) => {
            println!("Error: {}", err);
            process::exit(exitcodes::DATAERR);
        }
    }
}

fn run_repl() {
    use rustyline::error::ReadlineError;
    use rustyline::Editor;

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("rutox> ");
        match readline {
            Ok(line) => match line.trim() {
                "" => continue,
                "quit" | "exit" => break,
                _ => {
                    rl.add_history_entry(line.as_str());
                    run(line);
                }
            },
            Err(ReadlineError::Interrupted) => println!("^C"),
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Unexpected error: {:?}", err);
                break;
            }
        }
    }
}

fn run(source: String) -> Result<(), rutox_error::RutoxError> {
    match Scanner::new(source).scan_tokens() {
        Ok(tokens) => {
            let str: String = tokens
                .iter()
                .map(|token| format!("{}", token))
                .collect::<Vec<String>>()
                .join(", ");

            println!("{}", str);
            Ok(())
        }
        Err(err) => {
            println!("{err}");
            Err(err)
        }
    }
}

fn run2(source: String) -> Result<(), rutox_error::RutoxError> {
    Scanner::new(source)
        .scan_tokens()
        .and_then(|tokens| Parser::new(tokens).parse_exprs())
}
