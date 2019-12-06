pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
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
    use crate::utils::scanner::IO;
    use crate::utils::test_helper;
    use crate::utils::test_helper::Tester;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    #[test]
    fn solve_dsl_1_a() {
        let tester = Tester::new("./assets/DSL_1_A/in/", "./assets/DSL_1_A/out/");

        tester.test_solution(|sc| {
            let n = sc.read();
            let q = sc.read();
            let mut uf = UnionFind::new(n);
            for _ in 0..q {
                let com: usize = sc.read();
                let x = sc.read();
                let y = sc.read();
                if com == 0 {
                    uf.unite(x, y);
                } else {
                    let ans = if uf.find(x) == uf.find(y) { 1 } else { 0 };
                    sc.write(format!("{}\n", ans));
                }
            }
        });
    }
}
