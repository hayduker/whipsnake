use crate::token::{TokenKind, Token};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::from(""),
            line: self.line,
        });        

        &self.tokens
    }

    fn scan_token(&mut self) -> Token {
        let c = self.advance();
        // println!("scan_token got c = {c}");


        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Def),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            '/' => self.add_token(TokenKind::Slash),
            '*' => self.add_token(TokenKind::Star),
            _ => panic!("Scanning error: got unexpected character {c}")
        }

        Token { kind: TokenKind::Class, lexeme: String::from(""), line: 0 }
    }

    fn advance(&mut self) -> &char {
        // println!("advance current = {}", self.current);
        let c = self.source.get(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text: String = self.source.get(self.start..self.current).unwrap().iter().collect();
        self.tokens.push(Token {
            kind,
            lexeme: text,
            line: self.line,
        });
    }

    // fn add_token_with_literal(&mut self, kind: TokenKind)

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}