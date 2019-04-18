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
	BinaryOperator(BinaryOperator)
}
impl BinaryOperation {
	pub fn new(left: Expr, operator: BinaryOperator, right: Expr) -> BinaryOperation {
		BinaryOperation { left: Box::new(left), operator, right: Box::new(right)}
	}
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
		match self.token_stack.get(self.token_stack.len()-2)? {
			Token::BinaryOperator(op) => Some(op.precedence()),
			_ => None,
		}
	}
	fn pop_binary_operation(&mut self) -> Option<BinaryOperator> {
		if self.token_stack.len() < 3 {
			return None
		}
		let right_t = self.token_stack.pop().unwrap();
		let operator_t = self.token_stack.pop().unwrap();
		let left_t = self.token_stack.pop().unwrap();
		match (left_t, operator_t, right_t) {
			(Token::Expr(left), Token::BinaryOperator(operator), Token::Expr(right)) => Some(BinaryOperation::new(left, operator, right))
			_ => {

				self.token_stack.push(left_t);
				self.token_stack.push(operator_t);
				self.token_stack.push(right_t);
				None
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
