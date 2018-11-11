pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Rem, Sub, SubAssign};

    #[derive(Copy)]
    pub struct ModInt<T> {
        pub value: T,
        modulo: T,
    }

    impl<T> Clone for ModInt<T>
    where
        T: Copy,
    {
        fn clone(&self) -> Self {
            ModInt {
                value: self.value,
                modulo: self.modulo,
            }
        }

        fn clone_from(&mut self, source: &ModInt<T>) {
            self.value = source.value;
            self.modulo = source.modulo;
        }
    }

    impl<T> Add<ModInt<T>> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn add(self, rhs: ModInt<T>) -> ModInt<T> {
            self + rhs.value
        }
    }

    impl<T> Add<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn add(self, rhs: T) -> ModInt<T> {
            let m = self.modulo;
            let mut t = rhs + self.value;
            if t >= m {
                t = t - m;
            }
            ModInt {
                value: t,
                modulo: self.modulo,
            }
        }
    }

    impl<T> Sub<T> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: T) -> ModInt<T> {
            let rhs = if rhs >= self.modulo {
                rhs % self.modulo
            } else {
                rhs
            };
            let value = if self.value < rhs {
                self.value + self.modulo
            } else {
                self.value
            };
            ModInt {
                value: value - rhs,
                modulo: self.modulo,
            }
        }
    }

    impl<T> Sub<ModInt<T>> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
            self - rhs.value
        }
    }

    impl<T> AddAssign<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<T> AddAssign<ModInt<T>> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        fn add_assign(&mut self, other: ModInt<T>) {
            *self = *self + other;
        }
    }

    impl<T> SubAssign<T> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }

    impl<T> SubAssign<ModInt<T>> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        fn sub_assign(&mut self, other: ModInt<T>) {
            *self = *self - other;
        }
    }

    impl<T> Mul<ModInt<T>> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        type Output = ModInt<T>;

        fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
            self * rhs.value
        }
    }
    impl<T> Mul<T> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        type Output = ModInt<T>;

        fn mul(self, rhs: T) -> ModInt<T> {
            let t = (self.value * rhs) % self.modulo;
            ModInt {
                value: t,
                modulo: self.modulo,
            }
        }
    }

    impl<T> MulAssign<T> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }

    impl<T> MulAssign<ModInt<T>> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        fn mul_assign(&mut self, rhs: ModInt<T>) {
            *self = *self * rhs;
        }
    }

    impl<T> ModInt<T> {
        pub fn new(value: T, modulo: T) -> ModInt<T> {
            ModInt {
                value: value,
                modulo: modulo,
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use self::rand::distributions::{IndependentSample, Range};
    use super::mod_int::*;

    #[test]
    fn random_add_sub() {
        let modulo = 1_000_000_007;
        let between = Range::new(0, modulo);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: usize = between.ind_sample(&mut rng);
            let y: usize = between.ind_sample(&mut rng);

            let mx = ModInt::new(x, modulo);
            let my = ModInt::new(y, modulo);

            assert_eq!((mx + my).value, (x + y) % modulo);
            assert_eq!((mx + y).value, (x + y) % modulo);
            assert_eq!((mx - my).value, (x + modulo - y) % modulo);
            assert_eq!((mx - y).value, (x + modulo - y) % modulo);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.value, x % modulo);

            mx += y;
            x += y;
            assert_eq!(mx.value, x % modulo);

            mx -= my;
            x = (x + modulo - y % modulo) % modulo;
            assert_eq!(mx.value, x);

            mx -= y;
            x = (x + modulo - y % modulo) % modulo;
            assert_eq!(mx.value, x);
        }
    }

    #[test]
    fn random_mul() {
        let modulo = 1_000_000_007;
        let between = Range::new(0, modulo);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: usize = between.ind_sample(&mut rng);
            let y: usize = between.ind_sample(&mut rng);

            let mx = ModInt::new(x, modulo);
            let my = ModInt::new(y, modulo);

            assert_eq!((mx * my).value, (x * y) % modulo);
            assert_eq!((mx * y).value, (x * y) % modulo);
        }
    }

    #[test]
    fn zero_test() {
        let modulo = 5;
        let a = ModInt::new(2, modulo);
        let b = ModInt::new(3, modulo);
        let c = a + b;
        assert_eq!(c.value, 0);
    }
}
