use crate::{
    ast::{Expr, Stmt},
    error::{ErrorReporter, ParseError},
    token::{Literal, SourceLocation, Token, TokenKind::{self, NewLine}},
};

use std::iter::Peekable;

pub struct Parser<'src, 'err> {
    previous: Option<Token<'src>>,
    error_reporter: &'err mut ErrorReporter,
}

impl<'src, 'err> Parser<'src, 'err> {
    pub fn new(error_reporter: &'err mut ErrorReporter) -> Self {
        Parser {
            previous: None,
            error_reporter,
        }
    }

    pub fn parse<I>(&mut self, tokens: &mut Peekable<I>) -> Vec<Stmt<'src>>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut all_statements = Vec::new();

        while !self.peek_matches(tokens, TokenKind::Eof) {
            let mut statements = self.statements(tokens);
            if statements.len() == 0 {
                break;
            }

            all_statements.append(&mut statements);
        }

        all_statements
    }

    fn statements<I>(&mut self, tokens: &mut Peekable<I>) -> Vec<Stmt<'src>>
    where
        I: Iterator<Item = Token<'src>>,
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

    fn statement<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.advance_if_peek_matches_any(tokens, &[TokenKind::Print]) {
            return self.print_statement(tokens);
        }

        if self.peek_matches(tokens, TokenKind::If) {
            return self.if_statement(tokens);
        }

        if self.advance_if_peek_matches_any(tokens, &[NewLine]) {
            if self.peek_matches(tokens, TokenKind::Indent) {
                return Ok(Stmt::Block(self.block(tokens)?));
            }


        }

        // It appears than we have an expression (including the beginning of an assignment)

        let expr = self.expression(tokens)?;

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::Equal]) {
            return self.assignment_statement(tokens, &expr);
        }

        // Definitely expression statement, we expect newline or EOF now

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::NewLine]) || self.is_at_end(tokens)
        {
            return Ok(Stmt::Expression(expr));
        }

        Err(ParseError::ParseError(
            SourceLocation {
                line: tokens.peek().unwrap().line,
            },
            String::from("expected newline or EOF after expression statement."),
        ))
    }

    fn block<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Vec<Stmt<'src>>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        // if !self.advance_if_peek_matches_any(tokens, &[TokenKind::NewLine]) {
        //     return Err(ParseError::ParseError(
        //         SourceLocation {
        //             line: tokens.peek().unwrap().line,
        //         },
        //         String::from("expected new line at start of block"),
        //     ));
        // }

        if !self.advance_if_peek_matches_any(tokens, &[TokenKind::Indent]) {
            return Err(ParseError::ParseError(
                SourceLocation {
                    line: tokens.peek().unwrap().line,
                },
                String::from("expected indent at start of block"),
            ));
        }

        let statements = self.statements(tokens);

        if !self.advance_if_peek_matches_any(tokens, &[TokenKind::Dedent]) {
            return Err(ParseError::ParseError(
                SourceLocation {
                    line: tokens.peek().unwrap().line,
                },
                String::from("expected dedent at end of block"),
            ));
        }

        Ok(statements)
    }

    fn print_statement<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        // This function is sort of a hack. It consumes open and close parens to make
        // the print statement look like the Python standard library function 'print'
        // without actually having functions implemented yet.

        if !self.advance_if_peek_matches_any(tokens, &[TokenKind::LeftParen]) {
            return Err(ParseError::ParseError(
                SourceLocation {
                    line: tokens.peek().unwrap().line,
                },
                String::from("expected '(' after print keyword."),
            ));
        }

        let value = self.expression(tokens)?;

        if !self.advance_if_peek_matches_any(tokens, &[TokenKind::RightParen]) {
            return Err(ParseError::ParseError(
                SourceLocation {
                    line: tokens.peek().unwrap().line,
                },
                String::from("expected ')' after print keyword."),
            ));
        }

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::NewLine]) || self.is_at_end(tokens)
        {
            return Ok(Stmt::Print(value));
        }

        Err(ParseError::ParseError(
            SourceLocation {
                line: tokens.peek().unwrap().line,
            },
            String::from("expected newline or EOF after print statement."),
        ))
    }

    fn assignment_statement<I>(
        &mut self,
        tokens: &mut Peekable<I>,
        l_value: &Expr<'src>,
    ) -> Result<Stmt<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let r_value = self.expression(tokens)?;

        if let Expr::Variable(token) = l_value {
            if self.advance_if_peek_matches_any(tokens, &[TokenKind::NewLine])
                || self.is_at_end(tokens)
            {
                return Ok(Stmt::Assignment {
                    name: *token,
                    initializer: r_value,
                });
            }

            return Err(ParseError::ParseError(
                SourceLocation {
                    line: tokens.peek().unwrap().line,
                },
                String::from("expected newline or EOF after assignment statement."),
            ));
        }

        Err(ParseError::ParseError(
            SourceLocation {
                line: tokens.peek().unwrap().line,
            },
            String::from("cannot assign to expression here. Maybe you meant '==' instead of '='?"),
        ))
    }

    fn if_statement<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Stmt<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        self.advance(tokens); // consume "if" or "elif"

        let condition = self.expression(tokens)?;

        if !self.advance_if_peek_matches_any(tokens, &[TokenKind::Colon]) {
            return Err(ParseError::ParseError(
                SourceLocation {
                    line: tokens.peek().unwrap().line,
                },
                String::from("expected ':' after if conditional"),
            ));
        }

        let then_body = self.statement(tokens)?;

        let mut else_body = None;

        if self.peek_matches(tokens, TokenKind::Elif) {
            else_body = Some(Box::new(self.if_statement(tokens)?));
        }

        if self.peek_matches(tokens, TokenKind::Else) {
            self.advance(tokens); // consume "else"

            if !self.advance_if_peek_matches_any(tokens, &[TokenKind::Colon]) {
                return Err(ParseError::ParseError(
                    SourceLocation {
                        line: tokens.peek().unwrap().line,
                    },
                    String::from("expected ':' after 'else'"),
                ));
            }

            else_body = Some(Box::new(self.statement(tokens)?));
        }

        Ok(Stmt::If {
            condition,
            then_body: Box::new(then_body),
            else_body: else_body,
        })
    }

    fn expression<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        self.logical_or(tokens)
    }

    fn logical_or<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.logical_and(tokens)?;

        while self.advance_if_peek_matches_any(tokens, &[TokenKind::Or]) {
            let operator = self.previous.unwrap();
            let right = self.logical_and(tokens)?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }


    fn logical_and<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.logical_not(tokens)?;

        while self.advance_if_peek_matches_any(tokens, &[TokenKind::And]) {
            let operator = self.previous.unwrap();
            let right = self.logical_not(tokens)?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical_not<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.advance_if_peek_matches_any(tokens, &[TokenKind::Not,]) {
            let operator = self.previous.unwrap();
            let right = self.unary(tokens)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        let expr = self.equality(tokens)?;
        Ok(expr)
    }

    fn equality<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.comparison(tokens)?;

        while self
            .advance_if_peek_matches_any(tokens, &[TokenKind::BangEqual, TokenKind::EqualEqual])
        {
            let operator = self.previous.unwrap();
            let right = self.comparison(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.term(tokens)?;

        while self.advance_if_peek_matches_any(
            tokens,
            &[
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                TokenKind::Less,
                TokenKind::LessEqual,
            ],
        ) {
            let operator = self.previous.unwrap();
            let right = self.term(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.factor(tokens)?;

        while self.advance_if_peek_matches_any(tokens, &[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous.unwrap();
            let right = self.factor(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.unary(tokens)?;

        while self.advance_if_peek_matches_any(tokens, &[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous.unwrap();
            let right = self.unary(tokens)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.advance_if_peek_matches_any(
            tokens,
            &[
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Tilde,
            ],
        ) {
            let operator = self.previous.unwrap();
            let right = self.unary(tokens)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        let expr = self.primary(tokens)?;
        Ok(expr)
    }

    // TODO: currently the lexer doesn't fill out the literal field of
    // tokens representing None, True, or False in Python. But the parser
    // does put a Literal in the AST here. This means I have Literal variants
    // that are never used in Token.
    fn primary<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Expr<'src>, ParseError>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.advance_if_peek_matches_any(tokens, &[TokenKind::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::None]) {
            return Ok(Expr::Literal(Literal::None));
        }

        if self.advance_if_peek_matches_any(
            tokens,
            &[TokenKind::Int, TokenKind::Float, TokenKind::String],
        ) {
            return Ok(Expr::Literal(self.previous.unwrap().literal));
        }

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::Identifier]) {
            return Ok(Expr::Variable(self.previous.unwrap()));
        }

        if self.advance_if_peek_matches_any(tokens, &[TokenKind::LeftParen]) {
            let expr = self.expression(tokens)?;

            if self.peek_matches(tokens, TokenKind::RightParen) {
                self.advance(tokens);
                return Ok(Expr::Grouping(Box::new(expr)));
            } else {
                return Err(ParseError::ParseError(
                    SourceLocation {
                        line: tokens.peek().unwrap().line,
                    },
                    String::from("'(' was never closed"),
                ));
            }
        }

        Err(ParseError::ParseError(
            SourceLocation {
                line: tokens.peek().unwrap().line,
            },
            format!(
                "don't know how to parse token {:?} here",
                tokens.peek().unwrap().kind
            ),
        ))
    }

    fn advance<I>(&mut self, tokens: &mut Peekable<I>) -> Token<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if let Some(next_token) = tokens.next() {
            self.previous = Some(next_token);
        }
        self.previous.unwrap()
    }

    fn advance_if_peek_matches_any<I>(
        &mut self,
        tokens: &mut Peekable<I>,
        kinds: &[TokenKind],
    ) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
        for kind in kinds {
            if self.peek_matches(tokens, *kind) {
                self.advance(tokens);
                return true;
            }
        }
        false
    }

    fn peek_matches_any<I>(&mut self, tokens: &mut Peekable<I>, kinds: &[TokenKind]) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
        let peeked = tokens.peek();
        for kind in kinds {
            if peeked.map_or(false, |t| t.kind == *kind) {
                return true;
            }
        }
        false
    }

    fn previous_matches_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.previous.map_or(false, |t| t.kind == *kind) {
                return true;
            }
        }
        false
    }

    fn peek_matches<I>(&mut self, tokens: &mut Peekable<I>, kind: TokenKind) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
        tokens.peek().map_or(false, |t| t.kind == kind)
    }

    fn is_at_end<I>(&mut self, tokens: &mut Peekable<I>) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
        self.peek_matches(tokens, TokenKind::Eof)
    }

    fn synchronize<I>(&mut self, tokens: &mut Peekable<I>)
    where
        I: Iterator<Item = Token<'src>>,
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
            ]) {
                if self.peek_matches_any(
                    tokens,
                    &[
                        TokenKind::Return,
                        TokenKind::Def,
                        TokenKind::If,
                        TokenKind::Class,
                        TokenKind::For,
                        TokenKind::While,
                        TokenKind::Print,
                    ],
                ) {
                    return;
                }
            }

            self.advance(tokens);
        }
    }
}
