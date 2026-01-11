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

    fn scan_token(&mut self) {
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
            },
            ' ' | '\t' | '\r' => (),
            _ => println!("Unexpected character"),
        }
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

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".as_bytes(),
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
    }
}
