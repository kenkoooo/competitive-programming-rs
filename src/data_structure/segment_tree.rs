/// Segment Tree for range queries
pub struct SegmentTree<T, Op> {
    seg: Vec<Option<T>>,
    n: usize,
    op: Op,
}

impl<T, Op> SegmentTree<T, Op>
where
    T: Copy,
    Op: Fn(T, T) -> T + Copy,
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
            let left = self.seg[k * 2 + 1];
            let right = self.seg[k * 2 + 2];
            self.seg[k] = Self::op(left, right, self.op);
        }
    }

    /// Get the result in the array of the range
    pub fn query<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<T> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(t) => *t,
            std::ops::Bound::Excluded(t) => *t+1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(t) => *t+1,
            std::ops::Bound::Excluded(t) => *t,
            std::ops::Bound::Unbounded => self.n,
        };

        self.query_range(start..end, 0, 0..self.n)
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
            Self::op(x, y, self.op)
        }
    }

    fn op(a: Option<T>, b: Option<T>, f: Op) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => a.or(b),
        }
    }
}

pub struct SegmentTree2d<T, Op> {
    n: usize,
    seg: Vec<SegmentTree<T, Op>>,
    op: Op,
}

impl<T, Op> SegmentTree2d<T, Op>
where
    T: Copy,
    Op: Fn(T, T) -> T + Copy,
{
    pub fn new(h: usize, w: usize, op: Op) -> Self {
        let mut n = h.next_power_of_two();
        if n == h {
            n *= 2;
        }
        let mut seg = Vec::with_capacity(n * 2);
        for _ in 0..(n * 2) {
            seg.push(SegmentTree::new(w, op));
        }
        Self { seg, n, op }
    }

    pub fn update(&mut self, i: usize, j: usize, value: T) {
        let mut k = i;
        k += self.n - 1;
        self.seg[k].update(j, value);
        while k > 0 {
            k = (k - 1) >> 1;
            let left = self.seg[k * 2 + 1].query(j..(j + 1));
            let right = self.seg[k * 2 + 2].query(j..(j + 1));
            if let Some(value) = Self::op(left, right, self.op) {
                self.seg[k].update(j, value);
            }
        }
    }

    pub fn query(&self, r: std::ops::Range<usize>, c: std::ops::Range<usize>) -> Option<T> {
        self.query_range(r, 0, 0..self.n, c)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
        c: std::ops::Range<usize>,
    ) -> Option<T> {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            None
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k].query(c)
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid, c.clone());
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end, c);
            Self::op(x, y, self.op)
        }
    }
    fn op(a: Option<T>, b: Option<T>, f: Op) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => a.or(b),
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

            let mut seg = SegmentTree::new(N, |a: i64, b: i64| a.min(b));
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
            let mut seg = SegmentTree::new(N, |a: i64, b: i64| a.min(b));

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

    #[test]
    fn random_array_2d() {
        const N: usize = 30;
        let mut rng = thread_rng();

        let mut arr = vec![vec![0; N]; N];
        let mut seg = SegmentTree2d::new(N, N, |a: i64, b: i64| a.min(b));
        for i in 0..N {
            for j in 0..N {
                arr[i][j] = rng.gen_range(0, INF);
                seg.update(i, j, arr[i][j]);
            }
        }

        for i1 in 0..N {
            for j1 in 0..N {
                for i2 in (i1 + 1)..=N {
                    for j2 in (j1 + 1)..=N {
                        let mut minimum = INF;

                        for i in i1..i2 {
                            for j in j1..j2 {
                                minimum = minimum.min(arr[i][j]);
                            }
                        }

                        assert_eq!(seg.query(i1..i2, j1..j2), Some(minimum));
                    }
                }
            }
        }
    }
}
