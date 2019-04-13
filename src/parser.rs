
use crate::scanner::Scanner;
use crate::si;
pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Equality,
}
pub struct BinaryOperation {
	operation: BinaryOperator,
	left: Expr,
	right: Expr,
}
pub enum UnaryOperator {
	Plus,
	Negate,
}
pub struct UnaryOperation {
	operation: UnaryOperator,
	operand: Expr,
}
pub enum Expr {
	Value(si::Value),
	BinaryOperation(BinaryOperation)
}
enum Token {
	Expr(Expr),
}

impl BinaryOperator {
	pub fn precedence(&self) -> i32 {
		match *self { //using cppreference.com 4/13/2019
			BinaryOperator::Add => 6,
			BinaryOperator::Subtract => 6,
			BinaryOperator::Divide => 5,
			BinaryOperator::Multiply => 5,
			BinaryOperator::Equality => 10
		}
	}
}

use std::fmt;

impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			BinaryOperator::Add => "+",
			BinaryOperator::Subtract => "-",
			BinaryOperator::Multiply => "*",
			BinaryOperator::Divide => "/",
			BinaryOperator::Equality => "="
		})
	}
}
pub struct Parser<'a> {
	scanner: Scanner<'a>,
	token_stack: Vec<Token>
}
impl<'a> Parser<'a> {
	pub fn highest_precedence() -> i32 {
		18
	}

	pub fn next_expr(&mut self) -> Option<Expr> {
		let last_precedence = Parser::highest_precedence();

	}
	pub fn new(input: &'a str) -> Parser<'a> {
		Parser {
			scanner: Scanner::new(input)
		}
	}

}
