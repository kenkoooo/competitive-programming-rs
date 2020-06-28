/// Segment Tree for range queries
pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Copy, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f,
            initial_value,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> T {
        assert!(a < b);
        self.query_range(a, b, 0, 0, self.n)
    }

    fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            self.initial_value
        } else if a <= l && r <= b {
            self.seg[k]
        } else {
            let x = self.query_range(a, b, k * 2 + 1, l, (l + r) >> 1);
            let y = self.query_range(a, b, k * 2 + 2, (l + r) >> 1, r);
            (self.f)(x, y)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rand::Rng;
    use std::cmp;
    use std::i64::MAX;

    #[test]
    fn random_array() {
        let n = 1000;
        let arr = (0..n)
            .map(|_| {
                return rand::thread_rng().gen::<i64>();
            })
            .collect::<Vec<_>>();

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
}
