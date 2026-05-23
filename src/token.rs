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
    Bang,
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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    // literal: idk what Nystrom is using this for yet,
    pub line: usize,
}
