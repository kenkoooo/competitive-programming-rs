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
    for i in 0..n {
        let mut next = vec![zero; n + 1];
        for j in 0..n {
            next[j + 1] = all_c[j];
        }
        for j in 0..n {
            next[j] -= xs[i] * all_c[j];
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
    use crate::math::mod_int::mod_int::ModInt;
    use rand::{thread_rng, Rng};

    const MOD: u64 = 1e9 as u64 + 7;

    #[test]
    fn test_lagrange_interpolation() {
        let mut rng = thread_rng();

        let n = 500;
        for _ in 0..10 {
            let mut xs = vec![];
            let mut ys = vec![];
            for _ in 0..n {
                xs.push(ModInt::new(rng.gen(), MOD));
                ys.push(ModInt::new(rng.gen(), MOD));
            }

            let c = lagrange_interpolation(&xs, &ys, ModInt::new(1, MOD), ModInt::new(0, MOD));

            for i in 0..n {
                let mut y = ModInt::new(0, MOD);
                let x = xs[i];
                for i in 0..n {
                    y += x.pow(i as u64) * c[i];
                }
                assert_eq!(y.v, ys[i].v);
            }
        }
    }
}
