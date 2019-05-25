use crate::scanner::{Token, Scanner, TokenType, Operator, Separator};
use std::iter::Peekable;
use crate::parser::ParserError::{ExpectedOperator, EarlyEOF, ExpectedSeparator};
use crate::parser::UnaryOperator::Negate;
use crate::parser::Node::{UnaryOperation, Function};
use crate::scanner::Separator::{OpenParentheses, Comma};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
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
	InfixOperation(InfixOperator, Box<Node<'a>>, Box<Node<'a>>)
}
type ParserResult<'a> = Result<Option<Node<'a>>, ParserError<'a>>;
impl<'a> iterator<'a> for Parser<'a> {
	type Item = Node<'a>;
	fn next(&mut self) -> Option<Result<Node<'a>, ParserError<'a>>> {


	}

}
impl<'a> Node<'a> {

}
impl<'a> Parser<'a> {
	pub fn from_scanner(scanner: Scanner<'a>) -> Parser<'a> {
		Parser { scanner: scanner.peekable() }
	}
	fn next_node(&mut self, previous: Option<&mut Node<'a>>) -> Result<Node<'a>, ParserError<'a>> {

	}
	fn next_unary_operation(&mut self) -> ParserResult {
		let mut self_clone = self.clone();
		let unary_op = match self_clone.scanner.next().ok_or_else(EarlyEOF)?.token_type() {
			TokenType::Operator(Operator::Minus) => Negate,
			_ => return Some(None)
		};
		let operand = Box::new(self_clone.next_node()??);
		self.clone_from(&self_clone);
		Ok(Some(UnaryOperation(unary_op, operand)))
	}
	fn next_number(&mut self) -> Option<Number> {
		match self.scanner.peek()?.token_type() {
			TokenType::Float(f) => Some({self.next_node()?; Number::Float(f) }),
			TokenType::Int(i) => Some({self.next_node()?; Number::Int(i) }),
			_ => None
		}
	}
	fn expect_separator(&mut self, sep: Separator) -> Result<(), ParserError<'a>> {
		match self.scanner.peek().ok_or_else(EarlyEOF)? {
			TokenType::Separator(sep)=> {self.scanner.next(); Ok(())},
			tok => Err(ExpectedSeparator(Separator::OpenParentheses, &tok))
		}
	}
	fn next_function(&mut self) -> ParserResult {
		let mut self_clone = self.clone();
		let next_token = self_clone.scanner.next();
		let func_name = match next_token.token_type() {
			TokenType::Word => next_token.content(),
			_ => return Some(None)
		};
		if self_clone.expect_separator(OpenParentheses).is_err() {
			return Some(None)
		}
		let is_next_closing = |self_clone| self_clone.scanner.peek().ok_or(EarlyEOF)?? != TokenType::Separator(Separator::CloseParentheses);
		let mut args = Vec::<Node<'a>>::with_capacity(4);
		while !is_next_closing() {
			args.push(self_clone.next_node().ok_or(EarlyEOF)??);
			if !is_next_closing() {
				self_clone.expect_separator(Comma);
			}
		}
		self.clone_from(&self_clone);
		Ok(Some(Function {name: func_name, args}))
	}
}