use std::ops::{Add, Rem, Sub, Mul};

#[derive(Copy)]
pub struct ModInt<T> {
    value: T,
    modulo: T,
}

impl<T> Clone for ModInt<T> where T: Copy {
    fn clone(&self) -> Self { ModInt { value: self.value, modulo: self.modulo } }

    fn clone_from(&mut self, source: &ModInt<T>) {
        self.value = source.value;
        self.modulo = source.modulo;
    }
}

impl<T> Add<ModInt<T>> for ModInt<T> where T: Add<Output=T> + Sub<Output=T> + Copy + PartialOrd {
    type Output = ModInt<T>;

    fn add(self, rhs: ModInt<T>) -> ModInt<T> {
        let m = rhs.modulo;
        let mut t = rhs.value + self.value;
        if t > m { t = t - m; }
        ModInt { value: t, modulo: self.modulo }
    }
}

impl<T> Add<T> for ModInt<T> where T: Add<Output=T> + Sub<Output=T> + Copy + PartialOrd {
    type Output = ModInt<T>;

    fn add(self, rhs: T) -> ModInt<T> {
        let m = self.modulo;
        let mut t = rhs + self.value;
        if t > m { t = t - m; }
        ModInt { value: t, modulo: self.modulo }
    }
}

impl<T> Mul<ModInt<T>> for ModInt<T> where T: Mul<Output=T> + Rem<Output=T> + Copy {
    type Output = ModInt<T>;

    fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
        let t = (self.value * rhs.value) % self.modulo;
        ModInt { value: t, modulo: self.modulo }
    }
}

impl<T> Mul<T> for ModInt<T> where T: Mul<Output=T> + Rem<Output=T> + Copy {
    type Output = ModInt<T>;

    fn mul(self, rhs: T) -> ModInt<T> {
        let t = (self.value * rhs) % self.modulo;
        ModInt { value: t, modulo: self.modulo }
    }
}

impl<T> ModInt<T> {
    pub fn new(value: T, modulo: T) -> ModInt<T> {
        ModInt { value: value, modulo: modulo }
    }
}


#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::distributions::{IndependentSample, Range};


    #[test]
    fn random_add() {
        let modulo = 1_000_000_007;
        let between = Range::new(0, modulo);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x = between.ind_sample(&mut rng);
            let y = between.ind_sample(&mut rng);

            let mx = ModInt::new(x, modulo);
            let my = ModInt::new(y, modulo);

            assert_eq!((mx + my).value, (x + y) % modulo);
            assert_eq!((mx + y).value, (x + y) % modulo);
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
}