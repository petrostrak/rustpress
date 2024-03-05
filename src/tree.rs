use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    key: char,
    value: T,
    code: String,
    left: Subtree<T>,
    right: Subtree<T>,
}

#[derive(Debug)]
struct Subtree<T: Ord> {
    node: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
    code: String,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
            code: String::new(),
        }
    }

    pub fn insert(&mut self, key: char, value: T) {
        println!("{} - {}", key, self.code);
        self.root.insert(key, value, &mut self.code);
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
        Self { node: None }
    }

    fn insert(&mut self, key: char, value: T, code: &mut String) {
        match &mut self.node {
            None => self.node = Some(Box::new(Node::new(key, value, code.to_string()))),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => {
                    code.push_str("0");
                    n.code = code.clone();
                    n.left.insert(key, value, &mut n.code);
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    code.push_str("1");
                    n.code = code.clone();
                    n.right.insert(key, value, &mut n.code);
                }
            },
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => n.right.has(value),
            },
        }
    }

    fn len(&self) -> usize {
        match &self.node {
            None => 0,
            Some(n) => 1 + n.left.len() + n.left.len() + n.right.len(),
        }
    }
}

impl<T: Ord> Node<T> {
    fn new(key: char, value: T, code: String) -> Self {
        Self {
            key,
            value,
            code,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}
