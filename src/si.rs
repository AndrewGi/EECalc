
pub struct Unit {
	meter: i32,
	gram: i32,
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
			candela: self.candela + other.candela
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
}
pub struct Value {
	unit: BaseUnit,
	value: f32
}