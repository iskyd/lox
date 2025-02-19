use crate::errors::LexicalError;
use crate::tokens::{Literal, Token, TokenType};
use crate::utils::{is_alpha, is_alphanumeric};
use std::str;

struct Scanner<'a> {
    source: &'a [u8],
    current_line: usize,
    current_position: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Result<Self, &'static str> {
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
    pub fn scan(mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = vec![];
        while self.current_position < self.source.len() {
            match self.source[self.current_position] as char {
                '(' => tokens.push(Token::new(
                    TokenType::LeftParen,
                    String::from("("),
                    self.current_line,
                    self.current_position,
                    self.current_position,
                    None,
                )),
                ')' => tokens.push(Token::new(
                    TokenType::RightParen,
                    String::from(")"),
                    self.current_line,
                    self.current_position,
                    self.current_position,
                    None,
                )),
                '{' => tokens.push(Token::new(
                    TokenType::LeftBrace,
                    String::from("{"),
                    self.current_line,
                    self.current_position,
                    self.current_position,
                    None,
                )),
                '}' => tokens.push(Token::new(
                    TokenType::RightBrace,
                    String::from("{"),
                    self.current_line,
                    self.current_position,
                    self.current_position,
                    None,
                )),
                '!' => {
                    if self.current_position + 1 < self.source.len()
                        && self.source[self.current_position + 1] as char == '='
                    {
                        tokens.push(Token::new(
                            TokenType::BangEq,
                            String::from("!="),
                            self.current_line,
                            self.current_position,
                            self.current_position + 1,
                            None,
                        ))
                    } else {
                        tokens.push(Token::new(
                            TokenType::Bang,
                            String::from("!"),
                            self.current_line,
                            self.current_position,
                            self.current_position,
                            None,
                        ))
                    }
                }
                '=' => {
                    if self.current_position + 1 < self.source.len()
                        && self.source[self.current_position + 1] as char == '='
                    {
                        tokens.push(Token::new(
                            TokenType::EqEq,
                            String::from("=="),
                            self.current_line,
                            self.current_position,
                            self.current_position + 1,
                            None,
                        ));
                        self.current_position += 1;
                    } else {
                        tokens.push(Token::new(
                            TokenType::Eq,
                            String::from("="),
                            self.current_line,
                            self.current_position,
                            self.current_position,
                            None,
                        ))
                    }
                }
                '<' => {
                    if self.current_position + 1 < self.source.len()
                        && self.source[self.current_position + 1] as char == '='
                    {
                        tokens.push(Token::new(
                            TokenType::Lte,
                            String::from("<="),
                            self.current_line,
                            self.current_position,
                            self.current_position + 1,
                            None,
                        ))
                    } else {
                        tokens.push(Token::new(
                            TokenType::Lt,
                            String::from("<"),
                            self.current_line,
                            self.current_position,
                            self.current_position,
                            None,
                        ))
                    }
                }
                '>' => {
                    if self.current_position + 1 < self.source.len()
                        && self.source[self.current_position + 1] as char == '='
                    {
                        tokens.push(Token::new(
                            TokenType::Gte,
                            String::from(">="),
                            self.current_line,
                            self.current_position,
                            self.current_position + 1,
                            None,
                        ))
                    } else {
                        tokens.push(Token::new(
                            TokenType::Gt,
                            String::from(">"),
                            self.current_line,
                            self.current_position,
                            self.current_position,
                            None,
                        ))
                    }
                }
                '/' => {
                    if self.current_position + 1 < self.source.len()
                        && self.source[self.current_position + 1] as char == '/'
                    {
                        // Comment, consume everything untill the end of the line
                        while self.source[self.current_position] as char != '\n' {
                            self.current_position += 1;
                        }
                        self.current_line += 1;
                    } else {
                        tokens.push(Token::new(
                            TokenType::Slash,
                            String::from("/"),
                            self.current_line,
                            self.current_position,
                            self.current_position,
                            None,
                        ))
                    }
                }
                '"' => {
                    // String literal
                    let start_position = self.current_position;
                    let start_line = self.current_line;
                    self.current_position += 1;
                    while self.source[self.current_position] as char != '"' {
                        self.current_position += 1;
                        if self.source[self.current_position] as char == '\n' {
                            self.current_line += 1;
                        }

                        if self.current_position > self.source.len() {
                            return Err(LexicalError::new(
                                String::from("Non terminated string starting at line"),
                                start_line,
                                start_position,
                            ));
                        }
                    }

                    println!(
                        "start position {}, end position {}",
                        start_position + 1,
                        self.current_position - 1
                    );
                    let substr = String::from(
                        str::from_utf8(&self.source[start_position + 1..self.current_position - 1])
                            .unwrap(),
                    );
                    tokens.push(Token::new(
                        TokenType::String,
                        substr.clone(),
                        self.current_line,
                        self.current_position,
                        self.current_position,
                        Some(Literal::Str { value: substr }),
                    ));
                }
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => {
                    self.current_line += 1;
                }
                _ => {
                    if (self.source[self.current_position] as char).is_digit(10) {
                        let mut accept_dot = true;
                        let start_position = self.current_position;
                        self.current_position += 1;
                        loop {
                            let cur_char = self.source[self.current_position] as char;
                            if cur_char.is_digit(10) {
                                self.current_position += 1;
                            } else if cur_char == '.'
                                && accept_dot == true
                                && self.current_position + 1 < self.source.len()
                                && (self.source[self.current_position + 1] as char).is_digit(10)
                            {
                                accept_dot = false;
                                self.current_position += 1;
                            } else {
                                break;
                            }
                        }

                        let substr = String::from(
                            str::from_utf8(&self.source[start_position..self.current_position])
                                .unwrap(),
                        );

                        tokens.push(Token::new(
                            TokenType::Number,
                            substr.clone(),
                            self.current_line,
                            start_position,
                            self.current_position - 1,
                            Some(Literal::Number {
                                value: substr.parse().unwrap(),
                            }),
                        ));

                        self.current_position -= 1;
                    } else if is_alpha(self.source[self.current_position] as char) {
                        // identifier, consume it to the end
                        let start_position = self.current_position;
                        self.current_position += 1;

                        loop {
                            let cur_char = self.source[self.current_position] as char;
                            if is_alphanumeric(cur_char) {
                                self.current_position += 1;
                            } else {
                                break;
                            }
                        }

                        let substr = String::from(
                            str::from_utf8(&self.source[start_position..self.current_position])
                                .unwrap(),
                        );

                        match TokenType::get_type_by_reserved_keyword(&substr) {
                            Some(t) => tokens.push(Token::new(
                                t,
                                substr.clone(),
                                self.current_line,
                                start_position,
                                self.current_position - 1,
                                None,
                            )),
                            None => tokens.push(Token::new(
                                TokenType::Identifier,
                                substr.clone(),
                                self.current_line,
                                start_position,
                                self.current_position - 1,
                                Some(Literal::Str {
                                    value: substr.parse().unwrap(),
                                }),
                            )),
                        };

                        self.current_position -= 1;
                    } else {
                        return Err(LexicalError::new(
                            String::from("Invalid character"),
                            self.current_line,
                            self.current_position,
                        ));
                    }
                }
            };

            self.current_position += 1;
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::tokens::TokenType;

    #[test]
    fn test_reserved_keyword() {
        let v = String::from("if(var == 12) {\nprint(\"value\")}");
        let scanner = Scanner::new(v.as_bytes()).unwrap();
        let tokens = scanner.scan().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::Var);
        assert_eq!(tokens[3].token_type, TokenType::EqEq);
        assert_eq!(tokens[4].token_type, TokenType::Number);
        assert_eq!(tokens[5].token_type, TokenType::RightParen);
        assert_eq!(tokens[6].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[7].token_type, TokenType::Print);
        assert_eq!(tokens[8].token_type, TokenType::LeftParen);
        assert_eq!(tokens[9].token_type, TokenType::String);
        assert_eq!(tokens[10].token_type, TokenType::RightParen);
        assert_eq!(tokens[11].token_type, TokenType::RightBrace);
        assert_eq!(tokens.len(), 12);
    }
}
