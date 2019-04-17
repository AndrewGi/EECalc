use std::collections::HashMap;
use std::fmt;

use crate::si::ParseUnitError::{IntegerParseError, UnrecognizedUnit};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Unit {
	meter: i32,
	kilogram: i32,
	second: i32,
	ampere: i32,
	kelvin: i32,
	mole: i32,
	candela: i32,
}

impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.meter != 0 {
			write!(f, "m{}", self.meter)?
		}
		if self.kilogram != 0 {
			write!(f, "g{}", self.kilogram)?
		}
		if self.second != 0 {
			write!(f, "s{}", self.second)?
		}
		if self.ampere != 0 {
			write!(f, "a{}", self.ampere)?
		}
		if self.kelvin != 0 {
			write!(f, "k{}", self.kelvin)?
		}
		if self.mole != 0 {
			write!(f, "m{}", self.mole)?
		}
		if self.candela != 0 {
			write!(f, "cd{}", self.candela)?
		}
		Ok(())
	}
}

pub struct UnitRules {
	longhand_hm: HashMap<&'static str, (&'static str, (Uint, i32))>,
	shorthand_hm: HashMap<&'static str, (&'static str, (Unit, i32))>,
	unit_hm: HashMap<Unit, (&'static str, &'static str)>,
}

impl Default for UnitRules {
	fn default() -> UnitRules {
		let mut r = UnitRules::new();
		let scalar = Unit { meter: 0, kilogram: 0, second: 0, ampere: 0, kelvin: 0, mole: 0, candela: 0 };
		r.new_unit("scalar", "_", scalar);
		r.new_unit("meter", "m", Unit { meter: 1, ..scalar });
		r.new_unit("kilogram", "kg", Unit { kilogram: 1, ..scalar });
		r.new_unit("second", "s", Unit { second: 1, ..scalar });
		r.new_unit("ampere", "a", Unit { ampere: 1, ..scalar });
		r.new_unit("kelvin", "k", Unit { kelvin: 1, ..scalar });
		r.new_unit("mole", "mol", Unit { mole: 1, ..scalar });
		r.new_unit("candela", "cd", Unit { candela: 1, ..scalar });
		r.new_unit_rule("gram", "g", "mkg");
		r.new_unit_rule("newton", "n", "kg*m/s^2");
		r.new_unit_rule("joule", "j", "n*m");
		r.new_unit_rule("watt", "w", "j*s");
		r.new_unit_rule("volt", "v", "w/a");
		r
	}
}

impl UnitRules {
	pub fn new() -> UnitRules {
		UnitRules { longhand_hm: HashMap::new(), shorthand_hm: HashMap::new(), unit_hm: HashMap::new() }
	}
	fn new_unit(longhand: &'static str, shorthand: &'static str, unit: Unit) {
		longhand_hm.insert(longhand, (shorthand, (unit.clone(), 0)));
		shorthand_hm.insert(shorthand, (longhand, (unit.clone(), 0)));
		unit_hm.insert(unit.clone(), (longhand, shorthand));
	}

	fn new_unit_rule(longhand: &'static str, shorthand: &'static str, rule: &str) {
		let mut iter = rule.split(|c: char| c == '/' || c == '*' || c == '^');
	}
}

pub static mut GLOBAL_RULES: UnitRules = UnitRules::default();

impl Unit {
	fn get_single_unit(s: &str) -> Option<(Unit, i32)> {
		unsafe {
			if let Some(result) = GLOBAL_RULES.shorthand_hm.get(&s) {
				Some(((result.1).0, (result.1).1))
			} else {
				if let Some(result) = GLOBAL_RULES.shorthand_hm.get(&s[1..]) {
					let prefix = get_prefix(s.chars.next())?;
					Some(((result.1).0, prefix + (result.1).1))
				} else {
					None
				}
			}
		}
	}


	pub fn raise(&self, power: i32) -> Unit {
		Unit {
			meter: self.meter * power,
			kilogram: self.kilogram * power,
			second: self.second * power,
			ampere: self.ampere * power,
			kelvin: self.kelvin * power,
			mole: self.mole * power,
			candela: self.candela * power,
		}
	}

	pub fn invert(&self) -> Unit {
		Unit {
			meter: -self.meter,
			kilogram: -self.kilogram,
			second: -self.second,
			ampere: -self.ampere,
			kelvin: -self.kelvin,
			mole: -self.mole,
			candela: -self.candela,
		}
	}

	pub fn get_prefix(c: char) -> Option<i32> {
		match c {
			'g' => Some(9),
			'M' => Some(6),
			'k' => Some(3),
			'c' => Some(-2),
			'm' => Some(-3),
			'u' => Some(-6),
			'n' => Some(-9),
			'p' => Some(-12),
			'f' => Some(-15),
			'a' => Some(-18),
			_ => None,
		}
	}
}

