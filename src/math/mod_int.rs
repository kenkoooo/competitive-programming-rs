pub mod mod_int {
    use std::cell::RefCell;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type InternalNum = u64;
    thread_local!(
        static MOD: RefCell<InternalNum> = RefCell::new(0);
    );

    pub fn set_mod_int<T>(v: T)
    where
        InternalNum: From<T>,
    {
        MOD.with(|x| x.replace(InternalNum::from(v)));
    }
    fn modulo() -> InternalNum {
        MOD.with(|x| *x.borrow())
    }

    pub struct ModInt(InternalNum);
    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }
    impl Copy for ModInt {}

    impl ModInt {
        pub fn new<T>(v: T) -> Self
        where
            InternalNum: From<T>,
        {
            let mut v = InternalNum::from(v);
            let m = modulo();
            if v >= m {
                v %= m;
            }
            Self(v)
        }

        pub fn internal_pow(&self, mut e: InternalNum) -> Self {
            let mut result = 1;
            let mut cur = self.0;
            let modulo = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= modulo;
                }
                e >>= 1;
                cur = (cur * cur) % modulo;
            }
            Self(result)
        }

        pub fn pow<T>(&self, e: T) -> Self
        where
            InternalNum: From<T>,
        {
            self.internal_pow(InternalNum::from(e))
        }

        pub fn value(&self) -> InternalNum {
            self.0
        }
    }
    impl From<ModInt> for InternalNum {
        fn from(m: ModInt) -> Self {
            m.value()
        }
    }

    impl<T> AddAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }

            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T> Add<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }
    impl<T> SubAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
    impl<T> Sub<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = Self;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }
    impl<T> MulAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }
    impl<T> Mul<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T> DivAssign<T> for ModInt
    where
        InternalNum: From<T>,
    {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = InternalNum::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = Self(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T> Div<T> for ModInt
    where
        InternalNum: From<T>,
    {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_int::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    const PRIME_MOD: [u64; 3] = [1_000_000_007, 1_000_000_009, 998244353];

    fn random_add_sub(prime_mod: u64) {
        let mut rng = rand::thread_rng();
        set_mod_int(prime_mod);
        for _ in 0..10000 {
            let x: u64 = rng.sample(Uniform::from(0..prime_mod));
            let y: u64 = rng.sample(Uniform::from(0..prime_mod));

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

            assert_eq!((mx + my).value(), (x + y) % prime_mod);
            assert_eq!((mx + y).value(), (x + y) % prime_mod);
            assert_eq!((mx - my).value(), (x + prime_mod - y) % prime_mod);
            assert_eq!((mx - y).value(), (x + prime_mod - y) % prime_mod);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.value(), x % prime_mod);

            mx += y;
            x += y;
            assert_eq!(mx.value(), x % prime_mod);

            mx -= my;
            x = (x + prime_mod - y % prime_mod) % prime_mod;
            assert_eq!(mx.value(), x);

            mx -= y;
            x = (x + prime_mod - y % prime_mod) % prime_mod;
            assert_eq!(mx.value(), x);
        }
    }

    #[test]
    fn test_random_add_sub1() {
        random_add_sub(PRIME_MOD[0]);
    }

    #[test]
    fn test_random_add_sub2() {
        random_add_sub(PRIME_MOD[1]);
    }

    #[test]
    fn test_random_add_sub3() {
        random_add_sub(PRIME_MOD[2]);
    }

    fn random_mul(prime_mod: u64) {
        let mut rng = rand::thread_rng();
        set_mod_int(prime_mod);
        for _ in 0..10000 {
            let x: u64 = rng.sample(Uniform::from(0..prime_mod));
            let y: u64 = rng.sample(Uniform::from(0..prime_mod));

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

            assert_eq!((mx * my).value(), (x * y) % prime_mod);
            assert_eq!((mx * y).value(), (x * y) % prime_mod);
        }
    }
    #[test]
    fn test_random_mul1() {
        random_mul(PRIME_MOD[0]);
    }
    #[test]
    fn test_random_mul2() {
        random_mul(PRIME_MOD[1]);
    }
    #[test]
    fn test_random_mul3() {
        random_mul(PRIME_MOD[2]);
    }

    #[test]
    fn zero_test() {
        set_mod_int(1_000_000_007u64);
        let a = ModInt::new(1_000_000_000u64);
        let b = ModInt::new(7u64);
        let c = a + b;
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn pow_test() {
        set_mod_int(1_000_000_007u64);
        let a = ModInt::new(3u64);
        let a = a.pow(4u64);
        assert_eq!(a.value(), 81);
    }

    #[test]
    fn div_test() {
        set_mod_int(1_000_000_007u64);
        for i in 1..100000u64 {
            let mut a = ModInt::new(1u64);
            a /= i;
            a *= i;
            assert_eq!(a.value(), 1);
        }
    }

    #[test]
    fn edge_cases() {
        const MOD: u128 = 1_000_000_007;
        set_mod_int(1_000_000_007u64);

        let a = ModInt::new(1_000_000_000u64) * std::u64::MAX;
        assert_eq!(
            a.value(),
            ((1_000_000_000u128 * u128::from(std::u64::MAX)) % MOD) as u64
        );

        let mut a = ModInt::new(1_000_000_000u64);
        a *= std::u64::MAX;
        assert_eq!(
            a.value(),
            ((1_000_000_000u128 * u128::from(std::u64::MAX)) % MOD) as u64
        );

        let a = ModInt::new(1_000_000_000u64) + std::u64::MAX;
        assert_eq!(
            a.value(),
            ((1_000_000_000u128 + u128::from(std::u64::MAX)) % MOD) as u64
        );

        let mut a = ModInt::new(1_000_000_000u64);
        a += std::u64::MAX;
        assert_eq!(
            a.value(),
            ((1_000_000_000u128 + u128::from(std::u64::MAX)) % MOD) as u64
        );

        let a = ModInt::new(1_000_000_000u64) - std::u64::MAX;
        assert_eq!(
            a.value(),
            ((1_000_000_000u128 + MOD - (std::u64::MAX as u128) % MOD) % MOD) as u64
        );

        let mut a = ModInt::new(1_000_000_000u64);
        a -= std::u64::MAX;
        assert_eq!(
            a.value(),
            ((1_000_000_000u128 + MOD - (std::u64::MAX as u128) % MOD) % MOD) as u64
        );

        let a = ModInt::new(1_000_000_000u64) / std::u64::MAX;
        assert_eq!(a.value(), 605455209);

        let mut a = ModInt::new(1_000_000_000u64);
        a /= std::u64::MAX;
        assert_eq!(a.value(), 605455209);
    }

    #[test]
    fn overflow_guard() {
        set_mod_int(1_000_000_007u64);
        let a = ModInt::new(1_000_000_007u64 * 10);
        assert_eq!(a.value(), 0);
    }
}
