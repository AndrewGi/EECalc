use std::str::CharIndices;
use crate::si;
use crate::si::UnitWithExponent;

#[derive(Clone)]
pub struct Cursor<'a> {
	iter: CharIndices<'a>,
}

impl<'a> Cursor<'a> {
	#[warn(dead_code)]
	pub fn peek(&self) -> char {
		self.iter.clone().next().unwrap_or((0, '\u{0}')).1
	}
	fn next(&mut self) -> char {
		self.iter.next().expect("unexpected end of file").1
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
			return false;
		}
		while self.maybe_digit() {}
		return true;
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
	fn current_pos(&self) -> usize {
		self.iter.clone().last().unwrap_or((0, ' ')).0
	}
	fn as_str(&self) -> &'a str {
		let s = self.iter.as_str();
		&s[..self.iter.clone().next().unwrap_or((s.len(), char::from(0))).0]
	}
	pub fn next_unit_exponent(&mut self) -> Option<si::UnitWithExponent> {
		let pos = self.iter.clone();
		let start_index = self.current_pos();
		let undo = || self.iter = pos;
		loop {
			if self.next_word().is_none() {
				break;
			}
			if self.maybe('^') {
				if self.next_int().is_none() {
					undo();
					return None;
				}
			} else {
				if !(self.maybe('*') || self.maybe('/')) {
					break;
				}
			}
		}
		if self.current_pos() == start_index {
			None
		} else {
			Some(self.as_str()[start_index..].parse().unwrap())
		}
	}
	pub fn next_value(&mut self) -> Option<si::Value> {
		let (number, did_get_number) = match self.next_float() {
			Some(f) => (f, true),
			None => (1f64, false)
		};
		Some(match self.next_unit_exponent() {
			Some(ue) => ue,
			None if did_get_number => UnitWithExponent::default(),
			None => return None
		}.make_value(number))
	}

	pub fn next_int(&mut self) -> Option<i32> {
		let start_iter = self.iter.clone();
		{ //So undo can borrow pos
			let undo = || self.iter = start_iter;
			self.maybe('-');
			if !self.maybe_digits() {
				undo();
				return None;
			}
			if self.peek() == '.' {
				undo();
				return None;
			}
		}
		Some(self.as_str()[(start_iter.last()?.0)..].parse().unwrap())
	}

	pub fn next_float(&mut self) -> Option<f64> {
		let pos = self.iter.clone();
		let start_index = self.current_pos();
		self.maybe('-');

		if !self.maybe_digits() || !self.maybe('.') {
			self.iter = pos;
			return None;
		}
		self.maybe_digits();
		Some(self.as_str()[start_index..].parse().unwrap())
	}

	pub fn next_word(&mut self) -> Option<&'a str> {
		if !(self.maybe_alpha() || self.maybe('_')) {
			return None;
		}
		while self.maybe_alphanumeric() || self.maybe('_') {};
		Some(self.as_str())
	}

	pub fn next_operator(&mut self) -> Option<char> {
		let is_operator = match self.peek() {
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
			return None;
		}
		Some(self.next())
	}
}

pub struct Scanner<'a> {
	content: &'a str,
}

pub enum Token<'a> {
	Float(f64),
	Integer(i32),
	Word(&'a str),
	Operator(char),
}

impl<'a> Scanner<'a> {
	fn iter(&self) -> CharIndices<'a> {
		self.content.char_indices().clone()
	}
	pub fn get_cursor(&mut self) -> Cursor<'a> {
		self.content = &self.content[self.content.find(' ').unwrap_or(0)..];
		Cursor {
			iter: self.iter()
		}
	}
	fn apply(&mut self, cursor: &Cursor<'a>) {
		self.content = cursor.iter.as_str();
	}

	pub fn new(input: &'a str) -> Scanner<'a> {
		Scanner {
			content: input
		}
	}
}
