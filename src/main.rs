use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
};
use whipsnake::{
    error::ErrorReporter,
    lexer::Lexer,
    token::Token
};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: whipsnake [script]");
            return Err("whipsnake not called correctly");
        }
    }

    Ok(())
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim_end().to_string();
        run(input);
    }
}

fn run_file(filename: &str) {
    let source = read_to_string(filename).unwrap();
    run(source);
}

fn run(source: String) {
    let mut reporter = ErrorReporter::new();

    let lexer = Lexer::new(source.as_str(), &mut reporter);
    let tokens: Vec<Token> = lexer.collect();

    if reporter.has_errors() {
        reporter.print_errors();
        return;
    }

    for token in tokens {
        println!("{token:?}");
    }
}

fn _error(line: u32, message: &str) {
    _report(line, "", message);
}

fn _report(line: u32, donde: &str, message: &str) {
    println!("[line {line}] Error{donde}: {message}");
    // had_error = true; // Nystrom sets this in the top-level Lox class in Java
}
