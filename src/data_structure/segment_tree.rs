/// Segment Tree for range minimum queries
pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Clone, F> SegmentTree<T, F> where F: Fn(T, T) -> T {
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initial_value.clone(); m * 2],
            n: m,
            f: f,
            initial_value: initial_value.clone(),
        }
    }

    pub fn update(&mut self, mut k: usize, value: T) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1].clone(), self.seg[k * 2 + 2].clone());
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> T {
        assert!(a < b);
        return self.query_range(a, b, 0, 0, self.n);
    }

    pub fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.initial_value.clone();
        }
        if a <= l && r <= b {
            return self.seg[k].clone();
        }
        let x = self.query_range(a, b, k * 2 + 1, l, (l + r) >> 1);
        let y = self.query_range(a, b, k * 2 + 2, (l + r) >> 1, r);
        (self.f)(x, y)
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::Rng;
    use test::Bencher;
    use std::i64::MAX;
    use std::cmp;

    #[test]
    fn random_array() {
        let n = 1000;
        let arr = (0..n).map(|_| {
            return rand::thread_rng().gen::<i64>();
        }).collect::<Vec<_>>();

        let mut seg = SegmentTree::new(n, MAX, |a, b| cmp::min(a, b));
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
        let mut seg = SegmentTree::new(n, MAX, |a, b| cmp::min(a, b));

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

            let mut seg = SegmentTree::new(n, MAX, |a, b| cmp::min(a, b));

            for _ in 0..n {
                let value = rand::thread_rng().gen::<i64>();
                let k = rand::thread_rng().gen_range(0, n);
                seg.update(k, value);
            }
        });
    }
}