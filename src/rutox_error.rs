pub enum RutoxError {
    SyntaxError(String),
}

impl std::fmt::Display for RutoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RutoxError::SyntaxError(msg) => write!(f, "SyntaxError: {}", msg),
        }
    }
}
