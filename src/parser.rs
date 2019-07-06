use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenType;
use std::iter::Peekable;
use crate::si::UnitWithExponent;
use crate::parser::ParserError::ExpectedNumber;

#[derive(Clone)]
pub struct Parser<'a> {
	scanner: Peekable<Scanner<'a>>
}
#[derive(Clone)]
pub struct ParserCursor<'a> {
	parent: &'a mut Parser<'a>,
	copy: Parser<'a>
}
#[derive(Clone)]
pub enum ParserToken {
	UnitExponent(UnitWithExponent),
	BinaryOperator(super::scanner::Operator),
	UnaryOperator(super::scanner::Operator),
	Parentheses(Vec<ParserToken>),
}
pub enum ParserError {
	ExpectedScanner,
	UnexpectedOperator(super::scanner::Operator),
	ExpectedNumber,
}
impl<'a> Parser<'a> {
	pub fn new(scanner: Peekable<Scanner<'a>>) -> Parser<'a> {
		Parser { scanner }
	}
	pub fn cursor(&mut self) -> ParserCursor<'a> {
		ParserCursor {
			parent: self,
			copy: self.clone(),
		}
	}
}
impl<'a> ParserCursor<'a> {

	pub fn apply(self) {
		*self.parent = self.copy
	}
	fn scanner(&mut self) -> &mut Peekable<Scanner<'_>> {
		&mut self.parent.scanner
	}
	pub fn next_int(&mut self) -> Result<i64, ParserError> {
		let mut clone = self.clone();
		let start = clone.scanner().peek().ok_or(ExpectedNumber)?;

		loop {


		}
	}
}
