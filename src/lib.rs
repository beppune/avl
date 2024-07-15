use std::cmp;

#[derive(Debug)]
struct Tree {
    root: SubTree,
    size: usize,
}

type SubTree = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    value: i32,
    right: SubTree,
    left: SubTree,
}

enum Ballance{
    Ballanced,
    LeftHeavy,
    RightHeavy,
}

impl Tree {

    pub fn new() -> Self {
        Tree{ root: None, size: 0 }
    }

    pub fn put(&mut self, value:i32) -> bool {
        let mut current = &mut self.root;
        while let Some(box_node) = current {

            if box_node.value < value { current = &mut box_node.right; }
            else if box_node.value > value { current = &mut box_node.left; }
            else { return false; }

        }

        *current = Some(Box::new(Node{
            value,
            right:None,
            left:None,
        }));

        self.size += 1;

        true
    }

    pub fn height(&mut self) -> usize {

        Self::h(&self.root)

    }

    fn h(sub:&SubTree) -> usize {
        match sub {
            Some(boxed) => 1 + cmp::max( Self::h(&boxed.left), Self::h(&boxed.right) ),
            None => 0,
        }
    }


}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {

        let mut tree = Tree::new();
        tree.put(12);
        tree.put(15);
        tree.put(9);
        println!("Height: {}", tree.height());
        assert!( tree.height() == 2 );
        dbg!(tree.root);

    }

}
