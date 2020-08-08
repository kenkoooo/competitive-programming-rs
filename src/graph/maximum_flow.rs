pub mod dinitz {

    struct Edge {
        pub to: usize,
        pub rev: usize,
        pub cap: i64,
    }

    pub struct Dinitz {
        g: Vec<Vec<Edge>>,
    }

    impl Dinitz {
        pub fn new(v: usize) -> Dinitz {
            let mut g: Vec<Vec<Edge>> = Vec::new();
            for _ in 0..v {
                g.push(Vec::new());
            }
            Dinitz { g }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            let to_len = self.g[to].len();
            let from_len = self.g[from].len();
            self.g[from].push(Edge {
                to,
                rev: to_len,
                cap,
            });
            self.g[to].push(Edge {
                to: from,
                rev: from_len,
                cap: 0,
            });
        }

        fn dfs(
            &mut self,
            v: usize,
            sink: usize,
            flow: i64,
            level: &[i32],
            iter: &mut [usize],
        ) -> i64 {
            if v == sink {
                return flow;
            }
            while iter[v] < self.g[v].len() {
                let flow = std::cmp::min(flow, self.g[v][iter[v]].cap);
                let to = self.g[v][iter[v]].to;
                if flow > 0 && level[v] < level[to] {
                    let flowed = self.dfs(to, sink, flow, level, iter);
                    if flowed > 0 {
                        let rev = self.g[v][iter[v]].rev;
                        self.g[v][iter[v]].cap -= flowed;
                        self.g[to][rev].cap += flowed;
                        return flowed;
                    }
                }
                iter[v] += 1;
            }
            0
        }

        fn bfs(&self, s: usize) -> Vec<i32> {
            let v = self.g.len();
            let mut level = vec![-1; v];
            level[s] = 0;
            let mut deque = std::collections::VecDeque::new();
            deque.push_back(s);
            while let Some(v) = deque.pop_front() {
                for e in self.g[v].iter() {
                    if e.cap > 0 && level[e.to] < 0 {
                        level[e.to] = level[v] + 1;
                        deque.push_back(e.to);
                    }
                }
            }
            level
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let v = self.g.len();
            let mut flow: i64 = 0;
            loop {
                let level = self.bfs(s);
                if level[t] < 0 {
                    return flow;
                }
                let mut iter = vec![0; v];
                loop {
                    let f = self.dfs(s, t, std::i64::MAX, &level, &mut iter);
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
