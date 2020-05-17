pub mod mod_int {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

    #[derive(Clone, Copy)]
    pub struct ModInt<T: Copy + Clone> {
        pub v: T,
        pub modulo: T,
    }

    impl<T> Add<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Clone + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn add(self, mut rhs: T) -> ModInt<T> {
            if rhs >= self.modulo {
                rhs = rhs % self.modulo;
            }
            let mut t = rhs + self.v;
            if t >= self.modulo {
                t = t - self.modulo;
            }
            ModInt {
                v: t,
                modulo: self.modulo,
            }
        }
    }

    impl<T> Add<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Add<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn add(self, rhs: ModInt<T>) -> ModInt<T> {
            self + rhs.v
        }
    }

    impl<T> Sub<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Clone + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: T) -> ModInt<T> {
            let rhs = if rhs >= self.modulo {
                rhs % self.modulo
            } else {
                rhs
            };
            let value = if self.v < rhs {
                self.v + self.modulo
            } else {
                self.v
            };
            ModInt {
                v: value - rhs,
                modulo: self.modulo,
            }
        }
    }

    impl<T> Sub<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Sub<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
            self - rhs.v
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
            if rhs >= self.modulo {
                rhs %= self.modulo;
            }
            self * ModInt {
                v: rhs,
                modulo: self.modulo,
            }
            .pow(self.modulo - 2)
        }
    }

    impl<T> Div<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Div<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn div(self, rhs: ModInt<T>) -> ModInt<T> {
            self / rhs.v
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
            if rhs >= self.modulo {
                rhs = rhs % self.modulo;
            }
            let t = (self.v * rhs) % self.modulo;
            ModInt {
                v: t,
                modulo: self.modulo,
            }
        }
    }
    impl<T> Mul<ModInt<T>> for ModInt<T>
    where
        T: Clone + Copy,
        ModInt<T>: Mul<T, Output = ModInt<T>>,
    {
        type Output = ModInt<T>;
        fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
            self * rhs.v
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
            Self {
                v: v % modulo,
                modulo,
            }
        }

        pub fn pow(self, e: u64) -> ModInt<u64> {
            let mut result = ModInt::new(1, self.modulo);
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

            let mx = ModInt::new(x, MOD);
            let my = ModInt::new(y, MOD);

            assert_eq!((mx + my).v, (x + y) % MOD);
            assert_eq!((mx + y).v, (x + y) % MOD);
            assert_eq!((mx - my).v, (x + MOD - y) % MOD);
            assert_eq!((mx - y).v, (x + MOD - y) % MOD);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.v, x % MOD);

            mx += y;
            x += y;
            assert_eq!(mx.v, x % MOD);

            mx -= my;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.v, x);

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.v, x);
        }
    }

    #[test]
    fn random_mul() {
        let between = Range::new(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: u64 = between.ind_sample(&mut rng);
            let y: u64 = between.ind_sample(&mut rng);

            let mx = ModInt::new(x, MOD);
            let my = ModInt::new(y, MOD);

            assert_eq!((mx * my).v, (x * y) % MOD);
            assert_eq!((mx * y).v, (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt::new(1_000_000_000, MOD);
        let b = ModInt::new(7, MOD);
        let c = a + b;
        assert_eq!(c.v, 0);
    }

    #[test]
    fn pow_test() {
        let a = ModInt::new(3, MOD);
        let a = a.pow(4);
        assert_eq!(a.v, 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = ModInt::new(1, MOD);
            a /= i;
            a *= i;
            assert_eq!(a.v, 1);
        }
    }

    #[test]
    fn edge_cases() {
        let a = ModInt::new(1_000_000_000, MOD) * std::u64::MAX;
        assert_eq!(a.v, 923591986);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a *= std::u64::MAX;
        assert_eq!(a.v, 923591986);

        let a = ModInt::new(1_000_000_000, MOD) + std::u64::MAX;
        assert_eq!(a.v, 582344000);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a += std::u64::MAX;
        assert_eq!(a.v, 582344000);

        let a = ModInt::new(1_000_000_000, MOD) - std::u64::MAX;
        assert_eq!(a.v, 417655993);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a -= std::u64::MAX;
        assert_eq!(a.v, 417655993);

        let a = ModInt::new(1_000_000_000, MOD) / std::u64::MAX;
        assert_eq!(a.v, 605455209);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a /= std::u64::MAX;
        assert_eq!(a.v, 605455209);
    }

    #[test]
    fn overflow_guard() {
        let a = ModInt::new(MOD * 10, MOD);
        assert_eq!(a.v, 0);
    }
}
