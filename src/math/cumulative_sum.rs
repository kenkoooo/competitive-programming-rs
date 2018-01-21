use std::cmp;

struct CumulativeSum {
    ny: usize,
    nx: usize,
    sum: Vec<Vec<usize>>,
}

impl CumulativeSum {
    fn new(a: &Vec<Vec<usize>>) -> CumulativeSum {
        let ny = a.len();
        let nx = a[0].len();
        let mut sum = vec![vec![0; nx + 1]; ny + 1];
        for i in 0..ny {
            for j in 0..nx {
                sum[i + 1][j + 1] = a[i][j] + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
            }
        }
        CumulativeSum { ny: ny, nx: nx, sum: sum }
    }

    fn get_sum(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> usize {
        if y1 > y2 || x1 > x2 {
            return 0;
        }
        let y2 = cmp::min(y2, self.ny - 1);
        let x2 = cmp::min(x2, self.nx - 1);
        return self.sum[y2 + 1][x2 + 1] + self.sum[y1][x1] - self.sum[y1][x2 + 1] - self.sum[y2 + 1][x1];
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::Rng;
    use self::rand::distributions::{IndependentSample, Range};
    use std::cmp;

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

            let sum = CumulativeSum::new(&array);
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