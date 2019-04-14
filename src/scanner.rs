use std::str::CharIndices;

#[derive(Clone)]
pub struct Cursor<'a> {
	iter: CharIndices<'a>,
}
impl<'a> Cursor<'a> {
	pub fn peek(&mut self) -> char {
		match self.iter.peekable().peek() {
			Some((s, c)) => *c,
			None => char::from(0)
		}
	}
	fn next(&mut self) -> char {
		let (index, c) = self.iter.next().expect("unexpected end of file");
	}
	fn maybe(&mut self, c: char) -> bool {
		if self.peek() == c {
			self.next();
			return true;
		}
		return false;
	}
	fn maybe_digits(&mut self) -> bool {
		if !self.maybe_digit() {
			return false
		}
		while self.maybe_digit() {}
		return true
	}
	fn maybe_digit(&mut self) -> bool {
		if self.peek().is_digit(10) {
			self.next();
			return true;
		}
		return false;
	}

	fn maybe_alpha(&mut self) -> bool {
		if self.peek().is_alphabetic() {
			self.next();
			return true;
		}
		return false;
	}
	fn maybe_alphanumeric(&mut self) -> bool {
		if self.peek().is_alphanumeric() {
			self.next();
			return true;
		}
		return false;
	}
	fn maybe_alphanumerics(&mut self) -> bool {
		if !self.maybe_alphanumeric() {
			return false;
		}
		while self.maybe_alphanumeric() {}
		return true;
	}
	fn content(&self) -> &'a str {
		let s = self.iter.as_str();
		s[..self.iter.clone().next().unwrap_or((s.len(), char::from(0))).1]
	}
}

pub struct Scanner<'a> {
	content: &'a str,
}

pub enum Token<'a> {
	Float(f64),
	Integer(i32),
	Word(&'a str),
	Operator(char)
}
impl<'a> Scanner<'a> {
	fn iter(&self) -> CharIndices<'a> {
		self.content[self.position..].chars_indices().clone()
	}
	pub fn get_cursor(&mut self) -> Cursor<'a> {
		while *self.position.peek().unwrap_or(&'\u{0}') == ' ' {
			self.position.next();
		}
		Cursor {
			iter: self.iter(),
		}

	}
	fn apply(&mut self, cursor: &Cursor<'a>) {
		self.position += cursor.size;
	}
	pub fn next_int(&mut self) -> Option<i32> {
		let mut c = self.get_cursor();
		c.maybe('-');
		if !c.maybe_digits() {
			return None
		}
		if c.peek() == '.' {
			return None
		}
		self.apply(c);
		Some(c.content().parse().unwrap())
	}
	pub fn next_float(&mut self) -> Option<f64> {
		let mut c = self.get_cursor();
		c.maybe('-');

		if !c.maybe_digits() || !c.maybe('.') {
			return None
		}
		c.maybe_digits();
		Some(c.content().parse().unwrap())
	}
	pub fn next_word(&mut self) -> Option<&'a str> {
		let mut c = self.get_cursor();
		if !c.maybe_alpha() {
			return None
		}
		c.maybe_alphanumeric();
		self.apply(&c);
		Some(c.content())
	}
	pub fn next_operator(&mut self) -> Option<char> {
		let mut c = self.get_cursor();
		let is_operator = match c.peek() {
			'+' => true,
			'-' => true,
			'*' => true,
			'/' => true,
			//'(' => true,
			//')' => true,
			'=' => true,
			//'>' => true,
			//'<' => true,
			_ => false
		};
		if !is_operator {
			return None
		}
		let op = c.next();
		self.apply(&c);
		Some(op)
	}
	pub fn next_token(&mut self) -> Option<Token> {
		if let Some(word) = self.next_word() {
			return Some(Token::Word(word));
		}
		if let Some(int) = self.next_int() {
			return Some(Token::Integer(int));
		}
		if let Some(float) = self.next_float() {
			return Some(Token::Float(float));
		}
		if let Some(operator) = self.next_operator() {
			return Some(Token::Operator(operator));
		}
		None
	}
	pub fn new(input: &'a str) -> Scanner<'a> {
		Scanner {
			content: input
		}
	}
}
