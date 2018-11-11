pub mod bitset {
    use std::ops::{BitOrAssign, Shl};
    const ONE_VALUE_LENGTH: usize = 60;
    const MAXIMUM: u64 = (1 << ONE_VALUE_LENGTH) - 1;

    pub fn get_bit_position(index: usize) -> (usize, usize) {
        let data_index = index / ONE_VALUE_LENGTH;
        let bit_index = index % ONE_VALUE_LENGTH;
        (data_index, bit_index)
    }

    #[derive(PartialEq, Clone, Debug)]
    pub struct BitSet {
        data: Vec<u64>,
    }

    impl BitOrAssign for BitSet {
        fn bitor_assign(&mut self, rhs: Self) {
            if self.data.len() < rhs.data.len() {
                self.data.resize(rhs.data.len(), 0);
            }
            let n = if self.data.len() > rhs.data.len() {
                rhs.data.len()
            } else {
                self.data.len()
            };
            for i in 0..n {
                assert!(self.data[i] <= MAXIMUM);
                assert!(rhs.data[i] <= MAXIMUM);
                self.data[i] |= rhs.data[i];
            }
        }
    }

    impl Shl<usize> for BitSet {
        type Output = Self;
        fn shl(self, rhs: usize) -> Self {
            self.shift_left(rhs)
        }
    }

    impl BitSet {
        pub fn new(n: usize) -> Self {
            let size = (n + ONE_VALUE_LENGTH - 1) / ONE_VALUE_LENGTH;
            BitSet {
                data: vec![0; size],
            }
        }

        pub fn new_from(value: u64) -> Self {
            BitSet { data: vec![value] }
        }

        pub fn set(&mut self, index: usize, value: bool) {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(self.data.len() > data_index);
            if value {
                self.data[data_index] |= 1 << bit_index;
            } else {
                let tmp = MAXIMUM ^ (1 << bit_index);
                self.data[data_index] &= tmp;
            }
        }

        pub fn get(&mut self, index: usize) -> bool {
            let (data_index, bit_index) = get_bit_position(index);
            assert!(self.data.len() > data_index);
            self.data[data_index] & (1 << bit_index) != 0
        }

        pub fn shift_left(&self, shift: usize) -> Self {
            let mut next_data = Vec::new();
            let prefix_empty_count = shift / ONE_VALUE_LENGTH;
            let shift_count = shift % ONE_VALUE_LENGTH;
            for _ in 0..prefix_empty_count {
                next_data.push(0);
            }

            let mut from_previous = 0;
            let room = ONE_VALUE_LENGTH - shift_count;
            for &data in self.data.iter() {
                let overflow = (data >> room) << room;
                let rest = data - overflow;
                let value = (rest << shift_count) + from_previous;
                assert!(value <= MAXIMUM);
                next_data.push(value);
                from_previous = overflow >> room;
            }
            if from_previous > 0 {
                next_data.push(from_previous);
            }
            BitSet { data: next_data }
        }
    }
}

#[cfg(test)]
mod test {
    use super::bitset::*;
    use test::Bencher;

    #[test]
    fn test_set_bit() {
        let n = 10;
        let value = 717;
        let mut bitset = BitSet::new(n);
        for i in 0..n {
            if value & (1 << i) != 0 {
                bitset.set(i, true);
            }
        }

        for i in 0..n {
            if value & (1 << i) != 0 {
                assert!(bitset.get(i));
            } else {
                assert!(!bitset.get(i));
            }
        }
    }

    #[test]
    fn test_bitset_or() {
        let mut value1 = 717;
        let mut bitset1 = BitSet::new_from(value1);

        let value2 = 127;
        let bitset2 = BitSet::new_from(value2);

        value1 |= value2;
        bitset1 |= bitset2;

        for i in 0..50 {
            if value1 & (1 << i) != 0 {
                assert!(bitset1.get(i));
            } else {
                assert!(!bitset1.get(i));
            }
        }
    }

    #[test]
    fn test_bitset_shift_left() {
        let value1 = 717;
        let first_shift = 30;
        let second_shift = 40;
        let bitset1 = BitSet::new_from(value1) << (first_shift + second_shift);
        let bitset2 = BitSet::new_from(value1 << first_shift) << second_shift;
        assert!(bitset1 == bitset2);
    }

    #[bench]
    fn bench_bitset(b: &mut Bencher) {
        let n = 2000;
        b.iter(|| {
            let mut set = BitSet::new(n);
            for i in 1..=n {
                let next = set.shift_left(i);
                set |= next;
            }
            set
        })
    }
}
