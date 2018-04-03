use std::fs;
use std::collections::VecDeque;

const MAX_PARENT: usize = 1 << 50;

struct LowestCommonAncestor {
    graph: Vec<Vec<usize>>,
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
    root: usize,
    log_v: usize,
}

impl LowestCommonAncestor {
    fn new(graph: &Vec<Vec<usize>>) -> LowestCommonAncestor {
        let num_v = graph.len();
        let root = 0;
        let graph = graph.clone();
        let mut depth = vec![0; num_v];

        let mut log_v = 1;
        let mut i = 1;
        while i <= num_v {
            i *= 2;
            log_v += 1;
        }
        let mut parent: Vec<Vec<usize>> = vec![vec![0; num_v]; log_v];

        let mut depth_vis = vec![false; num_v];
        let mut stack = VecDeque::new();
        stack.push_front(root);
        parent[0][root] = MAX_PARENT;
        depth[root] = 0;
        depth_vis[root] = true;
        while !stack.is_empty() {
            let v = stack.pop_front().unwrap();
            stack.push_front(v);
            for u in &graph[v] {
                let u = *u;
                if depth_vis[u] { continue; }
                parent[0][u] = v;
                depth[u] = depth[v] + 1;
                depth_vis[u] = true;
                stack.push_front(u);
            }

            let head = stack.pop_front().unwrap();
            if head != v { stack.push_front(head); }
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

        LowestCommonAncestor { graph: graph, parent: parent, depth: depth, root: root, log_v: log_v }
    }

    fn get_lca(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] <= self.depth[v] { (u, v) } else { (v, u) };
        for k in 0..self.log_v {
            if ((self.depth[v] - self.depth[u]) & (1 << k)) != 0 {
                v = self.parent[k][v];
            }
        }
        if u == v { return u; }

        for k in (0..self.log_v).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u];
                v = self.parent[k][v];
            }
        }
        return self.parent[0][u];
    }

    fn get_dist(&self, u: usize, v: usize) -> usize {
        let lca = self.get_lca(u, v);
        self.depth[u] + self.depth[v] - self.depth[lca] * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helper::TestCaseProducer;

    #[test]
    fn solve_grl_5_c() {
        let mut input = TestCaseProducer::new_from_directory("./assets/GRL_5_C/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/GRL_5_C/out/");

        while !input.is_empty() {
            let n: usize = input.next();
            let mut graph = (0..n).map(|_| Vec::new()).collect::<Vec<_>>();
            for i in 0..n {
                let k = input.next();
                for _ in 0..k {
                    let c: usize = input.next();
                    graph[i].push(c);
                    graph[c].push(i);
                }
            }

            let lca = LowestCommonAncestor::new(&graph);

            let q: usize = input.next();
            for _ in 0..q {
                let u: usize = input.next();
                let v: usize = input.next();
                let ans = lca.get_lca(u, v);

                let expected: usize = output.next();
                assert_eq!(ans, expected);
            }
        }
    }
}
