use crate::{
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
        // Eventually this will optionally call a method for parsing
        // statements. This is where we need to catch parse errors
        // so we can synchronize at this point and skip ahead to the
        // next statement. The lower-level methods will probably need
        // to return Result instead of a raw Expr to do this.
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

        while self.advance_if_match_any(tokens, &[TokenKind::BangEqual, TokenKind::EqualEqual]) {
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

        while self.advance_if_match_any(tokens, &[
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

        while self.advance_if_match_any(tokens, &[
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

        while self.advance_if_match_any(tokens, &[
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
        if self.advance_if_match_any(tokens, &[
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

    // TODO: currently the lexer doesn't fill out the literal field of 
    // tokens representing None, True, or False in Python. But the parser
    // does put a Literal in the AST here. This means I have Literal variants
    // that are never used in Token.
    fn primary<I>(&mut self, tokens: &mut Peekable<I>) -> Expr<'src>
    where
        I: Iterator<Item = Token<'src>>,
    {
        if self.advance_if_match_any(tokens, &[TokenKind::False]) {
            return Expr::Literal(Literal::Bool(false));
        }

        if self.advance_if_match_any(tokens, &[TokenKind::True]) {
            return Expr::Literal(Literal::Bool(true));
        }

        if self.advance_if_match_any(tokens, &[TokenKind::None]) {
            return Expr::Literal(Literal::None);
        }

        if self.advance_if_match_any(tokens, &[TokenKind::Number, TokenKind::String]) {
            return Expr::Literal(self.previous.unwrap().literal);
        }

        if self.advance_if_match_any(tokens, &[TokenKind::LeftParen]) {
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
        if self.peek_matches(tokens, kind) {
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

    fn advance_if_match_any<I>(&mut self, tokens: &mut Peekable<I>, kinds: &[TokenKind]) -> bool
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

    fn previous_matches_any<I>(&mut self, tokens: &mut Peekable<I>, kinds: &[TokenKind]) -> bool
    where
        I: Iterator<Item = Token<'src>>,
    {
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
            if self.previous_matches_any(tokens, &[
                TokenKind::NewLine, TokenKind::Indent, TokenKind::Dedent
            ]) {
                if self.peek_matches_any(tokens, &[
                    TokenKind::Return, TokenKind::Def, TokenKind::If,
                    TokenKind::Class, TokenKind::For, TokenKind::While,
                    TokenKind::Print
                ]) {
                    return;
                }
            }
        }
    }
}
