use std::collections::HashMap;
use std::fmt;
use crate::si::ParseUnitError::{IntegerParseError, UnrecognizedUnit, EarlyEndOfLine};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
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
            write!(f, "m^{}", self.meter)?
        }
        if self.kilogram != 0 {
            write!(f, "g^{}", self.kilogram)?
        }
        if self.second != 0 {
            write!(f, "s^{}", self.second)?
        }
        if self.ampere != 0 {
            write!(f, "a^{}", self.ampere)?
        }
        if self.kelvin != 0 {
            write!(f, "k^{}", self.kelvin)?
        }
        if self.mole != 0 {
            write!(f, "m^{}", self.mole)?
        }
        if self.candela != 0 {
            write!(f, "cd^{}", self.candela)?
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct UnitWithExponent {
    unit: Unit,
    power10: i32,
}

impl UnitWithExponent {
    pub fn new(unit: Unit, power10: i32) -> UnitWithExponent {
        UnitWithExponent {
            unit,
            power10,
        }
    }
    pub fn raise_scalar_exponent(mut self, power10: i32) -> UnitWithExponent {
        self.power10 += power10;
        self
    }
}

pub struct UnitRules {
    longhand_hm: HashMap<&'static str, (&'static str, UnitWithExponent)>,
    shorthand_hm: HashMap<&'static str, (&'static str, UnitWithExponent)>,
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
#[derive(Debug)]
pub enum ParseUnitError {
    UnrecognizedUnit(usize, usize),
    IntegerParseError(usize, usize),
    EarlyEndOfLine,
}

impl std::str::FromStr for UnitWithExponent {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_operator = |c: char| c == '*' || c == '/';
        let mut out_unit = Unit::default();
        let mut current_pos = 0;
        let mut last_operator = '*';
        let mut exponent_scalar = 0;
        loop {
            let unit_end_index = s[current_pos..].find(is_operator).unwrap_or_else(|| s.len());
            let char_at = |pos: usize| if pos == s.len() { ' ' } else { char::from(s.as_bytes()[pos]) };
            let unit_s = &s[unit_end_index..unit_end_index];


            //check for '^(some_number)`
            let (raise_power, part_end_index) = if char_at(unit_end_index) == '^' {
                let exponent_start_index = unit_end_index + 1;
                if exponent_start_index == s.len() {
                    break Err(EarlyEndOfLine);
                }
                let exponent_end_index = s[exponent_start_index..].find(is_operator).unwrap_or_else(|| s.len());
                if let Ok(exponent) = s[exponent_start_index..exponent_end_index].parse::<i32>() {
                    (exponent, exponent_end_index)
                } else {
                    break Err(IntegerParseError(exponent_start_index, exponent_end_index));
                }
            } else {
                (1, unit_end_index)
            };

            let unit = {
                let ue = match UnitWithExponent::from_str_single(&unit_s) {
                    Some(result) => result,
                    None => break Err(UnrecognizedUnit(current_pos, part_end_index))
                };
                exponent_scalar += ue.power10; //TODO: check if scalar needs to be raised to a power too
                ue.unit.raise(raise_power)
            };
            out_unit = match last_operator {
                '*' => &out_unit * &unit,
                '/' => &out_unit / &unit,
                _ => panic!("unhandled unit operator")
            };
            last_operator = char_at(current_pos);
            if last_operator == ' ' {
                break Ok(UnitWithExponent { unit, power10: exponent_scalar });
            }
            current_pos = part_end_index;
        }
    }
}

impl UnitWithExponent {
    pub fn make_value(&self, value: f64) -> Value {
        Value {
            unit: self.unit.clone(),
            number: value * 10f64.powi(self.power10),
        }
    }

    fn from_str_single(s: &str) -> Option<UnitWithExponent> {
        if let Some((_, ue)) = GLOBAL_RULES.shorthand_hm.get(&s) {
            Some(ue.clone())
        } else {
            if let Some((_, ue)) = GLOBAL_RULES.shorthand_hm.get(&s[1..]) {
                let prefix = Unit::get_prefix(s.chars().next()?)?;
                Some(ue.clone().raise_scalar_exponent(prefix))
            } else {
                None
            }
        }
    }
}

impl UnitRules {
    pub fn new() -> UnitRules {
        UnitRules { longhand_hm: HashMap::new(), shorthand_hm: HashMap::new(), unit_hm: HashMap::new() }
    }
    pub fn new_unit_exponent(&mut self, longhand: &'static str, shorthand: &'static str, ue: UnitWithExponent) {
        debug_assert!(longhand.len() > shorthand.len());
        self.longhand_hm.insert(longhand, (shorthand, ue.clone()));
        self.shorthand_hm.insert(shorthand, (longhand, ue.clone()));
        self.unit_hm.insert(ue.unit.clone(), (longhand, shorthand));
    }
    pub fn new_unit(&mut self, longhand: &'static str, shorthand: &'static str, unit: Unit) {
        self.new_unit_exponent(longhand, shorthand, UnitWithExponent::new(unit, 0));
    }

    pub fn new_unit_rule(&mut self, longhand: &'static str, shorthand: &'static str, rule: &str) {
        self.new_unit_exponent(longhand, shorthand, rule.parse().ok().unwrap())
    }
}

lazy_static! {
    static ref GLOBAL_RULES: UnitRules = UnitRules::default();
}
impl Unit {
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

impl std::ops::Mul for &Unit {
    type Output = Unit;
    fn mul(self, other: &Unit) -> Self::Output {
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

impl std::ops::Div for &Unit {
    type Output = Unit;
    fn div(self, other: &Unit) -> Self::Output {
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Value {
    unit: Unit,
    number: f64,
}

impl Value {
    pub fn new(number: f64, unit: Unit) -> Value {
        Value { unit, number }
    }
    pub fn from_str(s: &str) -> Result<Value, ParseUnitError> {
        Ok(if let Some(unit_index) = s.chars().position(char::is_alphabetic) {
            let ue: UnitWithExponent = s[unit_index..].parse()?;
            let number: f64 = match s[..unit_index].parse() {
                Ok(number) => number,
                Err(_) => return Err(IntegerParseError(0, unit_index))
            };
            ue.make_value(number)
        } else {
            s.parse::<UnitWithExponent>()?.make_value(1f64)
        })
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.unit)
    }
}
impl std::ops::Add<&Value> for Value {
    type Output = Result<Value, ()>;
    fn add(mut self, other: &Value) -> Self::Output {
        if self.unit != other.unit {
            Err(())
        } else {
            self.number += other.number;
            Ok(self)
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

impl std::ops::Sub<&Self> for Value {
    type Output = Result<Value, ()>;
    fn sub(mut self, other: &Value) -> Self::Output {
        if self.unit != other.unit {
            Err(())
        } else {
            self.number -= other.number;
            Ok(self)
        }
    }
}

impl std::ops::Mul for &Value {
    type Output = Value;
    fn mul(self, other: &Value) -> Self::Output {
        Value { unit: &self.unit * &other.unit, number: self.number * other.number }
    }
}

impl std::ops::Div for &Value {
    type Output = Result<Value, ()>;
    fn div(self, other: &Value) -> Self::Output {
        if other.number == 0f64 {
            Err(())
        } else {
            Ok(Value { unit: &self.unit / &other.unit, number: self.number / other.number })
        }
    }
}

impl std::ops::Mul<f64> for Value {
    type Output = Value;
    fn mul(mut self, scalar: f64) -> Self::Output {
        self.number *= scalar;
        self
    }
}

