pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

    const MOD: usize = 1_000_000_007;
    type Num = usize;

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
        pub fn new(value: Num) -> Self {
            ModInt(value)
        }
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use self::rand::distributions::{IndependentSample, Range};
    use super::mod_int::*;

    const MOD: usize = 1_000_000_007;

    #[test]
    fn random_add_sub() {
        let between = Range::new(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: usize = between.ind_sample(&mut rng);
            let y: usize = between.ind_sample(&mut rng);

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

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

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

            assert_eq!((mx * my).0, (x * y) % MOD);
            assert_eq!((mx * y).0, (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;
        assert_eq!(c.0, 0);
    }
}
