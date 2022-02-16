use std::{env, fs, process};
mod exitcodes;
mod parser;
mod rutox_error;
mod scanner;
use parser::{ast::Expr, ast_printer::AstPrinter, Parser};
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

    match run(file_content) {
        Ok(expr) => AstPrinter::print(&expr),
        Err(err) => {
            println!("{err}");
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

                    match run(line) {
                        Ok(expr) => AstPrinter::print(&expr),
                        Err(error) => println!("{error}"),
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

fn run(source: String) -> Result<Expr, rutox_error::RutoxError> {
    Scanner::new(source)
        .scan_tokens()
        .and_then(|tokens| Parser::new(tokens).parse())
}
