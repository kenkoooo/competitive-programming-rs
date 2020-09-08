pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }

    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == 0 {
        ans
    } else {
        ans += (n - (x_max + a - 1) / a) * y_max;
        ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_sum() {
        assert_eq!(3, floor_sum(4, 10, 6, 3));
        assert_eq!(13, floor_sum(6, 5, 4, 3));
        assert_eq!(0, floor_sum(1, 1, 0, 0));
        assert_eq!(314095480, floor_sum(31415, 92653, 58979, 32384));
        assert_eq!(
            499999999500000000,
            floor_sum(1000000000, 1000000000, 999999999, 999999999)
        );
    }
}
