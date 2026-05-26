use crate::{
    scanner::Scanner,
    token::{Token, TokenKind, Literal},
    ast::Expr,
    error::ErrorReporter,

};

use std::iter::Peekable;

struct Parser<'src, 'err> {
    scanner: Peekable<Scanner<'src, 'err>>,
    previous: Token<'src>,
    error_reporter: &'err mut ErrorReporter,
}

impl<'src, 'err> Parser<'src, 'err> {
    fn expression(&mut self) -> Expr<'src> {
        self.equality()
    }

    fn equality(&mut self) -> Expr<'src> {
        let mut expr = self.comparison();

        while self.match_any(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous;
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr<'src> {
        let mut expr = self.term();

        while self.match_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous;
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn term(&mut self) -> Expr<'src> {
        let mut expr = self.factor();

        while self.match_any(&[
            TokenKind::Plus,
            TokenKind::Minus,
        ]) {
            let operator = self.previous;
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn factor(&mut self) -> Expr<'src> {
        let mut expr = self.unary();

        while self.match_any(&[
            TokenKind::Star,
            TokenKind::Slash,
        ]) {
            let operator = self.previous;
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn unary(&mut self) -> Expr<'src> {
        if self.match_any(&[
            TokenKind::Not,
            TokenKind::Minus,
        ]) {
            let operator = self.previous;
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr<'src> {
        if self.match_any(&[TokenKind::False]) {
            return Expr::Literal(Literal::Bool(false));
        }

        if self.match_any(&[TokenKind::True]) {
            return Expr::Literal(Literal::Bool(true));
        }

        if self.match_any(&[TokenKind::None]) {
            return Expr::Literal(Literal::None);
        }

        if self.match_any(&[TokenKind::Number, TokenKind::String]) {
            return Expr::Literal(self.previous.literal);
        }

        if self.match_any(&[TokenKind::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenKind::RightParen, "Expected ')' after expression");
            return Expr::Grouping(Box::new(expr));
        }

        panic!("WTF how did I get here?!");
    }

    fn consume(&mut self, kind: TokenKind, error: &str) -> Token<'src> {
        if self.check(kind) {
            return self.advance();
        }

        panic!("ParseError: {error}");
    }

    fn advance(&mut self) -> Token<'src> {
        if let Some(next_token) = self.scanner.next() {
            self.previous = next_token;
        }
        self.previous
    }

    fn match_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        self.scanner.peek().map_or(false, |t| t.kind == kind)
    }
}
