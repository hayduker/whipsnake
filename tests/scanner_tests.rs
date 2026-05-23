use whipsnake::scanner::Scanner;
use whipsnake::token::{Token, TokenKind};

macro_rules! test_no_errors {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let tokens: Vec<Token> = Scanner::new($input)
                .map(|r| r.unwrap())
                .collect();

            assert_eq!(tokens, $expected);
        }
    };
}

macro_rules! test_single_char {
    ($name:ident, $input:expr, $kind:expr) => {
        #[test]
        fn $name() {
            let tokens: Vec<Token> = Scanner::new($input)
                .map(|r| r.unwrap())
                .collect();

            assert_eq!(
                tokens,
                vec![
                    Token::new($kind, String::from($input), 1),
                    Token::new(TokenKind::Eof, String::from(""), 1),
                ]
            );
        }
    };
}

test_single_char!(scan_left_paren, "(", TokenKind::LeftParen);
test_single_char!(scan_right_paren, ")", TokenKind::RightParen);
test_single_char!(scan_colon, ":", TokenKind::Colon);
test_single_char!(scan_comma, ",", TokenKind::Comma);
test_single_char!(scan_dot, ".", TokenKind::Dot);
test_single_char!(scan_minus, "-", TokenKind::Minus);
test_single_char!(scan_plus, "+", TokenKind::Plus);
test_single_char!(scan_slash, "/", TokenKind::Slash);
test_single_char!(scan_star, "*", TokenKind::Star);

test_no_errors!(
    scan_multiple_chars,
    "()!==+-",
    vec![
        Token::new(TokenKind::LeftParen, String::from("("), 1),
        Token::new(TokenKind::RightParen, String::from(")"), 1),
        Token::new(TokenKind::BangEqual, String::from("!="), 1),
        Token::new(TokenKind::Equal, String::from("="), 1),
        Token::new(TokenKind::Plus, String::from("+"), 1),
        Token::new(TokenKind::Minus, String::from("-"), 1),
        Token::new(TokenKind::Eof, String::from(""), 1),
    ]
);
