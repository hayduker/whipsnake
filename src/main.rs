use clap::Parser as ClapParser;
use std::{
    fs::read_to_string,
    io::{self, Write},
    path::PathBuf,
};
use whipsnake::{
    environment::Environment, error::ErrorReporter, evaluator::Evaluator, lexer::Lexer,
    parser::Parser, printer::print_ast, token::Token,
};

#[derive(ClapParser, Debug)]
#[command(author, version, about = "An interpreter for the Whipsnake language", long_about = None)]
struct Args {
    /// The script file to run. If omitted, starts the REPL.
    script: Option<PathBuf>,

    /// Show verbose debug output (tokens, AST, etc.)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    match args.script {
        Some(path) => {
            let filename = path.to_str().ok_or("Invalid filename provided")?;
            run_file(filename, args.verbose);
        }
        None => run_repl(args.verbose),
    }

    Ok(())
}

fn run_repl(verbose: bool) {
    let mut input = String::new();
    let mut environment = Environment::new_global();

    loop {
        input.clear();

        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if next_to_last_equals(&input, ':') {
            let mut last_line = String::new();

            while last_line != "\n" {
                last_line.clear();

                print!("... ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut last_line)
                    .expect("Failed to read line");

                input.push_str(last_line.as_str());
            }
        }

        if verbose {
            println!("Got input: >{input}<");
        }

        let mut reporter = ErrorReporter::new();
        let mut lexer = Lexer::new(&mut reporter);
        let tokens: Vec<Token> = lexer.lex(input.as_str());

        if verbose {
            println!("Tokens:");
            for token in tokens.clone() {
                println!("{token:?}");
            }
        }

        if reporter.has_errors() {
            reporter.print_errors();
            continue;
        }

        let mut parser = Parser::new(&mut reporter);
        let statements = parser.parse(&mut tokens.into_iter().peekable());

        if verbose {
            println!("\nSyntax tree:");
            println!("{}", print_ast(&statements));
        }

        if verbose {
            println!("\nValue:");
        }

        let mut evaluator = Evaluator::new(&mut reporter);
        evaluator.interpret(&statements, &mut environment, true);

        if reporter.has_errors() {
            reporter.print_errors();
            continue;
        }

        reporter.clear();
    }
}

fn run_file(filename: &str, verbose: bool) {
    let source = read_to_string(filename).unwrap();

    if verbose {
        println!("Input:");
        println!(">{source}<");
    }

    let mut reporter = ErrorReporter::new();
    let mut environment = Environment::new_global();

    let mut lexer = Lexer::new(&mut reporter);
    let tokens: Vec<Token> = lexer.lex(source.as_str());

    if verbose {
        println!("Tokens:");
        for token in tokens.clone() {
            println!("{token:?}");
        }
    }

    if reporter.has_errors() {
        reporter.print_errors();
        return;
    }

    let mut parser = Parser::new(&mut reporter);
    let statements = parser.parse(&mut tokens.into_iter().peekable());

    if verbose {
        println!("\nSyntax tree:");
        println!("{}", print_ast(&statements));
    }

    let mut evaluator = Evaluator::new(&mut reporter);
    evaluator.interpret(&statements, &mut environment, false);

    if reporter.has_errors() {
        reporter.print_errors();
    }
}

fn next_to_last_equals(s: &str, target: char) -> bool {
    let mut rev_chars = s.chars().rev();
    rev_chars.next();
    rev_chars.next() == Some(target)
}

fn _error(line: u32, message: &str) {
    _report(line, "", message);
}

fn _report(line: u32, donde: &str, message: &str) {
    println!("[line {line}] Error{donde}: {message}");
    // had_error = true; // Nystrom sets this in the top-level Lox class in Java
}
