/// Segment Tree for range queries
pub struct SegmentTree<T, Op> {
    seg: Vec<Option<T>>,
    n: usize,
    op: Op,
}

impl<T, Op> SegmentTree<T, Op>
where
    T: Copy,
    Op: Fn(Option<T>, Option<T>) -> Option<T>,
{
    pub fn new(size: usize, op: Op) -> SegmentTree<T, Op> {
        let mut m = size.next_power_of_two();
        if m == size {
            m *= 2;
        }
        SegmentTree {
            seg: vec![None; m * 2],
            n: m,
            op,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = Some(value);
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.op)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the result in the array of the range
    pub fn query(&self, range: std::ops::Range<usize>) -> Option<T> {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> Option<T> {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            None
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
    use rand::prelude::*;

    const INF: i64 = 1 << 60;

    #[test]
    fn random_array() {
        const N: usize = 1000;
        let mut rng = thread_rng();

        for _ in 0..5 {
            let mut arr = vec![0; N];
            for i in 0..N {
                arr[i] = rng.gen_range(0, INF);
            }

            let mut seg = SegmentTree::new(N, |a: Option<i64>, b: Option<i64>| {
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a.min(b))
                } else {
                    a.or(b)
                }
            });
            for i in 0..N {
                let mut minimum = INF;
                for j in 0..=i {
                    minimum = minimum.min(arr[j]);
                }
                seg.update(i, arr[i]);
                assert_eq!(seg.query(0..N), Some(minimum));
                assert_eq!(seg.query(0..(i + 1)), Some(minimum));
            }
        }
    }

    #[test]
    fn random_array_online_update() {
        const N: usize = 1000;
        let mut rng = thread_rng();

        for _ in 0..5 {
            let mut arr = vec![INF; N];
            let mut seg = SegmentTree::new(N, |a: Option<i64>, b: Option<i64>| {
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a.min(b))
                } else {
                    a.or(b)
                }
            });

            for _ in 0..N {
                let value = rng.gen_range(0, INF);
                let k = rand::thread_rng().gen_range(0, N);
                seg.update(k, value);

                arr[k] = value;
                let mut minimum = INF;
                for i in 0..N {
                    minimum = minimum.min(arr[i]);
                }
                assert_eq!(seg.query(0..N), Some(minimum));
            }
        }
    }
}
