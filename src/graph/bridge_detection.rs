struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    k: usize,
}

impl BridgeDetector {
    fn new(n: usize) -> Self {
        BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            ord: vec![0; n],
            low: vec![0; n],
            k: 0,
        }
    }

    fn run(&mut self, graph: &Vec<Vec<usize>>) {
        let n = graph.len();
        for i in 0..n {
            if !self.visit[i] {
                self.dfs(i, None, graph);
            }
        }
    }
    fn dfs(&mut self, v: usize, p: Option<usize>, graph: &Vec<Vec<usize>>) {
        self.visit[v] = true;
        self.ord[v] = self.k;
        self.k += 1;
        self.low[v] = self.ord[v];

        let mut is_articulation = false;
        let mut count = 0;
        for &next in graph[v].iter() {
            if !self.visit[next] {
                count += 1;
                self.dfs(next, Some(v), graph);
                if self.low[v] > self.low[next] {
                    self.low[v] = self.low[next];
                }
                if p.is_some() && self.ord[v] <= self.low[next] {
                    is_articulation = true;
                }
                if self.ord[v] < self.low[next] {
                    let (v, next) = if v < next { (v, next) } else { (next, v) };
                    self.bridges.push((v, next));
                }
            } else if p.is_none() || next != p.unwrap() && self.low[v] > self.ord[next] {
                self.low[v] = self.ord[next];
            }
        }

        if p.is_none() && count > 1 {
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
    use test_helper::TestCaseProducer;

    #[test]
    fn solve_grl_3_a() {
        let mut input = TestCaseProducer::new_from_directory("./assets/GRL_3_A/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/GRL_3_A/out/");

        while !input.is_empty() {
            let n: usize = input.next();
            let m: usize = input.next();
            println!("{} {}", n, m);
            let mut graph = vec![vec![]; n];
            for _ in 0..m {
                let u: usize = input.next();
                let v: usize = input.next();
                graph[u].push(v);
                graph[v].push(u);
            }

            let mut low_link = BridgeDetector::new(n);
            low_link.run(&graph);
            low_link.articulations.sort();

            if low_link.articulations.is_empty() {
                output.next::<String>();
            }

            for &v in low_link.articulations.iter() {
                let ans: usize = output.next();
                assert_eq!(ans, v);
            }
        }
    }
}