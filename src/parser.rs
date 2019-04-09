
use crate::scanner::Scanner;
use crate::parser::Number::{Int, Float};
use crate::parser::BaseUnit::Scalar;

pub struct Parser<'a> {
	scanner: Scanner<'a>
}
#[derive(Clone)]
pub enum BaseUnit {
	Scalar,
	Meter,
	Watt,
	Gram,
	Amp,
	Volt,
	Newton,
	Second,
	Ohm,
	Joule
}
pub enum Number {
	Int(i32),
	Float(f32)
}
pub struct Value {
	number: Number,
	unit: BaseUnit,
}
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Assignment,
}
use std::fmt;

impl fmt::Display for BaseUnit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use BaseUnit::*;
		write!(f, "{}", match *self {
			Scalar => "_",
			Gram => "g",
			Watt => "w",
			Meter => "m",
			Amp => "a",
			Second => "s",
			Volt => "v",
			Ohm => "r",
			Joule => "j",
		})
	}
}
impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			Operator::Add => "+",
			Operator::Subtract => "-",
			Operator::Multiply => "*",
			Operator::Divide => "/",
			Operator::Assignment => "="
		})
	}
}
impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Int(int) => write!(f, "{}", int),
			Float(float) => write!(f, "{}", float)
		}
	}
}
impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.number, self.unit)
	}
}

pub fn which_unit(c: char) -> Option<BaseUnit> {
	match c {
		'm' => Some(BaseUnit::Meter),
		'w' => Some(BaseUnit::Watt),
		'g' => Some(BaseUnit::Gram),
		'v' => Some(BaseUnit::Volt),
		'a' => Some(BaseUnit::Amp),
		's' => Some(BaseUnit::Second),
		'r' => Some(BaseUnit::Ohm),
		'j' => Some(BaseUnit::Joule),
		'_' => Some(BaseUnit::Scalar),
		_ => None
	}
}
pub fn which_scalar(c: char) -> i32 {
	match c {
		'g' => 9,
		'M' => 6,
		'k' => 3,
		'm' => -3,
		'u'=> -6,
		'n' => -9,
		'p' => -12,
		'f' => -15,
		'a' => -18,
		_ => 0,
	}
}



impl BaseUnit {
	pub fn multiply_unit(&self, other: &BaseUnit) -> Option<BaseUnit> {
		if self == Scalar {
			return Some(other.clone())
		} else if other == Scalar {
			return Some(self.clone())
		}
		match self {
			Meter=> match other {
				Newton => Some(BaseUnit::Watt),
				_ => None
			},
			Watt => match other {
				_ => None
			},
			Gram=> match other {

			}
		}
	}
}
pub fn get_unit(word: &str) -> Option<(i32, Option<BaseUnit>)> {
	let mut iter = word.chars();
	match word.len() {
	 	1 => Some((0, which_unit(iter.next().unwrap()))),
		2 => {
			let scalar_c = iter.next().unwrap();
			let unit_c = iter.next().unwrap();
			return Some((which_scalar(scalar_c), which_unit(unit_c)))
		}
		_ => None
	}
}
impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		Parser {
			scanner: Scanner::new(input)
		}
	}
	pub fn next_number(&mut self) -> Option<Number> {
		if let Some(f) = self.scanner.next_float() {
			return Some(Number::Float(f))
		}
		if let Some(i) = self.scanner.next_int() {
			return Some(Number::Int(i))
		}
		return None
	}
	pub fn next_word(&mut self) -> Option<(i32, Option<BaseUnit>)> {
		let word = self.scanner.next_word();
		if word.is_none() {
			return None
		}
		get_unit(&word.unwrap())
	}
	pub fn next_operator(&mut self) -> Option<Operator> {
		if let Some(c) = self.scanner.next_operator() {
			return match c {
				'+' => Some(Operator::Add),
				'-' => Some(Operator::Subtract),
				'*' => Some(Operator::Multiply),
				'/' => Some(Operator::Divide),
				'=' => Some(Operator::Assignment),
				_ => None
			}
		}
		None

	}
	pub fn next_value(&mut self) -> Option<Value> {

		let number = self.next_number()?;
		if let Some((scalar, unit)) = self.next_word() {
			let out_number = if scalar == 0 {
				number
			} else {
				match number {
					Int(i) => Float(i as f32 * f32::powi(10.0, scalar)),
					Float(f) => Float(f * f32::powi(10.0, scalar))
				}
			};
			return Some(Value { number: out_number,
				unit: unit.unwrap_or(Scalar)
			})
		}
		None //TODO: undo parser movement
	}
}

