use std::mem;
use std::cmp;
use std::collections::VecDeq;

enum Tree {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    right: Tree,
    left: Tree
}

impl Tree {

    pub fn new() -> Tree { Tree::Empty }

    pub fn put(&mut self, elem:i32) -> &mut Self {
        match self {
            Tree::Empty => {
                let n = Node { elem, right: Tree::Empty, left: Tree::Empty };
                let _ = mem::replace( self, Tree::More(Box::new(n)) );
            },
            Tree::More(n) => {

                if elem > n.elem {
                    n.right.put(elem);
                }
                else if elem < n.elem {
                    n.left.put(elem);
                }

            }
        }
        self
    }

    pub fn print(&self) {
        match self {
            Tree::Empty => {
                //do nothing
            },
            Tree::More(n) => {
                println!("{: >width$}", n.elem, width=self.height()*2 );
                n.left.print();
                n.right.print();
            }
        }
    }

    fn height(&self) -> usize {
        match self {
            Tree::More(n) => {
                return 1 + cmp::max( n.left.height(), n.right.height() );
            },
            Tree::Empty => 0,
        }
    }

    fn breadth(&self) {
       let mut Q : VecDeq<Self> = VecDeq::new();

       if let Tree::More(_) = self {
           Q.push_back(self);
       }

       while !Q.is_empty() {
            
       }
    }

}


fn main() {
    let mut tree = Tree::new();
    tree.put(34).put(12).put(40);
    tree.print();
}
