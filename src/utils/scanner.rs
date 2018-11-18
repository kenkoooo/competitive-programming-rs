pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn scanner_test() {
        let cursor = io::Cursor::new(b"1 2 3 \n4 5 6");
        let mut sc = Scanner { reader: cursor };

        assert_eq!(1, sc.read());
        assert_eq!(2, sc.read());
        assert_eq!(3, sc.read());
        assert_eq!(4, sc.read());
        assert_eq!(5, sc.read());
        assert_eq!(6, sc.read());
        let cursor = io::Cursor::new(b"1 a 0.1");
        let mut sc = Scanner { reader: cursor };

        assert_eq!(1, sc.read());
        assert_eq!("a".to_string(), sc.read::<String>());
        assert_eq!(0.1, sc.read());
    }
}
