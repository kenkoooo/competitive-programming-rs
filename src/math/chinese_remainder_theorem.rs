pub fn extended_gcd(a: i64, b: i64, p: &mut i64, q: &mut i64) -> i64 {
    if b == 0 {
        *p = 1;
        *q = 0;
        a
    } else {
        let d = extended_gcd(b, a % b, q, p);
        *q -= a / b * *p;
        d
    }
}

pub fn chinese_remainder_theorem(b: &Vec<i64>, modulo: &Vec<i64>) -> Option<(i64, i64)> {
    let (mut r, mut m) = (0, 1);
    for i in 0..b.len() {
        let (mut p, mut q) = (0, 0);
        let d = extended_gcd(m, modulo[i], &mut p, &mut q);
        if (b[i] - r) % d != 0 {
            return None;
        }
        let tmp = ((b[i] - r) / d * p) % (modulo[i] / d);
        r += m * tmp;
        m *= modulo[i] / d;
    }
    Some(((r % m + m) % m, m))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::Rng;

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

            let (a, ma) = chinese_remainder_theorem(&b, &m).unwrap();
            for i in 0..n {
                assert_eq!(a % m[i], b[i]);
            }
        }
    }
}
