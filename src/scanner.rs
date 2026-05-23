use crate::token::{TokenKind, Token};

enum ScannerError {
    UnexpectedCharacter(char),
}

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    indent_level: usize,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            indent_level: 0,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        // let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            
            match self.scan_token() {
                Ok(token) => {
                    if token.kind != TokenKind::Inert {
                        self.tokens.push(token)
                    }
                },
                Err(ScannerError::UnexpectedCharacter(c)) => {
                    eprintln!("ScannerError::UnexpectedCharacter {c}")
                }
            }
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::from(""),
            line: self.line,
        });        

        // self.tokens
    }

    fn scan_token(&mut self) -> Result<Token, ScannerError> {
        let c = self.advance().unwrap();

        println!("scan_token c = >{c}<");

        let kind = match c {
            '\n' => {
                println!("got newline!");
                self.line += 1;
                
                // we have to check the next line for indentation now
                let mut num_spaces: usize = 0;
                while self.peek() == Some(' ') {
                    num_spaces += 1;
                    self.advance();
                }

                println!("num_spaces = {num_spaces}");

                let level = num_spaces / 4;

                println!("new level = {}, old level = {}", level, self.indent_level);

                if level == self.indent_level {
                    // self.tokens.push(self.build_token(TokenKind::Inert));
                } else if level == self.indent_level + 1 {
                    self.tokens.push(self.build_token(TokenKind::Indent));
                } else if level < self.indent_level {
                    let num_dedents = self.indent_level - level;
                    for _ in 0..num_dedents {
                        self.tokens.push(self.build_token(TokenKind::Dedent));
                    }
                }
                self.indent_level = level;

                TokenKind::Inert
            }
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Def,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Star,
            '!' => {
                if self.advance_if_match('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            },
            '=' => {
                if self.advance_if_match('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            },
            '<' => {
                if self.advance_if_match('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            },
            '>' => {
                if self.advance_if_match('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            },
            // a python comment
            '#' => {
                while self.peek() != Some('\n') && !self.is_at_end() {
                    self.advance();
                }
                TokenKind::Inert
            }
            _ => return Err(ScannerError::UnexpectedCharacter(c))
        };

        Ok(self.build_token(kind))
    }

    fn advance(&mut self) -> Option<char> {
        match self.peek() {
            Some(c) => {
                self.current += 1;
                Some(c)
            },
            None => None
        }
    }

    fn advance_if_match(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.current += 1;
                true
            },
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(*self.source.get(self.current).unwrap())
        }
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