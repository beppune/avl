
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

impl Tree {

    fn new() -> Self {
        Tree{ root: None, size: 0 }
    }
    
    fn put(&mut self, value:i32) -> bool {
        let mut current = &mut self.root;
        while let Some(box_node) = current {
            
            if box_node.value > 0 { current = &mut box_node.right; }
            else if box_node.value < 0 { current = &mut box_node.left; }
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
        assert!( tree.size == 3 );
        
    }
    
}
