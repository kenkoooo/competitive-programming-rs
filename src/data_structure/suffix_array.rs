pub struct SuffixArray {
    n: usize,
    s: Vec<u8>,
    array: Vec<usize>,
}

fn compare_node(i: usize, j: usize, k: usize, rank: &[i32]) -> std::cmp::Ordering {
    if rank[i] != rank[j] {
        rank[i].cmp(&rank[j])
    } else {
        let ri = if i + k <= rank.len() { rank[i + k] } else { -1 };
        let rj = if j + k <= rank.len() { rank[j + k] } else { -1 };
        ri.cmp(&rj)
    }
}

impl SuffixArray {
    pub fn new(s: &[u8]) -> SuffixArray {
        let n = s.len();
        let mut rank = vec![0; n + 1];
        let mut array = vec![0; n + 1];

        for i in 0..(n + 1) {
            array[i] = i;
            rank[i] = if i < n { s[i] as i32 } else { -1 };
        }

        let mut tmp = vec![0; n + 1];
        let mut k = 1;
        while k <= n {
            array.sort_by(|a, b| compare_node(*a, *b, k, &rank));

            tmp[array[0]] = 0;
            for i in 1..(n + 1) {
                tmp[array[i]] = tmp[array[i - 1]]
                    + if compare_node(array[i - 1], array[i], k, &rank) == std::cmp::Ordering::Less
                    {
                        1
                    } else {
                        0
                    }
            }
            rank[..(n + 1)].clone_from_slice(&tmp[..(n + 1)]);
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
            let end = std::cmp::min(t.len() + start, self.s.len());
            let sub = &self.s[start..end];
            sub.cmp(t) == std::cmp::Ordering::Equal
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
            let end = std::cmp::min(start + string.len(), self.s.len());
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
        let check_function = |sub: &[u8], s: &[u8]| sub.cmp(s) == std::cmp::Ordering::Less;
        self.binary_search(t, check_function)
    }

    pub fn upper_bound(&self, t: &[u8]) -> usize {
        let check_function = |sub: &[u8], s: &[u8]| sub.cmp(s) != std::cmp::Ordering::Greater;
        self.binary_search(t, check_function)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_structure::segment_tree::SegmentTree;
    use crate::utils::test_helper::Tester;
    use std;
    use std::cmp;

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

            let mut rmq = SegmentTree::new(n + 1, std::i64::MAX, |a, b| cmp::min(a, b));
            let mut reverse_rmq = SegmentTree::new(n + 1, std::i64::MAX, |a, b| cmp::min(a, b));
            for i in 0..(n + 1) {
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
