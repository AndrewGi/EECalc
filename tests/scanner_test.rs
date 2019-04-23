extern crate ee_calc;

#[cfg(test)]
mod scanner_tests {
    #[test]
    fn scanner_test_1() {
        let scanner = ee_calc::scanner::Scanner::new("10w * 24s");
        let mut c = scanner.get_cursor();
        assert_eq!(10f64, c.next_float().unwrap());
        assert_eq!("w", c.next_word().unwrap());
        assert_eq!('*', c.next_operator().unwrap());
        assert_eq!(24, c.next_int().unwrap());
        assert_eq!("s", c.next_word().unwrap());
    }
}