#[derive(Debug)]
struct Node {
    order: i32,
    values: Vec<String>,
    keys: Option<Vec<Vec<Box<Node>>>>,
    next_key: Option<Box<Node>>,
    parent: Option<Box<Node>>,
    is_leaf: bool
}

fn main() {
    println!("Hello, world!");
}
