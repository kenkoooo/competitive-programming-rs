pub mod cost_scaling_push_relabel {
    use std::collections::VecDeque;

    type Flow = i64;
    type Cost = i64;

    const INF_POTENTIAL: f64 = 1e10;
    const SCALING_FACTOR: f64 = 2.0;

    #[derive(Clone)]
    struct Edge {
        to: usize,
        rev: usize,
        flow: Flow,
        capacity: Flow,
        cost: Cost,
        is_rev: bool,
    }

    impl Edge {
        fn residual(&self) -> Flow {
            self.capacity - self.flow
        }
    }

    #[derive(Clone)]
    struct Node {
        excess_flow: Flow,
        potential: f64,
    }

    pub struct Solver {
        nodes: Vec<Node>,
        graph: Vec<Vec<Edge>>,
        active_nodes: VecDeque<usize>,

        cost_scaling_factor: f64,
        eps: f64,
    }

    impl Solver {
        pub fn new(num_nodes: usize) -> Self {
            Self {
                nodes: vec![
                    Node {
                        excess_flow: 0,
                        potential: 0.0
                    };
                    num_nodes
                ],
                graph: vec![vec![]; num_nodes],
                active_nodes: VecDeque::new(),
                eps: 1.0,
                cost_scaling_factor: num_nodes as f64 * 2.0,
            }
        }
        pub fn add_edge(&mut self, from: usize, to: usize, capacity: Flow, cost: Cost) {
            let rev = self.graph[to].len();
            self.graph[from].push(Edge {
                to,
                rev,
                flow: 0,
                capacity,
                cost,
                is_rev: false,
            });

            let rev = self.graph[from].len() - 1;
            self.graph[to].push(Edge {
                to: from,
                rev,
                flow: capacity,
                capacity,
                cost: -cost,
                is_rev: true,
            });

            self.eps = max(self.eps, cost.abs() as f64 * self.cost_scaling_factor);
        }

        pub fn solve(&mut self, source: usize, sink: usize, flow: Flow) -> Flow {
            self.nodes[source].excess_flow = flow;
            self.nodes[sink].excess_flow = -flow;

            while self.eps > 1.0 {
                for node in 0..self.nodes.len() {
                    for edge in 0..self.graph[node].len() {
                        if self.graph[node][edge].is_rev {
                            continue;
                        }

                        let reduced_cost = self.calc_reduced_cost(node, edge);
                        if reduced_cost < 0.0 && self.graph[node][edge].residual() > 0 {
                            let f = self.graph[node][edge].residual();
                            self.push_flow(node, edge, f);
                        }
                        if reduced_cost > 0.0 && self.graph[node][edge].flow > 0 {
                            let f = -self.graph[node][edge].flow;
                            self.push_flow(node, edge, f);
                        }
                    }
                }

                self.get_active_nodes();
                while let Some(node) = self.active_nodes.pop_front() {
                    while self.nodes[node].excess_flow > 0 {
                        if !self.push(node) {
                            self.relabel(node);
                            self.active_nodes.push_back(node);
                            break;
                        }
                    }
                }

                self.eps = max(1.0, self.eps / SCALING_FACTOR);
            }

            let mut total_cost = 0;
            for e in self.graph.iter().flat_map(|g| g.iter()) {
                if e.is_rev {
                    continue;
                }
                total_cost += e.flow * e.cost;
            }
            total_cost
        }

        fn push_flow(&mut self, node: usize, edge: usize, flow: Flow) {
            self.graph[node][edge].flow += flow;

            let to = self.graph[node][edge].to;
            let rev = self.graph[node][edge].rev;
            let from = node;

            self.graph[to][rev].flow -= flow;
            self.nodes[from].excess_flow -= flow;
            self.nodes[to].excess_flow += flow;
        }
        fn calc_reduced_cost(&self, node: usize, edge: usize) -> f64 {
            let cost = self.graph[node][edge].cost;
            let from = node;
            let to = self.graph[node][edge].to;
            cost as f64 * self.cost_scaling_factor - self.nodes[from].potential
                + self.nodes[to].potential
        }

        fn get_active_nodes(&mut self) {
            for u in 0..self.nodes.len() {
                if self.nodes[u].excess_flow > 0 {
                    self.active_nodes.push_back(u);
                }
            }
        }

        fn push(&mut self, from: usize) -> bool {
            if self.nodes[from].excess_flow == 0 {
                return false;
            }
            assert!(self.nodes[from].excess_flow > 0);
            for i in (0..self.graph[from].len()).rev() {
                if self.graph[from][i].residual() == 0 {
                    continue;
                }
                let reduced_cost = self.calc_reduced_cost(from, i);

                if reduced_cost < 0.0 {
                    let flow = min(self.graph[from][i].residual(), self.nodes[from].excess_flow);
                    self.push_flow(from, i, flow);

                    let to = self.graph[from][i].to;
                    if self.nodes[to].excess_flow > 0 && self.nodes[to].excess_flow <= flow {
                        self.active_nodes.push_back(to);
                    }
                    return true;
                }
            }
            false
        }

        fn relabel(&mut self, from: usize) {
            let mut min_potential = INF_POTENTIAL;
            for e in self.graph[from].iter() {
                if e.residual() > 0 {
                    min_potential = min(
                        min_potential,
                        e.cost as f64 * self.cost_scaling_factor
                            + self.nodes[e.to].potential
                            + self.eps,
                    );
                }
            }

            assert!(min_potential < INF_POTENTIAL);
            self.nodes[from].potential = min_potential;
        }
    }

    fn min<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            b
        } else {
            a
        }
    }
    fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a < b {
            b
        } else {
            a
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::cost_scaling_push_relabel::cost_scaling_push_relabel;
    use crate::graph::min_cost_flow::primal_dual;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_grl_6_b() {
        let tester = Tester::new("./assets/GRL_6_B/in/", "./assets/GRL_6_B/out/");
        tester.test_solution(|sc| {
            let v: usize = sc.read();
            let e: usize = sc.read();
            let f: i64 = sc.read();

            let mut solver = cost_scaling_push_relabel::Solver::new(v);
            let mut verify = primal_dual::MinimumCostFlowSolver::new(v);
            for _ in 0..e {
                let u: usize = sc.read();
                let v: usize = sc.read();
                let c: i64 = sc.read();
                let d: i64 = sc.read();
                solver.add_edge(u, v, c, d);
                verify.add_edge(u, v, c, d);
            }

            match verify.solve(0, v - 1, f) {
                Some(ans) => {
                    sc.write(format!("{}\n", ans));
                    assert_eq!(ans, solver.solve(0, v - 1, f));
                }
                _ => {
                    sc.write("-1\n");
                }
            }
        });
    }
}
