pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, q, p) = extended_gcd(b, a % b);
        (d, p, q - a / b * p)
    }
}

pub fn chinese_remainder_theorem(b: &[i64], modulo: &[i64]) -> Option<(i64, i64)> {
    let (mut result, mut m) = (0, 1);
    for i in 0..b.len() {
        let (d, p, _) = extended_gcd(m, modulo[i]);
        if (b[i] - result) % d != 0 {
            return None;
        }
        let tmp = ((b[i] - result) / d * p) % (modulo[i] / d);
        result += m * tmp;
        m *= modulo[i] / d;
    }
    Some(((result % m + m) % m, m))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::Rng;

    #[test]
    fn test_extended_gcd() {
        for i in 1..10000 {
            for j in (i + 1)..10000 {
                let (gcd, x, y) = extended_gcd(i, j);
                assert_eq!(i % gcd, 0);
                assert_eq!(j % gcd, 0);
                assert_eq!(i * x + j * y, gcd);
            }
        }
    }

    #[test]
    fn test_crt() {
        let mut rng = rand::thread_rng();
        let n = 10;
        let max_m = 100;
        for _ in 0..1000 {
            let ans = rng.gen::<u32>() as i64;
            let mut b = vec![0; n];
            let mut m = vec![0; n];
            for i in 0..n {
                m[i] = rng.gen::<u8>() as i64;
                m[i] %= max_m;
                m[i] += 1;
                b[i] = ans % m[i];
            }

            let (a, _) = chinese_remainder_theorem(&b, &m).unwrap();
            for i in 0..n {
                assert_eq!(a % m[i], b[i]);
            }
        }
    }
}
