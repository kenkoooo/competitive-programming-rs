pub fn cc_intersect((p1, r1): (Point, f64), (p2, r2): (Point, f64)) -> Vec<Point> {
    let dist = (p1 - p2).abs();
    if dist > r1 + r2 {
        return Vec::new();
    }

    let rc = (dist * dist + r1 * r1 - r2 * r2) / (2. * dist);
    let rs = (r1 * r1 - rc * rc).sqrt();
    let diff = (p2 - p1) / dist;
    vec![p1 + diff * Point(rc, rs), p1 + diff * Point(rc, -rs)]
}

#[derive(Copy, Clone)]
pub struct Point(f64, f64);

impl Point {
    fn abs(self) -> f64 {
        let x = self.0;
        let y = self.1;
        (x * x + y * y).sqrt()
    }
}

impl ::std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ::std::ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ::std::ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl ::std::ops::Mul<Point> for Point {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        let Point(x1, y1) = self;
        let Point(x2, y2) = rhs;
        Point(x1 * x2 - y1 * y2, x1 * y2 + y1 * x2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_cgl_7_e() {
        let tester = Tester::new("./assets/CGL_7_E/in/", "./assets/CGL_7_E/out/");
        tester.test_solution_with(
            |sc| {
                let c1 = (Point(sc.read(), sc.read()), sc.read());
                let c2 = (Point(sc.read(), sc.read()), sc.read());

                let mut points = cc_intersect(c1, c2)
                    .into_iter()
                    .map(|Point(x, y)| (x, y))
                    .collect::<Vec<_>>();
                if points[0] > points[1] {
                    points.swap(0, 1);
                }
                sc.write(format!(
                    "{} {} {} {}\n",
                    points[0].0, points[0].1, points[1].0, points[1].1
                ));
            },
            |expected, actual| {
                assert!((expected.read::<f64>() - actual.read::<f64>()).abs() < 1e-6);
                assert!((expected.read::<f64>() - actual.read::<f64>()).abs() < 1e-6);
                assert!((expected.read::<f64>() - actual.read::<f64>()).abs() < 1e-6);
                assert!((expected.read::<f64>() - actual.read::<f64>()).abs() < 1e-6);
            },
        );
    }
}
