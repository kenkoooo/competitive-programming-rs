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
    use test_helper::load_test_cases;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    #[test]
    fn solve_dsl_1_a() {
        for i in 1..33 {
            let input_file: String = format!("./assets/DSL_1_A/DSL_1_A_{}.in", i);
            let output_file: String = format!("./assets/DSL_1_A/DSL_1_A_{}.out", i);
            let mut input = load_test_cases::<usize>(&input_file);
            let mut output = load_test_cases::<usize>(&output_file);

            let n = input.pop_front().unwrap();
            let q = input.pop_front().unwrap();
            let mut uf = UnionFind::new(n);
            for _ in 0..q {
                let com = input.pop_front().unwrap();
                let x = input.pop_front().unwrap();
                let y = input.pop_front().unwrap();
                if com == 0 {
                    uf.unite(x, y);
                } else {
                    let ans = if uf.find(x) == uf.find(y) {
                        1
                    } else {
                        0
                    };
                    assert_eq!(ans, output.pop_front().unwrap());
                }
            }
        }
    }
}