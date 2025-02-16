use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LexicalError {
    message: String,
    line: usize,
    position: usize,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {}. Line {}, position {}",
            self.message, self.line, self.position
        )
    }
}

impl Error for LexicalError {}

impl LexicalError {
    pub fn new(message: String, line: usize, position: usize) -> LexicalError {
        LexicalError {
            message,
            line,
            position,
        }
    }
}
