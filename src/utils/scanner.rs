pub mod scanner {
    use std::fmt::Debug;
    use std::io::Read;
    use std::str::{self, FromStr};

    pub struct Scanner<R: Read> {
        reader: R,
        buf: Vec<u8>,
    }

    impl<R: Read> Scanner<R> {
        pub fn new(reader: R) -> Self {
            Scanner {
                reader: reader,
                buf: Vec::new(),
            }
        }

        pub fn read<T>(&mut self) -> T
        where
            T: FromStr,
            T::Err: Debug,
        {
            self.buf.clear();
            for c in self
                .reader
                .by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n')
                .take_while(|&b| b != b' ' && b != b'\n')
            {
                self.buf.push(c);
            }

            unsafe { str::from_utf8_unchecked(&self.buf) }
                .parse()
                .expect("Parse error.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn scanner_test() {
        let cursor = io::Cursor::new(b"1 2 3 \n4 5 6");
        let mut sc = scanner::Scanner::new(cursor);

        assert_eq!(1, sc.read());
        assert_eq!(2, sc.read());
        assert_eq!(3, sc.read());
        assert_eq!(4, sc.read());
        assert_eq!(5, sc.read());
        assert_eq!(6, sc.read());
    }
}
