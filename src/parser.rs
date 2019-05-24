use crate::scanner::{Token, Scanner};

pub struct Parser<'a> {
    in_stack: vec<Token<'a>>,
    scanner: Scanner<'a>
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> Option<>
}