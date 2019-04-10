use std::cmp;
use std::cmp::Ordering;

pub struct SuffixArray {
    n: usize,
    s: Vec<u8>,
    array: Vec<usize>,
}

fn compare_node(i: usize, j: usize, k: usize, rank: &Vec<i32>) -> Ordering {
    if rank[i] != rank[j] {
        rank[i].cmp(&rank[j])
    } else {
        let ri = if i + k <= rank.len() { rank[i + k] } else { -1 };
        let rj = if j + k <= rank.len() { rank[j + k] } else { -1 };
        ri.cmp(&rj)
    }
}

impl SuffixArray {
    pub fn new(s: &Vec<u8>) -> SuffixArray {
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
                    + if compare_node(array[i - 1], array[i], k, &rank) == Ordering::Less {
                        1
                    } else {
                        0
                    }
            }
            for i in 0..(n + 1) {
                rank[i] = tmp[i];
            }
            k *= 2;
        }

        SuffixArray {
            n: n,
            array: array,
            s: s.clone(),
        }
    }

    pub fn contains(&self, t: &Vec<u8>) -> bool {
        let b = self.lower_bound(t);
        if b >= self.array.len() {
            false
        } else {
            let start = self.array[b];
            let end = cmp::min(t.len() + start, self.s.len());
            let sub = &self.s[start..end];
            sub.cmp(t) == Ordering::Equal
        }
    }

    fn binary_search<F>(&self, t: &Vec<u8>, f: F) -> usize
    where
        F: Fn(&[u8], &Vec<u8>) -> bool,
    {
        let (mut a, mut b) = (-1, self.n as i32 + 1);
        while b - a > 1 {
            let c = (a + b) / 2;
            let start = self.array[c as usize];
            let end = cmp::min(start + t.len(), self.s.len());
            let sub = &self.s[start..end];
            if f(sub, t) {
                a = c;
            } else {
                b = c;
            }
        }
        b as usize
    }

    pub fn lower_bound(&self, t: &Vec<u8>) -> usize {
        let check_function = |sub: &[u8], s: &Vec<u8>| sub.cmp(s) == Ordering::Less;
        self.binary_search(t, check_function)
    }

    pub fn upper_bound(&self, t: &Vec<u8>) -> usize {
        let check_function = |sub: &[u8], s: &Vec<u8>| sub.cmp(s) != Ordering::Greater;
        self.binary_search(t, check_function)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_structure::segment_tree::SegmentTree;
    use crate::utils::test_helper::TestCaseProducer;
    use std;

    #[test]
    fn small_test() {
        let string = "abcdeabcde".to_owned().bytes().collect();
        let sa = SuffixArray::new(&string);
        assert_eq!(sa.lower_bound(&"a".to_owned().bytes().collect()), 1);
        assert_eq!(sa.upper_bound(&"a".to_owned().bytes().collect()), 3);

        assert!(sa.contains(&"abcde".to_owned().bytes().collect()));
        assert!(!sa.contains(&"abce".to_owned().bytes().collect()));
    }

    #[test]
    fn corner_case() {
        let string = "cba".to_owned().bytes().collect();
        let sa = SuffixArray::new(&string);
        assert_eq!(sa.lower_bound(&"c".to_owned().bytes().collect()), 3);
        assert_eq!(sa.upper_bound(&"c".to_owned().bytes().collect()), 4);
    }

    #[test]
    fn jag2014summer_day4_f() {
        let mut input = TestCaseProducer::new_from_directory("./assets/jag2014summer-day4/F/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/jag2014summer-day4/F/out/");

        while !input.is_empty() {
            let s: Vec<u8> = input.next::<String>().bytes().collect();
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

            let m = input.next();
            for _ in 0..m {
                let x = input.next::<String>().bytes().collect();
                let y = {
                    let mut y: Vec<u8> = input.next::<String>().bytes().collect();
                    y.reverse();
                    y
                };

                if !sa.contains(&x) {
                    assert_eq!(output.next::<String>(), "0");
                    continue;
                }
                let low = sa.lower_bound(&x);
                let up = sa.upper_bound(&x);

                if !reverse_sa.contains(&y) {
                    assert_eq!(output.next::<String>(), "0");
                    continue;
                }
                let reverse_low = reverse_sa.lower_bound(&y);
                let reverse_up = reverse_sa.upper_bound(&y);

                if low >= up || reverse_low >= reverse_up {
                    assert_eq!(output.next::<String>(), "0");
                }

                let s = rmq.query(low, up) as usize;
                let t = n - reverse_rmq.query(reverse_low, reverse_up) as usize;
                if s + x.len() <= t && s <= t - y.len() {
                    assert_eq!(output.next::<usize>(), t - s);
                } else {
                    assert_eq!(output.next::<String>(), "0");
                }
            }
        }
    }
}
