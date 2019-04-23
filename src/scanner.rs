use std::str::CharIndices;
use crate::si;
use crate::si::UnitWithExponent;

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    start: &'a str,
    current_index: usize,
    iter: CharIndices<'a>,
}

impl<'a> Cursor<'a> {
    #[warn(dead_code)]
    pub fn peek(&self) -> char {
        self.iter.clone().next().unwrap_or((0, ' ')).1
    }
    fn next(&mut self) -> char {
        let (i, c) = self.iter.next().expect("unexpected end of file");
        self.current_index = i;
        c
    }
    fn eat_whitespace(iter: &mut CharIndices<'a>) {
        while (iter.clone().next().unwrap_or((0, char::from(0u8))).1).is_whitespace() {
            iter.next();
        }
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
        self.current_index
    }
    fn len(&self) -> usize {
        self.iter.clone().next().unwrap_or_else(|| (self.iter.as_str().len(), ' ')).0
    }
    fn as_str(&self) -> &'a str {
        &self.start[..self.current_pos()]
    }
    fn excluding_as_str(&self) -> &'a str {
        &self.iter.as_str()[self.len()..]
    }
    pub fn next_unit_exponent(&mut self) -> Option<si::UnitWithExponent> {
        let pos = self.iter.clone();
        let start_index = self.current_pos();
        Self::eat_whitespace(&mut self.iter);
        loop {
            if self.next_word().is_none() {
                break;
            }
            if self.maybe('^') {
                if self.next_int().is_none() {
                    self.iter = pos;
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
        Self::eat_whitespace(&mut self.iter);
        let start_index = self.current_pos();
        self.maybe('-');
        if !self.maybe_digits() {
            self.iter = start_iter;
            return None;
        }
        if self.peek() == '.' {
            self.iter = start_iter;
            return None;
        }
        Some(self.as_str()[start_index..].parse().unwrap())
    }

    pub fn next_float(&mut self) -> Option<f64> {
        let pos = self.iter.clone();
        let start_index = self.current_pos();
        Self::eat_whitespace(&mut self.iter);
        self.maybe('-');
        let was_digits = self.maybe_digits();
        let was_period = self.maybe('.');

        if !was_digits && !was_period {
            self.iter = pos; //no float
            return None;
        }
        self.maybe_digits();
        Some(self.as_str()[start_index..].parse().unwrap())
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        let start_iter = self.iter.clone();
        Self::eat_whitespace(&mut self.iter);
        let start_index = self.current_pos();
        println!("index {}", self.peek());
        if !(self.maybe_alpha() || self.maybe('_')) {
            self.iter = start_iter;
            return None;
        }
        while self.maybe_alphanumeric() || self.maybe('_') {};
        Some(&self.as_str()[start_index..])
    }

    pub fn peek_operator(&self) -> Option<char> {
        let mut iter = self.iter.clone();
        Self::eat_whitespace(&mut iter);
        let op = iter.next()?.1;
        println!("{}", iter.next()?.1);
        let is_operator = match op {
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
        Some(op)
    }
    pub fn next_operator(&mut self) -> Option<char> {
        let c = self.peek_operator()?;
        while self.next() != c {};
        Some(c)
    }
}

#[derive(Debug, Clone)]
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
    pub fn get_cursor(&self) -> Cursor<'a> {
        let iter = self.iter();
        let start = iter.as_str();
        Cursor {
            iter,
            current_index: 0,
            start,
        }
    }
    fn apply(&mut self, cursor: &Cursor<'a>) {
        self.content = cursor.excluding_as_str().trim_start();
    }

    pub fn new(input: &'a str) -> Scanner<'a> {
        Scanner {
            content: input.trim_start()
        }
    }
}
