#[derive(Debug, PartialEq, Eq)]
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

    // one or two character tokens
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Def,
    Elif,
    Else,
    False,
    For,
    If,
    None,
    Not,
    Or,
    Print,
    Return,
    Selph,
    Super,
    True,
    While,

    // python uses whitespace, I guess
    Indent,
    Dedent,

    // makes the parser cleaner
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    None,
    String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self::with_literal(kind, lexeme, Literal::None, line)
    }

    pub fn with_literal(kind: TokenKind, lexeme: String, literal: Literal, line: usize) -> Token {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}
