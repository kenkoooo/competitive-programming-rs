pub mod strongly_connected_components {
    use std::collections::VecDeque;

    pub fn decompose(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut vs = Vec::new();
        let num_v = graph.len();
        let mut cmp = vec![0; num_v];

        let mut reverse_graph = vec![vec![]; num_v];
        for i in 0..num_v {
            for v in &graph[i] {
                reverse_graph[*v].push(i);
            }
        }
        let mut used = vec![false; num_v];

        let mut stack = VecDeque::new();
        let mut added = vec![false; num_v];
        for i in 0..num_v {
            if used[i] { continue; }
            stack.push_front(i);
            while !stack.is_empty() {
                let v = stack.pop_front().unwrap();
                stack.push_front(v);
                used[v] = true;
                let mut pushed = false;
                for j in (0..graph[v].len()).rev() {
                    let u = graph[v][j];
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
        for i in &vs {
            let i = *i;
            if used[i] { continue; }
            stack.push_front(i);
            used[i] = true;
            cmp[i] = k;
            while !stack.is_empty() {
                let v = stack.pop_front().unwrap();
                stack.push_front(v);
                let mut pushed = false;
                for u in &reverse_graph[v] {
                    let u = *u;
                    if used[u] { continue; }
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

        return cmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_helper::load_test_cases;

    #[test]
    fn solve_grl_3_c() {
        let mut input_files = fs::read_dir("./assets/GRL_3_C/in/").unwrap().map(|result| { result.unwrap().path().display().to_string() }).collect::<Vec<_>>();
        let mut output_files = fs::read_dir("./assets/GRL_3_C/out/").unwrap().map(|result| { result.unwrap().path().display().to_string() }).collect::<Vec<_>>();

        input_files.sort();
        output_files.sort();

        for i in 0..input_files.len() {
            let mut input = load_test_cases::<usize>(&input_files[i]);
            let mut output = load_test_cases::<usize>(&output_files[i]);

            let v: usize = input.pop_front().unwrap();
            let e: usize = input.pop_front().unwrap();
            let mut graph = vec![vec![]; v];
            for _ in 0..e {
                let s: usize = input.pop_front().unwrap();
                let t: usize = input.pop_front().unwrap();
                graph[s].push(t);
            }

            let cmp = strongly_connected_components::decompose(&graph);
            let q: usize = input.pop_front().unwrap();
            for _ in 0..q {
                let u: usize = input.pop_front().unwrap();
                let v: usize = input.pop_front().unwrap();

                let expected = output.pop_front().unwrap();

                if cmp[u] == cmp[v] {
                    assert_eq!(1, expected);
                } else {
                    assert_eq!(0, expected);
                }
            }
        }
    }
}