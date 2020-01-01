pub mod fenwick_tree {
    /// `FenwickTree` is a data structure that can efficiently update elements
    /// and calculate prefix sums in a table of numbers.
    /// [https://en.wikipedia.org/wiki/Fenwick_tree](https://en.wikipedia.org/wiki/Fenwick_tree)
    pub struct FenwickTree<T> {
        n: usize,
        data: Vec<T>,
        init: T,
    }

    impl<T: Copy + std::ops::AddAssign + std::ops::Sub<Output = T>> FenwickTree<T> {
        /// Constructs a new `FenwickTree`. The size of `FenwickTree` should be specified by `size`.
        pub fn new(size: usize, init: T) -> FenwickTree<T> {
            FenwickTree {
                n: size + 1,
                data: vec![init; size + 1],
                init: init,
            }
        }

        pub fn add(&mut self, k: usize, value: T) {
            let mut x = k;
            while x < self.n {
                self.data[x] += value;
                x |= x + 1;
            }
        }

        /// Returns a sum of range `[l, r)`
        pub fn sum(&self, l: usize, r: usize) -> T {
            self.sum_one(r) - self.sum_one(l)
        }

        /// Returns a sum of range `[0, k)`
        pub fn sum_one(&self, k: usize) -> T {
            assert!(k < self.n, "k={} n={}", k, self.n);

            let mut result = self.init;
            let mut x = k as i32 - 1;
            while x >= 0 {
                result += self.data[x as usize];
                x = (x & (x + 1)) - 1;
            }

            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::fenwick_tree::FenwickTree;
    use rand::{thread_rng, Rng};

    #[test]
    fn random_array() {
        let n = 1000;
        let mut bit = FenwickTree::new(n, 0);
        let mut v = vec![0; n];

        for _ in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, n);
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
