pub mod fibonacci_heap {
    use std::collections::{HashMap, LinkedList};

    #[derive(Debug)]
    struct Node<T> {
        value: T,
        children: Vec<Node<T>>,
    }

    #[derive(Debug)]
    pub struct FibonacciHeap<T> {
        nodes: LinkedList<Node<T>>,
        size: usize,
    }

    impl<T> FibonacciHeap<T> {
        pub fn new() -> Self {
            Self {
                nodes: LinkedList::new(),
                size: 0,
            }
        }
    }

    impl<T> FibonacciHeap<T> {
        pub fn len(&self) -> usize {
            self.size
        }
    }

    impl<T: PartialOrd> FibonacciHeap<T> {
        pub fn push(&mut self, value: T) {
            let node = Node {
                value,
                children: vec![],
            };
            self.push_node(node);
            self.size += 1;
        }

        fn push_node(&mut self, node: Node<T>) {
            match self.nodes.iter().next() {
                Some(first) => {
                    if first.value > node.value {
                        self.nodes.push_front(node);
                    } else {
                        self.nodes.push_back(node);
                    }
                }
                None => {
                    self.nodes.push_back(node);
                }
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            let min = match self.nodes.pop_front() {
                Some(min) => min,
                None => {
                    assert_eq!(self.size, 0);
                    return None;
                }
            };

            let mut node_by_deg = HashMap::new();
            let popped = min.value;
            for node in min.children {
                consolidate(node, &mut node_by_deg);
            }

            while let Some(node) = self.nodes.pop_front() {
                consolidate(node, &mut node_by_deg);
            }
            assert!(self.nodes.is_empty());
            for (_, node) in node_by_deg {
                self.push_node(node);
            }
            assert!(self.size > 0);
            self.size -= 1;
            Some(popped)
        }

        pub fn peek(&self) -> Option<&T> {
            self.nodes.iter().next().map(|node| &node.value)
        }

        pub fn merge(self, other: Self) -> Self {
            let FibonacciHeap {
                nodes: mut a_nodes,
                size: a_size,
            } = self;
            let FibonacciHeap {
                nodes: mut b_nodes,
                size: b_size,
            } = other;

            let size = a_size + b_size;

            match (a_nodes.pop_front(), b_nodes.pop_front()) {
                (Some(a), Some(b)) => {
                    let (small, large) = if a.value < b.value { (a, b) } else { (b, a) };
                    a_nodes.append(&mut b_nodes);
                    a_nodes.push_front(large);
                    a_nodes.push_front(small);
                    Self {
                        nodes: a_nodes,
                        size,
                    }
                }
                (Some(a), None) => {
                    assert_eq!(a_size, size);
                    a_nodes.push_front(a);
                    Self {
                        nodes: a_nodes,
                        size,
                    }
                }
                (None, Some(b)) => {
                    assert_eq!(b_size, size);
                    b_nodes.push_front(b);
                    Self {
                        nodes: b_nodes,
                        size,
                    }
                }
                (None, None) => {
                    assert_eq!(size, 0);
                    Self {
                        nodes: LinkedList::new(),
                        size,
                    }
                }
            }
        }
    }

    fn consolidate<T: PartialOrd>(node: Node<T>, node_by_deg: &mut HashMap<usize, Node<T>>) {
        let degree = node.children.len();
        if let Some(same_degree_node) = node_by_deg.remove(&degree) {
            assert_eq!(same_degree_node.children.len(), degree);
            let (mut small, large) = if same_degree_node.value < node.value {
                (same_degree_node, node)
            } else {
                (node, same_degree_node)
            };
            small.children.push(large);
            consolidate(small, node_by_deg);
        } else {
            node_by_deg.insert(degree, node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fibonacci_heap::*;
    use rand::prelude::*;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[test]
    fn test_fibonacci_heap() {
        let mut min_heap = FibonacciHeap::new();

        assert_eq!(min_heap.len(), 0);
        min_heap.push(1);
        assert_eq!(min_heap.len(), 1);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.len(), 0);

        min_heap.push(1);
        assert_eq!(min_heap.len(), 1);
        min_heap.push(2);
        assert_eq!(min_heap.len(), 2);
        min_heap.push(3);
        assert_eq!(min_heap.len(), 3);

        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.len(), 2);
        assert_eq!(min_heap.pop(), Some(2));
        assert_eq!(min_heap.len(), 1);
        assert_eq!(min_heap.pop(), Some(3));
        assert_eq!(min_heap.len(), 0);

        min_heap.push(3);
        min_heap.push(2);
        min_heap.push(1);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(2));
        assert_eq!(min_heap.pop(), Some(3));
    }

    #[test]
    fn compare_to_binary_heap() {
        let mut rng = thread_rng();
        let mut max_heap = FibonacciHeap::new();
        let mut binary_heap = BinaryHeap::new();

        for _ in 0..2000 {
            let x = rng.gen::<usize>() % 10;

            if x == 0 {
                assert_eq!(max_heap.pop().map(|x: Reverse<u8>| x.0), binary_heap.pop());
            } else {
                let v = rng.gen::<u8>();
                max_heap.push(Reverse(v));
                binary_heap.push(v);
            }

            assert_eq!(max_heap.len(), binary_heap.len());
        }
    }

    #[test]
    fn merge_heaps() {
        let mut rng = thread_rng();
        let mut check = vec![];

        let mut a_heap = FibonacciHeap::new();
        for _ in 0..2000 {
            let x = rng.gen_range(0, 100);
            a_heap.push(x);
            check.push(x);
        }
        let mut b_heap = FibonacciHeap::new();
        for _ in 0..2000 {
            let x = rng.gen_range(0, 100);
            b_heap.push(x);
            check.push(x);
        }
        a_heap = a_heap.merge(b_heap);

        assert_eq!(a_heap.len(), check.len());
        check.sort();
        check.reverse();
        while let Some(v) = check.pop() {
            let head = a_heap.pop().unwrap();
            assert_eq!(head, v);
        }
    }
}
