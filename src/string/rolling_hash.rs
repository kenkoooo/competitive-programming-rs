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
            pow[i + 1] = pow[i].wrapping_mul(base);
            hash[i + 1] = hash[i].wrapping_mul(base).wrapping_add(s[i] as u64);
        }
        RollingHash {
            hash: hash,
            pow: pow,
        }
    }

    /// Get hash of [l, r)
    pub fn get_hash(&self, l: usize, r: usize) -> u64 {
        self.hash[r].wrapping_sub(self.hash[l].wrapping_mul(self.pow[r - l]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{IndependentSample, Range};
    const MOD: u64 = 1_000_000_007;

    #[test]
    fn test_rolling_hash() {
        let n = 100;
        let between = Range::new(0, 26);
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut s = String::new();
            for _ in 0..n {
                let c = (between.ind_sample(&mut rng) as u8 + 'a' as u8) as char;
                s.push(c);
            }

            let mut t = String::new() + s.as_str() + s.as_str();

            let rolling_hash = RollingHash::new(&t.as_bytes(), MOD);
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
