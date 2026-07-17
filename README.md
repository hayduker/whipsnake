# Whipsake: A Python Subset Interpreter

Whipsake is an interpreter for a subset of the Python programming language, implemented in Rust. It features a lexer, a recursive descent parser, and a tree-walking evaluator. Once the interpreter is in a good place, I aim to use WebAssembly as a compile target, and to implemented a simple Wasm runtime along with it to act as the stack-based virtual machine that runs the Wasm bytecode.

## Features

*   **Lexer**: Converts source code into a stream of tokens, handling Python's significant whitespace.
*   **Parser**: Builds an abstract syntax tree (AST) from the token stream using a recursive descent approach.
*   **Evaluator**: Interprets the AST, executing statements and evaluating expressions.
*   **Error Reporting**: Provides detailed error messages for lexical, parsing, and runtime issues.

## Getting Started

### Prerequisites

To build and run Whipsake, you need to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Building the Project

Navigate to the project's root directory and run the following command to build the project:

```bash
cargo build
```

### Running the Interpreter

To run the interpreter in interactive mode:

```bash
cargo run
```

You can then type Python code right into the REPL:

```python
>>> x = 10
>>> print(x + 5)
15
>>> def greet(name):
...     return "Hello, " + name + "!"
...
>>> greet("World")
Hello, World!
```

To run a specific Python:

```bash
cargo run -- <file_path>
```

## Project Structure

*   `src/lexer.rs`: Contains the lexer implementation, responsible for tokenizing the input.
*   `src/parser.rs`: Contains the parser, which constructs the AST from tokens.
*   `src/evaluator.rs`: Implements the tree-walking interpreter that evaluates the AST.
*   `src/token.rs`: Defines the `Token` and `TokenKind` enums, representing the lexical units.
*   `src/ast.rs`: Defines the `Expr` and `Stmt` enums, representing the nodes of the AST.
*   `src/error.rs`: Handles error reporting for lexical, parsing, and runtime errors.
*   `src/environment.rs`: Manages the runtime environment for variable and function bindings.
*   `src/object.rs`: Defines the `Object` enum, representing runtime values.
*   `src/callable.rs`: Defines traits and structs for callable objects (functions).

## License

This project is licensed under the MIT License.
