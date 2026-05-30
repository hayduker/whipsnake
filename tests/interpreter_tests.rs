use whipsnake::{
    environment::{self, Environment}, error::ErrorReporter, evaluator::Evaluator, lexer::Lexer, parser::Parser, token::Token
};

#[test]
fn interpret_string_literal() {
    let mut reporter = ErrorReporter::new();
    let mut environment = Environment::new();

    let source = "\"Hello, world!\"";

    let lexer = Lexer::new(source, &mut reporter);
    let tokens: Vec<Token> = lexer.collect();

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
    evaluator.interpret(&statements, &mut environment);

    if reporter.has_errors() {
        reporter.print_errors();
        assert!(false);
    }


}