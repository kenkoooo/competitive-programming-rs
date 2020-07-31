pub mod rolling_hash {
    const MASK_30: u64 = (1 << 30) - 1;
    const MASK_31: u64 = (1 << 31) - 1;
    const MOD: u64 = (1 << 61) - 1;

    pub struct RollingHash {
        hash: Vec<u64>,
        pow: Vec<u64>,
    }

    impl RollingHash {
        pub fn new(s: &[u8], base: u64) -> RollingHash {
            let n = s.len();
            let mut hash: Vec<u64> = vec![0; n + 1];
            let mut pow: Vec<u64> = vec![0; n + 1];
            pow[0] = 1;
            for i in 0..n {
                pow[i + 1] = modulo(mod_mul(pow[i], base));
                hash[i + 1] = modulo(mod_mul(hash[i], base) + s[i] as u64);
            }
            RollingHash { hash, pow }
        }

        /// Get hash of [l, r)
        pub fn get_hash(&self, l: usize, r: usize) -> u64 {
            modulo(self.hash[r] + MOD - mod_mul(self.hash[l], self.pow[r - l]))
        }
    }

    fn mod_mul(a: u64, b: u64) -> u64 {
        let (a_prefix, a_suffix) = (a >> 31, a & MASK_31);
        let (b_prefix, b_suffix) = (b >> 31, b & MASK_31);
        let m = a_suffix * b_prefix + a_prefix * b_suffix;
        modulo(a_prefix * b_prefix * 2 + (m >> 30) + ((m & MASK_30) << 31) + a_suffix * b_suffix)
    }

    fn modulo(v: u64) -> u64 {
        let v = (v & MOD) + (v >> 61);
        if v >= MOD {
            v - MOD
        } else {
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    const BASE: u64 = 1_000_000_007;

    #[test]
    fn test_rolling_hash() {
        let n = 30;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut s = String::new();
            for _ in 0..n {
                let c = (rng.sample(Uniform::from(0..26)) as u8 + 'a' as u8) as char;
                s.push(c);
            }

            let t = String::new() + s.as_str() + s.as_str();
            let n = t.len();
            let rolling_hash = rolling_hash::RollingHash::new(&t.as_bytes(), BASE);
            for i in 0..n {
                for j in i..n {
                    for k in (j + 1)..n {
                        let same = t[i..k] == t[j..k];
                        let same_hash = rolling_hash.get_hash(i, k) == rolling_hash.get_hash(j, k);
                        assert_eq!(
                            same,
                            same_hash,
                            "{:?} {:?} {} {}",
                            &t[i..k],
                            &t[j..k],
                            rolling_hash.get_hash(i, k),
                            rolling_hash.get_hash(j, k)
                        );
                    }
                }
            }
        }
    }
}
