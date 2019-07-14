pub mod mod_int {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;
    const MOD: Num = 1_000_000_007;

    #[derive(Clone, Copy)]
    pub struct ModInt<T: Copy + Clone>(pub T);

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.0
        }
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: Num) -> ModInt<Num> {
            let mut t = rhs + self.0;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt(t)
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.0 < rhs { self.0 + MOD } else { self.0 };
            ModInt(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.0
        }
    }

    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other;
        }
    }
    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other;
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other;
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other;
        }
    }

    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: Num) -> ModInt<Num> {
            self * ModInt(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.0
        }
    }

    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }
    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;

        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.0
        }
    }
    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;

        fn mul(self, rhs: Num) -> ModInt<Num> {
            let t = (self.0 * rhs) % MOD;
            ModInt(t)
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs;
        }
    }

    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs;
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: usize) -> ModInt<Num> {
            let mut result = ModInt(1);
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

    const MOD: usize = 1_000_000_007;

    #[test]
    fn random_add_sub() {
        let between = Range::new(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: usize = between.ind_sample(&mut rng);
            let y: usize = between.ind_sample(&mut rng);

            let mx = ModInt(x);
            let my = ModInt(y);

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
            let x: usize = between.ind_sample(&mut rng);
            let y: usize = between.ind_sample(&mut rng);

            let mx = ModInt(x);
            let my = ModInt(y);

            assert_eq!((mx * my).0, (x * y) % MOD);
            assert_eq!((mx * y).0, (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt(1_000_000_000);
        let b = ModInt(7);
        let c = a + b;
        assert_eq!(c.0, 0);
    }

    #[test]
    fn pow_test() {
        let a = ModInt(3);
        let a = a.pow(4);
        assert_eq!(a.0, 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = ModInt(1);
            a /= i;
            a *= i;
            assert_eq!(a.0, 1);
        }
    }
}
