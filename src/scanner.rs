use std::str::CharIndices;
use crate::si;
use crate::si::{UnitWithExponent, Unit};

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
    fn maybe_alphas(&mut self) -> bool {
        if !self.maybe_alpha() {
            return false;
        }
        while self.maybe_alpha() {}
        return true;
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
    fn content(&self) -> &'a str {
        let s = self.iter.as_str();
        &s[..self.iter.clone().next().unwrap_or((s.len(), char::from(0))).0]
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
    pub fn next_unit_exponent(&mut self) -> Option<UnitWithExponent> {
        let mut c = self.get_cursor();
        loop {
            if !c.maybe_alphas() {
                break;
            }
            if c.maybe('^') {
                c.maybe('-');
                if !c.maybe_digits() {
                    return None;
                }
            } else {
                if !(c.maybe('*') || c.maybe('/')) {
                    break;
                }
            }
        }
        None
    }
    pub fn next_value(&mut self) -> Option<Value> {
        let c = self.get_cursor();
        let undo = || self.apply(&c); //Undo any 'next_*' movement we did
        let (number, did_get_number) = match self.next_float() {
            Some(f) => (f, true),
            None => (1f64, false)
        };
        }.make_value(number))
    }

    pub fn next_int(&mut self) -> Option<i32> {
        let mut c = self.get_cursor();
        c.maybe('-');
        if !c.maybe_digits() {
            return None;
        }
        if c.peek() == '.' {
            return None;
        }
        self.apply(&c);
        Some(c.content().parse().unwrap())
    }

    pub fn next_float(&mut self) -> Option<f64> {
        let mut c = self.get_cursor();
        c.maybe('-');

        if !c.maybe_digits() || !c.maybe('.') {
            return None;
        }
        c.maybe_digits();
        Some(c.content().parse().unwrap())
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        let mut c = self.get_cursor();
        if !c.maybe_alpha() {
            return None;
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
            return None;
        }
        let op = c.next();
        self.apply(&c);
        Some(op)
    }

    pub fn new(input: &'a str) -> Scanner<'a> {
        Scanner {
            content: input
        }
    }
}
