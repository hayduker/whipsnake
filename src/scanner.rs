use std::collections::VecDeque;

use crate::token::{TokenKind, Token, Literal};

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacter(usize, char),
    UnterminatedString(usize),
    TooManyIndentations(usize, usize),
}

pub struct Scanner {
    source: Vec<char>,
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
            literal: Literal::None,
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
            '.' => TokenKind::Dot,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Star,
            '!' => {
                if self.advance_if_match('=') {
                    TokenKind::BangEqual
                } else {
                    return Err(ScannerError::UnexpectedCharacter(self.line, c))
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
            // '"' => {
            //     while self.peek() != Some('"') {
            //         if self.peek() == '\n' || self.is_at_end() {
            //             return Err(ScannerError::UnterminatedString((self.line)));
            //         }
            //         self.advance();
            //     }

            //     self.advance(); // eat the closing "
                
            //     let value = self.source.get(self.start..self.current);
            //     return self.build_token(TokenKind::String, value);
            // }
            _ => return Err(ScannerError::UnexpectedCharacter(self.line, c))
        };

        Ok(Some(vec![Token::new(kind, self.current_lexeme(), self.line)]))
    }

    fn scan_indentation(&mut self) -> Result<Option<Vec<Token>>, ScannerError> {
        let mut num_spaces: usize = 0;
        while self.advance_if_match(' ') { num_spaces += 1 }
        let level = num_spaces / 4;

        // println!("num_spaces = {num_spaces}, new level = {}, old level = {}", level, self.indent_level);

        let mut generated_tokens = vec![];

        if level == self.indent_level + 1 {
            generated_tokens.push(Token::new(
                TokenKind::Indent,
                String::from(""),
                self.line
            ));
        } else if level < self.indent_level {
            let num_dedents = self.indent_level - level;
            for _ in 0..num_dedents {
                generated_tokens.push(Token::new(
                    TokenKind::Dedent,
                    String::from(""),
                    self.line
                ));
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

    fn current_lexeme(&self) -> String {
        self.source.get(self.start..self.current).unwrap().iter().collect()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
