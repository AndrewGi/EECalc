
#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    total_input: &'a str,
    position: usize
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Raise,
    Equals,
}
#[derive(Debug, Clone)]
pub enum Separator {
    Comma,
    Period,
    Colon,
    OpenParentheses,
    CloseParentheses,
    EOL,
}
#[derive(Debug, Clone)]
pub enum TokenType {
    Word,
    Operator(Operator),
    Separator(Separator),
    Int(i64),
    Float(f64),
}
#[derive(Debug, Clone)]
pub struct Token<'a> {
    content: &'a str,
    token_type: TokenType
}
impl Separator {
    pub fn is_separator(c: char) -> Option<Separator> {
        match c {
            '(' => Some(Separator::OpenParentheses),
            ')' => Some(Separator::CloseParentheses),
            ',' => Some(Separator::Comma),
            '.' => Some(Separator::Period),
            ':' => Some(Separator::Colon),
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
impl<'a> Token<'a> {
    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }
    pub fn as_str(&self) -> &'a str {
        self.content
    }
    pub fn spanning_content(&self, other: &Self) -> &'a str{
        let start = self.as_str().as_ptr() as usize;
        let end = other.as_str().as_ptr() as usize;
        assert!(start<end);
        let size = (end-start)+other.as_str().len();
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.as_str().as_ptr(), size))
        }
    }
}
impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Token<'a>> {
        self.eat_whitespace();
        let content = self.content();
        let single_char: &'a str = &content[..content.char_indices().next()?.0];
        if let Some(sep) = Separator::is_separator(content.chars().next()?) {
            self.position += single_char.len();
            return Some(Token {content: &single_char, token_type: TokenType::Separator(sep)})
        }
        if let Some(operator) = Operator::is_operator(content.chars().next()?) {
            self.position += single_char.len();
            return Some(Token {content: &single_char, token_type: TokenType::Operator(operator)})
        }
        let mut end = content.len();
        for (i, c) in content.char_indices() {
            if c == ' ' || Separator::is_separator(c).is_some() || Operator::is_operator(c).is_some() {
                end = i;
                break;
            }
        }
        self.position = end;
        let word = &content[..end];
        if let Ok(i) = word.parse::<i64>() {
            return Some(Token{content: word, token_type: TokenType::Int(i)})
        }
        if let Ok(f) = word.parse::<f64>() {
            return Some(Token{content: word, token_type: TokenType::Float(f)})
        }
        Some(Token{content: word, token_type: TokenType::Word})
    }
}
impl<'a> Scanner<'a> {
    pub fn new(s: &str) -> Scanner {
        Scanner{total_input: s, position: 0}
    }
    pub fn is_done(&self) -> bool {
        self.content().is_empty()
    }
    fn content(&self) -> &'a str {
        &self.total_input[self.position..]
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