impl std::ops::Mul for Unit {
	type Output = Self;
	fn mul(&self, other: &self) -> self::Output {
		Unit {
			meter: self.meter + other.meter,
			kilogram: self.kilogram + other.kilogram,
			second: self.second + other.second,
			ampere: self.ampere + other.ampere,
			kelvin: self.kelvin + other.kelvin,
			mole: self.mole + other.mole,
			candela: self.candela + other.candela,
		}
	}
}

impl std::ops::Div for Unit {
	type Output = Self;
	fn div(&self, other: &self) -> self::Output {
		Unit {
			meter: self.meter - other.meter,
			kilogram: self.kilogram - other.kilogram,
			second: self.second - other.second,
			ampere: self.ampere - other.ampere,
			kelvin: self.kelvin - other.kelvin,
			mole: self.mole - other.mole,
			candela: self.candela - other.candela,
		}
	}
}

pub enum ParseUnitError {
	UnrecognizedUnit(usize, usize),
	IntegerParseError(usize, usize),
	EarlyEndOfLine,
}

impl std::str::FromStr for Unit {
	type Err = ParseUnitError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let is_operator = |c: char| c == '*' || c == '/';
		let mut out_unit = Unit::new();
		let mut last_pos = 0;
		let mut last_operator = '*';
		let mut exponent_scalar = 0;
		loop {
			let mut next_pos = s[last_pos..].find(is_operator).unwrap_or_else(s.len);
			let part = &s[last_pos..next_pos];
			let c = if next_pos == s.len() { ' ' } else { s[next_pos] };

			//check for '^(some_number)`
			let raise_power = if c == '^' {
				let exponent_start_index = next_pos + 1;
				let exponent_end_index = s[exponent_start_index..].find(is_operator).unwrap_or_else(s.len);
				next_pos = exponent_end_index;
				if let Ok(exponent) = s[exponent_start_index..exponent_end_index].parse::<i32>() {
					exponent
				} else {
					return Err(IntegerParseError(exponent_start_index, exponent_end_index));
				}
			} else {
				1
			};

			let unit = {
				let (unit, scalar) = match Unit::get_single_unit(&part) {
					Some(result) => result,
					None => return Err(UnrecognizedUnit(last_pos, next_pos));
				}
				exponent_scalar += scalar; //TODO: check if scalar needs to be raised to a power too
				unit.raise(raise_power)
			}W
			if next_pos == s.len() {
				break;
			}
			last_operator = c;
			last_pos = next_pos;

			let next_operator =
		}
		Ok(unit)
	}
}

#[derive(PartialEq, Clone, Copy)]
pub struct Value {
	unit: Unit,
	number: f64,
}

impl Value {
	pub fn new(number: f64, unit: Unit) -> Value {
		Value { unit, number }
	}
	pub fn from_str(s: &str) -> Option<Value> {
		if let Some(unit_index) = s.chars().position(|c| c.is_alphabetic()) {
			let (scalar, unit) = Unit::unit_and_scalar(&s[unit_index..])?;
			let num_result = s[..unit_index].parse::<f64>();
			if num_result.is_err() {
				return None;
			}
			let number = num_result.unwrap();
			Some(Value { number: number * 10f64.powi(scalar), unit: unit? })
		} else {
			let (scalar, unit) = Unit::unit_and_scalar(s)?;
			Some(Value { number: 10f64.powi(scalar), unit: unit? })
		}
	}
}

impl std::ops::Add<Self> for Value {
	type Output = Result<(), Value>;
	fn add(self, other: &Value) -> Self::Output {
		if self.unit != other.unit {
			Err(())
		} else {
			Ok(self.number + other.number)
		}
	}
}

impl std::ops::Neg for Value {
	type Output = Self;
	fn neg(mut self) -> Self::Output {
		self.number = -self.number;
		self
	}
}

impl std::ops::Sub<Self> for Value {
	type Output = Self;
	fn sub(self, other: &Value) -> Self::Output {
		self + -other
	}
}

impl std::ops::Mul<Self> for Value {
	type Output = Value;
	fn mul(&self, other: &Value) -> Self::Output {
		Value { unit: self.unit * other.unit, number: self.number * other.number }
	}
}

impl std::ops::Div<Self> for Value {
	type Output = Result<(), Value>;
	fn mul(&self, other: &Value) -> Self::Output {
		if other.number == 0f64 {
			Err(())
		} else {
			Some(Value { unit: self.unit / other.unit, number: self.number / other.number })
		}
	}
}

impl std::ops::Mul<f64> for Value {
	type Output = Value;
	fn mul(self, scalar: f64) -> Self::Output {
		self.number * scalar;
		self
	}
}

