use crate::token::{TokenKind, Token};

enum ScannerError {
    UnexpectedCharacter(char),
}

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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            
            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(ScannerError::UnexpectedCharacter(c)) => {
                    eprintln!("ScannerError::UnexpectedCharacter {c}")
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::from(""),
            line: self.line,
        });        

        tokens
    }

    fn scan_token(&mut self) -> Result<Token, ScannerError> {
        let c = self.advance();
        // println!("scan_token got c = {c}");

        match c {
            '(' => Ok(self.build_token(TokenKind::LeftParen)),
            ')' => Ok(self.build_token(TokenKind::RightParen)),
            ',' => Ok(self.build_token(TokenKind::Comma)),
            '.' => Ok(self.build_token(TokenKind::Def)),
            '-' => Ok(self.build_token(TokenKind::Minus)),
            '+' => Ok(self.build_token(TokenKind::Plus)),
            '*' => Ok(self.build_token(TokenKind::Star)),
            _ => Err(ScannerError::UnexpectedCharacter(*c))
        }
    }

    fn advance(&mut self) -> &char {
        // println!("advance current = {}", self.current);
        let c = self.source.get(self.current).unwrap();
        self.current += 1;
        c
    }

    fn build_token(&self, kind: TokenKind) -> Token {
        let text: String = self.source.get(self.start..self.current).unwrap().iter().collect();
        Token {
            kind,
            lexeme: text,
            line: self.line,
        }
    }

    // fn add_token_with_literal(&mut self, kind: TokenKind)

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}