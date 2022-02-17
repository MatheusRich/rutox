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
            RutoxError::Syntax(msg, _) => {
                let error_type = "SyntaxError".red();
                let msg = format!("{error_type}: {msg}.").bold();

                write!(f, "{}", msg)
            }
            RutoxError::Runtime(msg, _) => {
                let error_type = "RuntimeError".red();
                let msg = format!("{error_type}: {msg}.").bold();

                write!(f, "{}", msg)
            }
            RutoxError::Programmer(msg, location) => {
                let error_type = "ProgrammerError".red();
                let msg = format!(
                    "{error_type}: {msg} at {location}.\nThis is a bug in rutox. Please report it at https://github.com/MatheusRich/rutox."
                )
                .bold();

                write!(f, "{}", msg)
            }
        }
    }
}

impl RutoxError {
    pub fn location(&self) -> SrcLocation {
        match self {
            RutoxError::Syntax(_, location) => location.clone(),
            RutoxError::Programmer(_, location) => location.clone(),
            RutoxError::Runtime(_, location) => location.clone(),
        }
    }

    pub fn details(&self, source_file: &str) -> String {
        let arrow = "  -->".blue();

        format!("{} {}:{}", arrow, source_file, self.location())
    }
}
