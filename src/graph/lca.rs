pub mod lca {
    const MAX_PARENT: usize = 1 << 50;
    pub struct LowestCommonAncestor {
        parent: Vec<Vec<usize>>,
        depth: Vec<usize>,
        log_v: usize,
    }

    impl LowestCommonAncestor {
        pub fn new(graph: &[Vec<usize>]) -> Self {
            let num_v = graph.len();
            let root = 0;
            let mut depth = vec![0; num_v];

            let mut log_v = 1;
            let mut i = 1;
            while i <= num_v {
                i *= 2;
                log_v += 1;
            }
            let mut parent: Vec<Vec<usize>> = vec![vec![0; num_v]; log_v];

            let mut depth_vis = vec![false; num_v];
            let mut stack = ::std::collections::VecDeque::new();
            stack.push_front(root);
            parent[0][root] = MAX_PARENT;
            depth[root] = 0;
            depth_vis[root] = true;
            while !stack.is_empty() {
                let v = stack.pop_front().unwrap();
                stack.push_front(v);
                for &u in &graph[v] {
                    if depth_vis[u] {
                        continue;
                    }
                    parent[0][u] = v;
                    depth[u] = depth[v] + 1;
                    depth_vis[u] = true;
                    stack.push_front(u);
                }

                let head = stack.pop_front().unwrap();
                if head != v {
                    stack.push_front(head);
                }
            }

            for k in 0..(log_v - 1) {
                for u in 0..num_v {
                    parent[k + 1][u] = if parent[k][u] == MAX_PARENT {
                        MAX_PARENT
                    } else {
                        parent[k][parent[k][u]]
                    };
                }
            }

            LowestCommonAncestor {
                parent,
                depth,
                log_v,
            }
        }

        pub fn get_lca(&self, u: usize, v: usize) -> usize {
            let (mut u, mut v) = if self.depth[u] <= self.depth[v] {
                (u, v)
            } else {
                (v, u)
            };
            for k in 0..self.log_v {
                if ((self.depth[v] - self.depth[u]) & (1 << k)) != 0 {
                    v = self.parent[k][v];
                }
            }
            if u == v {
                return u;
            }

            for k in (0..self.log_v).rev() {
                if self.parent[k][u] != self.parent[k][v] {
                    u = self.parent[k][u];
                    v = self.parent[k][v];
                }
            }
            self.parent[0][u]
        }

        pub fn get_dist(&self, u: usize, v: usize) -> usize {
            let lca = self.get_lca(u, v);
            self.depth[u] + self.depth[v] - self.depth[lca] * 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lca::*;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_grl_5_c() {
        let tester = Tester::new("./assets/GRL_5_C/in/", "./assets/GRL_5_C/out/");
        tester.test_solution(|sc| {
            let n: usize = sc.read();
            let mut graph = (0..n).map(|_| Vec::new()).collect::<Vec<_>>();
            for i in 0..n {
                let k = sc.read();
                for _ in 0..k {
                    let c: usize = sc.read();
                    graph[i].push(c);
                    graph[c].push(i);
                }
            }

            let lca = LowestCommonAncestor::new(&graph);

            let q: usize = sc.read();
            for _ in 0..q {
                let u: usize = sc.read();
                let v: usize = sc.read();
                let ans = lca.get_lca(u, v);
                sc.write(format!("{}\n", ans));
            }
        });
    }
}
