use std::{env, fs::read_to_string, io::{self, Write}};

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
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

struct Scanner {}

impl Scanner {
    fn new(_source: String) -> Scanner {
        Scanner {}  
    }

    fn scan_tokens(&self) -> Vec<Token> {
        vec!()
    }
}

#[derive(Debug)]
struct Token {}


fn _error(line: u32, message: &str) {
    _report(line, "", message);
}

fn _report(line: u32, donde: &str, message: &str) {
    println!("[line {line}] Error{donde}: {message}");
    // had_error = true; // Nystrom sets this in the top-level Lox class in Java
}