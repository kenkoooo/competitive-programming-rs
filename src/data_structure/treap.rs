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
            rank(&self.root, n).as_ref().map(|node| &node.key)
        }
    }

    impl<T: PartialOrd> Treap<T> {
        pub fn insert(&mut self, key: T) {
            if !self.contains(&key) {
                self.root = Some(insert(self.root.take(), key, &mut self.random_state));
            }
        }
        pub fn contains(&self, key: &T) -> bool {
            self.index_of(key).is_some()
        }
        pub fn index_of(&self, key: &T) -> Option<usize> {
            find(&self.root, key)
        }
    }
    impl<T: PartialOrd + Clone> Treap<T> {
        pub fn erase(&mut self, key: &T) {
            self.root = erase(self.root.take().unwrap(), key);
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

    fn insert<T: PartialOrd>(node: Option<BNode<T>>, key: T, rand: &mut XorShift) -> BNode<T> {
        if let Some(mut node) = node {
            match cmp(&node.key, &key) {
                Less => {
                    let right = insert(node.right.take(), key, rand);
                    if right.priority < node.priority {
                        node = rotate_left(node, right);
                    } else {
                        node.right = Some(right);
                    }
                }
                _ => {
                    let left = insert(node.left.take(), key, rand);
                    if left.priority < node.priority {
                        node = rotate_right(node, left);
                    } else {
                        node.left = Some(left);
                    }
                }
            }
            node.update_count();
            node
        } else {
            Node::new(key, rand.next())
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
        match cmp(&node.key, key) {
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
                    let right_min_key = min(&right).key.clone();
                    node.key = right_min_key;
                    node.right = erase(right, &node.key);
                    node.left = Some(left);
                    node.update_count();
                    Some(node)
                }
                (None, Some(right)) => Some(right),
                (Some(left), None) => Some(left),
                _ => None,
            },
        }
    }

    fn rank<T>(node: &Option<BNode<T>>, r: usize) -> &Option<BNode<T>> {
        match node {
            Some(c) => {
                let left = count(&c.left);
                match left.cmp(&r) {
                    Equal => node,
                    Less => rank(&c.right, r - left - 1),
                    Greater => rank(&c.left, r),
                }
            }
            None => panic!(),
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
        let mut treap = Treap::new(71);

        let max = 100000;
        for i in 0..max {
            treap.insert(i * 2);
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
                    treap.erase(&x);
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
                    treap.insert(x);
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
