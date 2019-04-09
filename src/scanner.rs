use std::str::Chars;
use std::iter::Peekable;

#[derive(Clone)]
pub struct Cursor<'a> {
	start: Peekable<Chars<'a>>,
	iter: Peekable<Chars<'a>>,
	size: usize,
}
impl<'a> Cursor<'a> {
	pub fn peek(&mut self) -> char {
		match self.iter.peek() {
			Some(c) => *c,
			None => char::from(0)
		}
	}
	fn next(&mut self) -> char {
		self.size += 1;
		self.iter.next().expect("unexpected end of file")
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
	fn content(self) -> String {
		self.start.take(self.size).collect()
	}
}

pub struct Scanner<'a> {
	position: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
	pub fn get_cursor(&mut self) -> Cursor<'a> {
		while *self.position.peek().unwrap_or(&'\u{0}') == ' ' {
			self.position.next();
		}
		Cursor {
			start: self.position.clone(),
			iter: self.position.clone(),
			size: 0,
		}

	}
	fn apply(&mut self, cursor: &Cursor<'a>) {
		self.position = cursor.iter.clone();
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
		self.apply(&c);
		Some(c.content().parse().unwrap())
	}
	pub fn next_float(&mut self) -> Option<f32> {
		let mut c = self.get_cursor();
		c.maybe('-');

		if !c.maybe_digits() || !c.maybe('.') {
			return None
		}
		c.maybe_digits();
		Some(c.content().parse().unwrap())
	}
	pub fn next_word(&mut self) -> Option<String> {
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
	pub fn new(input: &'a str) -> Scanner<'a> {
		Scanner {
			position: input.chars().peekable()
		}
	}
}
