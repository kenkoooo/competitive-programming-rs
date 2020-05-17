pub mod max_rectangle {
    use std::cmp;
    use std::collections::VecDeque;

    fn calc(hist: &[usize]) -> usize {
        let n = hist.len();
        let mut ans = 0;
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        for (i, &hist) in hist.iter().enumerate() {
            let mut reachable_min = i;
            while let Some((pos, height)) = q.pop_front() {
                if height <= hist {
                    q.push_front((pos, height));
                    break;
                }
                reachable_min = pos;
                ans = cmp::max(ans, (i - reachable_min) * height);
            }

            if q.is_empty() || q.iter().next().unwrap().1 < hist {
                q.push_front((reachable_min, hist));
            }
        }
        while let Some((pos, height)) = q.pop_front() {
            ans = cmp::max(ans, (n - pos) * height);
        }
        ans
    }

    pub fn maximize(map: &[Vec<bool>]) -> usize {
        let h = map.len();
        let w = map[0].len();
        let mut hist = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                if !map[i][j] {
                    continue;
                }
                if i == 0 {
                    hist[i][j] = 1;
                } else {
                    hist[i][j] = hist[i - 1][j] + 1;
                }
            }
        }

        let mut ans = 0;
        for hist in hist.iter() {
            ans = cmp::max(ans, calc(hist));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_dpl_3_b() {
        let tester = Tester::new("./assets/DPL_3_B/in/", "./assets/DPL_3_B/out/");

        tester.test_solution(|sc| {
            let h: usize = sc.read();
            let w: usize = sc.read();

            let mut c = vec![vec![false; w]; h];
            for i in 0..h {
                for j in 0..w {
                    c[i][j] = sc.read::<usize>() == 0;
                }
            }

            let ans = max_rectangle::maximize(&c);
            sc.write(format!("{}\n", ans));
        });
    }
}
