use std::cmp;

pub struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    order: Vec<usize>,
    low_link: Vec<usize>,
    k: usize,
}

impl BridgeDetector {
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut d = BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            order: vec![0; n],
            low_link: vec![0; n],
            k: 0,
        };
        d.run(graph);
        d
    }

    fn run(&mut self, graph: &Vec<Vec<usize>>) {
        let n = graph.len();
        for i in 0..n {
            if !self.visit[i] {
                self.dfs(i, 0, graph, i);
            }
        }
    }

    fn dfs(&mut self, v: usize, previous: usize, graph: &Vec<Vec<usize>>, root: usize) {
        self.visit[v] = true;
        self.order[v] = self.k;
        self.k += 1;
        self.low_link[v] = self.order[v];

        let mut is_articulation = false;
        let mut dimension = 0;
        for &next in graph[v].iter() {
            if !self.visit[next] {
                // The edge (v->next) is not a backedge.
                dimension += 1;
                self.dfs(next, v, graph, root);
                self.low_link[v] = cmp::min(self.low_link[v], self.low_link[next]);
                if v != root && self.order[v] <= self.low_link[next] {
                    is_articulation = true;
                }
                if self.order[v] < self.low_link[next] {
                    let min = cmp::min(v, next);
                    let max = cmp::max(v, next);
                    self.bridges.push((min, max));
                }
            } else if v == root || next != previous {
                // The edge (v->next) is a backedge.
                self.low_link[v] = cmp::min(self.low_link[v], self.order[next]);
            }
        }

        if v == root && dimension > 1 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulations.push(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::scanner::IO;
    use crate::utils::test_helper;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_grl_3_a() {
        let tester = Tester::new("./assets/GRL_3_A/in/", "./assets/GRL_3_A/out/");

        tester.test_solution(|sc| {
            let n: usize = sc.read();
            let m: usize = sc.read();

            let mut graph = vec![vec![]; n];
            for _ in 0..m {
                let u: usize = sc.read();
                let v: usize = sc.read();
                graph[u].push(v);
                graph[v].push(u);
            }

            let mut low_link_link = BridgeDetector::new(&graph);
            low_link_link.articulations.sort();

            for v in low_link_link.articulations.into_iter() {
                sc.write(format!("{}\n", v));
            }
        });
    }

    #[test]
    fn solve_grl_3_b() {
        let tester = Tester::new("./assets/GRL_3_B/in/", "./assets/GRL_3_B/out/");
        tester.test_solution(|sc| {
            let n: usize = sc.read();
            let m: usize = sc.read();
            let mut graph = vec![vec![]; n];
            for _ in 0..m {
                let u: usize = sc.read();
                let v: usize = sc.read();
                graph[u].push(v);
                graph[v].push(u);
            }

            let mut low_link_link = BridgeDetector::new(&graph);
            low_link_link.bridges.sort();

            for (a, b) in low_link_link.bridges.into_iter() {
                sc.write(format!("{} {}\n", a, b));
            }
        });
    }
}
