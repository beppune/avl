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
    //LeftRightHeavy,
    RightHeavy,
    //RightLeftHeavy,
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

    fn bf(sub:&SubTree) -> Ballance {

        match sub {
            Some(boxed) => {
                let b = Self::h(&boxed.left) as i32 - Self::h(&boxed.right) as i32;
                match b {
                    0 => Ballance::Ballanced,
                    _ if b > 0 => Ballance::LeftHeavy,
                    _ if b < 0 => Ballance::RightHeavy,
                    _ => panic!("ballance value: {}", b),
                }
            }
            None => Ballance::Ballanced,
        }

    }

    fn rotate(sub:&SubTree) {

        let b = Self::bf(sub);
        match b {
            Ballance::LeftHeavy => println!("LeftHeavy"),
            Ballance::RightHeavy => println!("RightHeavy"),
            Ballance::Ballanced => println!("Ballanced"),
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
