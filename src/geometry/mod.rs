pub mod circle;
pub mod convex_hull;
pub mod minimum_bounding_circle;

pub mod basic {
    pub struct Point<T> {
        pub x: T,
        pub y: T,
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
            if (self.to.x - self.from.x) * (seg.to.y - seg.from.y)
                == (self.to.y - self.from.y) * (seg.to.x - seg.from.x)
            {
                return None;
            }
            let c = (self.to.x - self.from.x) * (seg.to.y - seg.from.y)
                - (self.to.y - self.from.y) * (seg.to.x - seg.from.x);

            let ac = Point {
                x: seg.from.x - self.from.x,
                y: seg.from.y - self.from.y,
            };

            let r = ((seg.to.y - seg.from.y) * ac.x - (seg.to.x - seg.from.x) * ac.y) / c;
            let distance = Point {
                x: (self.to.x - self.from.x) * r,
                y: (self.to.y - self.from.y) * r,
            };
            let cross_point = Point {
                x: self.from.x + distance.x,
                y: self.from.y + distance.y,
            };
            Some(cross_point)
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
