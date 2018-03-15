pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Combination {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];
        inv[1] = 1;
        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..max { fact[i + 1] = fact[i] * (i + 1) % modulo; }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Combination { fact: fact, inv_fact: inv_fact, modulo: modulo }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        assert!(x >= y);
        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::Rng;
    use self::rand::distributions::{IndependentSample, Range};

    fn gcd(a: usize, b: usize) -> usize { if b == 0 { a } else { gcd(b, a % b) } }

    #[test]
    fn random_combination() {
        let modulo = 1_000_000_007;
        let mut rng = rand::thread_rng();

        for n in 100..200 {
            let comb = Combination::new(n, modulo);
            for m in 0..(n + 1) {
                let mut upper = (0..m).map(|i| n - i).collect::<Vec<_>>();
                for i in 0..m {
                    let mut divisor = i + 1;
                    for j in 0..(i + 1) {
                        if divisor == 1 { break; }

                        let g = gcd(divisor, upper[j]);
                        upper[j] /= g;
                        divisor /= g;
                    }
                }

                let mut check = 1;
                for u in &upper { check = (check * u) % modulo; }

                assert_eq!(comb.get(n, m), check);
            }
        }
    }
}
