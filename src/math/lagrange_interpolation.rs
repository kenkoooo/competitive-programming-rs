pub fn lagrange_interpolation<T>(xs: &[T], ys: &[T], one: T, zero: T) -> Vec<T>
where
    T: Clone
        + Copy
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign,
{
    let n = xs.len();

    let mut all_c = vec![zero; n + 1];
    all_c[0] = one;
    for &x in xs.iter() {
        let mut next = vec![zero; n + 1];
        next[1..(n + 1)].clone_from_slice(&all_c[..n]);
        for j in 0..n {
            next[j] -= x * all_c[j];
        }
        all_c = next;
    }

    let mut c = vec![zero; n];
    for i in 0..n {
        let mut qi = one;
        for j in 0..n {
            if i == j {
                continue;
            }
            qi *= xs[i] - xs[j];
        }

        let ri = ys[i] / qi;
        let mut tmp_c = all_c.clone();
        for j in (0..n).rev() {
            c[j] += ri * tmp_c[j + 1];
            let next_c = tmp_c[j + 1] * xs[i];
            tmp_c[j] += next_c;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::mod_int::mod_int::{set_mod_int, ModInt};
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};

    const MOD: u64 = 1_000_000_007;

    #[test]
    fn test_lagrange_interpolation() {
        set_mod_int(MOD);
        let range = Uniform::from(0..std::u64::MAX);
        let mut rng = thread_rng();

        let n = 500;
        for _ in 0..10 {
            let mut xs = vec![];
            let mut ys = vec![];
            for _ in 0..n {
                xs.push(ModInt::new(rng.sample(range)));
                ys.push(ModInt::new(rng.sample(range)));
            }

            let c = lagrange_interpolation(&xs, &ys, ModInt::new(1u64), ModInt::new(0u64));

            for i in 0..n {
                let mut y = ModInt::new(0u64);
                let x = xs[i];
                for i in 0..n {
                    y += x.pow(i as u64) * c[i];
                }
                assert_eq!(y.value(), ys[i].value());
            }
        }
    }
}
