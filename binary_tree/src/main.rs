use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    left: Subtree<T>,
    right: Subtree<T>,
    value: T,
}

#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
struct BinaryTree<T: Ord> {
     root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> { 
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
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

    fn insert(&mut self, value: T) {
        if self.0.is_none() {
            self.0 = Some(Box::new(Node {
                left: Subtree::new(),
                right: Subtree::new(),
                value: value,
            }));
        } else {
            let root = self.0.as_mut().unwrap();
            if value < root.value {
                root.left.insert(value)
            } else if value > root.value {
                root.right.insert(value)
            }
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0  {
            None => false,
            Some(root) => match value.cmp(&root.value) {
                Ordering::Less => root.left.has(value),
                Ordering::Greater => root.right.has(value),
                Ordering::Equal => true,
            }, 
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(current) => {
                1 + current.left.len() + current.right.len()
            }
        }
    }
}


fn main() {
    println!("hello");
    let mut tree = BinaryTree::new();
    println!("Hello tree: {:?}", tree);
    println!("Hello tree: {:?}", tree.len());
    tree.insert(1);
    println!("Hello tree: {:?}", tree);
    tree.insert(2);
    println!("Hello tree: {:?}", tree);
    tree.insert(8);
    println!("Hello tree: {:?}", tree);
    println!("Hello tree: {:?}", tree.len());
    tree.insert(4);
    println!("Hello tree: {:?}", tree);
    tree.insert(14);
    println!("Hello tree: {:?}", tree);
    tree.insert(3);
    println!("Hello tree: {:?}", tree);
    tree.insert(7);
    println!("Hello tree: {:?}", tree);
    println!("Hello tree: {:?}", tree.has(&10));
    println!("Hello tree: {:?}", tree.has(&8));
    println!("Hello tree: {:?}", tree.len());
}
