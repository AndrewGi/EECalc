use crate::si;
use crate::scanner::Scanner;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Equality,
}
#[derive(Debug, Clone)]
pub struct BinaryOperation {
	operator: BinaryOperator,
	left: Box<Expr>,
	right: Box<Expr>,
}
#[derive(Debug, Clone)]
pub enum Operation {
	Unary(UnaryOperation),
	Binary(BinaryOperation)
}
#[derive(Debug, Clone)]
pub enum UnaryOperator {
	Plus,
	Minus
}
#[derive(Debug, Clone)]
pub struct UnaryOperation {
	operator: UnaryOperator,
	operand: Box<Expr>
}
#[derive(Debug, Clone)]
pub enum Operator {
	Unary(UnaryOperator),
	Binary(BinaryOperator)
}
impl Operator {
	pub fn precedence(&self) -> i32 {
		match &self {
			Operator::Unary(u) => u.precedence(),
			Operator::Binary(b) => b.precedence()
		}
	}
	pub fn highest_precedence() -> i32 {
		18
	}
}
#[derive(Debug, Clone)]
pub enum Expr {
	Value(si::Value),
	BinaryOperation(BinaryOperation),
	UnaryOperation(UnaryOperation)
}
enum Token {
	Expr(Expr),
	BinaryOperator(BinaryOperator),
	UnaryOperator(UnaryOperator)
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
impl UnaryOperator {
	pub fn precedence(&self) -> i32 {
		match *self {
			UnaryOperator::Plus => 3,
			UnaryOperator::Minus => 3,
		}
	}
}

impl UnaryOperation {
	pub fn new(operator: UnaryOperator, operand: Expr) -> UnaryOperation {
		UnaryOperation {
			operator, operand: Box::new(operand)
		}
	}
}
use std::fmt;
use crate::parser::UnaryOperator::Plus;
use crate::parser::ParserError::{ExpectedValue, EarlyEndOfInput, ExpectedOperator};

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
#[derive(Debug)]
pub enum ParserError {
	ExpectedOperator(usize),
	ExpectedValue(usize),
	EarlyEndOfInput(usize)
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
	scanner: Scanner<'a>,
}
pub struct Cursor<'a> {
	scanner: crate::scanner::Scanner<'a>,
	t_stack: Vec<Token>
}
impl<'a> crate::parser::Cursor<'a> {


	fn stack_operator_precedence(&self) -> Option<i32> {
		match self.t_stack.get(self.t_stack.len()-2)? {
			Token::BinaryOperator(op) => Some(op.precedence()),
			_ => None,
		}
	}
	fn pop_binary_operation(&mut self) -> Option<BinaryOperation> {
		if self.t_stack.len() < 3 {
			return None
		}
		let right_t = self.t_stack.pop().unwrap();
		let operator_t = self.t_stack.pop().unwrap();
		let left_t = self.t_stack.pop().unwrap();
		match (left_t, operator_t, right_t) {
			(Token::Expr(left), Token::BinaryOperator(operator), Token::Expr(right)) => Some(BinaryOperation::new(left, operator, right)),
			(left_t, operator_t, right_t) => {

				self.t_stack.push(left_t);
				self.t_stack.push(operator_t);
				self.t_stack.push(right_t);
				None
			}
		}
	}
	fn pop_unary_operation(&mut self) -> Option<UnaryOperation> {
		if self.t_stack.len() < 2 {
			return None
		}
		let operand = self.t_stack.pop().unwrap();
		let operator = self.t_stack.pop().unwrap();
		match (operator, operand) {
			(Token::UnaryOperator(op), Token::Expr(ex)) => Some(UnaryOperation::new(op, ex)),
			(operator, operand) => {
				self.t_stack.push(operator);
				self.t_stack.push(operand);
				None
			}
		}
	}
	pub fn pop_operation(&mut self) -> Option<Operation> {
		match self.t_stack[self.t_stack.len()-2] {
			Token::UnaryOperator(_) => Some(Operation::Unary(self.pop_unary_operation()?)),
			Token::BinaryOperator(_) => Some(Operation::Binary(self.pop_binary_operation()?)),
			_ => None
		}
	}
	fn peek_operator(&self) -> Option<crate::parser::Operator> {
		let maybe_binary_op = || {
			match self.t_stack.last() {
				Some(token) => match token {
					Token::Expr(_) => true,
					_ => false
				},
				None => false
			}
		};
		let op_c = self.scanner.peek()?;
		match op_c {
			'/' => Some(crate::parser::Operator::Binary(BinaryOperator::Divide)),
			'*' => Some(crate::parser::Operator::Binary(BinaryOperator::Multiply)),
			'+' => {
				if maybe_binary_op() {
					Some(crate::parser::Operator::Binary(BinaryOperator::Add))
				} else {
					Some(crate::parser::Operator::Unary(UnaryOperator::Plus))
				}
			}
			'-' => {
				if maybe_binary_op() {
					Some(crate::parser::Operator::Binary(BinaryOperator::Subtract))
				} else {
					Some(crate::parser::Operator::Unary(UnaryOperator::Minus))
				}
			}
			_ => panic!("unhandled operator '{}'", op_c)
		}
	}
	fn next_operator(&mut self) -> Option<crate::parser::Operator> {
		let operator = self.peek_operator()?;
		self.scanner.next_operator().unwrap(); //there should be a next operator
		Some(operator)
	}
	fn peek_precedence(&self) -> Option<i32> {
		Some(self.peek_operator()?.precedence())
	}
	fn push(&mut self, token: Token) {
		self.t_stack.push(token);
	}
	pub fn next_expression(&mut self) -> Result<Expr, ParserError> {
		if self.t_stack.is_empty() {
			if let Some(value) = self.scanner.next_value() {
				self.push(Token::Expr(Expr::Value(value)))
			} else {
				return Err(ExpectedValue(self.scanner.index()))
			}
		}
		//Eat the scanner cursor
		while self.scanner.peek().unwrap_or(' ') != ')' && self.peek_precedence() < self.stack_operator_precedence() {
			let operator = match self.scanner.next().unwrap_or(' ') {
				Some(c) => c,
				None => return Err(ExpectedOperator(self.s_cursor.clone()))
			};
			let expression = if let Some(op) = self.s_cursor.peek_operator() {
				if op == '(' {
					debug_assert!(self.scanner.next_operator().unwrap() == '(');
					self.next_expression()?
				} else {
					return Err(ExpectedOperator(self.s_cursor.clone())); //Expected a '('
				}
			} else {
				Expr::Value(match self.scanner.next_value() {
					Some(value) => value,
					None => return Err(ExpectedValue(self.s_cursor.clone()))
				})
			};
			self.push(match operator {
				Operator::Unary(u) => Token::UnaryOperator(u),
				Operator::Binary(b) => Token::BinaryOperator(b)
			});
			self.push(Token::Expr(expression));
		}
		if self.scanner.peek_operator().unwrap_or(' ') == ')' {
			self.scanner.next_operator().unwrap(); //Eat the closing parathesis
		}
		//Eat a operation off the stack
		Ok(match self.pop_operation().expect("token stack invalid") {
			Operation::Unary(u) => Expr::UnaryOperation(u),
			Operation::Binary(b) => Expr::BinaryOperation(b)
		})
	}
}
impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		Parser {
			scanner: Scanner::new(input),
		}
	}
	pub fn get_cursor(&self) -> Cursor<'a> {

		Cursor {
			scanner: self.scanner.clone(),
			t_stack: vec![]
		}
	}
}
