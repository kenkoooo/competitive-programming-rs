pub mod mod_int {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

    #[derive(Clone, Copy)]
    pub struct ModInt<T: Copy + Clone>(pub T, pub T);

    impl<T> Add<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Clone + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn add(self, mut rhs: T) -> ModInt<T> {
            if rhs >= self.1 {
                rhs = rhs % self.1;
            }
            let mut t = rhs + self.0;
            if t >= self.1 {
                t = t - self.1;
            }
            ModInt(t, self.1)
        }
    }

    impl<T> Add<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Add<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn add(self, rhs: ModInt<T>) -> ModInt<T> {
            self + rhs.0
        }
    }

    impl<T> Sub<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Clone + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: T) -> ModInt<T> {
            let rhs = if rhs >= self.1 { rhs % self.1 } else { rhs };
            let value = if self.0 < rhs {
                self.0 + self.1
            } else {
                self.0
            };
            ModInt(value - rhs, self.1)
        }
    }

    impl<T> Sub<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Sub<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
            self - rhs.0
        }
    }

    impl<T> AddAssign<T> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Add<T, Output = ModInt<T>>,
    {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<T> AddAssign<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Add<ModInt<T>, Output = ModInt<T>>,
    {
        fn add_assign(&mut self, other: ModInt<T>) {
            *self = *self + other;
        }
    }

    impl<T> SubAssign<T> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Sub<T, Output = ModInt<T>>,
    {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }

    impl<T> SubAssign<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Sub<ModInt<T>, Output = ModInt<T>>,
    {
        fn sub_assign(&mut self, other: ModInt<T>) {
            *self = *self - other;
        }
    }

    impl Div<u64> for ModInt<u64> {
        type Output = ModInt<u64>;
        fn div(self, mut rhs: u64) -> ModInt<u64> {
            if rhs >= self.1 {
                rhs %= self.1;
            }
            self * ModInt(rhs, self.1).pow(self.1 - 2)
        }
    }

    impl<T> Div<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Div<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn div(self, rhs: ModInt<T>) -> ModInt<T> {
            self / rhs.0
        }
    }

    impl<T> DivAssign<T> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Div<T, Output = ModInt<T>>,
    {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs
        }
    }
    impl<T> DivAssign<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Div<ModInt<T>, Output = ModInt<T>>,
    {
        fn div_assign(&mut self, rhs: ModInt<T>) {
            *self = *self / rhs
        }
    }

    impl<T> Mul<T> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Clone + Copy + PartialOrd,
    {
        type Output = ModInt<T>;

        fn mul(self, mut rhs: T) -> ModInt<T> {
            if rhs >= self.1 {
                rhs = rhs % self.1;
            }
            let t = (self.0 * rhs) % self.1;
            ModInt(t, self.1)
        }
    }
    impl<T> Mul<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Mul<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
            self * rhs.0
        }
    }

    impl<T> MulAssign<T> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Mul<T, Output = ModInt<T>>,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }

    impl<T> MulAssign<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Mul<ModInt<T>, Output = ModInt<T>>,
    {
        fn mul_assign(&mut self, rhs: ModInt<T>) {
            *self = *self * rhs;
        }
    }

    impl ModInt<u64> {
        pub fn new(v: u64, modulo: u64) -> Self {
            Self(v % modulo, modulo)
        }

        pub fn pow(self, e: u64) -> ModInt<u64> {
            let mut result = ModInt(1, self.1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_int::*;
    use rand::distributions::{IndependentSample, Range};

    const MOD: u64 = 1_000_000_007;

    #[test]
    fn random_add_sub() {
        let between = Range::new(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: u64 = between.ind_sample(&mut rng);
            let y: u64 = between.ind_sample(&mut rng);

            let mx = ModInt(x, MOD);
            let my = ModInt(y, MOD);

            assert_eq!((mx + my).0, (x + y) % MOD);
            assert_eq!((mx + y).0, (x + y) % MOD);
            assert_eq!((mx - my).0, (x + MOD - y) % MOD);
            assert_eq!((mx - y).0, (x + MOD - y) % MOD);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.0, x % MOD);

            mx += y;
            x += y;
            assert_eq!(mx.0, x % MOD);

            mx -= my;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.0, x);

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.0, x);
        }
    }

    #[test]
    fn random_mul() {
        let between = Range::new(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: u64 = between.ind_sample(&mut rng);
            let y: u64 = between.ind_sample(&mut rng);

            let mx = ModInt(x, MOD);
            let my = ModInt(y, MOD);

            assert_eq!((mx * my).0, (x * y) % MOD);
            assert_eq!((mx * y).0, (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt(1_000_000_000, MOD);
        let b = ModInt(7, MOD);
        let c = a + b;
        assert_eq!(c.0, 0);
    }

    #[test]
    fn pow_test() {
        let a = ModInt(3, MOD);
        let a = a.pow(4);
        assert_eq!(a.0, 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = ModInt(1, MOD);
            a /= i;
            a *= i;
            assert_eq!(a.0, 1);
        }
    }

    #[test]
    fn edge_cases() {
        let a = ModInt(1_000_000_000, MOD) * std::u64::MAX;
        assert_eq!(a.0, 923591986);

        let mut a = ModInt(1_000_000_000, MOD);
        a *= std::u64::MAX;
        assert_eq!(a.0, 923591986);

        let a = ModInt(1_000_000_000, MOD) + std::u64::MAX;
        assert_eq!(a.0, 582344000);

        let mut a = ModInt(1_000_000_000, MOD);
        a += std::u64::MAX;
        assert_eq!(a.0, 582344000);

        let a = ModInt(1_000_000_000, MOD) - std::u64::MAX;
        assert_eq!(a.0, 417655993);

        let mut a = ModInt(1_000_000_000, MOD);
        a -= std::u64::MAX;
        assert_eq!(a.0, 417655993);

        let a = ModInt(1_000_000_000, MOD) / std::u64::MAX;
        assert_eq!(a.0, 605455209);

        let mut a = ModInt(1_000_000_000, MOD);
        a /= std::u64::MAX;
        assert_eq!(a.0, 605455209);
    }

    #[test]
    fn overflow_guard() {
        let a = ModInt::new(MOD * 10, MOD);
        assert_eq!(a.0, 0);
    }
}
