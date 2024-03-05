use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    key: char,
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    pub fn insert(&mut self, key: char, value: T) {
        self.root.insert(key, value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, key: char, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(key, value))),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert(key, value),
                Ordering::Equal => {}
                Ordering::Greater => n.right.insert(key, value),
            },
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => n.right.has(value),
            },
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.left.len() + n.right.len(),
        }
    }
}

impl<T: Ord> Node<T> {
    fn new(key: char, value: T) -> Self {
        Self {
            key,
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}
