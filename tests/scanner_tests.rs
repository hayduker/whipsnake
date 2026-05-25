use whipsnake::scanner::Scanner;
use whipsnake::token::{Literal, Token, TokenKind};

macro_rules! tok {
    ($kind:ident, $lexeme:expr, $line:expr) => {
        Token::new(TokenKind::$kind, $lexeme, $line)
    };
}

macro_rules! tok_with_literal {
    ($kind:ident, $lexeme:expr, $lit:ident, $line:expr) => {
        Token::with_literal(TokenKind::$kind, $lexeme, Literal::$lit, $line)
    };
}

macro_rules! test_no_errors {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let tokens: Vec<Token> = Scanner::new($input).map(|r| r.unwrap()).collect();

            assert_eq!(tokens, $expected);
        }
    };
}

macro_rules! test_single_char {
    ($name:ident, $input:expr, $kind:ident) => {
        test_no_errors![
            $name,
            $input,
            vec![
                Token::new(TokenKind::$kind, $input, 1),
                Token::new(TokenKind::Eof, "", 1),
            ]
        ];
    };
}

macro_rules! test_suite_no_errors {
    ([ $( ($name:ident, $input:expr, $expected:expr) ),* $(,)? ]) => {
        $(
            #[test]
            fn $name() {
                let tokens: Vec<Token> = Scanner::new($input).map(|r| r.unwrap()).collect();

                assert_eq!(tokens, $expected);
            }
        )*
    };
}

test_no_errors!(
    scan_string,
    "\"hello!\"",
    vec![
        Token::with_literal(
            TokenKind::String,
            "\"hello!\"",
            Literal::String(String::from("hello!")),
            1
        ),
        Token::new(TokenKind::Eof, "", 1),
    ]
);

test_no_errors!(
    scan_empty_string,
    "\"\"",
    vec![
        Token::with_literal(
            TokenKind::String,
            "\"\"",
            Literal::String(String::from("")),
            1
        ),
        Token::new(TokenKind::Eof, "", 1),
    ]
);

test_no_errors!(
    scan_float,
    "3.14159",
    vec![
        Token::with_literal(
            TokenKind::Number,
            "3.14159",
            Literal::Float(3.14159),
            1
        ),
        Token::new(TokenKind::Eof, "", 1),
    ]
);

test_no_errors!(
    scan_big_float,
    "39401.1",
    vec![
        Token::with_literal(
            TokenKind::Number,
            "39401.1",
            Literal::Float(39401.1),
            1
        ),
        Token::new(TokenKind::Eof, "", 1),
    ]
);

test_no_errors!(
    scan_integer,
    "3",
    vec![
        Token::with_literal(
            TokenKind::Number,
            "3",
            Literal::Float(3 as f64),
            1
        ),
        Token::new(TokenKind::Eof, "", 1),
    ]
);


test_single_char!(scan_left_paren, "(", LeftParen);
test_single_char!(scan_right_paren, ")", RightParen);
test_single_char!(scan_colon, ":", Colon);
test_single_char!(scan_comma, ",", Comma);
test_single_char!(scan_dot, ".", Dot);
test_single_char!(scan_minus, "-", Minus);
test_single_char!(scan_plus, "+", Plus);
test_single_char!(scan_slash, "/", Slash);
test_single_char!(scan_star, "*", Star);

test_no_errors!(
    scan_multiple_chars,
    "()!==+-",
    vec![
        tok!(LeftParen, "(", 1),
        tok!(RightParen, ")", 1),
        tok!(BangEqual, "!=", 1),
        tok!(Equal, "=", 1),
        tok!(Plus, "+", 1),
        tok!(Minus, "-", 1),
        tok!(Eof, "", 1),
    ]
);

