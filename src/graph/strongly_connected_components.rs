pub mod strongly_connected_components {
    use std::collections::VecDeque;

    pub fn decompose(graph: &[Vec<usize>]) -> Vec<usize> {
        let mut vs = Vec::new();
        let num_v = graph.len();
        let mut cmp = vec![0; num_v];

        let mut reverse_graph = vec![vec![]; num_v];

        for (i, edges) in graph.iter().enumerate() {
            for &v in edges.iter() {
                reverse_graph[v].push(i);
            }
        }
        let mut used = vec![false; num_v];

        let mut stack = VecDeque::new();
        let mut added = vec![false; num_v];
        for i in 0..num_v {
            if used[i] {
                continue;
            }
            stack.push_front(i);
            while let Some(v) = stack.pop_front() {
                stack.push_front(v);
                used[v] = true;
                let mut pushed = false;
                for &u in graph[v].iter().rev() {
                    if !used[u] {
                        stack.push_front(u);
                        pushed = true;
                    }
                }
                if !pushed {
                    stack.pop_front();
                    if !added[v] {
                        vs.push(v);
                        added[v] = true;
                    }
                }
            }
        }

        used = vec![false; num_v];
        let mut k = 0;
        vs.reverse();
        for &i in vs.iter() {
            if used[i] {
                continue;
            }
            stack.push_front(i);
            used[i] = true;
            cmp[i] = k;
            while let Some(v) = stack.pop_front() {
                stack.push_front(v);
                let mut pushed = false;
                for &u in reverse_graph[v].iter() {
                    if used[u] {
                        continue;
                    }
                    used[u] = true;
                    cmp[u] = k;
                    stack.push_front(u);
                    pushed = true;
                }
                if !pushed {
                    stack.pop_front();
                }
            }
            k += 1;
        }

        cmp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::Tester;

    #[test]
    fn solve_grl_3_c() {
        let tester = Tester::new("./assets/GRL_3_C/in/", "./assets/GRL_3_C/out/");
        tester.test_solution(|sc| {
            let v: usize = sc.read();
            let e: usize = sc.read();
            let mut graph = vec![vec![]; v];
            for _ in 0..e {
                let s: usize = sc.read();
                let t: usize = sc.read();
                graph[s].push(t);
            }

            let cmp = strongly_connected_components::decompose(&graph);
            let q: usize = sc.read();
            for _ in 0..q {
                let u: usize = sc.read();
                let v: usize = sc.read();

                if cmp[u] == cmp[v] {
                    sc.write("1\n");
                } else {
                    sc.write("0\n");
                }
            }
        });
    }
}
