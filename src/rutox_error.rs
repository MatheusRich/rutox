use super::scanner::src_location::SrcLocation;
use colored::*;

pub enum RutoxError {
    Programmer(String, SrcLocation),
    Syntax(String, SrcLocation),
    Runtime(String, SrcLocation),
}

impl std::fmt::Display for RutoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RutoxError::Syntax(msg, location) => {
                let error_type = "SyntaxError".red().bold();
                write!(f, "{error_type}: {msg} at {location}.")
            }
            RutoxError::Runtime(msg, location) => {
                let error_type = "RuntimeError".red().bold();
                write!(f, "{error_type}: {msg} (found at {location}).")
            }
            RutoxError::Programmer(msg, location) => {
                let error_type = "ProgrammerError".red().bold();

                write!(
                    f,
                    "{error_type}: {msg} at {location}.\nThis is a bug in rutox. Please report it."
                )
            }
        }
    }
}
