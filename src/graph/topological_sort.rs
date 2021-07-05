pub fn topological_sort(graph: &[Vec<usize>]) -> Vec<usize> {
    let n = graph.len();
    let mut in_deg = vec![0; n];
    for v in 0..n {
        for &to in graph[v].iter() {
            in_deg[to] += 1;
        }
    }

    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if in_deg[i] == 0 {
            q.push_back(i);
        }
    }

    let mut result = vec![];
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            in_deg[next] -= 1;
            if in_deg[next] == 0 {
                q.push_back(next);
            }
        }

        result.push(v);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let graph = vec![vec![1], vec![2], vec![], vec![1, 4], vec![5], vec![2]];
        let result = topological_sort(&graph);
        assert_eq!(vec![0, 3, 1, 4, 5, 2], result);
    }
}
