pub mod z_algorithm {

    pub fn calc_z_array<T: PartialEq>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        let mut z_array = vec![0; n];

        // [l, r) is a window which matches with prefix of s
        let mut l = 0;
        let mut r = 1;
        for i in 1..n {
            if i >= r {
                l = i;
                r = i + 1;
                while r <= n && s[r - 1 - l] == s[r - 1] {
                    r += 1;
                }
                z_array[i] = r - l - 1;
                r -= 1;
            } else if z_array[i - l] < r - i {
                z_array[i] = z_array[i - l];
            } else {
                l = i;
                while r <= n && s[r - 1 - l] == s[r - 1] {
                    r += 1;
                }
                z_array[i] = r - l - 1;
                r -= 1;
            }
        }
        z_array[0] = n;
        z_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    #[test]
    fn test_z() {
        let n = 1000;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut s = String::new();
            for _ in 0..n {
                let c = (rng.sample(Uniform::from(0..26)) as u8 + 'a' as u8) as char;
                s.push(c);
            }

            let t = String::new() + s.as_str() + s.as_str();

            let z_array = z_algorithm::calc_z_array(&t.as_bytes());
            let n = t.len();
            for i in 0..n {
                let l = z_array[i];
                assert_eq!(&t[0..l], &t[i..(i + l)]);
                assert!(i + l >= t.len() || t[l..(l + 1)] != t[(i + l)..(i + l + 1)]);
            }
        }
    }
}
