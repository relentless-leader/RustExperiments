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
            keys: Vec::with_capacity(2*t-1),
            t,
            children: Vec::with_capacity(2*t),
            is_leaf,
            next: None
        }
    }
    fn find_insert_position(&self, key: i32) -> usize {
        self.keys.iter().position(|&k| key < k).unwrap_or(self.keys.len())
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

    fn split_child(&mut self, parent: &NodeRef, index: usize) {
        let t = selt.t;
        let mut parent_node = parent.borrow_mut();
        let child_node = Rc::clone(&parent_node.children[index]).borrow_mut();

        let mid_key = child_node.keys[t-1];
        let new_child_node = Node::new(child_node.t, child_node.is_leaf);
        new_child_node.keys.extend_from_slice(&child_node.keys[t..]);
        child_node.keys.truncate(t-1);

        if (!child_node.is_leaf) {
            new_child_node.children.extend_from_slice(&child_node.children[t..]);
            child_node.truncate(t-1);
        }

        parent_node.keys.insert(index, mid_key);
        parent_node.children.insert(index+1, new_child_node)
    }

    fn insert_non_full(&mut self, node: &NodeRef, key: i32) {
        println!("{:?}", node);
        let mut n = node.borrow_mut();
        let mut i = n.keys.len();

        println!("{:?}", n.keys);

        if n.is_leaf {
            let pos = n.find_insert_position(key);
            n.keys.insert(pos, key);
        } else {
            let mut i = n.find_insert_position(key);
            if i < n.keys.len() && key == n.keys[i] {
                return;
            }

            if n.children[i].borrow().keys.len() == 2*n.t - 1 {
                self.split_child(node, i);
                if key > n.keys[i] {
                    i = i+1;
                }
            }

            drop(n);
            self.insert_non_full(&node.borrow().children[i], key)
        }
    }

    fn insert(&mut self, key: i32) {
        let root = Rc::clone(&self.root);
        println!("{:?}", root);
        self.insert_non_full(&root, key);
    }
}

fn main() {
    let mut btree: BTree = BTree::new(3);
    btree.insert(10);
    println!("Hello, world!");
}
