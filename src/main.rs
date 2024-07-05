
struct Node {
    value: i32,
    left: SubTree,
    right: SubTree,
}

type SubTree = Option<Box<Node>>;

struct Dict {
    root: SubTree,
}

impl Dict {
    fn new() -> Self {
        Dict { root: None }
    }
}

fn main() {
    println!("Hello, world!");
}
