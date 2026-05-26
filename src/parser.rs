use crate::{
    scanner::Scanner,
    token::{Token, TokenKind},
    ast::Expr,
};

use std::iter::Peekable;

struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
    current: usize,
}

// impl<'a> Parser<'a> {
//     fn expression(&self) -> Expr {
//         self.equality()
//     }

//     fn equality(&self) -> Expr {
//         let mut expr = comparison();
//         loop {
//             match 
//         }
//     }

//     fn is_at_end(&self) -> bool {
//         self.scanner.peek().is_some_and(|t| )
//     }
// }
