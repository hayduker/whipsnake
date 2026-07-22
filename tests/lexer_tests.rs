use whipsnake::error::ErrorReporter;
use whipsnake::lexer::Lexer;
use whipsnake::token::{Token, TokenKind};

mod common;

use common::*;

macro_rules! test_no_errors {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut reporter = ErrorReporter::new();
            let mut lexer = Lexer::new(&mut reporter);
            let tokens: Vec<Token> = lexer.lex($input);

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
                tok![NewLine, "\n", 1],
                tok![Eof, "", 2],
            ]
        ];
    };
}

macro_rules! test_suite_no_errors {
    ([ $( ($name:ident, $input:expr, $expected:expr) ),* $(,)? ]) => {
        $(
            #[test]
            fn $name() {
                let mut reporter = ErrorReporter::new();
                let mut lexer = Lexer::new(&mut reporter);
                let tokens: Vec<Token> = lexer.lex($input);

                assert_eq!(tokens, $expected);
            }
        )*
    };
}

test_no_errors!(
    lex_string,
    "\"hello!\"",
    vec![
        tok_string("hello!", 1),
        tok![NewLine, "\n", 1],
        tok![Eof, "", 2],
    ]
);

test_no_errors!(
    lex_empty_string,
    "\"\"",
    vec![tok_string("", 1), tok![NewLine, "\n", 1], tok![Eof, "", 2],]
);

test_no_errors!(
    lex_single_quotedstring,
    "'hello!'",
    vec![
        tok_sq_string("hello!", 1),
        tok![NewLine, "\n", 1],
        tok![Eof, "", 2],
    ]
);

test_no_errors!(
    lex_empty_single_quoted_string,
    "''",
    vec![
        tok_sq_string("", 1),
        tok![NewLine, "\n", 1],
        tok![Eof, "", 2],
    ]
);

test_no_errors!(
    lex_float,
    "1.2345",
    vec![
        tok_float(1.2345, 1),
        tok![NewLine, "\n", 1],
        tok![Eof, "", 2],
    ]
);

test_no_errors!(
    lex_big_float,
    "39401.1",
    vec![
        tok_float(39401.1, 1),
        tok![NewLine, "\n", 1],
        tok![Eof, "", 2],
    ]
);

test_no_errors!(
    lex_integer,
    "3",
    vec![tok_int(3, 1), tok![NewLine, "\n", 1], tok![Eof, "", 2],]
);

test_single_char!(lex_left_paren, "(", LeftParen);
test_single_char!(lex_right_paren, ")", RightParen);
test_single_char!(lex_colon, ":", Colon);
test_single_char!(lex_comma, ",", Comma);
test_single_char!(lex_dot, ".", Dot);
test_single_char!(lex_minus, "-", Minus);
test_single_char!(lex_plus, "+", Plus);
test_single_char!(lex_slash, "/", Slash);
test_single_char!(lex_star, "*", Star);
test_single_char!(lex_tilde, "~", Tilde);

test_no_errors!(
    lex_multiple_chars,
    "()!==+-",
    vec![
        tok!(LeftParen, "(", 1),
        tok!(RightParen, ")", 1),
        tok!(BangEqual, "!=", 1),
        tok!(Equal, "=", 1),
        tok!(Plus, "+", 1),
        tok!(Minus, "-", 1),
        tok![NewLine, "\n", 1],
        tok!(Eof, "", 2),
    ]
);

test_no_errors!(
    lex_plus_equal,
    "+=",
    vec![
        tok!(PlusEqual, "+=", 1),
        tok![NewLine, "\n", 1],
        tok!(Eof, "", 2),
    ]
);

test_no_errors!(
    lex_minus_equal,
    "-=",
    vec![
        tok!(MinusEqual, "-=", 1),
        tok![NewLine, "\n", 1],
        tok!(Eof, "", 2),
    ]
);

test_no_errors!(
    lex_star_equal,
    "*=",
    vec![
        tok!(StarEqual, "*=", 1),
        tok![NewLine, "\n", 1],
        tok!(Eof, "", 2),
    ]
);

test_no_errors!(
    lex_slash_equal,
    "/=",
    vec![
        tok!(SlashEqual, "/=", 1),
        tok![NewLine, "\n", 1],
        tok!(Eof, "", 2),
    ]
);

