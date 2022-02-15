use super::scanner::src_location::SrcLocation;

pub enum RutoxError {
    SyntaxError(String, SrcLocation),
}

impl std::fmt::Display for RutoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RutoxError::SyntaxError(msg, location) => write!(f, "SyntaxError: {msg} at {location}"),
        }
    }
}
