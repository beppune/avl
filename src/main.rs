
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

}


fn main() {
    let mut tree = Tree::new();


}
