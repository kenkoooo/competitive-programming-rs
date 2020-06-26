pub mod mod_int {
    use std::ops::{
        Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, RemAssign, ShrAssign, Sub,
        SubAssign,
    };

    pub struct ModInt<T> {
        v: T,
        m: T,
    }

    impl<T> ModInt<T>
    where
        T: Copy,
    {
        pub fn value(&self) -> T {
            self.v
        }
        pub fn modulo(&self) -> T {
            self.m
        }
    }

    impl<T> Copy for ModInt<T> where T: Copy {}
    impl<T> Clone for ModInt<T>
    where
        T: Copy,
    {
        fn clone(&self) -> Self {
            Self::new_unchecked(self.value(), self.modulo())
        }
    }

    impl<T> Add<T> for ModInt<T>
    where
        T: AddAssign + SubAssign + RemAssign + Copy + PartialOrd,
    {
        type Output = Self;
        fn add(self, mut rhs: T) -> Self::Output {
            if rhs >= self.modulo() {
                rhs %= self.modulo();
            }
            rhs += self.value();
            if rhs >= self.modulo() {
                rhs -= self.modulo();
            }
            Self::new_unchecked(rhs, self.modulo())
        }
    }

    impl<T> Sub<T> for ModInt<T>
    where
        T: AddAssign + SubAssign + RemAssign + Copy + PartialOrd,
    {
        type Output = Self;
        fn sub(self, mut rhs: T) -> Self::Output {
            if rhs >= self.modulo() {
                rhs %= self.modulo();
            }

            let mut result = self.value();
            result += self.modulo();
            result -= rhs;

            if result >= self.modulo() {
                result -= self.modulo();
            }
            Self::new_unchecked(result, self.modulo())
        }
    }

    impl<T> Mul<T> for ModInt<T>
    where
        T: MulAssign + RemAssign + Copy + PartialOrd,
    {
        type Output = Self;
        fn mul(self, mut rhs: T) -> Self::Output {
            if rhs >= self.modulo() {
                rhs %= self.modulo();
            }
            rhs *= self.value();
            rhs %= self.modulo();
            Self::new_unchecked(rhs, self.modulo())
        }
    }

    impl<T> Add<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Add<T, Output = ModInt<T>>,
    {
        type Output = Self;
        fn add(self, rhs: ModInt<T>) -> Self::Output {
            self + rhs.value()
        }
    }
    impl<T> Sub<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Sub<T, Output = ModInt<T>>,
    {
        type Output = Self;
        fn sub(self, rhs: ModInt<T>) -> Self::Output {
            self - rhs.value()
        }
    }
    impl<T> Mul<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Mul<T, Output = ModInt<T>>,
    {
        type Output = Self;
        fn mul(self, rhs: ModInt<T>) -> Self::Output {
            self * rhs.value()
        }
    }
    impl<T> Div<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Div<T, Output = ModInt<T>>,
    {
        type Output = Self;
        fn div(self, rhs: ModInt<T>) -> Self::Output {
            self / rhs.value()
        }
    }

    impl<T> AddAssign<T> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Add<T, Output = ModInt<T>>,
    {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<T> AddAssign<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Add<ModInt<T>, Output = ModInt<T>>,
    {
        fn add_assign(&mut self, other: ModInt<T>) {
            *self = *self + other;
        }
    }

    impl<T> SubAssign<T> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Sub<T, Output = ModInt<T>>,
    {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }

    impl<T> SubAssign<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Sub<ModInt<T>, Output = ModInt<T>>,
    {
        fn sub_assign(&mut self, other: ModInt<T>) {
            *self = *self - other;
        }
    }

    impl<T> DivAssign<T> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Div<T, Output = ModInt<T>>,
    {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs
        }
    }
    impl<T> DivAssign<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Div<ModInt<T>, Output = ModInt<T>>,
    {
        fn div_assign(&mut self, rhs: ModInt<T>) {
            *self = *self / rhs
        }
    }

    impl<T> MulAssign<T> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Mul<T, Output = ModInt<T>>,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }

    impl<T> MulAssign<ModInt<T>> for ModInt<T>
    where
        T: Copy,
        ModInt<T>: Mul<ModInt<T>, Output = ModInt<T>>,
    {
        fn mul_assign(&mut self, rhs: ModInt<T>) {
            *self = *self * rhs;
        }
    }

    impl<T> Div<T> for ModInt<T>
    where
        T: Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + BitAnd<Output = T>
            + PartialEq
            + PartialOrd
            + ShrAssign
            + RemAssign
            + MulAssign,
    {
        type Output = Self;
        fn div(self, mut rhs: T) -> Self::Output {
            if rhs >= self.modulo() {
                rhs %= self.modulo();
            }
            let one = self.modulo() / self.modulo();
            let two = one + one;
            self * Self::new_unchecked(rhs, self.modulo()).pow(self.modulo() - two)
        }
    }

    impl<T> ModInt<T> {
        fn new_unchecked(v: T, modulo: T) -> Self {
            Self { v, m: modulo }
        }
    }

    impl<T> ModInt<T>
    where
        T: Copy + RemAssign + PartialOrd,
    {
        pub fn new(mut v: T, modulo: T) -> Self {
            if v >= modulo {
                v %= modulo;
            }
            Self::new_unchecked(v, modulo)
        }
    }

    impl<T> ModInt<T>
    where
        T: Copy
            + Sub<Output = T>
            + ShrAssign
            + BitAnd<Output = T>
            + PartialEq
            + PartialOrd
            + Div<Output = T>
            + RemAssign,
        ModInt<T>: MulAssign,
    {
        pub fn pow(self, e: T) -> Self {
            let zero = self.modulo() - self.modulo();
            let one = self.modulo() / self.modulo();
            let mut e = e;
            let mut result = Self::new_unchecked(one, self.modulo());
            let mut cur = self;
            while e > zero {
                if e & one == one {
                    result *= cur;
                }
                e >>= one;
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

            assert_eq!((mx + my).value(), (x + y) % MOD);
            assert_eq!((mx + y).value(), (x + y) % MOD);
            assert_eq!((mx - my).value(), (x + MOD - y) % MOD);
            assert_eq!((mx - y).value(), (x + MOD - y) % MOD);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.value(), x % MOD);

            mx += y;
            x += y;
            assert_eq!(mx.value(), x % MOD);

            mx -= my;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.value(), x);

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.value(), x);
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

            assert_eq!((mx * my).value(), (x * y) % MOD);
            assert_eq!((mx * y).value(), (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt::new(1_000_000_000, MOD);
        let b = ModInt::new(7, MOD);
        let c = a + b;
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn pow_test() {
        let a = ModInt::new(3, MOD);
        let a = a.pow(4);
        assert_eq!(a.value(), 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = ModInt::new(1, MOD);
            a /= i;
            a *= i;
            assert_eq!(a.value(), 1);
        }
    }

    #[test]
    fn edge_cases() {
        let a = ModInt::new(1_000_000_000, MOD) * std::u64::MAX;
        assert_eq!(a.value(), 923591986);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a *= std::u64::MAX;
        assert_eq!(a.value(), 923591986);

        let a = ModInt::new(1_000_000_000, MOD) + std::u64::MAX;
        assert_eq!(a.value(), 582344000);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a += std::u64::MAX;
        assert_eq!(a.value(), 582344000);

        let a = ModInt::new(1_000_000_000, MOD) - std::u64::MAX;
        assert_eq!(a.value(), 417655993);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a -= std::u64::MAX;
        assert_eq!(a.value(), 417655993);

        let a = ModInt::new(1_000_000_000, MOD) / std::u64::MAX;
        assert_eq!(a.value(), 605455209);

        let mut a = ModInt::new(1_000_000_000, MOD);
        a /= std::u64::MAX;
        assert_eq!(a.value(), 605455209);
    }

    #[test]
    fn overflow_guard() {
        let a = ModInt::new(MOD * 10, MOD);
        assert_eq!(a.value(), 0);
    }
}
