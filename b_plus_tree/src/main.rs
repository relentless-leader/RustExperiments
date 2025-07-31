#[derive(Debug)]
struct Node {
    // Array of keys
    keys: Vec<i32>,
    // min degree ( defines thge range for the number of keys)
    t: i32 ,
    // Array to child pointers
    children: Vec<Box<Node>>,
    is_leaf: bool,
    // Pointer to next leaf Node
    next: Option<Box<Node>>,
}

impl Node {
    fn new(t: i32, is_leaf: bool) -> Self {
        Self  {
            keys: Vec::new(),
            t,
            children: Vec:: new(),
            is_leaf,
            next: None
        }
    }
}

#[derive(Debug)]
struct BTree {
    // Pointer to root Node
    root: Option<Box<Node>>,
    // Min degree
    t: i32,
}

impl BTree {
    fn new(t: i32) -> Self {
        Self {
            root: Some(Box::new(Node::new(t, true))),
            t,
        }
    }

    fn insert(&mut self, key: i32) {
        println!("{:?}", self.root);
    }
}

fn main() {
    let mut btree: BTree = BTree::new(3);
    btree.insert(10);
    println!("Hello, world!");
}
