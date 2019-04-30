pub mod sparse_table {
    use std::cmp;

    pub struct SparseTable<T, F> {
        data: Vec<Vec<T>>,
        op: F,
    }

    impl<T, F> SparseTable<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        pub fn from(v: &[T], init: T, op: F) -> Self {
            let size = v.len().next_power_of_two();
            let count = size.trailing_zeros() as usize + 1;
            let mut data = vec![vec![init; size]; count];
            for (i, v) in v.iter().cloned().enumerate() {
                data[0][i] = v;
            }
            for c in 1..count {
                for i in 0..size {
                    let next = cmp::min(size - 1, i + (1 << (c - 1)));
                    data[c][i] = op(data[c - 1][i], data[c - 1][next]);
                }
            }

            Self { data: data, op: op }
        }

        /// get the result for [l, r)
        pub fn get(&self, l: usize, r: usize) -> T {
            assert!(l < r);
            let length = r - l;
            if length == 1 {
                return self.data[0][l];
            }
            let block_size = length.next_power_of_two() >> 1;
            let c = block_size.trailing_zeros() as usize;
            let left = self.data[c][l];
            let right = self.data[c][r - block_size];
            (self.op)(left, right)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::cmp;

    #[test]
    fn test_small() {
        let v = vec![1, 9, 9, 1, 0, 7, 1, 7];
        let sparse_table = sparse_table::SparseTable::from(&v, 0, |a, b| cmp::min(a, b));
        for l in 0..8 {
            for r in (l + 1)..9 {
                let &min = v[l..r].iter().min().unwrap();
                assert_eq!(sparse_table.get(l, r), min);
            }
        }
    }

    #[test]
    fn test_sparse_table_with_random_array() {
        let n = 1000;
        let v: Vec<u64> = (0..n)
            .map(|_| thread_rng().gen_range(0, 1000000000))
            .collect();
        let sparse_table = sparse_table::SparseTable::from(&v, 0, cmp::min);
        for l in 0..n {
            for r in (l + 1)..(n + 1) {
                let &min = v[l..r].iter().min().unwrap();
                assert_eq!(sparse_table.get(l, r), min);
            }
        }
    }
}
