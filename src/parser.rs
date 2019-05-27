
use crate::scanner;

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    scanner: std::iter::Peekable<scanner::Scanner<'a>>
}
pub enum ParserError<'a> {
	ExpectedOperator(&'a scanner::Token<'a>),
	ExpectedSeparator(scanner::Separator, &'a scanner::Token<'a>),
	EarlyEOL
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
	Scope(Box<Node<'a>>)
}
type ParserResult<'a> = Result<Node<'a>, ParserError<'a>>;
impl InfixOperator {
	pub fn precedence(&self) -> i32 {
		match *self {
			InfixOperator::Minus | InfixOperator::Plus => 3,
			InfixOperator::Divide | InfixOperator::Times => 2,
			InfixOperator::Raise => 1
		}
	}
}
macro_rules! return_if_some {
( $($e:expr),+ ) => { {
   		$(
        	match $e {
            	Some(x) => return Some(x),
            	None => ()
        	};
        )+
    }
}
}
impl<'a> Iterator for Parser<'a> {
	type Item = ParserResult<'a>;
	fn next(&mut self) -> Option<ParserResult<'a>> {
		return_if_some!(
			self.next_prefix_unary(),
			self.next_parentheses(),
			self.next_expression()
		);
		None
	}

}
const HIGHEST_PRECEDENCE: i32 = 4;
impl<'a> Parser<'a> {
	pub fn from_scanner(scanner: scanner::Scanner<'a>) -> Parser<'a> {
		Parser { scanner: scanner.peekable() }
	}

	fn next_prefix_unary(&mut self) -> Option<ParserResult<'a>> {
		None
	}
	fn next_parentheses(&mut self) -> Option<ParserResult<'a>> {
		match self.scanner.peek()?.token_type() {
			scanner::TokenType::Separator(scanner::Separator::OpenParentheses) => (),
			_ => return None
		};
		let mut clone = self.clone();
		let mut next = match clone.next()? {
			Ok(node) => node,
			Err(err) => return Some(Err(err))
		};
		match clone.scanner.next()?.token_type() {
			scanner::TokenType::Separator(scanner::Separator::OpenParentheses) => Some(Ok(Node::Scope(Box::new(next)))),
			_ => None
		}

	}
	fn next_expression(&mut self) -> Option<ParserResult<'a>> {
		None
	}
}