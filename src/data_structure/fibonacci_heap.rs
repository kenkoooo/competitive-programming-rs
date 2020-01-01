pub mod fibonacci_heap {

    fn store_to_map<T, F>(
        map: &mut std::collections::HashMap<usize, Node<T>>,
        mut node: Node<T>,
        ordering: F,
    ) where
        T: Copy + PartialEq,
        F: Fn(T, T) -> T,
    {
        let d = node.children.len();
        match map.remove(&d) {
            Some(mut other) => {
                let node = if (ordering)(node.key, other.key) == node.key {
                    node.children.push(other);
                    node
                } else {
                    other.children.push(node);
                    other
                };
                store_to_map(map, node, ordering);
            }
            None => {
                map.insert(d, node);
            }
        }
    }

    pub struct FibonacciHeap<T, F> {
        pub nodes: Vec<Node<T>>,
        min_index: usize,
        ordering: F,
    }

    impl<T, F> FibonacciHeap<T, F>
    where
        T: Copy + PartialEq,
        F: Fn(T, T) -> T + Copy,
    {
        pub fn new(ordering: F) -> Self {
            Self {
                nodes: Vec::new(),
                min_index: 0,
                ordering: ordering,
            }
        }
        pub fn push(&mut self, x: T) {
            let node = Node::new(x);
            self.nodes.push(node);
            let cur_min = self.nodes[self.min_index].key;
            if (self.ordering)(cur_min, x) == x {
                self.min_index = self.nodes.len() - 1;
            }
        }
        pub fn pop(&mut self) -> Option<T> {
            let mut map = std::collections::HashMap::<usize, Node<T>>::new();
            let mut popped = None;

            let mut nodes = Vec::new();
            std::mem::swap(&mut self.nodes, &mut nodes);
            for (i, node) in nodes.into_iter().enumerate() {
                if i == self.min_index {
                    popped = Some(node.key);
                    for node in node.children.into_iter() {
                        store_to_map(&mut map, node, self.ordering);
                    }
                } else {
                    store_to_map(&mut map, node, self.ordering);
                }
            }

            self.nodes = map.into_iter().map(|(_, node)| node).collect();
            if !self.nodes.is_empty() {
                let mut min = self.nodes[0].key;
                for i in 0..self.nodes.len() {
                    if (self.ordering)(min, self.nodes[i].key) == self.nodes[i].key {
                        min = self.nodes[i].key;
                    }
                }

                self.min_index = self
                    .nodes
                    .iter()
                    .enumerate()
                    .find(|(_, node)| node.key == min)
                    .unwrap()
                    .0;
            } else {
                self.min_index = 0;
            }
            popped
        }
    }

    #[derive(Debug)]
    pub struct Node<T> {
        pub key: T,
        pub children: Vec<Node<T>>,
    }

    impl<T> Node<T> {
        fn new(x: T) -> Self {
            Self {
                key: x,
                children: Vec::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fibonacci_heap::*;
    use rand::Rng;
    use std::cmp;
    use std::collections::BinaryHeap;

    #[test]
    fn test_fibonacci_heap() {
        let mut min_heap = FibonacciHeap::new(cmp::min);
        min_heap.push(1);
        assert_eq!(min_heap.pop(), Some(1));

        min_heap.push(1);
        min_heap.push(2);
        min_heap.push(3);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(2));
        assert_eq!(min_heap.pop(), Some(3));

        min_heap.push(3);
        min_heap.push(2);
        min_heap.push(1);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(2));
        assert_eq!(min_heap.pop(), Some(3));
    }

    #[test]
    fn compare_to_binary_heap() {
        let mut rng = rand::thread_rng();
        let mut max_heap = FibonacciHeap::new(cmp::max);
        let mut binary_heap = BinaryHeap::new();

        for _ in 0..2000 {
            let x = rng.gen::<usize>() % 10;

            if x == 0 {
                assert_eq!(max_heap.pop(), binary_heap.pop());
            } else {
                let v = rng.gen::<u8>();
                max_heap.push(v);
                binary_heap.push(v);
            }
        }
    }
}
