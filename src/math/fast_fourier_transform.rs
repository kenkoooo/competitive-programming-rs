pub struct FastFourierTransform {
    modulo: i64,
    sum_e: [i64; 30],
    sum_ie: [i64; 30],
}

impl FastFourierTransform {
    pub fn new(modulo: i64) -> Self {
        let primitive_root = primitive_root(modulo);

        let mut es = [0; 30];
        let mut ies = [0; 30];
        let count2 = (modulo - 1).trailing_zeros();
        let mut e = mod_pow(primitive_root, (modulo - 1) >> count2, modulo);
        let mut ie = mod_inv(e, modulo);
        let count2 = count2 as usize;
        for i in (2..=count2).rev() {
            es[i - 2] = e;
            ies[i - 2] = ie;
            e = (e * e) % modulo;
            ie = (ie * ie) % modulo;
        }

        let mut sum_e = [0; 30];
        let mut now = 1;
        for i in 0..=(count2 - 2) {
            sum_e[i] = (es[i] * now) % modulo;
            now = (now * ies[i]) % modulo;
        }

        let mut es = [0; 30];
        let mut ies = [0; 30];
        let count2 = (modulo - 1).trailing_zeros();
        let mut e = mod_pow(primitive_root, (modulo - 1) >> count2, modulo);
        let mut ie = mod_inv(e, modulo);
        let count2 = count2 as usize;
        for i in (2..=count2).rev() {
            es[i - 2] = e;
            ies[i - 2] = ie;
            e = (e * e) % modulo;
            ie = (ie * ie) % modulo;
        }

        let mut sum_ie = [0; 30];
        let mut now = 1;
        for i in 0..=(count2 - 2) {
            sum_ie[i] = (ies[i] * now) % modulo;
            now = (now * es[i]) % modulo;
        }

        Self {
            sum_e,
            modulo,
            sum_ie,
        }
    }
    fn butterfly(&self, a: &mut [i64]) {
        let h = a.len().next_power_of_two().trailing_zeros();
        for ph in 1..=h {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut now = 1;
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset] % self.modulo;
                    let r = (a[i + offset + p] * now) % self.modulo;
                    a[i + offset] = (l + r) % self.modulo;
                    a[i + offset + p] = (l + self.modulo - r) % self.modulo;
                }

                now = (self.sum_e[(!s).trailing_zeros() as usize] * now) % self.modulo;
            }
        }
    }

    fn butterfly_inv(&self, a: &mut [i64]) {
        let h = a.len().next_power_of_two().trailing_zeros();
        for ph in (1..=h).rev() {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut inv_now = 1;
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    let l = a[i + offset] % self.modulo;
                    let r = a[i + offset + p] % self.modulo;
                    a[i + offset] = (l + r) % self.modulo;
                    a[i + offset + p] =
                        (((l + self.modulo - r) % self.modulo) * inv_now) % self.modulo;
                }

                inv_now = (self.sum_ie[(!s).trailing_zeros() as usize] * inv_now) % self.modulo;
            }
        }
    }

    pub fn convolution(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }

        let n = a.len();
        let m = b.len();

        let z = (n + m - 1).next_power_of_two();
        let mut a = Vec::from(a);
        a.resize(z, 0);
        self.butterfly(&mut a);

        let mut b = Vec::from(b);
        b.resize(z, 0);
        self.butterfly(&mut b);

        for i in 0..z {
            a[i] = (a[i] * b[i]) % self.modulo;
        }

        self.butterfly_inv(&mut a);
        a.resize(n + m - 1, 0);
        let iz = mod_inv(z as i64, self.modulo);
        for i in 0..a.len() {
            a[i] = (a[i] * iz) % self.modulo;
        }
        a
    }
}

fn mod_inv(x: i64, m: i64) -> i64 {
    mod_pow(x, m - 2, m)
}

fn mod_pow(x: i64, mut e: i64, m: i64) -> i64 {
    let mut cur = x;
    let mut result = 1;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % m;
        }
        e >>= 1;
        cur = (cur * cur) % m;
    }
    result
}

fn primitive_root(m: i64) -> i64 {
    if m == 2 {
        return 1;
    };
    if m == 167772161 {
        return 3;
    };
    if m == 469762049 {
        return 3;
    };
    if m == 754974721 {
        return 11;
    };
    if m == 998244353 {
        return 3;
    };
    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2
    }

    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
        i += 2;
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }

    for g in 2.. {
        let mut ok = true;
        for i in 0..cnt {
            if mod_pow(g, (m - 1) / divs[i], m) == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return g;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fft() {
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7, 8, 9];
        let m = 998244353;
        let fft = FastFourierTransform::new(m);
        let c = fft.convolution(&a, &b);
        assert_eq!(vec![5, 16, 34, 60, 70, 70, 59, 36], c);
    }

    #[test]
    fn test_primitive_root() {
        assert!(is_primitive_root(2, primitive_root(2)));
        assert!(is_primitive_root(3, primitive_root(3)));
        assert!(is_primitive_root(5, primitive_root(5)));
        assert!(is_primitive_root(7, primitive_root(7)));
        assert!(is_primitive_root(11, primitive_root(11)));
        assert!(is_primitive_root(998244353, primitive_root(998244353)));
        assert!(is_primitive_root(1000000007, primitive_root(1000000007)));
        assert!(is_primitive_root(469762049, primitive_root(469762049)));
        assert!(is_primitive_root(167772161, primitive_root(167772161)));
        assert!(is_primitive_root(754974721, primitive_root(754974721)));
        assert!(is_primitive_root(324013369, primitive_root(324013369)));
        assert!(is_primitive_root(831143041, primitive_root(831143041)));
        assert!(is_primitive_root(1685283601, primitive_root(1685283601)));
    }

    fn is_primitive_root(m: i64, g: i64) -> bool {
        let mut factors = vec![];
        let mut cur = 2;
        let mut t = m - 1;
        while cur * cur <= t {
            if t % cur == 0 {
                factors.push(cur);
            }
            while t % cur == 0 {
                t /= cur;
            }
            cur += 1;
        }
        if t > 1 {
            factors.push(t);
        }

        for factor in factors {
            if mod_pow(g, (m - 1) / factor, m) == 1 {
                return false;
            }
        }
        true
    }
}
