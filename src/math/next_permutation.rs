pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T> NextPermutation for [T]
where
    T: PartialOrd,
{
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }

        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        self.swap(j, i - 1);
        self[i..].reverse();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::NextPermutation;
    use std::collections::BTreeSet;

    #[test]
    fn test_next_permutation() {
        let mut x = vec![1, 2, 3, 4, 5];
        let mut set = BTreeSet::new();
        let mut count = 0;
        while x.next_permutation() {
            count += 1;
            set.insert(x.clone());
        }
        assert_eq!(count + 1, 5 * 4 * 3 * 2);
        assert_eq!(set.len(), count);
    }
}
