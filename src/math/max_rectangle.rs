pub mod max_rectangle {
    use std::cmp;
    use std::collections::VecDeque;

    fn calc(hist: &Vec<usize>) -> usize {
        let n = hist.len();
        let mut ans = 0;
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        for i in 0..n {
            let mut reachable_min = i;
            while let Some((pos, height)) = q.pop_front() {
                if height <= hist[i] {
                    q.push_front((pos, height));
                    break;
                }
                reachable_min = pos;
                ans = cmp::max(ans, (i - reachable_min) * height);
            }

            if q.is_empty() || q.iter().next().unwrap().1 < hist[i] {
                q.push_front((reachable_min, hist[i]));
            }
        }
        while let Some((pos, height)) = q.pop_front() {
            ans = cmp::max(ans, (n - pos) * height);
        }
        ans
    }

    pub fn maximize(map: &Vec<Vec<bool>>) -> usize {
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
        for i in 0..h {
            ans = cmp::max(ans, calc(&hist[i]));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_helper::TestCaseProducer;

    #[test]
    fn solve_dpl_3_b() {
        let mut input = TestCaseProducer::new_from_directory("./assets/DPL_3_B/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/DPL_3_B/out/");

        while !input.is_empty() {
            let h: usize = input.next();
            let w: usize = input.next();

            let mut c = vec![vec![false; w]; h];
            for i in 0..h {
                for j in 0..w {
                    c[i][j] = input.next::<usize>() == 0;
                }
            }

            let ans = max_rectangle::maximize(&c);
            let expected: usize = output.next();
            assert_eq!(expected, ans);
        }
    }
}
