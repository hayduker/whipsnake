use std::collections::VecDeque;

use crate::token::{TokenKind, Token};

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacter(usize, char),
    TooManyIndentations(usize, usize),
}

pub struct Scanner {
    source: Vec<char>,
    // pub tokens: Vec<Token>,
    indent_level: usize,
    start: usize,
    current: usize,
    line: usize,
    token_buffer: VecDeque<Token>,
    is_done: bool,
}

impl Iterator for Scanner {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.token_buffer.is_empty() {
            return Some(Ok(self.token_buffer.pop_front().unwrap()));
        }

        if self.is_done {
            return None;
        }
        
        while !self.is_at_end() {
            self.start = self.current;

            match self.next_token_group() {
                Ok(Some(tokens)) => {
                    self.token_buffer.extend(tokens);
                    if !self.token_buffer.is_empty() {
                        return Some(Ok(self.token_buffer.pop_front().unwrap()));
                    }
                }
                Ok(None) => { /* non-indentation whitespace or comment */ },
                Err(e) => return Some(Err(e)),
            }
        }

        self.is_done = true;
        Some(Ok(Token {
            kind: TokenKind::Eof,
            lexeme: String::from(""),
            line: self.line,
        }))
    }
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            // tokens: vec![],
            indent_level: 0,
            start: 0,
            current: 0,
            line: 1,
            token_buffer: VecDeque::new(),
            is_done: false,
        }
    }

    // pub fn scan_tokens(&mut self) {
    //     // let mut tokens = vec![];

    //     while !self.is_at_end() {
    //         self.start = self.current;
            
    //         match self.scan_token() {
    //             Ok(token) => {
    //                 if token.kind != TokenKind::Inert {
    //                     self.tokens.push(token)
    //                 }
    //             },
    //             Err(ScannerError::UnexpectedCharacter(l, c)) => {
    //                 eprintln!("ScannerError::UnexpectedCharacter line {l}: {c}")
    //             },
    //             Err(ScannerError::TooManyIndentations(l, n)) => {
    //                 eprintln!("ScannerError::TooManyIndentationsline {l}: {n} more than previous line")
    //             }
    //         }
    //     }

    //     self.tokens.push(Token {
    //         kind: TokenKind::Eof,
    //         lexeme: String::from(""),
    //         line: self.line,
    //     });        

    //     // self.tokens
    // }

    fn next_token_group(&mut self) -> Result<Option<Vec<Token>>, ScannerError> {
        let c = self.advance().unwrap();

        let kind = match c {
            '\n' => {
                self.line += 1;
                // after newlines we need to consider beginning-of-line whitespace
                // since python uses semantic indentation
                return self.scan_indentation();
            },
            // beginning-of-line indentation is consumed with self.scan_indentation
            // in '\n' pattern, so this is whitespace elsewhere in the line
            ' ' | '\t' | '\r' => return Ok(None),
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
                return Ok(None);
            }
            _ => return Err(ScannerError::UnexpectedCharacter(self.line, c))
        };

        Ok(Some(vec![self.build_token(kind)]))
    }

    // fn scan_token(&mut self) -> Result<Token, ScannerError> {
    //     let c = self.advance().unwrap();

    //     println!("scan_token c = >{c}<");

    //     let kind = match c {
    //         '\n' => {
    //             self.line += 1;
    //             self.scan_indentation()?
    //         },
    //         // beginning-of-line indentation is consumed with self.scan_indentation
    //         // in '\n' pattern, so this is whitespace elsewhere in the line
    //         ' ' | '\t' | '\r' => TokenKind::Inert,
    //         '(' => TokenKind::LeftParen,
    //         ')' => TokenKind::RightParen,
    //         ':' => TokenKind::Colon,
    //         ',' => TokenKind::Comma,
    //         '.' => TokenKind::Def,
    //         '-' => TokenKind::Minus,
    //         '+' => TokenKind::Plus,
    //         '/' => TokenKind::Slash,
    //         '*' => TokenKind::Star,
    //         '!' => {
    //             if self.advance_if_match('=') {
    //                 TokenKind::BangEqual
    //             } else {
    //                 TokenKind::Bang
    //             }
    //         },
    //         '=' => {
    //             if self.advance_if_match('=') {
    //                 TokenKind::EqualEqual
    //             } else {
    //                 TokenKind::Equal
    //             }
    //         },
    //         '<' => {
    //             if self.advance_if_match('=') {
    //                 TokenKind::LessEqual
    //             } else {
    //                 TokenKind::Less
    //             }
    //         },
    //         '>' => {
    //             if self.advance_if_match('=') {
    //                 TokenKind::GreaterEqual
    //             } else {
    //                 TokenKind::Greater
    //             }
    //         },
    //         // a python comment
    //         '#' => {
    //             while self.peek() != Some('\n') && !self.is_at_end() {
    //                 self.advance();
    //             }
    //             TokenKind::Inert
    //         }
    //         _ => return Err(ScannerError::UnexpectedCharacter(self.line, c))
    //     };

    //     Ok(self.build_token(kind))
    // }

    fn scan_indentation(&mut self) -> Result<Option<Vec<Token>>, ScannerError> {
        let mut num_spaces: usize = 0;
        while self.advance_if_match(' ') { num_spaces += 1 }
        let level = num_spaces / 4;

        // println!("num_spaces = {num_spaces}, new level = {}, old level = {}", level, self.indent_level);

        let mut generated_tokens = vec![];

        if level == self.indent_level + 1 {
            generated_tokens.push(self.build_token(TokenKind::Indent));
        } else if level < self.indent_level {
            let num_dedents = self.indent_level - level;
            for _ in 0..num_dedents {
                generated_tokens.push(self.build_token(TokenKind::Dedent));
            }
        } else if level != self.indent_level {
            let how_many = level - self.indent_level;
            return Err(ScannerError::TooManyIndentations(self.line, how_many));
        }
        self.indent_level = level;

        if generated_tokens.is_empty() {
            Ok(None)
        } else {
            Ok(Some(generated_tokens))
        }
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