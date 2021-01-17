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
        pub fn len(&self) -> usize {
            count(&self.root)
        }
        pub fn is_empty(&self) -> bool {
            self.len() == 0
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
        pub fn erase(&mut self, key: &T) -> bool {
            if let Some(root) = self.root.take() {
                let (root, removed) = erase(root, key);
                self.root = root;
                removed
            } else {
                false
            }
        }
    }

    fn find<T: PartialOrd>(node: &Option<BNode<T>>, key: &T) -> Option<usize> {
        let node = node.as_ref()?;
        match cmp_key(&node.key, key) {
            Equal => Some(count(&node.left)),
            Greater => find(&node.left, key),
            Less => find(&node.right, key).map(|pos| count(&node.left) + 1 + pos),
        }
    }

    fn count<T>(node: &Option<BNode<T>>) -> usize {
        node.as_ref().map(|node| node.count).unwrap_or(0)
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
        if let Some(mut node) = node {
            match cmp_key(&node.key, &key) {
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
        } else {
            Node::new(key, rand.next())
        }
    }

    fn erase_min<T>(mut node: BNode<T>) -> (Option<BNode<T>>, T) {
        if let Some(left) = node.left.take() {
            let (left, removed_value) = erase_min(left);
            node.left = left;
            node.update_count();
            (Some(node), removed_value)
        } else {
            (None, node.key)
        }
    }

    fn erase<T: PartialOrd>(mut node: BNode<T>, key: &T) -> (Option<BNode<T>>, bool) {
        match cmp_key(&node.key, key) {
            Less => {
                if let Some(right) = node.right.take() {
                    let (right, removed) = erase(right, key);
                    node.right = right;
                    node.update_count();
                    (Some(node), removed)
                } else {
                    (Some(node), false)
                }
            }
            Greater => {
                if let Some(left) = node.left.take() {
                    let (left, removed) = erase(left, key);
                    node.left = left;
                    node.update_count();
                    (Some(node), removed)
                } else {
                    (Some(node), false)
                }
            }
            Equal => match (node.left.take(), node.right.take()) {
                (Some(left), Some(right)) => {
                    node.left = Some(left);
                    let (right, min_key) = erase_min(right);
                    node.key = min_key;
                    node.right = right;
                    node.update_count();
                    (Some(node), true)
                }
                (None, Some(right)) => (Some(right), true),
                (Some(left), None) => (Some(left), true),
                (None, None) => (None, true),
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

    fn cmp_key<T: PartialOrd>(key1: &T, key2: &T) -> std::cmp::Ordering {
        key1.partial_cmp(key2)
            .expect("non-sortable data is not supported.")
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
    use super::treap::*;
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};
    use std::collections::BTreeSet;

    #[test]
    fn test_treap_insert_erase() {
        let mut treap = Treap::new(71);
        let max = 100_000;

        for i in 0..max {
            assert!(!treap.contains(&i));
            assert_eq!(treap.len(), i);
            treap.insert(i);
            assert!(treap.contains(&i));
            assert_eq!(treap.len(), i + 1);
        }

        for i in 0..max {
            assert!(treap.contains(&i));
            assert_eq!(treap.len(), max - i);
            assert!(treap.erase(&i));
            assert!(!treap.erase(&i));
            assert!(!treap.contains(&i));
            assert_eq!(treap.len(), max - i - 1);
        }
    }

    #[test]
    fn test_treap_nth() {
        let mut treap = Treap::new(71);

        let max = 100_000;
        for i in 0..max {
            treap.insert(i * 2);
        }

        for i in 0..max {
            assert_eq!(treap.nth(i).unwrap(), &(i * 2));
        }
    }

    #[test]
    fn test_edge_case() {
        let mut treap = Treap::new(10);
        treap.insert(0);
        assert_eq!(treap.len(), 1);
        treap.insert(0);
        assert_eq!(treap.len(), 1);
    }

    #[test]
    fn test_random_insertion() {
        let mut treap = Treap::new(81);
        let mut rng = thread_rng();
        let mut set = BTreeSet::new();
        for _ in 0..100_000 {
            let x = rng.sample(Uniform::from(0..100000000));

            if rng.sample(Uniform::from(0..10)) == 0 {
                if set.contains(&x) {
                    set.remove(&x);
                    assert!(treap.erase(&x));
                    assert!(!treap.erase(&x));
                } else {
                    assert!(!treap.contains(&x));
                    assert!(!treap.erase(&x));
                }
            } else {
                treap.insert(x);
                set.insert(x);
            }
            assert_eq!(treap.len(), set.len());
        }
    }

    #[test]
    fn test_random_nth() {
        let mut treap = Treap::new(81);
        let mut rng = thread_rng();
        let mut set = BTreeSet::new();
        for _ in 0..1000 {
            let x = rng.sample(Uniform::from(0..100000000));

            if rng.sample(Uniform::from(0..10)) == 0 {
                if set.contains(&x) {
                    set.remove(&x);
                    assert!(treap.erase(&x));
                    assert!(!treap.erase(&x));
                } else {
                    assert!(!treap.contains(&x));
                    assert!(!treap.erase(&x));
                }
            } else {
                treap.insert(x);
                set.insert(x);
            }

            for (i, &x) in set.iter().enumerate() {
                assert_eq!(treap.nth(i), Some(&x));
            }
        }
    }
}
