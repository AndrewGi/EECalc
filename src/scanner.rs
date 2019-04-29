#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    total_input: &'a str,
    position: usize
}
#[derive(Debug, Clone)]
pub struct SavedPosition<'a> {
    parent: * mut Scanner<'a>,
    saved_position: usize,
    should_restore: bool
}
impl<'a, 'b> SavedPosition<'a> {
    fn new(scanner: &'b mut Scanner<'a>) -> SavedPosition<'a> {
        SavedPosition {
            parent: scanner,
            saved_position: scanner.position,
            should_restore: true
        }
    }
    fn ok(&mut self) {
        self.should_restore = true;
    }
    fn bad(&mut self) {
        self.should_restore = false;
    }
    fn should_restore(&self) -> bool {
        self.should_restore
    }
    fn mut_scanner(&mut self) -> &mut Scanner<'a> {
        unsafe {
            self.parent.as_mut().unwrap()
        }
    }
    fn scanner(&self) -> &Scanner<'a> {
        unsafe {
            self.parent.as_ref().unwrap()
        }
    }
    fn as_str(&self) -> &'a str {
        &self.scanner().total_input[self.saved_position..self.scanner().position]
    }
    fn restore(&mut self) {
        self.mut_scanner().position = self.saved_position
    }
    fn collect(&mut self) -> &'a str {
        self.ok();
        self.as_str()
    }
    fn on_option<T>(&mut self, option: Option<T>) -> Option<T> {
        if option.is_some() {
            self.ok()
        }
        option
    }
}
impl<'a, 'b> Drop for SavedPosition<'a> {
    fn drop(&mut self) {
        if self.should_restore {
            self.restore();
        }
    }
}
impl<'a, 'b> Scanner<'a> {
    pub fn position(&'b mut self) -> SavedPosition<'a> {
        SavedPosition::new(&mut self)
    }
    pub fn eat_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c != ' ' {
                break;
            }
            self.next();
        }
    }
    pub fn peek(&self) -> Option<char> {
        self.as_str().chars().next()
    }
    pub fn as_str(&self) -> &'a str {
        &self.total_input[self.position..]
    }
    pub fn next(&mut self) -> Option<char> {
        let c = self.as_str().chars().next()?;
        self.position+=1;
        Some(c)
    }
    pub fn maybe(&mut self, c: char) -> bool {
        self.peek().map_or(false, |p| c==p)
    }
    pub fn only_if(&mut self, c: char) -> Option<char> {
        if self.peek()? == 'c' {
            Some(c)
        } else {
            None
        }
    }
    pub fn next_digit(&mut self) -> Option<u32> {
        self.peek()?.to_digit(10)
    }
    pub fn next_digits(&mut self) -> Option<u32> {
        let mut number = self.next_digit()?;
        let mut position: u32 = 1;
        while let Some(digit) = self.next_digit() {
            number += 10u32.pow(position) * digit;
            position+=1;
        }
        Some(number)
    }
    pub fn next_operator(&mut self) -> Option<char> {
        let mut pos = self.position();
        self.eat_whitespace();

        pos.on_option(match self.peek()? {
            '+' => Some('+'),
            _ => None
        })
    }
    pub fn next_int(&mut self) -> Option<i32> {
        let mut pos = self.position();
        let result: i32 = if self.maybe('-') {-1} else {1} * (self.next_digits()? as i32);
        pos.ok();
        Some(result)
    }
    pub fn next_float(&mut self) -> Option<f64> {
        let mut pos = self.position();
        let is_negative = self.maybe('-');

        match self.next_digits() {
            Some(_) => {
                self.only_if('.')?;
                let _ = self.next_digits(); //ignore option
            }
            None => {
                self.only_if('.')?;
                self.next_digits()?;
            }
        }
        Some(pos.collect().parse().unwrap())

    }

}