test_suite_no_errors!([
    (
        lex_multiple_lines,
        "+*<>=\n.!=",
        vec![
            tok!(Plus, "+", 1),
            tok!(Star, "*", 1),
            tok!(Less, "<", 1),
            tok!(GreaterEqual, ">=", 1),
            tok!(NewLine, "\n", 1),
            tok!(Dot, ".", 2),
            tok!(BangEqual, "!=", 2),
            tok![NewLine, "\n", 2],
            tok!(Eof, "", 3),
        ]
    ),
    (
        lex_internal_whitespace,
        "+ *\t<\r>   =\n.!=",
        vec![
            tok!(Plus, "+", 1),
            tok!(Star, "*", 1),
            tok!(Less, "<", 1),
            tok!(Greater, ">", 1),
            tok!(Equal, "=", 1),
            tok!(NewLine, "\n", 1),
            tok!(Dot, ".", 2),
            tok!(BangEqual, "!=", 2),
            tok![NewLine, "\n", 2],
            tok!(Eof, "", 3),
        ]
    ),
    (
        lex_comments,
        "+*<>=# blah blah blah",
        vec![
            tok!(Plus, "+", 1),
            tok!(Star, "*", 1),
            tok!(Less, "<", 1),
            tok!(GreaterEqual, ">=", 1),
            tok![NewLine, "\n", 1],
            tok!(Eof, "", 2),
        ]
    ),
    (
        lex_4_space_indentation,
        ":\n    :\n        :\n    :\n        :\n:\n:",
        vec![
            tok!(Colon, ":", 1),
            tok!(NewLine, "\n", 1),
            tok!(Indent, "", 2),
            tok!(Colon, ":", 2),
            tok!(NewLine, "\n", 2),
            tok!(Indent, "", 3),
            tok!(Colon, ":", 3),
            tok!(NewLine, "\n", 3),
            tok!(Dedent, "", 4),
            tok!(Colon, ":", 4),
            tok!(NewLine, "\n", 4),
            tok!(Indent, "", 5),
            tok!(Colon, ":", 5),
            tok!(NewLine, "\n", 5),
            tok!(Dedent, "", 6),
            tok!(Dedent, "", 6),
            tok!(Colon, ":", 6),
            tok!(NewLine, "\n", 6),
            tok!(Colon, ":", 7),
            tok![NewLine, "\n", 7],
            tok!(Eof, "", 8),
        ]
    ),
    (
        lex_variable_space_indentation_0,
        ":\n  :\n         :\n  :\n         :\n:\n:",
        vec![
            tok!(Colon, ":", 1),
            tok!(NewLine, "\n", 1),
            tok!(Indent, "", 2),
            tok!(Colon, ":", 2),
            tok!(NewLine, "\n", 2),
            tok!(Indent, "", 3),
            tok!(Colon, ":", 3),
            tok!(NewLine, "\n", 3),
            tok!(Dedent, "", 4),
            tok!(Colon, ":", 4),
            tok!(NewLine, "\n", 4),
            tok!(Indent, "", 5),
            tok!(Colon, ":", 5),
            tok!(NewLine, "\n", 5),
            tok!(Dedent, "", 6),
            tok!(Dedent, "", 6),
            tok!(Colon, ":", 6),
            tok!(NewLine, "\n", 6),
            tok!(Colon, ":", 7),
            tok![NewLine, "\n", 7],
            tok!(Eof, "", 8),
        ]
    ),
    (
        lex_variable_space_indentation_1,
        "if 1 < 2:\n  if False:\n       print(1)\n  elif True:\n       print(2)\n  else:\n       print(3)\nelse:\n     print(4)",
        vec![
            tok!(If, "if", 1),
            tok_int(1, 1),
            tok!(Less, "<", 1),
            tok_int(2, 1),
            tok!(Colon, ":", 1),
            tok!(NewLine, "\n", 1),
            tok!(Indent, "", 2),
            tok!(If, "if", 2),
            tok!(False, "False", 2),
            tok!(Colon, ":", 2),
            tok!(NewLine, "\n", 2),
            tok!(Indent, "", 3),
            tok!(Identifier, "print", 3),
            tok!(LeftParen, "(", 3),
            tok_int(1, 3),
            tok!(RightParen, ")", 3),
            tok!(NewLine, "\n", 3),
            tok!(Dedent, "", 4),
            tok!(Elif, "elif", 4),
            tok!(True, "True", 4),
            tok!(Colon, ":", 4),
            tok!(NewLine, "\n", 4),
            tok!(Indent, "", 5),
            tok!(Identifier, "print", 5),
            tok!(LeftParen, "(", 5),
            tok_int(2, 5),
            tok!(RightParen, ")", 5),
            tok!(NewLine, "\n", 5),
            tok!(Dedent, "", 6),
            tok!(Else, "else", 6),
            tok!(Colon, ":", 6),
            tok!(NewLine, "\n", 6),
            tok!(Indent, "", 7),
            tok!(Identifier, "print", 7),
            tok!(LeftParen, "(", 7),
            tok_int(3, 7),
            tok!(RightParen, ")", 7),
            tok!(NewLine, "\n", 7),
            tok!(Dedent, "", 8),
            tok!(Dedent, "", 8),
            tok!(Else, "else", 8),
            tok!(Colon, ":", 8),
            tok!(NewLine, "\n", 8),
            tok!(Indent, "", 9),
            tok!(Identifier, "print", 9),
            tok!(LeftParen, "(", 9),
            tok_int(4, 9),
            tok!(RightParen, ")", 9),
            tok!(NewLine, "\n", 9),
            tok!(Dedent, "", 10),
            tok!(Eof, "", 10),
        ]
    ),
    (
        lex_tab_indentation,
        ":\n\t:\n\t\t:\n\t:\n\t\t:\n:\n:",
        vec![
            tok!(Colon, ":", 1),
            tok!(NewLine, "\n", 1),
            tok!(Indent, "", 2),
            tok!(Colon, ":", 2),
            tok!(NewLine, "\n", 2),
            tok!(Indent, "", 3),
            tok!(Colon, ":", 3),
            tok!(NewLine, "\n", 3),
            tok!(Dedent, "", 4),
            tok!(Colon, ":", 4),
            tok!(NewLine, "\n", 4),
            tok!(Indent, "", 5),
            tok!(Colon, ":", 5),
            tok!(NewLine, "\n", 5),
            tok!(Dedent, "", 6),
            tok!(Dedent, "", 6),
            tok!(Colon, ":", 6),
            tok!(NewLine, "\n", 6),
            tok!(Colon, ":", 7),
            tok![NewLine, "\n", 7],
            tok!(Eof, "", 8),
        ]
    ),
    (
        lex_implicit_ending_dedents,
        ":\n    :\n        :\n            :",
        vec![
            tok!(Colon, ":", 1),
            tok!(NewLine, "\n", 1),
            tok!(Indent, "", 2),
            tok!(Colon, ":", 2),
            tok!(NewLine, "\n", 2),
            tok!(Indent, "", 3),
            tok!(Colon, ":", 3),
            tok!(NewLine, "\n", 3),
            tok!(Indent, "", 4),
            tok!(Colon, ":", 4),
            tok!(NewLine, "\n", 4),
            tok!(Dedent, "", 5),
            tok!(Dedent, "", 5),
            tok!(Dedent, "", 5),
            tok!(Eof, "", 5),
        ]
    ),
    (
        lex_single_char_identifier,
        "x",
        vec![
            tok!(Identifier, "x", 1),
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_pascal_identifier,
        "PascalCase",
        vec![
            tok!(Identifier, "PascalCase", 1),
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_snake_identifier,
        "snake_case",
        vec![
            tok!(Identifier, "snake_case", 1),
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_alphanum_identifier,
        "a1_B2_c3_D4",
        vec![
            tok!(Identifier, "a1_B2_c3_D4", 1),
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_and,
        "and",
        vec![
            tok![And, "and", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_class,
        "class",
        vec![
            tok![Class, "class", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_def,
        "def",
        vec![
            tok![Def, "def", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_elif,
        "elif",
        vec![
            tok![Elif, "elif", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_else,
        "else",
        vec![
            tok![Else, "else", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_false,
        "False",
        vec![
            tok![False, "False", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_for,
        "for",
        vec![
            tok![For, "for", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_if,
        "if",
        vec![tok![If, "if", 1], tok![NewLine, "\n", 1], tok![Eof, "", 2],]
    ),
    (
        lex_in,
        "in",
        vec![tok![In, "in", 1], tok![NewLine, "\n", 1], tok![Eof, "", 2],]
    ),
    (
        lex_is,
        "is",
        vec![tok![Is, "is", 1], tok![NewLine, "\n", 1], tok![Eof, "", 2],]
    ),
    (
        lex_none,
        "None",
        vec![
            tok![None, "None", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_not,
        "not",
        vec![
            tok![Not, "not", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_or,
        "or",
        vec![tok![Or, "or", 1], tok![NewLine, "\n", 1], tok![Eof, "", 2],]
    ),
    (
        lex_pass,
        "pass",
        vec![
            tok![Pass, "pass", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_return,
        "return",
        vec![
            tok![Return, "return", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_super,
        "super",
        vec![
            tok![Super, "super", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_self,
        "self",
        vec![
            tok![This, "self", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_true,
        "True",
        vec![
            tok![True, "True", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
    (
        lex_while,
        "while",
        vec![
            tok![While, "while", 1],
            tok![NewLine, "\n", 1],
            tok![Eof, "", 2],
        ]
    ),
]);
