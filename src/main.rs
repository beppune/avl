
use std::cmp;
use std::mem;

struct Node {
    elem: i32,
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
    println!("Height: {}", tree.height() );
    
}
