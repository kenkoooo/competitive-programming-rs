pub mod treap {
    use std::cmp::Ordering;
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
            self.len() == 0
        }
        pub fn len(&self) -> usize {
            count(&self.root)
        }
        pub fn nth(&self, n: usize) -> Option<&T> {
            if let Some(root) = self.root.as_ref() {
                rank(root, n).as_ref().map(|node| &node.key)
            } else {
                None
            }
        }
    }

    impl<T: PartialOrd> Treap<T> {
        pub fn insert(&mut self, key: T) -> bool {
            let (root, inserted) = insert(self.root.take(), key, &mut self.random_state);
            self.root = Some(root);
            inserted
        }
        pub fn contains(&self, key: &T) -> bool {
            self.index_of(key).is_some()
        }
        pub fn index_of(&self, key: &T) -> Option<usize> {
            find(&self.root, key)
        }
    }
    impl<T: PartialOrd + Clone> Treap<T> {
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
        match node {
            None => None,
            Some(node) => match cmp(&node.key, key) {
                Equal => Some(count(&node.left)),
                Greater => find(&node.left, key),
                Less => match find(&node.right, key) {
                    None => None,
                    Some(pos) => Some(count(&node.left) + 1 + pos),
                },
            },
        }
    }

    fn count<T>(node: &Option<BNode<T>>) -> usize {
        match node {
            None => 0,
            Some(node) => node.count,
        }
    }

    fn rotate_left<T>(mut node: BNode<T>, mut right: BNode<T>) -> BNode<T> {
        node.right = right.left.take();
        node.update_count();
        right.left = Some(node);
        right
    }

    fn rotate_right<T>(mut node: BNode<T>, mut left: BNode<T>) -> BNode<T> {
        node.left = left.right.take();
        node.update_count();
        left.right = Some(node);
        left
    }

    fn insert<T: PartialOrd>(
        node: Option<BNode<T>>,
        key: T,
        rand: &mut XorShift,
    ) -> (BNode<T>, bool) {
        if let Some(mut node) = node {
            match cmp(&node.key, &key) {
                Less => {
                    let (right, inserted) = insert(node.right.take(), key, rand);
                    if right.priority < node.priority {
                        node = rotate_left(node, right);
                    } else {
                        node.right = Some(right);
                    }
                    node.update_count();
                    (node, inserted)
                }
                Greater => {
                    let (left, inserted) = insert(node.left.take(), key, rand);
                    if left.priority < node.priority {
                        node = rotate_right(node, left);
                    } else {
                        node.left = Some(left);
                    }
                    node.update_count();
                    (node, inserted)
                }
                Equal => (node, false),
            }
        } else {
            (Node::new(key, rand.next()), true)
        }
    }

    fn min<T>(node: &BNode<T>) -> &BNode<T> {
        if let Some(left) = node.left.as_ref() {
            min(left)
        } else {
            node
        }
    }

    fn erase<T: PartialOrd + Clone>(mut node: BNode<T>, key: &T) -> (Option<BNode<T>>, bool) {
        match cmp(&node.key, key) {
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
                    let right_min_key = min(&right).key.clone();
                    let (right, removed) = erase(right, &right_min_key);
                    assert!(removed);
                    node.key = right_min_key;
                    node.right = right;
                    node.left = Some(left);
                    node.update_count();
                    (Some(node), true)
                }
                (None, Some(right)) => (Some(right), true),
                (Some(left), None) => (Some(left), true),
                _ => (None, true),
            },
        }
    }

    fn rank<T>(node: &BNode<T>, nth: usize) -> Option<&BNode<T>> {
        let left_count = node.left.as_ref().map(|left| left.count).unwrap_or(0);
        match left_count.cmp(&nth) {
            Equal => Some(node),
            Less => {
                if let Some(right) = node.right.as_ref() {
                    rank(right, nth - left_count - 1)
                } else {
                    None
                }
            }
            Greater => {
                if let Some(left) = node.left.as_ref() {
                    rank(left, nth)
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn cmp<T: PartialOrd>(key1: &T, key2: &T) -> Ordering {
        key1.partial_cmp(key2)
            .expect("unsortable data is not supported")
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
        let max = 1000000;

        for i in 0..max {
            assert!(!treap.contains(&i));
            assert!(treap.insert(i));
            assert!(!treap.insert(i));
            assert!(treap.contains(&i));
        }

        for i in 0..max {
            assert!(treap.contains(&i));
            assert!(treap.erase(&i));
            assert!(!treap.erase(&i));
            assert!(!treap.contains(&i));
        }
    }

    #[test]
    fn test_treap_nth() {
        let mut treap = Treap::new(71);

        let max = 100000;
        for i in 0..max {
            assert!(treap.insert(i * 2));
            assert!(!treap.insert(i * 2));
        }

        for i in 0..max {
            assert_eq!(treap.nth(i), Some(&(i * 2)));
        }
    }

    #[test]
    fn test_random_insertion() {
        let mut rng = thread_rng();
        let mut set = BTreeSet::new();
        let mut treap = Treap::new(42);
        for _ in 0..2000 {
            let x = rng.gen::<i64>();

            if rng.sample(Uniform::from(0..10)) == 0 {
                // remove
                if set.contains(&x) {
                    assert!(treap.contains(&x));
                    set.remove(&x);
                    assert!(treap.erase(&x));
                    assert!(!treap.erase(&x));
                    assert!(!treap.contains(&x));
                } else {
                    assert!(!treap.contains(&x));
                }
            } else {
                // insert
                if set.contains(&x) {
                    assert!(treap.contains(&x));
                } else {
                    assert!(!treap.contains(&x));
                    assert!(treap.insert(x));
                    assert!(!treap.insert(x));
                    set.insert(x);
                    assert!(treap.contains(&x));
                }
            }

            assert_eq!(treap.len(), set.len());
            for (i, x) in set.iter().enumerate() {
                assert_eq!(treap.nth(i), Some(x));
                assert_eq!(treap.index_of(x), Some(i));
            }
        }
    }
}
