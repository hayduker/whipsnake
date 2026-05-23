use crate::token::{TokenKind, Token};

enum ScannerError {
    UnexpectedCharacter(usize, char),
    TooManyIndentations(usize, usize),
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
                Err(ScannerError::UnexpectedCharacter(l, c)) => {
                    eprintln!("ScannerError::UnexpectedCharacter line {l}: {c}")
                },
                Err(ScannerError::TooManyIndentations(l, n)) => {
                    eprintln!("ScannerError::TooManyIndentationsline {l}: {n} more than previous line")
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
                self.line += 1;
                self.scan_indentation()?
            },
            // beginning-of-line indentation is consumed with self.scan_indentation
            // in '\n' pattern, so this is whitespace elsewhere in the line
            ' ' | '\t' | '\r' => TokenKind::Inert,
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
            _ => return Err(ScannerError::UnexpectedCharacter(self.line, c))
        };

        Ok(self.build_token(kind))
    }

    fn scan_indentation(&mut self) -> Result<TokenKind, ScannerError> {
        let mut num_spaces: usize = 0;
        while self.advance_if_match(' ') { num_spaces += 1 }
        let level = num_spaces / 4;
        println!("num_spaces = {num_spaces}, new level = {}, old level = {}", level, self.indent_level);

        if level == self.indent_level + 1 {
            self.tokens.push(self.build_token(TokenKind::Indent));
        } else if level < self.indent_level {
            let num_dedents = self.indent_level - level;
            for _ in 0..num_dedents {
                self.tokens.push(self.build_token(TokenKind::Dedent));
            }
        } else if level != self.indent_level {
            let how_many = level - self.indent_level;
            return Err(ScannerError::TooManyIndentations(self.line, how_many));
        }
        self.indent_level = level;

        Ok(TokenKind::Inert)
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