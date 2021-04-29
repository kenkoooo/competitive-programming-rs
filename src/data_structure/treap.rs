pub mod treap {
    type BNode<T> = Box<Node<T>>;
    pub struct Treap<T> {
        rng: XorShift,
        root: Option<BNode<T>>,
    }
    impl<T> Treap<T> {
        pub fn new(seed: u32) -> Self {
            Self {
                rng: XorShift { state: seed },
                root: None,
            }
        }
        pub fn len(&self) -> usize {
            size(&self.root)
        }
        pub fn nth(&self, k: usize) -> &T {
            let root = self
                .root
                .as_ref()
                .expect("Cannot fetch the k-th element of an empty set.");
            root.nth(k)
        }
    }

    impl<T: PartialOrd> Treap<T> {
        pub fn insert(&mut self, value: T) -> bool {
            let priority = self.rng.next();
            if let Some(root) = self.root.take() {
                let (contains, k) = root.find(&value);
                if !contains {
                    self.root = Some(insert(Some(root), k, value, priority));
                    true
                } else {
                    self.root = Some(root);
                    false
                }
            } else {
                self.root = Some(Node::new(value, priority));
                true
            }
        }
        pub fn contains(&self, value: &T) -> bool {
            if let Some(root) = self.root.as_ref() {
                root.find(value).0
            } else {
                false
            }
        }

        pub fn erase(&mut self, value: &T) -> Option<T> {
            if let Some(root) = self.root.take() {
                let (contains, k) = root.find(&value);
                if !contains {
                    self.root = Some(root);
                    None
                } else {
                    let (root, removed) = erase(Some(root), k);
                    self.root = root;
                    removed.map(|b| b.key)
                }
            } else {
                None
            }
        }
        pub fn binary_search(&self, value: &T) -> Result<usize, usize> {
            match self.root.as_ref() {
                Some(root) => {
                    let (contains, k) = root.find(value);
                    if contains {
                        Ok(k)
                    } else {
                        Err(k)
                    }
                }
                None => Err(0),
            }
        }
    }

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
            self.count = size(&self.left) + size(&self.right) + 1;
        }
        fn nth(&self, k: usize) -> &T {
            let left_size = size(&self.left);
            if left_size > k {
                let left = self.left.as_ref().expect("");
                left.nth(k)
            } else if left_size == k {
                &self.key
            } else {
                let right = self.right.as_ref().expect("");
                right.nth(k - left_size - 1)
            }
        }
    }

    impl<T: PartialOrd> Node<T> {
        fn find(&self, value: &T) -> (bool, usize) {
            let left_size = size(&self.left);
            if &self.key == value {
                (true, left_size)
            } else if &self.key > value {
                if let Some(left) = self.left.as_ref() {
                    left.find(value)
                } else {
                    (false, 0)
                }
            } else {
                if let Some(right) = self.right.as_ref() {
                    let (contained, size) = right.find(value);
                    (contained, size + left_size + 1)
                } else {
                    (false, left_size + 1)
                }
            }
        }
    }

    fn insert<T>(t: Option<BNode<T>>, k: usize, value: T, priority: u32) -> BNode<T> {
        let (first, second) = split(t, k);
        let node = merge(first, Some(Node::new(value, priority)));
        let mut node =
            merge(node, second).expect("It shouldn't be a none, since one node is added at least.");
        node.update_count();
        node
    }
    fn erase<T>(node: Option<BNode<T>>, k: usize) -> (Option<BNode<T>>, Option<BNode<T>>) {
        let (first, second) = split(node, k + 1);
        let (first, removed) = split(first, k);
        match merge(first, second) {
            Some(mut node) => {
                node.update_count();
                (Some(node), removed)
            }
            None => (None, removed),
        }
    }

    fn merge<T>(s: Option<BNode<T>>, t: Option<BNode<T>>) -> Option<BNode<T>> {
        match (s, t) {
            (Some(mut s), Some(mut t)) => {
                if s.priority > t.priority {
                    s.right = merge(s.right, Some(t));
                    s.update_count();
                    Some(s)
                } else {
                    t.left = merge(Some(s), t.left);
                    t.update_count();
                    Some(t)
                }
            }
            (Some(s), None) => Some(s),
            (None, Some(t)) => Some(t),
            (None, None) => None,
        }
    }

    fn split<T>(node: Option<BNode<T>>, k: usize) -> (Option<BNode<T>>, Option<BNode<T>>) {
        if let Some(mut node) = node {
            let left_size = size(&node.left);
            if k <= left_size {
                let left = node.left.take();
                let (first, second) = split(left, k);
                node.left = second;
                node.update_count();
                (first, Some(node))
            } else {
                let right = node.right.take();
                let (first, second) = split(right, k - left_size - 1);
                node.right = first;
                node.update_count();
                (Some(node), second)
            }
        } else {
            (None, None)
        }
    }

    fn size<T>(node: &Option<BNode<T>>) -> usize {
        node.as_ref().map(|node| node.count).unwrap_or(0)
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
    use rand::prelude::*;
    use rand::{thread_rng, Rng};
    use std::collections::BTreeSet;

    #[test]
    fn test_treap_insert_erase() {
        let mut treap = Treap::new(71);
        let mut rng = StdRng::seed_from_u64(141);
        let max = 1000000;

        let mut v = (0..max).collect::<Vec<_>>();
        v.shuffle(&mut rng);
        for &i in v.iter() {
            assert!(!treap.contains(&i));
            assert!(treap.insert(i));
            assert!(!treap.insert(i));
            assert!(treap.contains(&i));
        }

        v.shuffle(&mut rng);
        for &i in v.iter() {
            assert!(treap.contains(&i));
            assert_eq!(treap.erase(&i), Some(i));
            assert_eq!(treap.erase(&i), None);
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
            assert_eq!(treap.nth(i), &(i * 2));
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
                    assert_eq!(treap.erase(&x), Some(x));
                    assert_eq!(treap.erase(&x), None);
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
                assert_eq!(treap.nth(i), x);
                assert_eq!(treap.binary_search(x), Ok(i));
            }
        }
    }
}
