pub mod circle;
pub mod convex_hull;
pub mod minimum_bounding_circle;

pub mod basic {
    #[derive(Copy, Clone, Debug)]
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T>
    where
        T: std::ops::Mul<T, Output = T> + std::ops::Sub<T, Output = T> + Copy,
    {
        pub fn det(&self, p: Point<T>) -> T {
            self.x * p.y - self.y * p.x
        }
    }

    impl<T> std::ops::Add for Point<T>
    where
        T: std::ops::Add<T, Output = T>,
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T> std::ops::Sub for Point<T>
    where
        T: std::ops::Sub<T, Output = T>,
    {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    impl<T> std::ops::Mul<T> for Point<T>
    where
        T: std::ops::Mul<T, Output = T> + Copy,
    {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Point {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    pub struct Segment<T> {
        pub from: Point<T>,
        pub to: Point<T>,
    }

    impl<T> Segment<T>
    where
        T: PartialOrd
            + Copy
            + PartialEq
            + std::ops::Add<T, Output = T>
            + std::ops::Sub<T, Output = T>
            + std::ops::Mul<T, Output = T>
            + std::ops::Div<T, Output = T>,
    {
        pub fn cross_point(&self, seg: &Segment<T>) -> Option<Point<T>> {
            let (a, b) = (self.from, self.to);
            let (c, d) = (seg.from, seg.to);
            let dc = d - c;
            let ba = b - a;
            if dc.x * ba.y == dc.y * ba.x {
                return None;
            }

            let p = a + (b - a) * ((a - c).det(d - c) / (d - c).det(b - a));
            Some(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::Tester;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_2_C
    #[test]
    fn solve_cgl_2_c() {
        use basic::*;
        let tester = Tester::new("./assets/CGL_2_C/in/", "./assets/CGL_2_C/out/");
        tester.test_solution_with(
            |sc| {
                let q: usize = sc.read();
                for _ in 0..q {
                    let t: Vec<f64> = sc.vec(8);
                    let seg1 = Segment {
                        from: Point { x: t[0], y: t[1] },
                        to: Point { x: t[2], y: t[3] },
                    };
                    let seg2 = Segment {
                        from: Point { x: t[4], y: t[5] },
                        to: Point { x: t[6], y: t[7] },
                    };
                    let p = seg1.cross_point(&seg2).unwrap();
                    sc.write(format!("{} {}\n", p.x, p.y));
                }
            },
            |expected, actual| {
                assert!((expected.read::<f64>() - actual.read::<f64>()).abs() < 1e-6);
                assert!((expected.read::<f64>() - actual.read::<f64>()).abs() < 1e-6);
            },
        );
    }
}
