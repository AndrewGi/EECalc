#[derive(Eq)]
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
			write!(f, "m{}", self.meter);
		}
		if self.gram != 0 {
			write!(f, "g{}", self.gram);
		}
		if self.second != 0 {
			write!(f, "s{}", self.second);
		}
		if self.ampere != 0 {
			write!(f, "a{}", self.ampere);
		}
		if self.kelvin != 0 {
			write!(f, "k{}", self.kelvin);
		}
		if self.mole != 0 {
			write!(f, "m{}", self.mole);
		}
		if self.candela != 0 {
			write!(f, "cd{}", self.candela);
		}
		f
	}
}

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
	pub fn new(number: f32, unit: &Unit) -> Value {
		Value { unit: unit, value: number }
	}

	pub fn get_unit(name: &str) -> Option<Unit> {
		
	}

	pub fn from_string(s: &str) -> Option<Unit> {

	}
}

#[derive(Eq, Clone)]
pub struct Value {
	unit: BaseUnit,
	value: f32,
}

impl Value {
	pub fn add(&self, other: &Value) -> Option<Value> {
		if self != other { return None; }
		Value::new(self.value + other.value, self.unit)
	}
	pub fn negate(&self) -> Value {
		Value::new(-self.value, self.unit)
	}
	pub fn subtract(&self, other: &Value) -> Option<Value> {
		if let Some(v) = other.negate() {
			return self.add(v);
		}
		None
	}
	pub fn multiply(&self, other: &Value) -> Value {
		Value::new(self.value * other.value, self.unit.multiply(other.unit))
	}
	pub fn invert(&self) -> Value {
		Value::new(self.value.recip(), self.unit.invert())
	}
	pub fn divide(&self, other: &Value) -> Value {
		self.multiply(&other.invert())
	}
}

struct UnitInfo {
	shorthand: &'static str,
	longhand: &'static str,
}

impl UnitInfo {
	pub fn new(shorthand: &'static str, longhand: &'static str) -> UnitInfo {
		UnitInfo { shorthand: shorthand, longhand: longhand }
	}
}


static mut longhand_hm: HashMap<&'static str, (&'static str, Unit)> = HashMap::new();
static mut shorthand_hm: HashMap<&'static str, (&'static str, Unit)> = HashMap::new();
static mut unit_hm: HashMap<Unit, (&'static str, &'static str)> = HashMap::new();
fn new_unit(longhand: &'static str, shorthand: &'static str, unit: Unit) {
	longhand_hm.insert(longhand, (shorthand, unit.clone()));
	shorthand_hm.insert(shorthand, (longhand, unit.clone()));
	unit_hm.insert(unit.clone(), (longhand, shorthand));
}

pub fn new_unit_rule(longhand: &'static str, shorthand: &'static str, rule: &str) {

}


pub fn create_units() {

	let scalar = Unit { meter: 0, gram: 0, second: 0, ampere: 0, kelvin: 0, mole: 0, candela: 0 };
	new_baseunit("scalar", "_", scalar);
	new_baseunit("meter", "m", Unit { meter: 1, ..scaler });
	new_baseunit("gram", "g", Unit { gram: 1, ..scalar });
	new_baseunit("second", "s", Unit { second: 1, ..scalar });
	new_baseunit("ampere", "a", Unit { ampere: 1, ..scalar });
	new_baseunit("kelvin", "k", Unit { kelvin: 1, ..scalar });
	new_baseunit("mole", "mol", Unit { mole: 1, ..scalar });
	new_baseunit("candela", "cd", Unit { candela: 1, ..scalar });
	new_unit_rule("newton", "n", "kg*m/s^2");
	new_unit_rule("joule", "j", "n*m");
	new_unit_rule("watt", "w", "j*s");
	new_unit_rule("volt", "v", "w/v");
}

