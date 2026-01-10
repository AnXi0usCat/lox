use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,
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
        self.current += 1;
        &self.source[self.current]
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
            _ => println!("Unexpected character"),
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let token_bytes = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: str::from_utf8(token_bytes).expect("Invalid UTF-8").into(),
            literal,
            line: self.line,
        });
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".into(),
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
    }
}
