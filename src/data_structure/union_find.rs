struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| { i }).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        return true;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use test_helper::TestCaseProducer;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    #[test]
    fn solve_dsl_1_a() {
        let mut input = TestCaseProducer::new_from_directory("./assets/DSL_1_A/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/DSL_1_A/out/");

        while !input.is_empty() {
            let n = input.next();
            let q = input.next();
            let mut uf = UnionFind::new(n);
            for _ in 0..q {
                let com: usize = input.next();
                let x = input.next();
                let y = input.next();
                if com == 0 {
                    uf.unite(x, y);
                } else {
                    let ans = if uf.find(x) == uf.find(y) {
                        1
                    } else {
                        0
                    };
                    assert_eq!(ans, output.next());
                }
            }
        }
    }
}