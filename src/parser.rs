//! The `parser` module is responsible for parsing a stream of `Token`s into an abstract Syntax Tree (AST).
//! It implements a recursive descent parser to construct `Expr` and `Stmt` nodes based on the grammar
//! rules of the Python subset.

use crate::{
    ast::{Expr, Stmt},
    error::{ErrorReporter, ParseError},
    token::{Literal, SourceLocation, Token, TokenKind},
};

use std::iter::Peekable;

/// The `Parser` struct is responsible for transforming a peekable iterator of `Token`s
/// into a vector of `Stmt` (statement) nodes, representing the abstract syntax tree (AST).
/// It uses a recursive descent parsing strategy and reports any parsing errors encountered
/// via an `ErrorReporter`.
pub struct Parser<'err> {
    previous: Option<Token>,
    error_reporter: &'err mut ErrorReporter,
}

impl<'err> Parser<'err> {
    /// Creates a new `Parser` instance.
    ///
    /// Initializes the parser with an `ErrorReporter` to handle and report any syntax errors
    /// during the parsing process.
    ///
    /// # Arguments
    ///
    /// * `error_reporter` - A mutable reference to an `ErrorReporter` for error handling.
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Self {
        Parser {
            previous: None,
            error_reporter,
        }
    }

    /// Parses the given stream of `Token`s into a vector of `Stmt`.
    ///
    /// This is the entry point for the parsing process. It consumes tokens from the iterator
    /// and attempts to build a list of statements that form the program's AST.
    ///
    /// # Type Parameters
    ///
    /// * `I` - An iterator that yields `Token`s.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A mutable, peekable iterator over the input `Token`s.
    ///
    /// # Returns
    ///
    /// A `Vec<Stmt>` representing the parsed Abstract Syntax Tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use whipsnake::{error::ErrorReporter, lexer::Lexer, parser::Parser, ast::{Stmt, Expr, self}, token::{Token, TokenKind, SourceLocation, Literal}};
    ///
    /// let mut error_reporter = ErrorReporter::new();
    /// let mut lexer = Lexer::new(&mut error_reporter);
    /// let tokens = lexer.lex("x = 1 + 2");
    /// let mut token_iter = tokens.into_iter().peekable();
    /// let mut parser = Parser::new(&mut error_reporter);
    /// let statements = parser.parse(&mut token_iter);
    ///
    /// assert_eq!(statements.len(), 1);
    /// match &statements[0] {
    ///     Stmt::Assignment { name, initializer } => {
    ///         assert_eq!(name.kind, TokenKind::Identifier);
    ///         assert_eq!(name.lexeme, "x");
    ///         match initializer {
    ///             Expr::Binary { left, operator, right } => {
    ///                 assert_eq!(**left, Expr::Literal(Literal::Int(1)));
    ///                 assert_eq!(operator.kind, TokenKind::Plus);
    ///                 assert_eq!(**right, Expr::Literal(Literal::Int(2)));
    ///             },
    ///             _ => panic!("Expected binary expression"),
    ///         }
    ///     },
    ///     _ => panic!("Expected assignment statement"),
    /// }
    /// ```
    ///
    /// ```
    /// use whipsnake::{error::ErrorReporter, lexer::Lexer, parser::Parser, ast::{Stmt, Expr, self}, token::{Token, TokenKind, SourceLocation, Literal}};
    ///
    /// let mut error_reporter = ErrorReporter::new();
    /// let mut lexer = Lexer::new(&mut error_reporter);
    /// let tokens = lexer.lex("if True:\n    x = 1");
    /// let mut token_iter = tokens.into_iter().peekable();
    /// let mut parser = Parser::new(&mut error_reporter);
    /// let statements = parser.parse(&mut token_iter);
    ///
    /// assert_eq!(statements.len(), 1);
    /// if let Stmt::If { condition, then_body, else_body: _ } = &statements[0] {
    ///     assert_eq!(*condition, Expr::Literal(Literal::Bool(true)));
    ///     if let Stmt::Block(block_stmts) = &**then_body {
    ///         assert_eq!(block_stmts.len(), 1);
    ///     } else {
    ///         panic!("Expected a block statement");
    ///     }
    /// } else {
    ///     panic!("Expected an if statement");
    /// }
    /// ```
    pub fn parse<I>(&mut self, tokens: &mut Peekable<I>) -> Vec<Stmt>
    where
        I: Iterator<Item = Token>,
    {
        let mut all_statements = Vec::new();

        while !self.peek_matches(tokens, TokenKind::Eof) {
            let mut statements = self.statements(tokens);
            if statements.is_empty() {
                break;
            }

            all_statements.append(&mut statements);
        }

        all_statements
    }

