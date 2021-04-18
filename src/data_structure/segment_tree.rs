/// Segment Tree for range queries
pub struct SegmentTree<T, Op, Init> {
    seg: Vec<T>,
    n: usize,
    op: Op,
    initialize: Init,
}

impl<T, Op, Init> SegmentTree<T, Op, Init>
where
    T: Copy,
    Op: Fn(T, T) -> T,
    Init: Fn() -> T,
{
    pub fn new(size: usize, op: Op, initialize: Init) -> SegmentTree<T, Op, Init> {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initialize(); m * 2],
            n: m,
            op,
            initialize,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.op)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the minimum value in the array in the range
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> T {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            (self.initialize)()
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k]
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end);
            (self.op)(x, y)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::cmp;

    #[test]
    fn random_array() {
        let n = 1000;
        let arr = (0..n)
            .map(|_| {
                return rand::thread_rng().gen::<i64>();
            })
            .collect::<Vec<_>>();

        let mut seg = SegmentTree::new(n, |a, b| cmp::min(a, b), || i64::MAX);
        for i in 0..n {
            let mut minimum = i64::MAX;
            for j in 0..(i + 1) {
                minimum = cmp::min(minimum, arr[j]);
            }
            seg.update(i, arr[i]);
            assert_eq!(seg.query(0..n), minimum);
            assert_eq!(seg.query(0..(i + 1)), minimum);
        }
    }

    #[test]
    fn random_array_online_update() {
        let n = 1000;

        let mut arr = vec![i64::MAX; n];
        let mut seg = SegmentTree::new(n, |a, b| cmp::min(a, b), || i64::MAX);

        for _ in 0..n {
            let value = rand::thread_rng().gen::<i64>();
            let k = rand::thread_rng().gen_range(0, n);
            seg.update(k, value);

            arr[k] = value;
            let mut minimum = i64::MAX;
            for i in 0..n {
                minimum = cmp::min(minimum, arr[i]);
            }
            assert_eq!(seg.query(0..n), minimum);
        }
    }
}
