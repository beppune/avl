
use std::cmp;
use std::mem;

struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

enum Tree {
    Empty,
    More(Box<Node>),
}

impl Tree {

    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn put(&mut self, value: i32) {

        while let Tree::More(node) = self {

            if node.value < value {
                node = node.left;
            } else {
                node = node.right;
            }

        }

        *node = Tree::More( Self::make_node(value) );

    }

    fn make_node(value:i32) -> Box<Node> {
        let more = Box::new(Node {
            value,
            right: Tree::Empty,
            left: Tree::Empty,
        });
        Box::new( *more )
    }

    pub fn height(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::More(n) => 1 + cmp::max( n.right.height(), n.left.height() )
        }
    }

}

fn main() {

    let mut tree = Tree::new();
    tree.put(12);
    tree.put(2);
    tree.put(1);
    println!("Height: {}", tree.height() );
    
}
