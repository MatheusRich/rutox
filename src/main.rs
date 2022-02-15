use std::{env, fs, process};
mod exitcodes;
mod rutox_error;
mod scanner;
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

    run(file_content);
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

fn run(source: String) {
    match Scanner::new(source).scan_tokens() {
        Ok(tokens) => {
            let str: String = tokens
                .iter()
                .map(|token| format!("{}", token))
                .collect::<Vec<String>>()
                .join(", ");

                println!("{}", str);
        }
        Err(err) => {
            println!("{err}");
        }
    }
}
