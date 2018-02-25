pub struct FenwickTree {
    n: usize,
    data: Vec<usize>,
}

impl FenwickTree {
    fn new(size: usize) -> FenwickTree {
        FenwickTree { n: size + 1, data: vec![0; size + 1] }
    }

    fn add(&mut self, k: usize, value: usize) {
        let mut x = k;
        while x < self.n {
            self.data[x] += value;
            x |= (x + 1);
        }
    }

    /// returns sum of [l, r)
    fn sum(&self, l: usize, r: usize) -> usize {
        return self.sum_one(r) - self.sum_one(l);
    }

    /// returns sum of [0, k)
    fn sum_one(&self, k: usize) -> usize {
        if k >= self.n {
            panic!("");
        }

        let mut result = 0;
        let mut x = k as i32 - 1;
        while x >= 0 {
            result += self.data[x as usize];
            x = (x & (x + 1)) - 1;
        }

        return result;
    }
}


#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::Rng;

    #[test]
    fn random_array() {
        let n = 1000;
        let mut bit = FenwickTree::new(n);
        let mut v = vec![0; n];

        for _ in 0..10000 {
            let value = rand::thread_rng().gen_range(0, 1000);
            let k = rand::thread_rng().gen_range(0, n);
            v[k] += value;
            bit.add(k, value);

            let mut sum = 0;
            for i in 0..n {
                sum += v[i];
                assert_eq!(sum, bit.sum(0, i + 1));
            }
        }
    }
}

