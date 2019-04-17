use crate::si;
use crate::scanner::Scanner;

pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Equality,
}
pub struct BinaryOperation {
	operator: BinaryOperator,
	left: Box<Expr>,
	right: Box<Expr>,
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

impl fmt::Display for BinaryOperator {
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
	fn stack_operator_precedence(&self) -> Option<i32> {
		match self.token_stack.last()? {
			Token::Expr(expr) => match expr {
				Expr::BinaryOperation(bop) => Some(bop.operator.precedence()),
				_ => None
			}
		}
	}
	pub fn next_expr(&mut self) -> Option<Expr> {
		let last_precedence = self.stack_operator_precedence().unwrap_or(0);

	}
	pub fn new(input: &'a str) -> Parser<'a> {
		Parser {
			scanner: Scanner::new(input),
			token_stack: Vec::new()
		}
	}

}
