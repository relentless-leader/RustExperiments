#[derive(Debug)]
struct Node<T: Ord> {
    left: Subtree<T>,
    right: Subtree<T>,
    value: T,
}

#[derive(Debug)]
struct Subtree<T>(Option<Box<Node<T>>>);

#[derive(Debug)]
struct BinaryTree<T: Ord> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut Self, value: T) {
        self.root.insert(value);
    }

    fn has(&Self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&Self) -> usize {
        self.root.len()
    }
}

fn main() {
    println!("hello");
}
