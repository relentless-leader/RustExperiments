#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
}

fn insert_left(current: &mut Node, val: i32) {
    println!("Hello current: {:p}", &current);
    let left = Node {
        left: None,
        right: None,
        value: val
    };
    current.left = Some(Box::new(left));
}

fn insert_right(current: &mut Node, val: i32) {
    println!("Hello current: {:p}", &current);
    let right = Node {
        left: None,
        right: None,
        value: val
    };
    current.right = Some(Box::new(right));
}

fn main() {
    let mut root = Node {
        left: None,
        right: None,
        value: 10
    };
    println!("Hello root ref: {:p}", &root);
    insert_left(&mut root, 12);
    println!("Hello root: {:?}", root);
    println!("Hello root ref: {:p}", &root);
    println!("Hello root.left: {:?}", root.left);
    println!("Hello root.left ref: {:p}", &root.left);
    println!("Hello root.left ref: {:p}", &*root.left.unwrap());
}
