use crate::{
    scanner::Scanner,
    token::{Token, TokenKind, Literal},
    ast::Expr,
    error::ErrorReporter,

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
            error_reporter
        }
    }

    pub fn parse<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        self.expression(tokens)
    }

    fn expression<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        self.equality(tokens)
    }

    fn equality<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.comparison(tokens);

        while self.match_any(tokens, &[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous.unwrap();
            let right = self.comparison(tokens);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn comparison<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.term(tokens);

        while self.match_any(tokens, &[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous.unwrap();
            let right = self.term(tokens);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn term<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.factor(tokens);

        while self.match_any(tokens, &[
            TokenKind::Plus,
            TokenKind::Minus,
        ]) {
            let operator = self.previous.unwrap();
            let right = self.factor(tokens);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn factor<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        let mut expr = self.unary(tokens);

        while self.match_any(tokens, &[
            TokenKind::Star,
            TokenKind::Slash,
        ]) {
            let operator = self.previous.unwrap();
            let right = self.unary(tokens);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn unary<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.match_any(tokens, &[
            TokenKind::Not,
            TokenKind::Minus,
        ]) {
            let operator = self.previous.unwrap();
            let right = self.unary(tokens);
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        return self.primary(tokens);
    }

    // TODO: currently the scanner doesn't fill out the literal field of 
    // tokens representing None, True, or False in Python. But the parser
    // does put a Literal in the AST here. This means I have Literal variants
    // that are never used in Token. Maybe this is ok, but it seems a little
    // weird. Let's think about it some more later.
    fn primary<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.match_any(tokens, &[TokenKind::False]) {
            return Expr::Literal(Literal::Bool(false));
        }

        if self.match_any(tokens, &[TokenKind::True]) {
            return Expr::Literal(Literal::Bool(true));
        }

        if self.match_any(tokens, &[TokenKind::None]) {
            return Expr::Literal(Literal::None);
        }

        if self.match_any(tokens, &[TokenKind::Number, TokenKind::String]) {
            return Expr::Literal(self.previous.unwrap().literal);
        }

        if self.match_any(tokens, &[TokenKind::LeftParen]) {
            let expr = self.expression(tokens);
            self.consume(tokens, TokenKind::RightParen, "Expected ')' after expression");
            return Expr::Grouping(Box::new(expr));
        }

        panic!("WTF how did I get here?!");
    }

    fn consume<I>(&mut self, tokens: &mut Peekable<I>, kind: TokenKind, error: &str) -> Token<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.check(tokens, kind) {
            return self.advance(tokens);
        }

        panic!("ParseError: {error}");
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

    fn match_any<I>(&mut self, tokens: &mut Peekable<I>, kinds: &[TokenKind]) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
        for kind in kinds {
            if self.check(tokens, *kind) {
                self.advance(tokens);
                return true;
            }
        }
        false
    }

    fn check<I>(&mut self, tokens: &mut Peekable<I>, kind: TokenKind) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
        tokens.peek().map_or(false, |t| t.kind == kind)
    }
}
