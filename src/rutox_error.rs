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
                let error_type = "SyntaxError".red();
                let msg = format!("{error_type}: {msg} at {location}.").bold();

                write!(f, "{}", msg)
            }
            RutoxError::Runtime(msg, location) => {
                let error_type = "RuntimeError".red();
                let msg = format!("{error_type}: {msg} (found at {location}).").bold();

                write!(f, "{}", msg)
            }
            RutoxError::Programmer(msg, location) => {
                let error_type = "ProgrammerError".red();
                let msg = format!(
                    "{error_type}: {msg} at {location}.\nThis is a bug in rutox. Please report it."
                )
                .bold();

                write!(f, "{}", msg)
            }
        }
    }
}
