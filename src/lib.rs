use std::cmp;
use std::mem;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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

#[derive(Debug)]
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

    pub fn bf(sub: &SubTree) -> Ballance {

        match sub {
            None => Ballance::Ballanced,
            Some(boxed) => {
                let b = Self::h(&boxed.left) as i32 - Self::h(&boxed.right) as i32;

                match b {
                    _ if b > 0 => Ballance::LeftHeavy,
                    _ if b < 0 => Ballance::RightHeavy,
                    _ => Ballance::Ballanced,
                }
            }
        }

    }

    fn rotate_right(A:&mut SubTree) {

        let mut B = mem::replace( &mut A.as_mut().unwrap().left, None);
        mem::swap( &mut B.as_mut().unwrap().right, &mut A.as_mut().unwrap().left );
        mem::swap( &mut B, A );
        mem::swap( &mut A.as_mut().unwrap().right, &mut B);
    }

    fn rotate_left(A:&mut SubTree) {

        let mut B = mem::replace( &mut A.as_mut().unwrap().right, None);
        mem::swap( &mut B.as_mut().unwrap().left, &mut A.as_mut().unwrap().right );
        mem::swap( &mut B, A );
        mem::swap( &mut A.as_mut().unwrap().left, &mut B);
    }


    pub fn rotate(&mut self) {

        let ab: Ballance;
        let bb: Ballance;
        
        let A = &mut self.root;

        {
            ab = Self::bf(A);

            match ab {
                Ballance::LeftHeavy => {
                    bb = Self::bf( &A.as_ref().unwrap().left );
                },
                Ballance::RightHeavy => {
                    bb = Self::bf( &A.as_ref().unwrap().right );
                },
                Ballance::Ballanced =>  { return; }

            }
        }

        match ab {
            Ballance::LeftHeavy => {
                match bb {
                    Ballance::LeftHeavy => {

                        Self::rotate_right( A );

                    },
                    Ballance::RightHeavy => {
                        Self::rotate_left( &mut A.as_mut().unwrap().left );
                        Self::rotate_right( A );


                    },
                    Ballance::Ballanced => {},
                }
            },
            Ballance::RightHeavy => {
                match bb {
                    Ballance::LeftHeavy => {
                        Self::rotate_right(&mut A.as_mut().unwrap().right );
                        Self::rotate_left( A );
                        dbg!(A);
                    },
                    Ballance::RightHeavy => {
                        Self::rotate_left( A );
                    },
                    Ballance::Ballanced => {},
                }
            },
            Ballance::Ballanced => {}

        }

    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {

        let mut tree = Tree::new();
        tree.put(11);
        tree.put(15);
        tree.put(13);
        tree.rotate();
        assert!( true );

    }

}
