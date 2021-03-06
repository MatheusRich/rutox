use std::{env, fs, process};
mod exitcodes;
mod interpreter;
mod parser;
mod rutox_error;
mod scanner;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

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

    match eval(file_content.clone()) {
        Ok(_result) => {}
        Err(error) => {
            println!("{error}");
            println!("{}", error.details(path, &file_content));
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

                    match eval(line.clone()) {
                        Ok(_) => {}
                        Err(error) => {
                            println!("{error}");
                            println!("{}", error.details("repl", &line));
                        }
                    }
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

fn eval(source: String) -> Result<(), rutox_error::RutoxError> {
    Scanner::new(source)
        .scan_tokens()
        .and_then(|tokens| Parser::new(tokens).parse())
        .and_then(|stmts| Interpreter::new().interpret(stmts))
}
