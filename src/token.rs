#[derive(Debug, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // single-character tokens
    LeftParen,
    RightParen,
    Colon,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Tilde,

    // one or two character tokens
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // literals
    Identifier,
    String,
    Int,
    Float,

    // keywords
    And,
    Class,
    Def,
    Elif,
    Else,
    False,
    For,
    If,
    In,
    Is,
    IsNot,
    None,
    Not,
    Or,
    Return,
    Super,
    This,
    True,
    While,

    // python uses whitespace, I guess
    Indent,
    Dedent,
    NewLine,

    // makes the parser cleaner
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    None,
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, line: usize) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            literal: None,
            line,
        }
    }

    pub fn with_literal(kind: TokenKind, lexeme: &str, literal: Literal, line: usize) -> Token {
        Self {
            kind,
            lexeme: lexeme.into(),
            literal: Some(literal),
            line,
        }
    }
}
