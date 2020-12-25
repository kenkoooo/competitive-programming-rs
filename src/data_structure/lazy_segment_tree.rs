pub mod lazy_segment_tree {
    type Range = std::ops::Range<usize>;

    pub struct LazySegmentTree<S, Op, E, F, Mapping, Composition, Id> {
        n: usize,
        size: usize,
        log: usize,
        data: Vec<S>,
        lazy: Vec<F>,
        op: Op,
        e: E,
        mapping: Mapping,
        composition: Composition,
        id: Id,
    }

    impl<S, Op, E, F, Mapping, Composition, Id> LazySegmentTree<S, Op, E, F, Mapping, Composition, Id>
    where
        S: Clone,
        E: Fn() -> S,
        F: Clone,
        Op: Fn(&S, &S) -> S,
        Mapping: Fn(&F, &S) -> S,
        Composition: Fn(&F, &F) -> F,
        Id: Fn() -> F,
    {
        pub fn new(
            n: usize,
            e: E,
            op: Op,
            mapping: Mapping,
            composition: Composition,
            id: Id,
        ) -> Self {
            let size = n.next_power_of_two() as usize;
            LazySegmentTree {
                n,
                size,
                log: size.trailing_zeros() as usize,
                data: vec![e(); 2 * size],
                lazy: vec![id(); size],
                e,
                op,
                mapping,
                composition,
                id,
            }
        }
        pub fn set(&mut self, mut index: usize, value: S) {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index] = value;
            for i in 1..=self.log {
                self.update(index >> i);
            }
        }

        pub fn get(&mut self, mut index: usize) -> S {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index].clone()
        }

        pub fn prod(&mut self, range: Range) -> S {
            let mut l = range.start;
            let mut r = range.end;
            assert!(l < r && r <= self.n);

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sum_l = (self.e)();
            let mut sum_r = (self.e)();
            while l < r {
                if l & 1 != 0 {
                    sum_l = (self.op)(&sum_l, &self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    sum_r = (self.op)(&self.data[r], &sum_r);
                }
                l >>= 1;
                r >>= 1;
            }

            (self.op)(&sum_l, &sum_r)
        }

        pub fn all_prod(&self) -> S {
            self.data[1].clone()
        }

        pub fn apply(&mut self, mut index: usize, f: F) {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index] = (self.mapping)(&f, &self.data[index]);
            for i in 1..=self.log {
                self.update(index >> i);
            }
        }
        pub fn apply_range(&mut self, range: Range, f: F) {
            let mut l = range.start;
            let mut r = range.end;
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                let mut l = l;
                let mut r = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }

        fn update(&mut self, k: usize) {
            self.data[k] = (self.op)(&self.data[2 * k], &self.data[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F) {
            self.data[k] = (self.mapping)(&f, &self.data[k]);
            if k < self.size {
                self.lazy[k] = (self.composition)(&f, &self.lazy[k]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lazy[k].clone());
            self.all_apply(2 * k + 1, self.lazy[k].clone());
            self.lazy[k] = (self.id)();
        }
    }
}

#[cfg(test)]
mod test {
    use super::lazy_segment_tree::*;
    use rand::Rng;
    use std::cmp::{max, min};

    const INF: i64 = 1 << 60;

    #[test]
    fn edge_case() {
        let n = 5;
        let mut seg_min = LazySegmentTree::new(
            n,
            || INF,
            |s, t| min(*s, *t),
            |f, x| *f + *x,
            |f, g| *f + *g,
            || 0,
        );
        let mut values = vec![0; n];
        for i in 0..n {
            values[i] = i as i64;
            seg_min.set(i, i as i64);
        }

        let from = 1;
        let to = 4;
        let add = 2;
        for i in from..to {
            values[i] += add;
        }
        seg_min.apply_range(from..to, add);

        let pos = 2;
        let value = 1;
        let cur = seg_min.prod(pos..(pos + 1));
        seg_min.set(pos, cur - value);
        values[pos] -= value;

        for l in 0..n {
            for r in (l + 1)..(n + 1) {
                let min1 = seg_min.prod(l..r);
                let &min2 = values[l..r].iter().min().unwrap();
                assert_eq!(min1, min2);
            }
        }
    }

    #[test]
    fn random_add() {
        let n = 32;
        let mut array = vec![0; n];
        let mut seg_min = LazySegmentTree::new(
            n,
            || INF,
            |s, t| min(*s, *t),
            |f, x| *f + *x,
            |f, g| *f + *g,
            || 0,
        );
        let mut seg_max = LazySegmentTree::new(
            n,
            || -INF,
            |s, t| max(*s, *t),
            |f, x| *f + *x,
            |f, g| *f + *g,
            || 0,
        );
        for i in 0..n {
            let value = rand::thread_rng().gen::<i16>() as i64;
            array[i] = value;
            seg_min.set(i, value);
            seg_max.set(i, value);
        }

        for l in 0..n {
            for r in (l + 1)..n {
                let value = rand::thread_rng().gen::<i16>() as i64;
                seg_min.apply_range(l..r, value);
                seg_max.apply_range(l..r, value);

                for i in l..r {
                    array[i] += value;
                }

                for l in 0..n {
                    for r in (l + 1)..n {
                        let mut min = INF;
                        let mut max = -INF;
                        for i in l..r {
                            min = std::cmp::min(min, array[i]);
                            max = std::cmp::max(max, array[i]);
                        }

                        assert_eq!(seg_min.prod(l..r), min);
                        assert_eq!(seg_max.prod(l..r), max);
                    }
                }
            }
        }
    }
}
