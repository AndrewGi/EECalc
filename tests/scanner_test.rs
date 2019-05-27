extern crate ee_calc;

#[cfg(test)]
mod scanner_tests {
    #[test]
    fn scanner_test_1() {
        let scanner = ee_calc::scanner::Scanner::new("10w * 24s");
    }
}