    fn statements<I>(&mut self, tokens: &mut Peekable<I>) -> Vec<Stmt>
    where
        I: Iterator<Item = Token>,
    {
        let mut statements = Vec::new();

        // This eats any newlines at the beginning of the file
        while self.peek_matches(tokens, TokenKind::NewLine) {
            self.advance(tokens);
        }

        while !self.peek_matches_any(tokens, &[TokenKind::Eof, TokenKind::Dedent]) {
            match self.statement(tokens) {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(e) => {
                    self.error_reporter.register_parse_error(e);
                    self.synchronize(tokens);
                }
            }

            // This eats any newlines between statements or at the end of the file
            while self.peek_matches(tokens, TokenKind::NewLine) {
                self.advance(tokens);
            }
        }

        statements
    }

    fn statement<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        if self.peek_matches(tokens, TokenKind::Pass) {
            self.advance(tokens); // consume 'pass'
            return Ok(Stmt::Pass);
        }

        if self.peek_matches(tokens, TokenKind::If) {
            return self.if_statement(tokens);
        }

        if self.peek_matches(tokens, TokenKind::While) {
            return self.while_loop(tokens);
        }

        if self.advance_if(tokens, TokenKind::NewLine)
            && self.peek_matches(tokens, TokenKind::Indent)
        {
            return Ok(Stmt::Block(self.block(tokens)?));
        }

        if self.peek_matches(tokens, TokenKind::Def) {
            return self.function_def(tokens);
        }

        if self.peek_matches(tokens, TokenKind::Class) {
            return self.class_decl(tokens);
        }

        if self.peek_matches(tokens, TokenKind::Return) {
            return self.return_statement(tokens);
        }

        // It appears than we have an expression (including the beginning of an assignment)

        let expr = self.expression(tokens)?;

        if self.peek_matches_any(
            tokens,
            &[
                TokenKind::Equal,
                TokenKind::PlusEqual,
                TokenKind::MinusEqual,
                TokenKind::StarEqual,
                TokenKind::SlashEqual,
            ],
        ) {
            return self.assignment_statement(tokens, expr);
        }

        // Definitely expression statement, we expect newline or EOF now

        if self.advance_if(tokens, TokenKind::NewLine) || self.is_at_end(tokens) {
            return Ok(Stmt::Expression(expr));
        }

