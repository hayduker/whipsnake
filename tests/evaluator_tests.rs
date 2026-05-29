use whipsnake::{
    ast::Expr,
    evaluator::Evaluator,
    object::Object,
    token::{Literal, Token, TokenKind},
    error::ErrorReporter,
};

use common::*;

mod common;

macro_rules! test_no_errors {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut error_reporter = ErrorReporter::new();
            let interpreter = Evaluator::new(&mut error_reporter);
            match interpreter.evaluate(&$input) {
                Ok(value) => assert_eq!(value, $expected),
                Err(e) => {
                    eprintln!("RuntimeError: {:?}", e);
                    assert!(false);
                }
            }
        }
    };
}

test_no_errors!(
    evaluate_string_literal,
    Expr::Literal(Literal::String("hey")),
    Object::String(String::from("hey"))
);

test_no_errors!(
    evaluate_float_literal,
    Expr::Literal(Literal::Float(1.23)),
    Object::Float(1.23)
);

test_no_errors!(
    evaluate_true_literal,
    Expr::Literal(Literal::Bool(true)),
    Object::Bool(true)
);

test_no_errors!(
    evaluate_false_literal,
    Expr::Literal(Literal::Bool(false)),
    Object::Bool(false)
);

test_no_errors!(
    evaluate_none_literal,
    Expr::Literal(Literal::None),
    Object::None
);

test_no_errors!(
    evaluate_not_bool,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_none_box()
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_not_truthy_float,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_float_box(1.2)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_not_falsy_float,
    Expr::Unary {
        operator: tok!(Not, "not", 1),
        right: expr_float_box(0.0)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_unary_minus,
    Expr::Unary {
        operator: tok!(Minus, "-", 1),
        right: expr_float_box(3.4)
    },
    Object::Float(-3.4)
);

test_no_errors!(
    evaluate_plus_string,
    Expr::Binary {
        left: expr_string_box("oh"),
        operator: tok!(Plus, "+", 1),
        right: expr_string_box("io")
    },
    Object::String(String::from("ohio"))
);

test_no_errors!(
    evaluate_plus_float,
    Expr::Binary {
        left: expr_float_box(1.2),
        operator: tok!(Plus, "+", 1),
        right: expr_float_box(4.5)
    },
    Object::Float(5.7)
);

test_no_errors!(
    evaluate_minus,
    Expr::Binary {
        left: expr_float_box(4.5),
        operator: tok!(Minus, "-", 1),
        right: expr_float_box(1.2)
    },
    Object::Float(3.3)
);

test_no_errors!(
    evaluate_star,
    Expr::Binary {
        left: expr_float_box(4.5),
        operator: tok!(Star, "*", 1),
        right: expr_float_box(2.0)
    },
    Object::Float(9.0)
);

test_no_errors!(
    evaluate_slash,
    Expr::Binary {
        left: expr_float_box(9.0),
        operator: tok!(Slash, "/", 1),
        right: expr_float_box(2.0)
    },
    Object::Float(4.5)
);

test_no_errors!(
    evaluate_greater,
    Expr::Binary {
        left: expr_float_box(3.4),
        operator: tok!(Greater, ">", 1),
        right: expr_float_box(7.2)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_greater_equal_0,
    Expr::Binary {
        left: expr_float_box(3.1),
        operator: tok!(GreaterEqual, ">=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_greater_equal_1,
    Expr::Binary {
        left: expr_float_box(3.2),
        operator: tok!(GreaterEqual, ">=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_less,
    Expr::Binary {
        left: expr_float_box(9.1),
        operator: tok!(Less, "<", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_less_equal_0,
    Expr::Binary {
        left: expr_float_box(3.0),
        operator: tok!(LessEqual, "<=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_less_equal_1,
    Expr::Binary {
        left: expr_float_box(3.1),
        operator: tok!(LessEqual, "<=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_equal_equal_same_type,
    Expr::Binary {
        left: expr_float_box(3.1),
        operator: tok!(EqualEqual, "==", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(true)
);

test_no_errors!(
    evaluate_equal_equal_different_type,
    Expr::Binary {
        left: expr_float_box(3.0),
        operator: tok!(EqualEqual, "==", 1),
        right: expr_string_box("hi")
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_bang_equal_same_type,
    Expr::Binary {
        left: expr_float_box(3.1),
        operator: tok!(BangEqual, "!=", 1),
        right: expr_float_box(3.1)
    },
    Object::Bool(false)
);

test_no_errors!(
    evaluate_bang_equal_different_type,
    Expr::Binary {
        left: expr_float_box(3.0),
        operator: tok!(BangEqual, "!=", 1),
        right: expr_string_box("hi")
    },
    Object::Bool(true)
);