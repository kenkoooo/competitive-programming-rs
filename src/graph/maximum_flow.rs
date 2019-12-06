pub mod dinitz {
    use std::cmp;
    use std::collections::VecDeque;
    use std::i64;
    use std::usize;

    struct Edge {
        pub to: usize,
        pub rev: usize,
        pub cap: i64,
    }

    pub struct Dinitz {
        g: Vec<Vec<Edge>>,
        level: Vec<i32>,
        iter: Vec<usize>,
    }

    impl Dinitz {
        pub fn new(v: usize) -> Dinitz {
            let mut g: Vec<Vec<Edge>> = Vec::new();
            for _ in 0..v {
                g.push(Vec::new());
            }
            Dinitz {
                g: g,
                level: vec![0; v],
                iter: vec![0; v],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            let to_len = self.g[to].len();
            let from_len = self.g[from].len();
            self.g[from].push(Edge {
                to: to,
                rev: to_len,
                cap: cap,
            });
            self.g[to].push(Edge {
                to: from,
                rev: from_len,
                cap: 0,
            });
        }

        fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
            if v == t {
                return f;
            }
            while self.iter[v] < self.g[v].len() {
                let (e_cap, e_to, e_rev);
                {
                    let ref e = self.g[v][self.iter[v]];
                    e_cap = e.cap;
                    e_to = e.to;
                    e_rev = e.rev;
                }
                if e_cap > 0 && self.level[v] < self.level[e_to] {
                    let d = self.dfs(e_to, t, cmp::min(f, e_cap));
                    if d > 0 {
                        {
                            let ref mut e = self.g[v][self.iter[v]];
                            e.cap -= d;
                        }
                        {
                            let ref mut rev_edge = self.g[e_to][e_rev];
                            rev_edge.cap += d;
                        }
                        return d;
                    }
                }
                self.iter[v] += 1;
            }

            return 0;
        }

        fn bfs(&mut self, s: usize) {
            let v = self.level.len();
            self.level = vec![-1; v];
            self.level[s] = 0;
            let mut deque = VecDeque::new();
            deque.push_back(s);
            while !deque.is_empty() {
                let v = deque.pop_front().unwrap();
                for e in &self.g[v] {
                    if e.cap > 0 && self.level[e.to] < 0 {
                        self.level[e.to] = self.level[v] + 1;
                        deque.push_back(e.to);
                    }
                }
            }
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let v = self.level.len();
            let mut flow: i64 = 0;
            loop {
                self.bfs(s);
                if self.level[t] < 0 {
                    return flow;
                }
                self.iter = vec![0; v];
                loop {
                    let f = self.dfs(s, t, i64::MAX);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::Tester;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
    #[test]
    fn solve_grl_6_a() {
        let tester = Tester::new("./assets/GRL_6_A/in/", "./assets/GRL_6_A/out/");
        tester.test_solution(|sc| {
            let v = sc.read();
            let e = sc.read();

            let mut dinitz = dinitz::Dinitz::new(v);
            for _ in 0..e {
                let from = sc.read();
                let to = sc.read();
                let c = sc.read();
                dinitz.add_edge(from, to, c);
            }
            let flow = dinitz.max_flow(0, v - 1);
            sc.write(format!("{}\n", flow));
        });
    }
}