test_suite_no_errors!([

    (scan_multiple_lines, "+*<>=\n.!=", vec![
        tok!(Plus, "+", 1),
        tok!(Star, "*", 1),
        tok!(Less, "<", 1),
        tok!(GreaterEqual, ">=", 1),
        tok!(Dot, ".", 2),
        tok!(BangEqual, "!=", 2),
        tok!(Eof, "", 2),
    ]),

    (scan_internal_whitespace, "+ *\t<\r>   =\n.!=", vec![
        tok!(Plus, "+", 1),
        tok!(Star, "*", 1),
        tok!(Less, "<", 1),
        tok!(Greater, ">", 1),
        tok!(Equal, "=", 1),
        tok!(Dot, ".", 2),
        tok!(BangEqual, "!=", 2),
        tok!(Eof, "", 2),
    ]),

    (scan_comments, "+*<>=# blah blah blah", vec![
        tok!(Plus, "+", 1),
        tok!(Star, "*", 1),
        tok!(Less, "<", 1),
        tok!(GreaterEqual, ">=", 1),
        tok!(Eof, "", 1),
    ]),

    (scan_indentation, ":\n    :\n        :\n    :\n        :\n:\n:", vec![
        tok!(Colon, ":", 1),
        tok!(Indent, "", 2),
        tok!(Colon, ":", 2),
        tok!(Indent, "", 3),
        tok!(Colon, ":", 3),
        tok!(Dedent, "", 4),
        tok!(Colon, ":", 4),
        tok!(Indent, "", 5),
        tok!(Colon, ":", 5),
        tok!(Dedent, "", 6),
        tok!(Dedent, "", 6),
        tok!(Colon, ":", 6),
        tok!(Colon, ":", 7),
        tok!(Eof, "", 7),
    ]),

    (scan_single_char_identifier, "x", vec![
        tok_with_literal![Identifier, "x", None, 1],
        tok![Eof, "", 1],
    ]),

    (scan_pascal_identifier, "PascalCase", vec![
        tok_with_literal![Identifier, "PascalCase", None, 1],
        tok![Eof, "", 1],
    ]),

    (scan_snake_identifier, "snake_case", vec![
        tok_with_literal![Identifier, "snake_case", None, 1],
        tok![Eof, "", 1],
    ]),

    (scan_alphanum_identifier, "a1_B2_c3_D4", vec![
        tok_with_literal![Identifier, "a1_B2_c3_D4", None, 1],
        tok![Eof, "", 1],
    ]),

    (scan_and, "and", vec![
        tok![And, "and", 1],
        tok![Eof, "", 1],
    ]),

    (scan_class, "class", vec![
        tok![Class, "class", 1],
        tok![Eof, "", 1],
    ]),

    (scan_def, "def", vec![
        tok![Def, "def", 1],
        tok![Eof, "", 1],
    ]),

    (scan_elif, "elif", vec![
        tok![Elif, "elif", 1],
        tok![Eof, "", 1],
    ]),

    (scan_else, "else", vec![
        tok![Else, "else", 1],
        tok![Eof, "", 1],
    ]),

    (scan_false, "False", vec![
        tok![False, "False", 1],
        tok![Eof, "", 1],
    ]),

    (scan_for, "for", vec![
        tok![For, "for", 1],
        tok![Eof, "", 1],
    ]),

    (scan_if, "if", vec![
        tok![If, "if", 1],
        tok![Eof, "", 1],
    ]),

    (scan_none, "None", vec![
        tok![None, "None", 1],
        tok![Eof, "", 1],
    ]),

    (scan_not, "not", vec![
        tok![Not, "not", 1],
        tok![Eof, "", 1],
    ]),

    (scan_or, "or", vec![
        tok![Or, "or", 1],
        tok![Eof, "", 1],
    ]),

    (scan_print, "print", vec![
        tok![Print, "print", 1],
        tok![Eof, "", 1],
    ]),

    (scan_return, "return", vec![
        tok![Return, "return", 1],
        tok![Eof, "", 1],
    ]),

    (scan_super, "super", vec![
        tok![Super, "super", 1],
        tok![Eof, "", 1],
    ]),

    (scan_self, "self", vec![
        tok![This, "self", 1],
        tok![Eof, "", 1],
    ]),

    (scan_true, "True", vec![
        tok![True, "True", 1],
        tok![Eof, "", 1],
    ]),

    (scan_while, "while", vec![
        tok![While, "while", 1],
        tok![Eof, "", 1],
    ]),

]);
