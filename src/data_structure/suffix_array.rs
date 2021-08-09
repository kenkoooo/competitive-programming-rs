pub mod suffix_array {
    use std::cmp::Ordering;

    pub struct SuffixArray {
        pub n: usize,
        pub s: Vec<u8>,
        pub array: Vec<usize>,
    }

    fn compare_node(i: usize, j: usize, k: usize, rank: &[i32]) -> Ordering {
        if rank[i] != rank[j] {
            rank[i].cmp(&rank[j])
        } else {
            let ri = if i + k < rank.len() { rank[i + k] } else { -1 };
            let rj = if j + k < rank.len() { rank[j + k] } else { -1 };
            ri.cmp(&rj)
        }
    }

    impl SuffixArray {
        pub fn new(s: &[u8]) -> SuffixArray {
            let n = s.len();
            let mut rank = vec![0; n + 1];
            let mut array = vec![0; n + 1];

            for i in 0..=n {
                array[i] = i;
                rank[i] = if i < n { s[i] as i32 } else { -1 };
            }

            let mut tmp = vec![0; n + 1];
            let mut k = 1;
            while k <= n {
                array.sort_by(|a, b| compare_node(*a, *b, k, &rank));

                tmp[array[0]] = 0;
                for i in 1..=n {
                    let d = if compare_node(array[i - 1], array[i], k, &rank) == Ordering::Less {
                        1
                    } else {
                        0
                    };
                    tmp[array[i]] = tmp[array[i - 1]] + d;
                }
                std::mem::swap(&mut rank, &mut tmp);
                k *= 2;
            }

            SuffixArray {
                n,
                array,
                s: Vec::from(s),
            }
        }

        pub fn contains(&self, t: &[u8]) -> bool {
            let b = self.lower_bound(t);
            if b >= self.array.len() {
                false
            } else {
                let start = self.array[b];
                let end = (t.len() + start).min(self.s.len());
                let sub = &self.s[start..end];
                sub == t
            }
        }

        fn binary_search<F>(&self, string: &[u8], f: F) -> usize
        where
            F: Fn(&[u8], &[u8]) -> bool,
        {
            let (mut ng, mut ok) = (-1, self.n as i32 + 1);
            while ok - ng > 1 {
                let pos = (ng + ok) / 2;
                let start = self.array[pos as usize];
                let end = (start + string.len()).min(self.s.len());
                let substring = &self.s[start..end];
                if f(substring, string) {
                    ng = pos;
                } else {
                    ok = pos;
                }
            }
            ok as usize
        }

        pub fn lower_bound(&self, t: &[u8]) -> usize {
            let check_function = |sub: &[u8], s: &[u8]| sub.cmp(s) == Ordering::Less;
            self.binary_search(t, check_function)
        }

        pub fn upper_bound(&self, t: &[u8]) -> usize {
            let check_function = |sub: &[u8], s: &[u8]| sub.cmp(s) != Ordering::Greater;
            self.binary_search(t, check_function)
        }
    }

    pub fn construct_lcp<T: Ord>(string: &[T], suffix_array: &[usize]) -> Vec<usize> {
        assert_eq!(string.len() + 1, suffix_array.len());
        let n = string.len();
        let mut lcp = vec![0; n];
        let mut rank = vec![0; n + 1];
        for i in 0..=n {
            rank[suffix_array[i]] = i;
        }

        let mut height = 0;
        lcp[0] = 0;
        for i in 0..n {
            let j = suffix_array[rank[i] - 1];

            if height > 0 {
                height -= 1;
            }
            while j + height < n && i + height < n {
                if string[j + height] != string[i + height] {
                    break;
                }
                height += 1;
            }

            lcp[rank[i] - 1] = height;
        }

        lcp
    }
}
#[cfg(test)]
mod test {
    use super::suffix_array::*;
    use crate::data_structure::segment_tree::SegmentTree;
    use crate::utils::test_helper::Tester;
    use rand::{thread_rng, Rng};
    use std;
    use std::cmp;

    const INF: i64 = 1 << 60;

