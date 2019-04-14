
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Unit {
	meter: i32,
	gram: i32,
	second: i32,
	ampere: i32,
	kelvin: i32,
	mole: i32,
	candela: i32,
}

use std::fmt;

impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.meter != 0 {
			write!(f, "m{}", self.meter)?
		}
		if self.gram != 0 {
			write!(f, "g{}", self.gram)?
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
		fmt::Result::Ok()
}
struct UnitScalar {
	unit: Unit,
	tenth_power: i32,
}
use std::collections::HashMap;

static mut longhand_hm: HashMap<&'static str, (&'static str, Unit)> = HashMap::new();
static mut shorthand_hm: HashMap<&'static str, (&'static str, Unit)> = HashMap::new();
static mut unit_hm: HashMap<Unit, (&'static str, &'static str)> = HashMap::new();

impl Unit {
	pub fn multiply(&self, other: &Unit) -> Unit {
		Unit {
			meter: self.meter + other.meter,
			gram: self.gram + other.gram,
			second: self.second + other.second,
			ampere: self.ampere + other.ampere,
			kelvin: self.kelvin + other.kelvin,
			mole: self.mole + other.mole,
			candela: self.candela + other.candela,
		}
	}
	pub fn invert(&self) -> Unit {
		Unit {
			meter: -self.meter,
			gram: -self.gram,
			second: -self.second,
			ampere: -self.ampere,
			kelvin: -self.kelvin,
			mole: -self.mole,
			candela: -self.candela,
		}
	}
	pub fn divide(&self, other: &Unit) -> Unit {
		self.mulitply(other.invert())
	}
	pub fn get_exponent_scalar(c: char) -> i32 {
		match c {
			'g' => 9,
			'M' => 6,
			'k' => 3,
			'c' => -2,
			'm' => -3,
			'u' => -6,
			'n' => -9,
			'p' => -12,
			'f' => -15,
			'a' => -18,
			_ => 0,
		}
	}
	pub fn from_shorthand(s: &str) -> Option<Unit> {
		unsafe { //Reeks
			shorthand_hm.get(s).clone();
		}
	}
	pub fn unit_and_scalar(s: &str) -> Option<(i32, Option<Unit>)> {
		if s.length() == 0 {
			return None
		}
		let c = s.chars().next().unwrap();
		let scalar = Unit::get_exponent_scalar(c);
		if s.length() == 1 && scalar != 0{
			Some((scalar, None))
		}
		Some((scalar, Some(Unit::from_shorthand(if scalar == 0 {s} else {s[1..]})?)))

	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Value {
	unit: Unit,
	number: f64,
}

impl Value {

	pub fn new(number: f64, unit: Unit) -> Value {
		Value { unit, number: number }
	}
	pub fn from_str(s: &str) -> Option<Value> {
		if let Some(unit_index) = s.chars().position(|c| c.is_alphabetic()) {
			let (scalar, unit) = Unit::unit_and_scalar(&s[unit_index..])?;
			let number: f64 = s[..unit_index].parse()?;
			return Some(Value {number: number*10f64.powi(scalar), unit: unit?});
		} else {
			let (scalar, unit) = Unit::unit_and_scalar(s)?;
			return Some(Value {number: 10f64.powi(scalar), unit: unit?});
		}
	}
	pub fn add(&self, other: &Value) -> Option<Value> {
		if self != other { return None; }
		Some(Value::new(self.number + other.number, self.unit))
	}
	pub fn negate(&self) -> Value {
		Value::new(-self.number, self.unit)
	}
	pub fn subtract(&self, other: &Value) -> Option<Value> {
		if let Some(v) = other.negate() {
			return self.add(v);
		}
		None
	}
	pub fn multiply(&self, other: &Value) -> Value {
		Value::new(self.number * other.number, self.unit.multiply(other.unit))
	}
	pub fn invert(&self) -> Value {
		Value::new(self.number.recip(), self.unit.invert())
	}
	pub fn divide(&self, other: &Value) -> Value {
		self.multiply(&other.invert())
	}
}

fn new_unit(longhand: &'static str, shorthand: &'static str, unit: Unit) {
	unsafe { //Reeks
		longhand_hm.insert(longhand, (shorthand, unit.clone()));
		shorthand_hm.insert(shorthand, (longhand, unit.clone()));
		unit_hm.insert(unit.clone(), (longhand, shorthand));
	}
}

pub fn new_unit_rule(longhand: &'static str, shorthand: &'static str, rule: &str) {

}


pub fn create_units() {

	let scalar = Unit { meter: 0, gram: 0, second: 0, ampere: 0, kelvin: 0, mole: 0, candela: 0 };
	new_unit("scalar", "_", scalar);
	new_unit("meter", "m", Unit { meter: 1, ..scalar });
	new_unit("gram", "g", Unit { gram: 1, ..scalar });
	new_unit("second", "s", Unit { second: 1, ..scalar });
	new_unit("ampere", "a", Unit { ampere: 1, ..scalar });
	new_unit("kelvin", "k", Unit { kelvin: 1, ..scalar });
	new_unit("mole", "mol", Unit { mole: 1, ..scalar });
	new_unit("candela", "cd", Unit { candela: 1, ..scalar });
	new_unit_rule("newton", "n", "kg*m/s^2");
	new_unit_rule("joule", "j", "n*m");
	new_unit_rule("watt", "w", "j*s");
	new_unit_rule("volt", "v", "w/a");
}

