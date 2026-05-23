use std::{env, fs::read_to_string, io::{self, Write}};
use whipsnake::scanner::{Scanner, ScannerError};

fn main() -> Result<(), &'static str>{
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: whipsnake [script]");
            return Err("whipsnake not called correctly")
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
    let scanner = Scanner::new(source.as_str());
    for result in scanner {
        match result {
            Ok(token) => println!("{token:?}"),
            Err(ScannerError::UnexpectedCharacter(l, c)) => {
                eprintln!("ScannerError: unexpected character {c} at line {l}");
            },
            Err(ScannerError::TooManyIndentations(l, n)) => {
                eprintln!("ScannerError: too many indentations at line {l}, got {n} more than previous line");
            },
            Err(ScannerError::UnterminatedString(l)) => {
                eprintln!("ScannerError: unterminated string at line {l}");
            }
        }
    }
}

fn _error(line: u32, message: &str) {
    _report(line, "", message);
}

fn _report(line: u32, donde: &str, message: &str) {
    println!("[line {line}] Error{donde}: {message}");
    // had_error = true; // Nystrom sets this in the top-level Lox class in Java
}