        Err(self.error(
            tokens,
            "expected newline or EOF after expression statement.",
        ))
    }

    fn block<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Vec<Stmt>, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        self.consume(
            tokens,
            TokenKind::Indent,
            "expected indent at start of block",
        )?;

        let statements = self.statements(tokens);

        self.consume(tokens, TokenKind::Dedent, "expected dedent at end of block")?;

        Ok(statements)
    }

    fn assignment_statement<I>(
        &mut self,
        tokens: &mut Peekable<I>,
        l_value: Expr,
    ) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let operator = self.advance(tokens);

        let r_value = self.expression(tokens)?;

        match l_value.clone() {
            Expr::Variable(token) => {
                if self.advance_if(tokens, TokenKind::NewLine) || self.is_at_end(tokens) {
                    match operator.kind {
                        TokenKind::Equal => {
                            return Ok(Stmt::Assignment {
                                name: token,
                                initializer: r_value,
                            });
                        }
                        TokenKind::PlusEqual => {
                            return Ok(Stmt::Assignment {
                                name: token,
                                initializer: Expr::Binary {
                                    left: Box::new(l_value),
                                    operator: Token::new(TokenKind::Plus, "+", operator.line),
                                    right: Box::new(r_value),
                                },
                            });
                        }
                        TokenKind::MinusEqual => {
                            return Ok(Stmt::Assignment {
                                name: token,
                                initializer: Expr::Binary {
                                    left: Box::new(l_value),
                                    operator: Token::new(TokenKind::Minus, "-", operator.line),
                                    right: Box::new(r_value),
                                },
                            });
                        }
                        TokenKind::StarEqual => {
                            return Ok(Stmt::Assignment {
                                name: token,
                                initializer: Expr::Binary {
                                    left: Box::new(l_value),
                                    operator: Token::new(TokenKind::Star, "*", operator.line),
                                    right: Box::new(r_value),
                                },
                            });
                        }
                        TokenKind::SlashEqual => {
                            return Ok(Stmt::Assignment {
                                name: token,
                                initializer: Expr::Binary {
                                    left: Box::new(l_value),
                                    operator: Token::new(TokenKind::Slash, "/", operator.line),
                                    right: Box::new(r_value),
                                },
                            });
                        }
                        _ => {
                            return Err(self.error(
                                tokens,
                                &format!(
                                    "unexpected operator {:?} treated as assignment",
                                    operator
                                ),
                            ));
                        }
                    }
                }
            }

            Expr::Get { object, name } => {
                return Ok(Stmt::Set {
                    object: *object,
                    name,
                    value: r_value,
                });
            }

            _ => {
                return Err(self.error(
                    tokens,
                    "expected newline or EOF after assignment statement.",
                ));
            }
        }

        Err(self.error(
            tokens,
            "cannot assign to expression here. Maybe you meant '==' instead of '='?",
        ))
    }

    fn if_statement<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        self.advance(tokens); // consume "if" or "elif"

        let condition = self.expression(tokens)?;

        self.consume(
            tokens,
            TokenKind::Colon,
            "expected ':' after if conditional",
        )?;

        let then_body = self.statement(tokens)?;

        let mut else_body = None;
        if self.peek_matches(tokens, TokenKind::Elif) {
            else_body = Some(Box::new(self.if_statement(tokens)?));
        }

        if self.peek_matches(tokens, TokenKind::Else) {
            self.advance(tokens); // consume "else"
            self.consume(tokens, TokenKind::Colon, "expected ':' after 'else'")?;
            else_body = Some(Box::new(self.statement(tokens)?));
        }

        Ok(Stmt::If {
            condition,
            then_body: Box::new(then_body),
            else_body,
        })
    }

    fn while_loop<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        self.advance(tokens); // consume "while"

        let condition = self.expression(tokens)?;

        self.consume(
            tokens,
            TokenKind::Colon,
            "expected ':' after while conditional",
        )?;

        let body = self.statement(tokens)?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn function_def<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        self.advance(tokens); // consume "def"

        let name = self.consume(
            tokens,
            TokenKind::Identifier,
            "expected identifier name after 'def'",
        )?;

        self.consume(
            tokens,
            TokenKind::LeftParen,
            "expected '(' after function name",
        )?;

        let mut params = vec![];
        if !self.peek_matches(tokens, TokenKind::RightParen) {
            loop {
                if params.len() > 255 {
                    return Err(self.error(tokens, "can't have more than 255 parameters."));
                }

                params.push(self.consume(
                    tokens,
                    TokenKind::Identifier,
                    "expected parameter name",
                )?);

                if !self.advance_if(tokens, TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(
            tokens,
            TokenKind::RightParen,
            "expected ')' after parameters",
        )?;
        self.consume(tokens, TokenKind::Colon, "expected ':' after ')'")?;
        self.consume(tokens, TokenKind::NewLine, "expected new line after ':'")?;

        let body = self.block(tokens)?;

        Ok(Stmt::Function { name, params, body })
    }

    fn class_decl<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        self.advance(tokens); // consume "class"

        let name = self.consume(
            tokens,
            TokenKind::Identifier,
            "expected identifier name after 'class'",
        )?;

        self.consume(tokens, TokenKind::Colon, "expected ':' after ')'")?;
        self.consume(tokens, TokenKind::NewLine, "expected new line after ':'")?;

        let body = self.block(tokens)?;

        Ok(Stmt::Class { name, body })
    }

    fn return_statement<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let keyword = self.advance(tokens); // consume "return";

        let value = if self.peek_matches(tokens, TokenKind::NewLine) || self.is_at_end(tokens) {
            None
        } else {
            Some(self.expression(tokens)?)
        };

        Ok(Stmt::Return { keyword, value })
    }

    fn expression<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        self.logical_or(tokens)
    }

    fn logical_or<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.logical_and(tokens)?;

        while self.advance_if(tokens, TokenKind::Or) {
            let operator = self.previous.clone().unwrap();
            let right = self.logical_and(tokens)?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical_and<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.logical_not(tokens)?;

        while self.advance_if(tokens, TokenKind::And) {
            let operator = self.previous.clone().unwrap();
            let right = self.logical_not(tokens)?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical_not<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        if self.advance_if(tokens, TokenKind::Not) {
            let operator = self.previous.clone().unwrap();
            let right = self.equality(tokens)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        let expr = self.equality(tokens)?;
        Ok(expr)
    }

    fn equality<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.identity(tokens)?;

        while self.advance_if_any(tokens, &[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous.clone().unwrap();
            let right = self.identity(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn identity<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.comparison(tokens)?;

        if self.advance_if(tokens, TokenKind::Is) {
            let is_operator = self.previous.clone().unwrap();

            if self.advance_if(tokens, TokenKind::Not) {
                let not_operator = self.previous.clone().unwrap();
                let right = self.comparison(tokens)?;

                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator: is_operator,
                    right: Box::new(right),
                };

                expr = Expr::Unary {
                    operator: not_operator,
                    right: Box::new(expr),
                };
            } else {
                let right = self.comparison(tokens)?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    operator: is_operator,
                    right: Box::new(right),
                };
            }
        }

        Ok(expr)
    }

    fn comparison<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.term(tokens)?;

        while self.advance_if_any(
            tokens,
            &[
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                TokenKind::Less,
                TokenKind::LessEqual,
            ],
        ) {
            let operator = self.previous.clone().unwrap();
            let right = self.term(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.factor(tokens)?;

        while self.advance_if_any(tokens, &[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous.clone().unwrap();
            let right = self.factor(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.unary(tokens)?;

        while self.advance_if_any(tokens, &[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous.clone().unwrap();
            let right = self.unary(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        if self.advance_if_any(
            tokens,
            &[TokenKind::Plus, TokenKind::Minus, TokenKind::Tilde],
        ) {
            let operator = self.previous.clone().unwrap();
            let right = self.unary(tokens)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        let expr = self.call(tokens)?;
        Ok(expr)
    }

    fn call<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut expr = self.primary(tokens)?;

        loop {
            if self.advance_if(tokens, TokenKind::LeftParen) {
                expr = self.finish_call(tokens, expr)?;
            } else if self.advance_if(tokens, TokenKind::Dot) {
                let name = self.consume(
                    tokens,
                    TokenKind::Identifier,
                    "expected property name after '.'.",
                )?;

                expr = Expr::Get {
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        if self.advance_if(tokens, TokenKind::False) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.advance_if(tokens, TokenKind::True) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.advance_if(tokens, TokenKind::None) {
            return Ok(Expr::Literal(Literal::None));
        }

        if self.advance_if_any(
            tokens,
            &[TokenKind::Int, TokenKind::Float, TokenKind::String],
        ) {
            let token = self.previous.clone().unwrap();
            return match token.literal {
                Some(literal) => Ok(Expr::Literal(literal)),
                None => Err(self.error(
                    tokens,
                    &format!("got token type {:?} without literal", token.kind),
                )),
            };
        }

        if self.advance_if(tokens, TokenKind::Identifier) {
            return Ok(Expr::Variable(self.previous.clone().unwrap()));
        }

        if self.advance_if(tokens, TokenKind::LeftParen) {
            let expr = self.expression(tokens)?;
            self.consume(tokens, TokenKind::RightParen, "'(' was never closed")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let peek_kind = tokens.peek().unwrap().kind;
        Err(self.error(
            tokens,
            &format!("don't know how to parse token {:?} here", peek_kind),
        ))
    }

    fn finish_call<I>(&mut self, tokens: &mut Peekable<I>, callee: Expr) -> Result<Expr, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        let mut arguments = vec![];
        if !self.peek_matches(tokens, TokenKind::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(self.error(tokens, "can't have more than 255 arguments"));
                }
                arguments.push(self.expression(tokens)?);
                if !self.advance_if(tokens, TokenKind::Comma) {
                    break;
                }
            }
        }

        let right_paren = self.consume(tokens, TokenKind::RightParen, "'(' was never closed")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: right_paren,
            arguments,
        })
    }

    fn error<I>(&mut self, tokens: &mut Peekable<I>, msg: &str) -> ParseError
    where
        I: Iterator<Item = Token>,
    {
        ParseError::ParseError(
            SourceLocation {
                line: tokens.peek().unwrap().line,
            },
            msg.into(),
        )
    }

    fn consume<I>(
        &mut self,
        tokens: &mut Peekable<I>,
        kind: TokenKind,
        err_msg: &'static str,
    ) -> Result<Token, ParseError>
    where
        I: Iterator<Item = Token>,
    {
        if self.peek_matches(tokens, kind) {
            Ok(self.advance(tokens))
        } else {
            Err(self.error(tokens, err_msg))
        }
    }

    fn advance<I>(&mut self, tokens: &mut Peekable<I>) -> Token
    where
        I: Iterator<Item = Token>,
    {
        if let Some(next_token) = tokens.next() {
            self.previous = Some(next_token);
        }
        self.previous.clone().unwrap()
    }

    fn advance_if_any<I>(&mut self, tokens: &mut Peekable<I>, kinds: &[TokenKind]) -> bool
    where
        I: Iterator<Item = Token>,
    {
        if self.peek_matches_any(tokens, kinds) {
            self.advance(tokens);
            true
        } else {
            false
        }
    }

    fn advance_if<I>(&mut self, tokens: &mut Peekable<I>, kind: TokenKind) -> bool
    where
        I: Iterator<Item = Token>,
    {
        if self.peek_matches(tokens, kind) {
            self.advance(tokens);
            true
        } else {
            false
        }
    }

    fn peek_matches_any<I>(&mut self, tokens: &mut Peekable<I>, kinds: &[TokenKind]) -> bool
    where
        I: Iterator<Item = Token>,
    {
        for kind in kinds {
            if self.peek_matches(tokens, *kind) {
                return true;
            }
        }
        false
    }

    fn previous_matches_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if let Some(t) = self.previous.take()
                && t.kind == *kind
            {
                return true;
            }
        }
        false
    }

    fn peek_matches<I>(&mut self, tokens: &mut Peekable<I>, kind: TokenKind) -> bool
    where
        I: Iterator<Item = Token>,
    {
        if let Some(t) = tokens.peek()
            && t.kind == kind
        {
            return true;
        }

        false
    }

    fn is_at_end<I>(&mut self, tokens: &mut Peekable<I>) -> bool
    where
        I: Iterator<Item = Token>,
    {
        self.peek_matches(tokens, TokenKind::Eof)
    }

    fn synchronize<I>(&mut self, tokens: &mut Peekable<I>)
    where
        I: Iterator<Item = Token>,
    {
        // Consume tokens until we are probably at the beginning of
        // another statement. Not sure if skipping indentation like
        // this will cause issues with parsing down the line though.
        self.advance(tokens);

        while !self.is_at_end(tokens) {
            if self.previous_matches_any(&[
                TokenKind::NewLine,
                TokenKind::Indent,
                TokenKind::Dedent,
            ]) && self.peek_matches_any(
                tokens,
                &[
                    TokenKind::Return,
                    TokenKind::Def,
                    TokenKind::If,
                    TokenKind::Class,
                    TokenKind::For,
                    TokenKind::While,
                ],
            ) {
                return;
            }

            self.advance(tokens);
        }
    }
}
