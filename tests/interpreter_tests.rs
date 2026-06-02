use whipsnake::{
    environment::Environment, error::ErrorReporter, evaluator::Evaluator, lexer::Lexer,
    parser::Parser,
    object::Object,
};

macro_rules! test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut reporter = ErrorReporter::new();
            let mut environment = Environment::new_global();

            let source = "\"Hello, world!\"";

            let mut lexer = Lexer::new(&mut reporter);
            let tokens = lexer.lex(source);

            if reporter.has_errors() {
                reporter.print_errors();
                assert!(false);
            }

            let mut parser = Parser::new(&mut reporter);
            let statements = parser.parse(&mut tokens.into_iter().peekable());

            if reporter.has_errors() {
                reporter.print_errors();
                assert!(false);
            }

            let mut evaluator = Evaluator::new(&mut reporter);
            let value = evaluator.interpret(&statements, &mut environment, true);

            if reporter.has_errors() {
                reporter.print_errors();
                assert!(false);
            }

            assert_eq!(value, Some(Object::String("Hello, world!".to_string())));
        }
    };
}

/************/
/* Literals */
/************/

test_case!(
    interpret_string_literal,
    "\"Hello, world!\"",
    Object::String("Hello, world!")
);

test_case!(
    interpret_int_literal,
    "99",
    Object::Int(99)
);

test_case!(
    interpret_float_literal,
    "1.23",
    Object::Flaot(1.23)
);

test_case!(
    interpret_true_literal,
    "True",
    Object::Bool(true)
);

test_case!(
    interpret_false_literal,
    "False",
    Object::Bool(false)
);

/*********************/
/* Logical operators */
/*********************/

test_case!(
    interpret_not_true,
    "not True",
    Object::Bool(false)
);

test_case!(
    interpret_not_false,
    "not False",
    Object::Bool(true)
);

test_case!(
    interpret_not_truthy_int,
    "not 42",
    Object::Bool(false)
);

test_case!(
    interpret_not_falsy_int,
    "not 0",
    Object::Bool(true)
);

test_case!(
    interpret_not_truthy_float,
    "not 1.2",
    Object::Bool(false)
);

test_case!(
    interpret_not_falsy_float,
    "not 0.0",
    Object::Bool(true)
);

test_case!(
    interpret_not_truthy_string,
    r#"not "hi""#,
    Object::Bool(false)
);

test_case!(
    interpret_not_falsy_string,
    r#"not """#,
    Object::Bool(true)
);

/*******************/
/* Unary operators */
/*******************/

test_case!(
    interpret_negative_int,
    "-2",
    Object::Int(-2)
);

test_case!(
    interpret_double_negative_int,
    "--2",
    Object::Int(2)
);

test_case!(
    interpret_negative_float,
    "-1.2",
    Object::Float(-1.2)
);

test_case!(
    interpret_double_negative_float,
    "--1.2",
    Object::Float(1.2)
);