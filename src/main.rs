use std::{env, fs, process};
mod exitcodes;
mod interpreter;
mod parser;
mod rutox_error;
mod scanner;
use interpreter::{Interpreter, LoxObj};
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
                        Ok(result) => println!("=> {}", result.as_colored_string()),
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

fn eval(source: String) -> Result<LoxObj, rutox_error::RutoxError> {
    Scanner::new(source)
        .scan_tokens()
        .and_then(|tokens| Parser::new(tokens).parse())
        .and_then(|expr| Interpreter::new().interpret(&expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_knows_math() {
        let input = "1.0 + 1.0 * 2.0 - 3.0 / 4.0";

        let result = eval_to_string(input);

        assert_eq!(result, "2.25");
    }

    #[test]
    fn it_understands_equality() {
        let result = eval_to_string("1.0 == 1.0");
        assert_eq!(result, "true");

        let result = eval_to_string("1.0 != 2.0");
        assert_eq!(result, "true");

        let result = eval_to_string("\"hi\" == \"hi\"");
        assert_eq!(result, "true");

        let result = eval_to_string("\"hi\" != \"hello\"");
        assert_eq!(result, "true");

        let result = eval_to_string("true == true");
        assert_eq!(result, "true");

        let result = eval_to_string("false == false");
        assert_eq!(result, "true");

        let result = eval_to_string("true != false");
        assert_eq!(result, "true");

        let result = eval_to_string("nil == nil");
        assert_eq!(result, "true");

        let result = eval_to_string("nil != false");
        assert_eq!(result, "true");
    }

    #[test]
    fn it_understands_comparison() {
        let result = eval_to_string("1.0 < 2.0");
        assert_eq!(result, "true");

        let result = eval_to_string("1.0 <= 2.0");
        assert_eq!(result, "true");

        let result = eval_to_string("2.0 > 1.0");
        assert_eq!(result, "true");

        let result = eval_to_string("2.0 >= 1.0");
        assert_eq!(result, "true");

        let result = eval_to_string("1.0 > 2.0");
        assert_eq!(result, "false");

        let result = eval_to_string("1.0 >= 2.0");
        assert_eq!(result, "false");

        let result = eval_to_string("2 < 1");
        assert_eq!(result, "false");

        let result = eval_to_string("2 <= 1");
        assert_eq!(result, "false");

        let result = eval_to_string(r#""b" > "a""#);
        assert_eq!(result, "true");

        let result = eval_to_string(r#""b" >= "a""#);
        assert_eq!(result, "true");

        let result = eval_to_string(r#""b" >= "b""#);
        assert_eq!(result, "true");

        let result = eval_to_string(r#""b" < "a""#);
        assert_eq!(result, "false");

        let result = eval_to_string(r#""b" <= "b""#);
        assert_eq!(result, "true");

        let result = eval_to_string(r#""a" < "b""#);
        assert_eq!(result, "true");

        let result = eval_to_string(r#""a" <= "b""#);
        assert_eq!(result, "true");

        for op in ["<", "<=", ">", ">="] {
            let error = get_error(format!("1 {op} true"));
            assert!(error.contains("RuntimeError"));
            assert!(error.contains("Cannot compare number 1 and boolean true."));

            let error = get_error(format!(r#"1 {op} "hi""#));
            assert!(error.contains("RuntimeError"));
            assert!(error.contains("Cannot compare number 1 and string \"hi\"."));

            let error = get_error(format!("1 {op} nil"));
            assert!(error.contains("RuntimeError"));
            assert!(error.contains("Cannot compare number 1 and nil."));

            let error = get_error(format!(r#""hi" {op} false"#));
            assert!(error.contains("RuntimeError"));
            assert!(error.contains("Cannot compare string \"hi\" and boolean false."));

            let error = get_error(format!(r#""hi" {op} nil"#));
            assert!(error.contains("RuntimeError"));
            assert!(error.contains("Cannot compare string \"hi\" and nil."));

            let error = get_error(format!("true {op} nil"));
            assert!(error.contains("RuntimeError"));
            assert!(error.contains("Cannot compare boolean true and nil."));
        }
    }

    fn eval_to_string(input: &str) -> String {
        eval(input.to_string()).ok().unwrap().to_string()
    }

    fn get_error(input: String) -> String {
        eval(input).err().unwrap().to_string()
    }
}
