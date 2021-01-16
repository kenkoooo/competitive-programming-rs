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
        fn new(key: T, priority: u32) -> Option<BNode<T>> {
            Some(Box::new(Node {
                left: None,
                right: None,
                key,
                priority,
                count: 1,
            }))
        }

        fn update(&mut self) {
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
                self.root = insert(self.root.take(), key, &mut self.random_state);
            }
        }

        pub fn contains(&self, key: &T) -> bool {
            find(&self.root, key).is_some()
        }
    }

    impl<T: PartialOrd + Clone> Treap<T> {
        pub fn erase(&mut self, key: &T) {
            self.root = erase(self.root.take(), key);
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

    fn rotate_left<T>(node: BNode<T>) -> BNode<T> {
        let mut node = node;
        let mut r = node.right.take().unwrap();
        node.right = r.left.take();
        node.update();
        r.left = Some(node);
        r
    }

    fn rotate_right<T>(node: BNode<T>) -> BNode<T> {
        let mut node = node;
        let mut l = node.left.take().unwrap();
        node.left = l.right.take();
        node.update();
        l.right = Some(node);
        l
    }

    fn insert<T: PartialOrd>(
        node: Option<BNode<T>>,
        key: T,
        rand: &mut XorShift,
    ) -> Option<BNode<T>> {
        match node {
            None => Node::new(key, rand.next()),
            Some(mut node) => {
                match node.key.partial_cmp(&key).unwrap() {
                    Less => {
                        node.right = insert(node.right.take(), key, rand);
                        if priority(&node.right) < node.priority {
                            node = rotate_left(node);
                        }
                    }
                    _ => {
                        node.left = insert(node.left.take(), key, rand);
                        if priority(&node.left) < node.priority {
                            node = rotate_right(node);
                        }
                    }
                }
                node.update();
                Some(node)
            }
        }
    }

    fn priority<T>(node: &Option<BNode<T>>) -> u32 {
        node.as_ref().unwrap().priority
    }

    fn min<T>(node: &Option<BNode<T>>) -> &Option<BNode<T>> {
        let x = node.as_ref().unwrap();
        match x.left {
            Some(_) => min(&x.left),
            None => node,
        }
    }

    fn erase<T: PartialOrd + Clone>(node: Option<BNode<T>>, key: &T) -> Option<BNode<T>> {
        let mut node = node.unwrap();
        match node.key.partial_cmp(key).unwrap() {
            Equal if node.left.is_none() => node.right,
            Equal if node.right.is_none() => node.left,
            Equal => {
                node.key = match min(&node.right) {
                    Some(m) => m.key.clone(),
                    None => unreachable!(),
                };
                node.right = erase(node.right.take(), &node.key);
                node.update();
                Some(node)
            }
            Less => {
                node.right = erase(node.right.take(), key);
                node.update();
                Some(node)
            }
            Greater => {
                node.left = erase(node.left.take(), key);
                node.update();
                Some(node)
            }
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
        let mut treap = treap::Treap::new(71);

        let max = 100000;
        for i in 0..max {
            treap.insert(i * 2);
        }

        for i in 0..max {
            assert_eq!(treap.nth(i).unwrap(), &(i * 2));
        }
    }
}
