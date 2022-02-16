use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SrcLocation {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for SrcLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
