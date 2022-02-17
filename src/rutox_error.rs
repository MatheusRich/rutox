use super::scanner::src_location::SrcLocation;

pub enum RutoxError {
    Programmer(String, SrcLocation),
    Syntax(String, SrcLocation),
    Runtime(String, SrcLocation),
}

impl std::fmt::Display for RutoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RutoxError::Syntax(msg, location) => {
                write!(f, "SyntaxError: {msg} at {location}.")
            }
            RutoxError::Runtime(msg, location) => {
                write!(f, "RuntimeError: {msg} (found at {location}).")
            }
            RutoxError::Programmer(msg, location) => write!(
                f,
                "ProgrammerError: {msg} at {location}.\nThis is a bug in rutox. Please report it."
            ),
        }
    }
}
