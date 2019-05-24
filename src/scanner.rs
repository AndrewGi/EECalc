use crate::scanner::Separator::{OpenParentheses, CloseParentheses, Comma, Period, Colon};
use std::path::is_separator;
use crate::scanner::Operator::Plus;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    total_input: &'a str,
    position: usize
}

pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Raise,
    Equals,
}
pub enum Separator {
    Comma,
    Period,
    Colon,
    OpenParentheses,
    CloseParentheses,
}
pub enum TokenType {
    Word,
    Operator(Operator),
    Separator(Separator)
}
pub struct Token<'a> {
    content: &'a str,
    start: usize,
    token_type: TokenType
}
impl Separator {
    pub fn is_separator(c: char) -> Option<Separator> {
        match c {
            '(' => Some(OpenParentheses),
            ')' => Some(CloseParentheses),
            ',' => Some(Comma),
            '.' => Some(Period),
            ':' => Some(Colon),
            _ => None
        }
    }
}
impl Operator {
    pub fn is_operator(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Times),
            '/' => Some(Operator::Divide),
            '^' => Some(Operator::Raise),
            '=' => Some(Operator::Equals),
            _ => None
        }
    }
}
impl<'a> Iterator<'a> for Scanner<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Token<'a>> {
        self.eat_whitespace();
        let content = self.content();
        let single_char: &'a str = content[..content.char_indices().next()?.1];
        let start = self.position;
        if let Some(sep) = Separator::is_separator(content.chars().next()?) {
            self.position += single_char.len();
            return Some(Token {start, content: &single_char, token_type: TokenType::Separator(sep)})
        }
        if let Some(operator) = Operator::is_operator(content.chars().next()?) {
            self.position += single_char.len();
            return Some(Token {start, content: &single_char, token_type: TokenType::Operator(operator)})
        }
        let mut end = content.len();
        for (i, c) in content.char_indices() {
            if c == ' ' || Separator::is_separator(c).is_some() || Operator::is_operator(c).is_some() {
                end = i;
                break;
            }
        }
        self.position = end;
        Some(Token{start, content: &content[..end], token_type: TokenType::Word})
    }
}
impl<'a> Scanner {
    fn content(&self) -> &'a str {
        &self.total_input[&self.position..]
    }
    fn eat_whitespace(&mut self) {
        for c in self.content().chars() {
            if c == ' ' {
                return
            }
            self.position += c.len_utf8();
        }
    }

}