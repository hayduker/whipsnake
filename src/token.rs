#[derive(Debug)]
enum TokenType {
// single-character tokens
    LeftParen,
    RightParen,
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
}

#[derive(Debug)]
pub struct Token{
    typ: TokenType,
    lexeme: String,
    // literal: idk what Nystrom is using this for yet,
    line: u32,
}
