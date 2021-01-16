pub mod treap {
    use std::cmp::Ordering::*;

    type BNode<T> = Box<Node<T>>;

    #[derive(Debug)]
    struct Node<T> {
        left: Option<BNode<T>>,
        right: Option<BNode<T>>,
        key: T,
        priority: u32,
        count: usize,
    }

    impl<T> Node<T> {
        fn new(key: T, priority: u32) -> BNode<T> {
            Box::new(Node {
                left: None,
                right: None,
                key,
                priority,
                count: 1,
            })
        }

        fn update_count(&mut self) {
            self.count = 1 + count(&self.left) + count(&self.right);
        }
    }

    /// `Treap` is a binary search tree data structure that maintain a dynamic set
    /// of ordered keys and allow binary searches among the keys,
    /// [https://en.wikipedia.org/wiki/Treap](https://en.wikipedia.org/wiki/Treap)
    #[derive(Debug)]
    pub struct Treap<T> {
        random_state: XorShift,
        root: Option<BNode<T>>,
    }

    impl<T> Treap<T> {
        pub fn new(seed: u32) -> Self {
            Treap {
                random_state: XorShift { state: seed },
                root: None,
            }
        }
        pub fn clear(&mut self) {
            self.root = None
        }

        pub fn is_empty(&self) -> bool {
            count(&self.root) == 0
        }
        pub fn nth(&self, n: usize) -> Option<&T> {
            rank(&self.root, n).as_ref().map(|r| &r.key)
        }
    }

    impl<T: PartialOrd> Treap<T> {
        pub fn insert(&mut self, key: T) {
            if !self.contains(&key) {
                self.root = Some(insert(self.root.take(), key, &mut self.random_state));
            }
        }

        pub fn contains(&self, key: &T) -> bool {
            find(&self.root, key).is_some()
        }
    }

    impl<T: PartialOrd + Clone> Treap<T> {
        pub fn erase(&mut self, key: &T) {
            self.root = erase(self.root.take().unwrap(), key);
        }
    }

    fn find<T: PartialOrd>(node: &Option<BNode<T>>, key: &T) -> Option<usize> {
        let node = node.as_ref()?;
        match node.key.partial_cmp(key).unwrap() {
            Equal => Some(count(&node.left)),
            Greater => find(&node.left, key),
            Less => match find(&node.right, key) {
                None => None,
                Some(pos) => Some(count(&node.left) + 1 + pos),
            },
        }
    }

    fn count<T>(node: &Option<BNode<T>>) -> usize {
        match node {
            None => 0,
            Some(node) => node.count,
        }
    }

    fn rotate_left<T>(mut node: BNode<T>, mut right_child: BNode<T>) -> BNode<T> {
        node.right = right_child.left.take();
        node.update_count();
        right_child.left = Some(node);
        right_child
    }

    fn rotate_right<T>(mut node: BNode<T>, mut left_child: BNode<T>) -> BNode<T> {
        node.left = left_child.right.take();
        node.update_count();
        left_child.right = Some(node);
        left_child
    }

    fn insert<T: PartialOrd>(node: Option<BNode<T>>, key: T, rand: &mut XorShift) -> BNode<T> {
        match node {
            None => Node::new(key, rand.next()),
            Some(mut node) => {
                match node.key.partial_cmp(&key).unwrap() {
                    Less => {
                        let next_right = insert(node.right.take(), key, rand);
                        if next_right.priority < node.priority {
                            node = rotate_left(node, next_right);
                        } else {
                            node.right = Some(next_right);
                        }
                    }
                    _ => {
                        let next_left = insert(node.left.take(), key, rand);
                        if next_left.priority < node.priority {
                            node = rotate_right(node, next_left);
                        } else {
                            node.left = Some(next_left);
                        }
                    }
                }
                node.update_count();
                node
            }
        }
    }

    fn min<T>(node: &BNode<T>) -> &BNode<T> {
        if let Some(left) = node.left.as_ref() {
            min(left)
        } else {
            node
        }
    }

    fn erase<T: PartialOrd + Clone>(mut node: BNode<T>, key: &T) -> Option<BNode<T>> {
        match node.key.partial_cmp(key).unwrap() {
            Less => {
                node.right = erase(node.right.take().unwrap(), key);
                node.update_count();
                Some(node)
            }
            Greater => {
                node.left = erase(node.left.take().unwrap(), key);
                node.update_count();
                Some(node)
            }
            Equal => match (node.left.take(), node.right.take()) {
                (Some(left), Some(right)) => {
                    node.left = Some(left);
                    node.key = min(&right).key.clone();
                    node.right = erase(right, &node.key);
                    node.update_count();
                    Some(node)
                }
                (None, Some(right)) => Some(right),
                (Some(left), None) => Some(left),
                (None, None) => None,
            },
        }
    }

    fn rank<T>(node: &Option<BNode<T>>, r: usize) -> &Option<BNode<T>> {
        let c = node.as_ref().unwrap();
        let left = count(&c.left);
        match left.cmp(&r) {
            Equal => node,
            Less => rank(&c.right, r - left - 1),
            Greater => rank(&c.left, r),
        }
    }

    #[derive(Debug)]
    struct XorShift {
        state: u32,
    }

    impl XorShift {
        fn next(&mut self) -> u32 {
            self.state = xor_shift(self.state);
            self.state
        }
    }

    fn xor_shift(state: u32) -> u32 {
        let mut x = state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_treap_insert_erase() {
        let mut treap = treap::Treap::new(71);
        let max = 10_000_000;

        for i in 0..max {
            assert!(!treap.contains(&i));
            treap.insert(i);
            assert!(treap.contains(&i));
        }

        for i in 0..max {
            assert!(treap.contains(&i));
            treap.erase(&i);
            assert!(!treap.contains(&i));
        }
    }

    #[test]
    fn test_treap_nth() {
        let mut treap = treap::Treap::new(71);

        let max = 10_000_000;
        for i in 0..max {
            treap.insert(i * 2);
        }

        for i in 0..max {
            assert_eq!(treap.nth(i).unwrap(), &(i * 2));
        }
    }
}
