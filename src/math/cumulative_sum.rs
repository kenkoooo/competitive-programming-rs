pub struct CumulativeSum<T> {
    ny: usize,
    nx: usize,
    sum: Vec<Vec<T>>,
}

impl<T> CumulativeSum<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    pub fn new(a: &[Vec<T>], init: T) -> CumulativeSum<T> {
        assert!(!a.is_empty());
        let ny = a.len();
        let nx = a[0].len();
        let mut sum = vec![vec![init; nx + 1]; ny + 1];
        for i in 0..ny {
            for j in 0..nx {
                sum[i + 1][j + 1] = a[i][j] + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
            }
        }
        CumulativeSum { ny, nx, sum }
    }

    pub fn get_sum(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> T {
        assert!(y1 <= y2 && x1 <= x2);
        assert!(y2 < self.ny);
        assert!(x2 < self.nx);
        self.sum[y2 + 1][x2 + 1] + self.sum[y1][x1] - self.sum[y1][x2 + 1] - self.sum[y2 + 1][x1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::{IndependentSample, Range};

    #[test]
    fn random_array() {
        let h = 30;
        let w = 20;

        let between = Range::new(10, 10000);
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let mut array = vec![vec![0; w]; h];
            for i in 0..h {
                for j in 0..w {
                    array[i][j] = between.ind_sample(&mut rng);
                }
            }

            let sum = CumulativeSum::new(&array, 0);
            for i in 0..h {
                for j in 0..w {
                    for i2 in i..h {
                        for j2 in j..w {
                            let mut check = 0;
                            for k in i..(i2 + 1) {
                                for l in j..(j2 + 1) {
                                    check += array[k][l];
                                }
                            }

                            assert_eq!(check, sum.get_sum(i, j, i2, j2));
                        }
                    }
                }
            }
        }
    }
}
