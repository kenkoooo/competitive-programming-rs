pub mod persistent_array {
    const N: usize = 20;
    pub struct PersistentArray<T: Clone> {
        nodes: Vec<Node<T>>,
    }
    #[derive(Clone)]
    struct Node<T: Clone> {
        data: Option<T>,
        children: [Option<usize>; N],
    }
    impl<T: Clone> Node<T> {
        fn new(data: Option<T>, children: [Option<usize>; N]) -> Self {
            Self { data, children }
        }
    }

    impl<T: Clone> PersistentArray<T> {
        pub fn new() -> Self {
            Self {
                nodes: vec![Node::new(None, [None; N])],
            }
        }
        pub fn set(&mut self, index: usize, data: T, node_id: usize) -> usize {
            if index == 0 {
                let new_node_id = self.nodes.len();
                self.nodes
                    .push(Node::new(Some(data), self.nodes[node_id].children.clone()));
                new_node_id
            } else {
                let a = match self.nodes[node_id].children[index % N] {
                    Some(next_node_id) => self.set(index / N, data, next_node_id),
                    None => {
                        let intermediate_node_id = self.nodes.len();
                        self.nodes.push(Node::new(None, [None; N]));
                        self.nodes[node_id].children[index % N] = Some(intermediate_node_id);
                        self.set(index / N, data, intermediate_node_id)
                    }
                };
                let new_node_id = self.nodes.len();
                self.nodes.push(self.nodes[node_id].clone());
                self.nodes[new_node_id].children[index % N] = Some(a);
                new_node_id
            }
        }

        pub fn get(&self, index: usize, node_id: usize) -> Option<&T> {
            if index == 0 {
                self.nodes[node_id].data.as_ref()
            } else {
                match self.nodes[node_id].children[index % N] {
                    Some(next_node_id) => self.get(index / N, next_node_id),
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::persistent_array::persistent_array::PersistentArray;
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_persistent_array() {
        let mut rng = thread_rng();
        let mut array: PersistentArray<i64> = PersistentArray::new();
        let n: usize = 20;
        let mut vs = vec![vec![None; n]];
        let mut node_ids = vec![0];

        for _ in 0..10000 {
            let pick = rng.sample(Uniform::from(0..vs.len()));
            let mut new_vec = vs[pick].clone();
            let node_id = node_ids[pick];
            let pos = rng.sample(Uniform::from(0..n));
            let value: i64 = rng.gen();
            new_vec[pos] = Some(value);
            let new_node_id = array.set(pos, value, node_id);

            for i in 0..n {
                let expected = new_vec[i];
                let actual = array.get(i, new_node_id);
                assert_eq!(expected, actual.cloned());
            }
            vs.push(new_vec.clone());
            node_ids.push(new_node_id);

            let value: i64 = rng.gen();
            new_vec[pos] = Some(value);
            let new_node_id = array.set(pos, value, new_node_id);
            for i in 0..n {
                let expected = new_vec[i];
                let actual = array.get(i, new_node_id);
                assert_eq!(expected, actual.cloned());
            }
            vs.push(new_vec.clone());
            node_ids.push(new_node_id);
        }
    }
}
