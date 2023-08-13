pub mod persistent_array {
    use std::{convert::TryInto, rc::Rc};

    #[derive(Clone)]
    pub struct Node<T: Clone, const N: usize> {
        data: Option<T>,
        children: [Option<Rc<Node<T, N>>>; N],
    }

    impl<T: Clone, const N: usize> Node<T, N> {
        pub fn new(data: Option<T>, children: [Option<Rc<Node<T, N>>>; N]) -> Self {
            Self { data, children }
        }
    }

    impl<T: Clone, const N: usize> Default for Node<T, N> {
        fn default() -> Self {
            Self {
                data: None,
                children: default_array(),
            }
        }
    }

    pub fn set<T: Clone, const N: usize>(
        index: usize,
        data: T,
        node: Option<Rc<Node<T, N>>>,
    ) -> Rc<Node<T, N>> {
        if index == 0 {
            match node {
                Some(node) => {
                    let new_node = Node::new(Some(data), node.children.clone());
                    Rc::new(new_node)
                }
                None => Rc::new(Node::new(Some(data), default_array())),
            }
        } else {
            let child = match node
                .as_ref()
                .and_then::<&Rc<Node<T, N>>, _>(|node| node.children[index % N].as_ref())
            {
                Some(next_node) => set(index / N, data, Some(next_node.clone())),
                None => set(index / N, data, None),
            };
            let mut new_node = match node {
                Some(node) => node.as_ref().clone(),
                None => Node::default(),
            };
            new_node.children[index % N] = Some(child);
            Rc::new(new_node)
        }
    }

    pub fn get<T: Clone, const N: usize>(index: usize, node: &Rc<Node<T, N>>) -> Option<T> {
        if index == 0 {
            node.data.clone()
        } else {
            match node.children[index % N].as_ref() {
                Some(next_node) => get(index / N, next_node),
                None => None,
            }
        }
    }

    fn default_array<T: Clone, const N: usize>() -> [Option<Rc<Node<T, N>>>; N] {
        let mut children = Vec::with_capacity(N);
        for _ in 0..N {
            children.push(None);
        }
        children.try_into().unwrap_or_else(|_| panic!())
    }
}

#[cfg(test)]
mod tests {
    use super::persistent_array::{get, set, Node};
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};
    use std::rc::Rc;

    #[test]
    fn test_persistent_array() {
        let mut rng = thread_rng();
        let n: usize = 20;
        let mut vs = vec![vec![None; n]];
        let mut nodes = vec![Rc::new(Node::<_, 20>::default())];

        for _ in 0..10000 {
            let pick = rng.sample(Uniform::from(0..vs.len()));
            let mut new_vec = vs[pick].clone();
            let node = nodes[pick].clone();
            let pos = rng.sample(Uniform::from(0..n));
            let value: i64 = rng.gen();
            new_vec[pos] = Some(value);
            let new_node = set(pos, value, Some(node));

            for i in 0..n {
                let expected = new_vec[i];
                let actual = get(i, &new_node);
                assert_eq!(expected, actual);
            }
            vs.push(new_vec.clone());
            nodes.push(new_node.clone());

            let value: i64 = rng.gen();
            new_vec[pos] = Some(value);
            let new_node = set(pos, value, Some(new_node));
            for i in 0..n {
                let expected = new_vec[i];
                let actual = get(i, &new_node);
                assert_eq!(expected, actual);
            }
            vs.push(new_vec.clone());
            nodes.push(new_node);
        }
    }
}
