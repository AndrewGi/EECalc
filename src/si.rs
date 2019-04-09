
#[derive(Eq)]
pub struct Unit {
	meter: i32,
	kilogram: i32,
	second: i32,
	ampere: i32,
	kelvin: i32,
	mole: i32,
	candela: i32
}

use std::fmt;
impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.meter != 0 {
			write!(f, "m{}", self.meter);
		}
		if self.kilogram != 0 {
			write!(f, "g{}", self.kilogram);
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
			kilogram: self.kilogram + other.kilogram,
			second: self.second + other.second,
			ampere: self.ampere + other.ampere,
			kelvin: self.kelvin + other.kelvin,
			mole: self.mole + other.mole,
			candela: self.candela + other.candela
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
	pub fn divide(&self, other: &Unit) -> Unit {
		self.mulitply(other.invert())
	}
	pub fn new(number: f32, unit: &Unit) -> Value {
		Value {unit: unit, value: number}
	}
}
#[derive(Eq, Clone)]
pub struct Value {
	unit: BaseUnit,
	value: f32
}
impl Value {
	pub fn add(&self, other: &Value) -> Option<Value> {
		if self != other { return None }
		Value::new(self.value+other.value, self.unit)
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
		Value::new(self.value*other.value, self.unit.multiply(other.unit))
	}
	pub fn invert(&self) -> Value {
		Value::new(self.value.recip(), self.unit.invert())
	}
	pub fn divide(&self, other: &Value) -> Value {
		self.multiply(other.invert())
	}
}
static Meter: (Unit, &str) = (Unit { meter: 1, kilogram: 0, second: 0, ampere: 0, kelvin: 0, mole: 0, candela: 0 }, "m");
static Kilogram: Unit = Unit { meter: 0, kilogram: 1, second: 0, ampere: 0, kelvin: 0, mole: 0, candela: 0 };
static Second: Unit = Unit { meter: 0, kilogram: 0, second: 1, ampere: 0, kelvin: 0, mole: 0, candela: 0 };
static Ampere: Unit = Unit { meter: 0, kilogram: 0, second: 0, ampere: 1, kelvin: 0, mole: 0, candela: 0 };
static Kelvin: Unit = Unit { meter: 0, kilogram: 0, second: 0, ampere: 0, kelvin: 1, mole: 0, candela: 0 };
static Mole: Unit = Unit { meter: 0, kilogram: 0, second: 0, ampere: 0, kelvin: 0, mole: 1, candela: 0 };
static Candela: Unit = Unit { meter: 0, kilogram: 0, second: 0, ampere: 0, kelvin: 0, mole: 0, candela: 1 };
static Ohm: (Unit, &str) = (Ampere.divide(Volt), "v");
static Colomb: (Unit, &str) = (Ampere.mulitply(Second), "C");

