pub struct ReRooting<T, Identity, Merge, AddRoot> {
    dp: Vec<Vec<T>>,
    ans: Vec<T>,
    graph: Vec<Vec<usize>>,
    identity: Identity,
    merge: Merge,
    add_root: AddRoot,
}

impl<T, Identity, Merge, AddRoot> ReRooting<T, Identity, Merge, AddRoot>
where
    T: Clone,
    Identity: Fn() -> T,
    Merge: Fn(T, T) -> T,
    AddRoot: Fn(T) -> T,
{
    pub fn new(n: usize, identity: Identity, merge: Merge, add_root: AddRoot) -> Self {
        Self {
            dp: vec![vec![]; n],
            ans: vec![identity(); n],
            graph: vec![vec![]; n],
            identity,
            merge,
            add_root,
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.graph[a].push(b);
    }
    pub fn build(&mut self) {
        self.dfs(0, 0);
        self.dfs2(0, 0, (self.identity)());
    }

    fn dfs(&mut self, v: usize, p: usize) -> T {
        let mut sum = (self.identity)();
        let deg = self.graph[v].len();
        self.dp[v] = vec![(self.identity)(); deg];
        let next = self.graph[v].clone();
        for (i, next) in next.into_iter().enumerate() {
            if next == p {
                continue;
            }
            let t = self.dfs(next, v);
            self.dp[v][i] = t.clone();
            sum = (self.merge)(sum, t);
        }
        (self.add_root)(sum)
    }
    fn dfs2(&mut self, v: usize, p: usize, dp_p: T) {
        for (i, &next) in self.graph[v].iter().enumerate() {
            if next == p {
                self.dp[v][i] = dp_p.clone();
            }
        }

        let deg = self.graph[v].len();
        let mut dp_l = vec![(self.identity)(); deg + 1];
        let mut dp_r = vec![(self.identity)(); deg + 1];
        for i in 0..deg {
            dp_l[i + 1] = (self.merge)(dp_l[i].clone(), self.dp[v][i].clone());
        }
        for i in (0..deg).rev() {
            dp_r[i] = (self.merge)(dp_r[i + 1].clone(), self.dp[v][i].clone());
        }

        self.ans[v] = (self.add_root)(dp_l[deg].clone());

        let next = self.graph[v].clone();
        for (i, next) in next.into_iter().enumerate() {
            if next == p {
                continue;
            }
            self.dfs2(
                next,
                v,
                (self.add_root)((self.merge)(dp_l[i].clone(), dp_r[i + 1].clone())),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::re_rooting::ReRooting;

    #[test]
    fn test_re_rooting() {
        fn comb(n: usize, k: usize) -> usize {
            let mut ans = 1;
            for i in 0..k {
                ans *= n - i;
                ans /= i + 1;
            }
            ans
        }
        let merge = |e1: Option<(i64, usize)>, e2: Option<(i64, usize)>| {
            if let (Some((ans1, size1)), Some((ans2, size2))) = (e1, e2) {
                let c = comb(size1 + size2, size1);
                let ans = ans1 * ans2 * (c as i64);
                Some((ans, size1 + size2))
            } else {
                e1.or(e2)
            }
        };
        let add_root =
            |e: Option<(i64, usize)>| e.map(|(ans, size)| (ans, size + 1)).or(Some((1, 1)));

        let n = 8;
        let mut graph = ReRooting::new(n, || None, merge, add_root);
        let edges = vec![(1, 2), (2, 3), (3, 4), (3, 5), (3, 6), (6, 7), (6, 8)];
        for (u, v) in edges {
            let u = u - 1;
            let v = v - 1;
            graph.add_edge(u, v);
            graph.add_edge(v, u);
        }

        graph.build();
        assert_eq!(graph.ans[0].unwrap().0, 40);
        assert_eq!(graph.ans[1].unwrap().0, 280);
        assert_eq!(graph.ans[2].unwrap().0, 840);
        assert_eq!(graph.ans[3].unwrap().0, 120);
        assert_eq!(graph.ans[4].unwrap().0, 120);
        assert_eq!(graph.ans[5].unwrap().0, 504);
        assert_eq!(graph.ans[6].unwrap().0, 72);
        assert_eq!(graph.ans[7].unwrap().0, 72);
    }
}
