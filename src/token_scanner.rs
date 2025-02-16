use crate::errors::LexicalError;
use crate::tokens::{Literal, Token, TokenType};

struct Scanner {
    source: Vec<u8>,
    current_line: usize,
    current_position: usize,
}

impl Scanner {
    fn new(source: Vec<u8>) -> Result<Self, &'static str> {
        if source.is_empty() {
            return Err("Cannot create a Scanner with empty source");
        }

        Ok(Scanner {
            source: source,
            current_line: 1,
            current_position: 0,
        })
    }

    // Consume it
    fn scan(mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = vec![];
        while self.current_position < self.source.len() {
            match self.source[self.current_position] as char {
                '(' => tokens.push(Token::new(
                    TokenType::LEFT_PAREN,
                    String::from("("),
                    self.current_line,
                    None,
                )),
                '\n' => {
                    self.current_line += 1;
                    self.current_position = 0;
                }
                _ => {
                    return Err(LexicalError::new(
                        String::from("Invalid character"),
                        self.current_line,
                        self.current_position,
                    ))
                }
            };

            self.current_position += 1;
        }

        Ok(tokens)
    }
}