    #[test]
    fn small_test() {
        let string = "abcdeabcde".to_owned().bytes().collect::<Vec<_>>();
        let sa = SuffixArray::new(&string);
        assert_eq!(
            sa.lower_bound(&"a".to_owned().bytes().collect::<Vec<_>>()),
            1
        );
        assert_eq!(
            sa.upper_bound(&"a".to_owned().bytes().collect::<Vec<_>>()),
            3
        );

        assert!(sa.contains(&"abcde".to_owned().bytes().collect::<Vec<_>>()));
        assert!(!sa.contains(&"abce".to_owned().bytes().collect::<Vec<_>>()));
    }

    #[test]
    fn corner_case() {
        let string = "cba".to_owned().bytes().collect::<Vec<_>>();
        let sa = SuffixArray::new(&string);
        assert_eq!(
            sa.lower_bound(&"c".to_owned().bytes().collect::<Vec<_>>()),
            3
        );
        assert_eq!(
            sa.upper_bound(&"c".to_owned().bytes().collect::<Vec<_>>()),
            4
        );
    }

    #[test]
    fn test_suffix_array() {
        let mut rng = thread_rng();
        let n = 100;
        for _ in 0..100 {
            let string = (0..n).map(|_| rng.gen_range(0, 30)).collect::<Vec<_>>();
            let sa = SuffixArray::new(&string);

            let mut naive = vec![];
            for i in 0..=n {
                let substring = string[i..].to_vec();
                naive.push((substring, i));
            }
            naive.sort();

            for i in 0..=n {
                assert_eq!(sa.array[i], naive[i].1);
            }

            let lcp_array = construct_lcp(&string, &sa.array);
            for i in 0..n {
                let lcp = lcp_array[i];

                let prev = sa.array[i];
                let next = sa.array[i + 1];

                let prev_substring = &string[prev..(prev + lcp)];
                let next_substring = &string[next..(next + lcp)];
                assert_eq!(prev_substring, next_substring);
                assert_ne!(string.get(prev + lcp), string.get(next + lcp));
            }
        }
    }

    #[test]
    fn jag2014summer_day4_f() {
        let tester = Tester::new(
            "./assets/jag2014summer-day4/F/in/",
            "./assets/jag2014summer-day4/F/out/",
        );
        tester.test_solution(|sc| {
            let s: Vec<u8> = sc.read::<String>().bytes().collect();
            let n = s.len();
            let reverse_s = {
                let mut r = s.clone();
                r.reverse();
                r
            };
            let sa = SuffixArray::new(&s);
            let reverse_sa = SuffixArray::new(&reverse_s);

            let mut rmq = SegmentTree::new(n + 1, |a, b| cmp::min(a, b), || INF);
            let mut reverse_rmq = SegmentTree::new(n + 1, |a, b| cmp::min(a, b), || INF);
            for i in 0..=n {
                rmq.update(i, sa.array[i] as i64);
                reverse_rmq.update(i, reverse_sa.array[i] as i64);
            }

            let m: usize = sc.read();
            for _ in 0..m {
                let x = sc.read::<String>().bytes().collect::<Vec<_>>();
                let y = {
                    let mut y: Vec<u8> = sc.read::<String>().bytes().collect::<Vec<_>>();
                    y.reverse();
                    y
                };

                if !sa.contains(&x) {
                    sc.write("0\n");
                    continue;
                }
                let low = sa.lower_bound(&x);
                let up = sa.upper_bound(&x);

                if !reverse_sa.contains(&y) {
                    sc.write("0\n");
                    continue;
                }
                let reverse_low = reverse_sa.lower_bound(&y);
                let reverse_up = reverse_sa.upper_bound(&y);

                if low >= up || reverse_low >= reverse_up {
                    sc.write("0\n");
                }

                let s = rmq.query(low..up) as usize;
                let t = n - reverse_rmq.query(reverse_low..reverse_up) as usize;
                if s + x.len() <= t && s <= t - y.len() {
                    sc.write(format!("{}\n", t - s));
                } else {
                    sc.write("0\n");
                }
            }
        });
    }
}
