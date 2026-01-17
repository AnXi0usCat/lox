use std::error::Error;

use crate::error::UnterminatedStringError;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        if !source.is_ascii() {
            panic!("Non ascii character provided as part of input");
        }
        Scanner {
            source: source.as_bytes(),
            tokens: Vec::with_capacity(source.chars().count()),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> &u8 {
        let previous = &self.source[self.current];
        self.current += 1;
        previous
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] as char != expected {
            return false;
        }
        self.current += 1;

        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return '\0' as u8;
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return '\0' as u8;
        }
        self.source[self.current + 1]
    }

    fn is_digit(&self, c: u8) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            // consume the '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = str::from_utf8(&self.source[self.start..self.current]).expect("Invalid UTF-8");
        self.add_token(TokenType::Number, Some(String::from(value)));
    }

    fn string(&mut self) -> Result<(), Box<dyn Error>> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Box::new(UnterminatedStringError(format!(
                "Unterminated string on line: {}",
                self.line
            ))));
        }

        _ = self.advance();
        // trim quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(
            TokenType::String,
            Some(String::from(str::from_utf8(value).expect("Invalid UTF-8"))),
        );

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Box<dyn Error>> {
        match *self.advance() as char {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            ' ' | '\t' | '\r' => (),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' as u8 && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            '"' => self.string()?,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),
            _ => println!("Unexpected character on line {}", self.line),
        }
        Ok(())
    }

    pub fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let token_bytes = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: token_bytes,
            literal,
            line: self.line,
        });
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token<'a>>, Box<dyn Error>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".as_bytes(),
            literal: None,
            line: self.line,
        });
        Ok(self.tokens.clone())
    }
}
