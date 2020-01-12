pub mod minimum_bounding_circle {
    pub(crate) fn make_circle(ps: &[Point<f64>]) -> (Point<f64>, f64) {
        let n = ps.len();
        assert!(n >= 2);

        let mut c = make_circle2(ps[0], ps[1]);
        for i in 2..n {
            if is_included(ps[i], c) {
                continue;
            }
            c = make_circle2(ps[0], ps[i]);
            for j in 1..i {
                if is_included(ps[j], c) {
                    continue;
                }
                c = make_circle2(ps[i], ps[j]);
                for k in 0..j {
                    if is_included(ps[k], c) {
                        continue;
                    }
                    c = make_circle3(ps[i], ps[j], ps[k]);
                }
            }
        }
        c
    }

    fn make_circle3(a: Point<f64>, b: Point<f64>, c: Point<f64>) -> (Point<f64>, f64) {
        let ea = (b - c).norm();
        let eb = (c - a).norm();
        let ec = (a - b).norm();
        let s = (b - a).det(&(c - a));

        let p = (a * ea * (eb + ec - ea) + b * eb * (ec + ea - eb) + c * ec * (ea + eb - ec))
            / (s * s * 4.0);
        let r2 = (p - a).norm();
        (p, r2)
    }

    fn make_circle2(a: Point<f64>, b: Point<f64>) -> (Point<f64>, f64) {
        let c = (a + b) / 2.0;
        let r2 = (a - c).norm();
        (c, r2)
    }

    fn is_included(a: Point<f64>, circle: (Point<f64>, f64)) -> bool {
        let (center, r2) = circle;
        (a - center).norm() <= r2
    }

    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Copy for Point<T> where T: Copy {}

    impl<T> Clone for Point<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Point<T> {
            Point {
                x: self.x.clone(),
                y: self.y.clone(),
            }
        }
    }

    impl<T> std::ops::Sub for Point<T>
    where
        T: std::ops::Sub<Output = T>,
    {
        type Output = Point<T>;
        fn sub(self, other: Point<T>) -> Point<T> {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl<T> std::ops::Mul<T> for Point<T>
    where
        T: Copy + std::ops::Mul<Output = T>,
    {
        type Output = Point<T>;
        fn mul(self, rhs: T) -> Point<T> {
            Point {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl<T> std::ops::Div<T> for Point<T>
    where
        T: Copy + std::ops::Div<Output = T>,
    {
        type Output = Point<T>;
        fn div(self, rhs: T) -> Point<T> {
            Point {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    impl<T> std::ops::Add<Point<T>> for Point<T>
    where
        T: std::ops::Add<Output = T>,
    {
        type Output = Point<T>;

        fn add(self, rhs: Point<T>) -> Point<T> {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T> Point<T>
    where
        T: Copy + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
    {
        pub fn det(&self, other: &Point<T>) -> T {
            self.x * other.y - self.y * other.x
        }
        pub fn norm(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::minimum_bounding_circle::*;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_aoj2423() {
        let tester = Tester::new("./assets/AOJ2423/in/", "./assets/AOJ2423/out/");
        tester.test_solution(|sc| {
            let circles: usize = sc.read();
            let people: usize = sc.read();
            let rs: Vec<f64> = sc.vec(circles);
            let mut people_r2 = vec![0.0; people];
            let mut people_list = vec![];
            for i in 0..people {
                let n: usize = sc.read();
                let mut ps = vec![];
                for _ in 0..n {
                    let x: f64 = sc.read();
                    let y: f64 = sc.read();
                    ps.push(Point { x, y });
                }
                let (_, r2) = make_circle(&ps);
                people_list.push((r2, i));
                people_r2[i] = r2;
            }

            let mut circle_list = rs
                .iter()
                .enumerate()
                .map(|(i, &r)| (r, i))
                .collect::<Vec<_>>();
            circle_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            people_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            let mut ans = vec![0; people];

            for _ in 0..people {
                let mut circle_buf = 0;
                let mut people_buf = people_list.len();
                for p_pos in (0..people_list.len()).rev() {
                    for c_pos in (0..(circle_list.len() - circle_buf)).rev() {
                        if circle_list[c_pos].0 * circle_list[c_pos].0 >= people_list[p_pos].0 {
                            circle_buf += 1;
                        } else {
                            break;
                        }
                    }
                    if people_list.len() - p_pos > circle_buf {
                        sc.write("NG\n");
                        return;
                    } else if people_list.len() - p_pos == circle_buf {
                        people_buf = people_list.len() - p_pos;
                        break;
                    }
                }

                let youngest_people = people_list
                    .iter()
                    .rev()
                    .take(people_buf)
                    .map(|p| p.1)
                    .min()
                    .unwrap();
                let youngest_circle = circle_list
                    .iter()
                    .rev()
                    .take(circle_buf)
                    .map(|c| c.1)
                    .filter(|&i| rs[i] * rs[i] >= people_r2[youngest_people])
                    .min()
                    .unwrap();
                ans[youngest_people] = youngest_circle;
                people_list = people_list
                    .into_iter()
                    .filter(|p| p.1 != youngest_people)
                    .collect::<Vec<_>>();
                circle_list = circle_list
                    .into_iter()
                    .filter(|c| c.1 != youngest_circle)
                    .collect::<Vec<_>>();
            }

            for i in ans.into_iter() {
                sc.write(format!("{}\n", i + 1));
            }
        });
    }
}
