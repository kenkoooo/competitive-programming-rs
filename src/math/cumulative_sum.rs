pub struct CumulativeSum<T>(Vec<Vec<T>>);

impl<T> CumulativeSum<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    pub fn new(a: &[Vec<T>], init: T) -> CumulativeSum<T> {
        assert!(!a.is_empty());
        let h = a.len();
        let w = a[0].len();
        let mut sum = vec![vec![init; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                sum[i + 1][j + 1] = a[i][j] + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
            }
        }
        CumulativeSum(sum)
    }

    pub fn get_sum(&self, h_range: std::ops::Range<usize>, w_range: std::ops::Range<usize>) -> T {
        assert!(h_range.end <= self.0.len());
        assert!(w_range.end <= self.0[0].len());
        self.0[h_range.end][w_range.end] + self.0[h_range.start][w_range.start]
            - self.0[h_range.start][w_range.end]
            - self.0[h_range.end][w_range.start]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    #[test]
    fn random_array() {
        let h = 30;
        let w = 20;

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let mut array = vec![vec![0; w]; h];
            for i in 0..h {
                for j in 0..w {
                    array[i][j] = rng.sample(Uniform::from(10..10000));
                }
            }

            let sum = CumulativeSum::new(&array, 0);
            for from_i in 0..h {
                for to_i in (from_i + 1)..=h {
                    for from_j in 0..w {
                        for to_j in (from_j + 1)..=w {
                            let mut check = 0;
                            for i in from_i..to_i {
                                for j in from_j..to_j {
                                    check += array[i][j];
                                }
                            }

                            assert_eq!(check, sum.get_sum(from_i..to_i, from_j..to_j));
                        }
                    }
                }
            }
        }
    }
}
