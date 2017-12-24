use std::cmp;
use std::i64::MAX;

/// Segment Tree for range minimum queries
pub struct RangeMinimumQuery {
    seg: Vec<i64>,
    n: usize,
}

impl RangeMinimumQuery {
    pub fn new(size: usize) -> RangeMinimumQuery {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        RangeMinimumQuery {
            seg: vec![MAX; m * 2],
            n: m,
        }
    }

    pub fn update(&mut self, mut k: usize, value: i64) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = cmp::min(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> i64 {
        assert!(a < b);
        return self.query_range(a, b, 0, 0, self.n);
    }

    pub fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return MAX;
        }
        if a <= l && r <= b {
            return self.seg[k];
        }
        let x = self.query_range(a, b, k * 2 + 1, l, (l + r) >> 1);
        let y = self.query_range(a, b, k * 2 + 2, (l + r) >> 1, r);
        cmp::min(x, y)
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::Rng;
    use test::Bencher;

    #[test]
    fn random_array() {
        let n = 1000;
        let arr = (0..n).map(|_| {
            return rand::thread_rng().gen::<i64>();
        }).collect::<Vec<_>>();

        let mut seg = RangeMinimumQuery::new(n);
        for i in 0..n {
            let mut minimum = MAX;
            for j in 0..(i + 1) {
                minimum = cmp::min(minimum, arr[j]);
            }
            seg.update(i, arr[i]);
            assert_eq!(seg.query(0, n), minimum);
            assert_eq!(seg.query(0, i + 1), minimum);
        }
    }

    #[test]
    fn random_array_online_update() {
        let n = 1000;

        let mut arr = vec![MAX; n];
        let mut seg = RangeMinimumQuery::new(n);

        for _ in 0..n {
            let value = rand::thread_rng().gen::<i64>();
            let k = rand::thread_rng().gen_range(0, n);
            seg.update(k, value);

            arr[k] = value;
            let mut minimum = MAX;
            for i in 0..n {
                minimum = cmp::min(minimum, arr[i]);
            }
            assert_eq!(seg.query(0, n), minimum);
        }
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            let n = 100000;

            let mut seg = RangeMinimumQuery::new(n);

            for _ in 0..n {
                let value = rand::thread_rng().gen::<i64>();
                let k = rand::thread_rng().gen_range(0, n);
                seg.update(k, value);
            }
        });
    }
}