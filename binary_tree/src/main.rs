struct Node {
    left: Box<Node>,
    right: Box<Node>,
    value: i32,
}

fn main() {
    let root = Node {
        left = None,
        right = None,
        value = 10
    }
    println!("Hello, world!");
}
