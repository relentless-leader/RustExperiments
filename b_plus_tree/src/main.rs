 use std::cell::RefCell;
 use std::rc::Rc;

 type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    // Array of keys
    keys: Vec<i32>,
    // min degree ( defines thge range for the number of keys)
    t: usize,
    // Array to child pointers
    children: Vec<NodeRef>,
    is_leaf: bool,
    // Pointer to next leaf Node
    next: Option<NodeRef>,
}

impl Node {
    fn new(t: usize, is_leaf: bool) -> Self {
        Self  {
            keys: Vec::new(),
            t,
            children: Vec::new(),
            is_leaf,
            next: None
        }
    }
}

#[derive(Debug)]
struct BTree {
    // Pointer to root Node
    root: NodeRef,
    // Min degree
    t: usize,
}

impl BTree {
    fn new(t: usize) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new(t, true))),
            t,
        }
    }

    fn insertNonFull(&mut self, node: &NodeRef, key: i32) {
        println!("{:?}", node);
    }

    fn insert(&mut self, key: i32) {
        let root = Rc::clone(&self.root);
        println!("{:?}", root);
        self.insertNonFull(&root, key);
    }
}

fn main() {
    let mut btree: BTree = BTree::new(3);
    btree.insert(10);
    println!("Hello, world!");
}
