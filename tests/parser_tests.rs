use whipsnake::{
    token::{Token, TokenKind},
    parser::Parser,
    error::ErrorReporter,
    ast::{Stmt, Expr},
};

use common::*;

mod common;

macro_rules! test_no_errors {
    ($name:ident, $tokens:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut reporter = ErrorReporter::new();
            let mut parser = Parser::new(&mut reporter);

            let mut peekable = $tokens.into_iter().peekable();
            let stmts = parser.parse(&mut peekable);

            assert_eq!($expected, stmts);
        }
    };
}

macro_rules! test_binary_numeric_no_errors {
    ($name:ident, $kind:ident, $lexeme:expr) => {
        test_no_errors!(
            $name,
            vec![
                tok_float(1.234, 1),
                tok!($kind, $lexeme, 1),
                tok_float(9.876, 1),
                tok![Eof, "", 1],
            ],
            vec![Stmt::Expression(Expr::Binary {
                left: expr_float_box(1.234),
                operator: tok!($kind, $lexeme, 1),
                right: expr_float_box(9.876)
            })]
        );
    };
}

test_no_errors!(
    parse_string_literal,
    vec![
        tok_string("hi", 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(expr_string("hi"))]
);

test_no_errors!(
    parse_float_literal,
    vec![
        tok_float(1.234, 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(expr_float(1.234))]
);

test_no_errors!(
    parse_true_literal,
    vec![
        tok_true(1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(expr_true())]
);

test_no_errors!(
    parse_false_literal,
    vec![
        tok_false(1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(expr_false())]
);

test_no_errors!(
    parse_none_literal,
    vec![
        tok!(None, "None", 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(expr_none())]
);

test_no_errors!(
    parse_simple_grouping,
    vec![
        tok!(LeftParen, "(", 1),
        tok!(None, "None", 1),
        tok!(RightParen, ")", 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(Expr::Grouping(
        expr_none_box()
    ))]
);

test_no_errors!(
    parse_not_unary,
    vec![
        tok!(Not, "not", 1),
        tok_true(1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_true_box()
    })]
);

test_no_errors!(
    parse_minus_unary,
    vec![
        tok!(Minus, "-", 1),
        tok_float(1.234, 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(Expr::Unary {
        operator: tok!(Minus, "-", 1),
        right: expr_float_box(1.234)
    })]
);

test_binary_numeric_no_errors!(parse_star_factor, Star, "*");
test_binary_numeric_no_errors!(parse_slash_factor, Slash, "/");
test_binary_numeric_no_errors!(parse_plus_term, Plus, "+");
test_binary_numeric_no_errors!(parse_minus_term, Minus, "-");
test_binary_numeric_no_errors!(parse_greater_equality, Greater, ">");
test_binary_numeric_no_errors!(parse_greater_equal_equality, GreaterEqual, ">=");
test_binary_numeric_no_errors!(parse_less_equality, Less, "<");
test_binary_numeric_no_errors!(parse_less_equal_equality, LessEqual, "<=");
test_binary_numeric_no_errors!(parse_bang_equal_equality, BangEqual, "!=");
test_binary_numeric_no_errors!(parse_equal_equal_equality, EqualEqual, "==");

test_no_errors!(
    parse_precedence_0,
    vec![
        tok_float(1.2, 1),
        tok!(Star, "*", 1),
        tok_float(2.3, 1),
        tok!(Plus, "+", 1),
        tok_float(3.4, 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(Expr::Binary {
        left: Box::new(Expr::Binary {
            left: expr_float_box(1.2),
            operator: tok!(Star, "*", 1),
            right: expr_float_box(2.3)
        }),
        operator: tok!(Plus, "+", 1),
        right: expr_float_box(3.4)
    })]
);

test_no_errors!(
    parse_precedence_1,
    vec![
        tok_float(1.2, 1),
        tok!(Plus, "+", 1),
        tok_float(2.3, 1),
        tok!(Star, "*", 1),
        tok_float(3.4, 1),
        tok![Eof, "", 1],
    ],
    vec![Stmt::Expression(Expr::Binary {
        left: expr_float_box(1.2),
        operator: tok!(Plus, "+", 1),
        right: Box::new(Expr::Binary {
            left: expr_float_box(2.3),
            operator: tok!(Star, "*", 1),
            right: expr_float_box(3.4)
        })
    })]
);