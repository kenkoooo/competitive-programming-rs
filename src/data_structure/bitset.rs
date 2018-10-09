pub mod bitset {
    use std::ops::BitOrAssign;
    const ONE_SIZE: usize = 60;
    const MAXIMUM: u64 = (1 << ONE_SIZE) - 1;

    pub fn get_bit_position(index: usize) -> (usize, usize) {
        let data_index = index / ONE_SIZE;
        let bit_index = index % ONE_SIZE;
        (data_index, bit_index)
    }

    pub struct BitSet {
        data: Vec<u64>,
    }

    impl BitOrAssign for BitSet {
        fn bitor_assign(&mut self, rhs: Self) {
            assert_eq!(self.data.len(), rhs.data.len());
            let n = self.data.len();
            for i in 0..n {
                assert!(self.data[i] <= MAXIMUM);
                assert!(rhs.data[i] <= MAXIMUM);
                self.data[i] |= rhs.data[i];
            }
        }
    }

    impl BitSet {
        pub fn new(n: usize) -> Self {
            let size = (n + ONE_SIZE - 1) / ONE_SIZE;
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
    }
}

#[cfg(test)]
mod test {
    use super::bitset::*;

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
}
