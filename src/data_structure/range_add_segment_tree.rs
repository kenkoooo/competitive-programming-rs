pub mod range_add_segment_tree {
    type Range = std::ops::Range<usize>;

    pub struct RangeAddSegmentTree<T, F> {
        data: Vec<T>,
        lazy: Vec<T>,
        size: usize,
        f: F,
        init: T,
    }

    impl<T, F> RangeAddSegmentTree<T, F>
    where
        T: PartialOrd + ::std::ops::Add<Output = T> + ::std::ops::Sub<Output = T> + Copy,
        F: Fn(T, T) -> T + Copy,
    {
        pub fn new(n: usize, init: T, f: F, zero: T) -> Self {
            let size = n.next_power_of_two();
            RangeAddSegmentTree {
                data: vec![init; size * 2],
                lazy: vec![zero; size * 2],
                size,
                init,
                f,
            }
        }

        pub fn add(&mut self, a: usize, b: usize, value: T) {
            self.add_to_range(a..b, value, 0, 0..self.size);
        }

        fn add_to_range(&mut self, add_range: Range, value: T, mut k: usize, seg_range: Range) {
            if add_range.end <= seg_range.start || seg_range.end <= add_range.start {
                return;
            }
            if add_range.start <= seg_range.start && seg_range.end <= add_range.end {
                self.lazy[k] = self.lazy[k] + value;
                while k > 0 {
                    k = (k - 1) / 2;
                    self.data[k] = (self.f)(
                        self.data[k * 2 + 1] + self.lazy[k * 2 + 1],
                        self.data[k * 2 + 2] + self.lazy[k * 2 + 2],
                    );
                }
            } else {
                let mid = (seg_range.start + seg_range.end) / 2;
                self.add_to_range(add_range.clone(), value, k * 2 + 1, seg_range.start..mid);
                self.add_to_range(add_range, value, k * 2 + 2, mid..seg_range.end);
            }
        }

        pub fn update(&mut self, pos: usize, value: T) {
            let cur = self.get(pos, pos + 1);
            let mut k = pos + self.size - 1;
            let raw = self.data[k];
            self.data[k] = raw + value - cur;
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(self.data[k * 2 + 1], self.data[k * 2 + 2]);
            }
        }

        pub fn get(&self, a: usize, b: usize) -> T {
            self.get_from_range(a..b, 0, 0..self.size)
        }

        fn get_from_range(&self, get_range: Range, k: usize, seg_range: Range) -> T {
            if get_range.end <= seg_range.start || seg_range.end <= get_range.start {
                self.init
            } else if get_range.start <= seg_range.start && seg_range.end <= get_range.end {
                self.data[k] + self.lazy[k]
            } else {
                let mid = (seg_range.start + seg_range.end) / 2;
                let x = self.get_from_range(get_range.clone(), k * 2 + 1, seg_range.start..mid);
                let y = self.get_from_range(get_range, k * 2 + 2, mid..seg_range.end);
                (self.f)(x, y) + self.lazy[k]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::cmp;

    const INF: i64 = 1 << 60;

    #[test]
    fn edge_case() {
        let n = 5;
        let mut seg_min = range_add_segment_tree::RangeAddSegmentTree::new(
            n,
            INF as usize,
            |a, b| if a > b { b } else { a },
            0,
        );
        let mut values = vec![0; n];
        for i in 0..n {
            values[i] = i;
            seg_min.update(i, i);
        }

        let from = 1;
        let to = 4;
        let add = 2;
        for i in from..to {
            values[i] += add;
        }
        seg_min.add(from, to, add);

        let pos = 2;
        let value = 1;
        let cur = seg_min.get(pos, pos + 1);
        seg_min.update(pos, cur - value);
        values[pos] -= value;

        for l in 0..n {
            for r in (l + 1)..(n + 1) {
                let min1 = seg_min.get(l, r);
                let &min2 = values[l..r].iter().min().unwrap();
                assert_eq!(min1, min2);
            }
        }
    }

    #[test]
    fn random_add() {
        let n = 32;
        let mut array = vec![0; n];
        let mut seg_min = range_add_segment_tree::RangeAddSegmentTree::new(
            n,
            INF,
            |a, b| if a > b { b } else { a },
            0,
        );
        let mut seg_max = range_add_segment_tree::RangeAddSegmentTree::new(
            n,
            -INF,
            |a, b| if a < b { b } else { a },
            0,
        );
        for i in 0..n {
            let value = rand::thread_rng().gen::<i16>() as i64;
            array[i] = value;
            seg_min.update(i, value);
            seg_max.update(i, value);
        }

        for l in 0..n {
            for r in (l + 1)..n {
                let value = rand::thread_rng().gen::<i16>() as i64;
                seg_min.add(l, r, value);
                seg_max.add(l, r, value);

                for i in l..r {
                    array[i] += value;
                }

                for l in 0..n {
                    for r in (l + 1)..n {
                        let mut min = INF;
                        let mut max = -INF;
                        for i in l..r {
                            min = cmp::min(min, array[i]);
                            max = cmp::max(max, array[i]);
                        }

                        assert_eq!(seg_min.get(l, r), min);
                        assert_eq!(seg_max.get(l, r), max);
                    }
                }
            }
        }
    }
}
