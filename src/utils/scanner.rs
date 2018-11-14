pub trait Scanner {
    fn read_input<T: std::str::FromStr>(&mut self) -> T;
}

impl<R: std::io::Read> Scanner for R {
    fn read_input<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn scanner_test() {
        let cursor = io::Cursor::new(b"1 a 0.1");
        let mut sc = cursor;

        assert_eq!(1, sc.read_input());
        assert_eq!("a".to_string(), sc.read_input::<String>());
        assert_eq!(0.1, sc.read_input());
    }
}
