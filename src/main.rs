use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
};
use whipsnake::{
    environment::Environment, error::ErrorReporter, evaluator::Evaluator, lexer::Lexer, parser::Parser, printer::PrettyPrinter, token::Token
};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: whipsnake [script]");
            return Err("whipsnake not called correctly");
        }
    }

    Ok(())
}

fn run_repl() {
    let mut input = String::new();
    let mut environment = Environment::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut reporter = ErrorReporter::new();
        let mut lexer = Lexer::new(&mut reporter);
        let tokens: Vec<Token> = lexer.lex(input.as_str());

        println!("Tokens:");
        for token in tokens.clone() {
            println!("{token:?}");
        }

        if reporter.has_errors() {
            reporter.print_errors();
            continue;
        }

        let mut parser = Parser::new(&mut reporter);
        let statements = parser.parse(&mut tokens.into_iter().peekable());

        println!("\nSyntax tree:");
        println!("{}", PrettyPrinter::print(&statements));

        println!("Value:");
        let mut evaluator = Evaluator::new(&mut reporter);
        evaluator.interpret(&statements, &mut environment, true);

        if reporter.has_errors() {
            reporter.print_errors();
            return;
        }

        reporter.clear();
    }
}

fn run_file(filename: &str) {
    let source = read_to_string(filename).unwrap();

    let mut reporter = ErrorReporter::new();
    let mut environment = Environment::new();

    let mut lexer = Lexer::new(&mut reporter);
    let tokens: Vec<Token> = lexer.lex(source.as_str());

    println!("Tokens:");
    for token in tokens.clone() {
        println!("{token:?}");
    }

    if reporter.has_errors() {
        reporter.print_errors();
        return;
    }

    let mut parser = Parser::new(&mut reporter);
    let statements = parser.parse(&mut tokens.into_iter().peekable());

    println!("\nSyntax tree:");
    println!("{}", PrettyPrinter::print(&statements));

    let mut evaluator = Evaluator::new(&mut reporter);
    evaluator.interpret(&statements, &mut environment, false);

    if reporter.has_errors() {
        reporter.print_errors();
        return;
    }
}

fn _error(line: u32, message: &str) {
    _report(line, "", message);
}

fn _report(line: u32, donde: &str, message: &str) {
    println!("[line {line}] Error{donde}: {message}");
    // had_error = true; // Nystrom sets this in the top-level Lox class in Java
}
