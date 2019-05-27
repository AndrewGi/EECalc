use crate::scanner::{Token, Scanner, TokenType, Operator, Separator};
use std::iter::Peekable;
use crate::parser::ParserError::{ExpectedOperator, EarlyEOF, ExpectedSeparator};
use crate::parser::UnaryOperator::Negate;
use crate::parser::Node::{UnaryOperation, Function};
use crate::scanner::Separator::{OpenParentheses, Comma};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
	workspace: Vec<Node<'a>>
}
pub enum ParserError<'a> {
	ExpectedOperator(&'a Token<'a>),
	ExpectedSeparator(Separator, &'a Token<'a>),
	EarlyEOF
}
pub enum UnaryOperator {
	Negate
}
pub enum InfixOperator {
	Plus,
	Minus,
	Times,
	Divide,
	Raise
}
pub enum Number {
	Float(f64),
	Int(i64)
}
pub enum Node<'a> {
	Number(Number),
	Variable(&'a str),
	Function(&'a str,Vec<Node<'a>>),
	UnaryOperation(UnaryOperator, Box<Node<'a>>),
	InfixOperation(InfixOperator, Box<Node<'a>>, Box<Node<'a>>),
	Scope(Node<'a>)
}
type ReduceResult<'a> = Result<&'a Node<'a>, ParserError<'a>>;
impl InfixOperator {
	pub fn precedence(&self) -> i32 {
		match *self {
			InfixOperator::Minus | InfixOperator::Plus => 3,
			InfixOperator::Divide | InfixOperator::Times => 2,
			InfixOperator::Raise => 1
		}
	}
}
impl<'a> iterator<'a> for Parser<'a> {
	type Item = Node<'a>;
	fn next(&mut self) -> Option<Result<Node<'a>, ParserError<'a>>> {


	}

}
const HIGHEST_PRECEDENCE: i32 = 4;
impl<'a> Parser<'a> {
	pub fn from_scanner(scanner: Scanner<'a>) -> Parser<'a> {
		Parser { scanner: scanner.peekable(), last_precedence: HIGHEST_PRECEDENCE }
	}
	fn shift(&mut self) -> Option<()> {
		self.workspace.push(self.scanner.next()?)
	}
	fn reduce(&mut self) {

	}
}