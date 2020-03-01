#[derive(Copy, Clone)]
struct Point(f64, f64);

fn cc_intersect((p1, r1): (Point, f64), (p2, r2): (Point, f64)) -> Vec<Point> {
    let dist = (p1 - p2).abs();
    if dist > r1 + r2 {
        return Vec::new();
    }

    let rc = (dist * dist + r1 * r1 - r2 * r2) / (2. * dist);
    let rs = (r1 * r1 - rc * rc).sqrt();
    let diff = (p2 - p1) / dist;
    vec![p1 + diff * Point(rc, rs)]
}

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
