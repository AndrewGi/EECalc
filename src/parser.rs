
use crate::scanner::Scanner;
struct Parser<'a> {
	scanner: Scanner<'a>
}
enum BaseUnit {
	Meter,
	Watt,
	Gram,
	Amp,
	Volt,
	Second,
	Ohm,
	Joule
}
enum Number {
	Int(i32),
	Float(f32)
}
struct Value {
	number: Number,
	unit: BaseUnit,
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
		get_unit(word)
	}
	pub fn get_unit(word: &str) -> Option<(i32, Option<BaseUnit>)> {
		let s = word.unwrap().as_str();
		if s.len() == 0 {
			return None
		}
		let mut iter = s.chars();
		let c = iter.next().unwrap();
		let scaler = match c {
			'g' => 9,
			'k' => 3,
			'm' => -3,
			'u'=> -6,
			'n' => -9,
			'p' => -12,
			'f' => -15,
			'a' => -18,
			_ => 0,
		};
		if scaler == 0 && s.len() == 1 {
			None
		}
		let buc = if scaler == 0 {c} else {iter.next().unwrap()};
		let baseunit = match s[unit_offset..] {
			'M' => Some(BaseUnit::Meter),
			'W' => Some(BaseUnit::Watt),
			'G' => Some(BaseUnit::Gram),
			'A' => Some(BaseUnit::Amp),
			'S' => Some(BaseUnit::Second),
			'R' => Some(BaseUnit::Ohm),
			'J' => Some(BaseUnit::Joule),
			_ => None
		};
		return Some((scaler, baseunit))
	}
}
