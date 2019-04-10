pub mod big_int {
    use std::cmp;
    use std::num::ParseIntError;
    use std::ops::*;
    use std::str::FromStr;

    const DIGIT_SPACE: usize = 30;
    const DIGIT_MASK: u64 = (1 << DIGIT_SPACE) - 1;

    type Digit = u64;

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub struct BigInteger {
        pub data: Vec<Digit>,
    }

    impl BigInteger {
        pub fn from_u64(value: u64) -> Self {
            Self { data: vec![value] }
        }
        pub fn zero() -> Self {
            Self::from_u64(0)
        }
    }

    impl FromStr for BigInteger {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut v: Vec<Digit> = s
                .chars()
                .map(|c| match c {
                    '0'...'9' => Ok(c as Digit - '0' as Digit),
                    _ => Err(format!("{} is not a digit.", c)),
                })
                .collect::<Result<Vec<Digit>, _>>()?;
            let mut values = vec![];
            while !v.is_empty() {
                let mut next = vec![];
                let mut cur = 0;
                values.push(v[v.len() - 1] & 1);
                for c in v.into_iter() {
                    let x = c + cur * 10;
                    next.push(x >> 1);
                    cur = x & 1;
                }
                v = next.into_iter().skip_while(|&c| c == 0).collect();
            }

            let mut data = vec![];
            for (i, v) in values.into_iter().enumerate() {
                if data.len() <= i / DIGIT_SPACE {
                    data.push(0);
                }
                data[i / DIGIT_SPACE] |= v << (i % DIGIT_SPACE);
            }
            Ok(BigInteger { data: data })
        }
    }

    impl Add for BigInteger {
        type Output = BigInteger;

        fn add(self, rhs: Self) -> Self {
            let size = cmp::max(self.data.len(), rhs.data.len());
            let mut data = vec![];
            let mut carry_over = 0;
            for i in 0..size {
                let a = if self.data.len() > i { self.data[i] } else { 0 };
                let b = if rhs.data.len() > i { rhs.data[i] } else { 0 };
                let sum = a + b + carry_over;
                data.push(sum & DIGIT_MASK);
                carry_over = sum >> DIGIT_SPACE;
            }
            if carry_over > 0 {
                data.push(carry_over);
            }
            Self { data: data }
        }
    }

    impl Mul for BigInteger {
        type Output = BigInteger;

        fn mul(self, rhs: Self) -> Self {
            self.data
                .into_iter()
                .enumerate()
                .map(|(i, a)| {
                    let mut data = (0..i).map(|_| 0).collect::<Vec<Digit>>();
                    let mut carry_over = 0;
                    for &b in rhs.data.iter() {
                        let sum = a * b + carry_over;
                        data.push(sum & DIGIT_MASK);
                        carry_over = sum >> DIGIT_SPACE;
                    }
                    if carry_over > 0 {
                        data.push(carry_over);
                    }
                    BigInteger { data: data }
                })
                .fold(BigInteger::zero(), |acc, x| acc + x)
        }
    }
}

#[cfg(test)]
mod test {
    use super::big_int::*;
    #[test]
    fn test_parse() {
        let x = "123456789012345678901234567890"
            .parse::<BigInteger>()
            .unwrap();
        assert_eq!(
            x,
            BigInteger {
                data: vec![239012562, 231703481, 781254508, 99]
            }
        );
    }

    #[test]
    fn test_add() {
        let a = "2830183408104810830192803".parse::<BigInteger>().unwrap();
        let b = "182301820938029839281092".parse::<BigInteger>().unwrap();
        let sum = "3012485229042840669473895".parse::<BigInteger>().unwrap();
        assert_eq!(sum, a + b);
    }

    #[test]
    fn test_mul() {
        let a = "128127391287398".parse::<BigInteger>().unwrap();
        let b = "7982739179238".parse::<BigInteger>().unwrap();
        let mul = "1022807546363469582692642724"
            .parse::<BigInteger>()
            .unwrap();
        assert_eq!(mul, a * b);
    }
}
