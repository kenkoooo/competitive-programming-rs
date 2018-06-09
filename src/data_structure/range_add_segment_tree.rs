pub mod range_add_segment_tree {

    use std::cmp;
    pub const NUM: usize = 1 << 20;
    pub const INF: i64 = 1 << 60;

    pub struct RangeAddSegmentTree {
        seg_min: Vec<i64>,
        seg_max: Vec<i64>,
        seg_add: Vec<i64>,
    }

    impl RangeAddSegmentTree {
        pub fn new() -> Self {
            RangeAddSegmentTree {
                seg_min: vec![INF; NUM * 2],
                seg_max: vec![-INF; NUM * 2],
                seg_add: vec![0; NUM * 2],
            }
        }

        /// add to [a, b)
        pub fn add(&mut self, a: usize, b: usize, value: i64) {
            self.add_to_range(a, b, value, 0, 0, NUM);
        }

        fn add_to_range(
            &mut self,
            a: usize,
            b: usize,
            value: i64,
            k: usize,
            left: usize,
            right: usize,
        ) {
            if b <= left || right <= a {
                return;
            }
            if a <= left && right <= b {
                let mut k = k;
                self.seg_add[k] += value;
                while k > 0 {
                    k = (k - 1) / 2;
                    self.seg_min[k] = cmp::min(
                        self.seg_min[k * 2 + 1] + self.seg_add[k * 2 + 1],
                        self.seg_min[k * 2 + 2] + self.seg_add[k * 2 + 2],
                    );
                    self.seg_max[k] = cmp::max(
                        self.seg_max[k * 2 + 1] + self.seg_add[k * 2 + 1],
                        self.seg_max[k * 2 + 2] + self.seg_add[k * 2 + 2],
                    );
                }
            } else {
                self.add_to_range(a, b, value, k * 2 + 1, left, (left + right) / 2);
                self.add_to_range(a, b, value, k * 2 + 2, (left + right) / 2, right);
            }
        }

        pub fn update(&mut self, pos: usize, value: i64) {
            let mut k = pos + NUM - 1;
            self.seg_min[k] = value;
            self.seg_max[k] = value;
            while k > 0 {
                k = (k - 1) / 2;
                self.seg_min[k] = cmp::min(self.seg_min[k * 2 + 1], self.seg_min[k * 2 + 2]);
                self.seg_max[k] = cmp::max(self.seg_max[k * 2 + 1], self.seg_max[k * 2 + 2]);
            }
        }

        pub fn get_min(&self, a: usize, b: usize) -> i64 {
            get(
                &self.seg_min,
                &self.seg_add,
                INF,
                a,
                b,
                0,
                0,
                NUM,
                &cmp::min,
            )
        }

        pub fn get_max(&self, a: usize, b: usize) -> i64 {
            get(
                &self.seg_max,
                &self.seg_add,
                -INF,
                a,
                b,
                0,
                0,
                NUM,
                &cmp::max,
            )
        }
    }

    fn get<F>(
        seg: &Vec<i64>,
        add: &Vec<i64>,
        default_value: i64,
        a: usize,
        b: usize,
        k: usize,
        left: usize,
        right: usize,
        f: &F,
    ) -> i64
    where
        F: Fn(i64, i64) -> i64,
    {
        if b <= left || right <= a {
            default_value
        } else if a <= left && right <= b {
            seg[k] + add[k]
        } else {
            let mid = (left + right) / 2;
            let x = get(seg, add, default_value, a, b, k * 2 + 1, left, mid, f);
            let y = get(seg, add, default_value, a, b, k * 2 + 2, mid, right, f);
            f(x, y) + add[k]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate rand;
    use self::rand::Rng;
    use std::cmp;

    #[test]
    fn random_add() {
        let n = 30;
        let mut array = vec![0; n];
        let mut seg = range_add_segment_tree::RangeAddSegmentTree::new();
        for i in 0..n {
            seg.update(i, 0);
        }

        for l in 0..n {
            for r in (l + 1)..n {
                let value = rand::thread_rng().gen::<i16>() as i64;
                seg.add(l, r, value);

                for i in l..r {
                    array[i] += value;
                }

                for l in 0..n {
                    for r in (l + 1)..n {
                        let mut min = range_add_segment_tree::INF;
                        let mut max = -range_add_segment_tree::INF;
                        for i in l..r {
                            min = cmp::min(min, array[i]);
                            max = cmp::max(max, array[i]);
                        }

                        assert_eq!(seg.get_min(l, r), min);
                        assert_eq!(seg.get_max(l, r), max);
                    }
                }
            }
        }
    }
}
