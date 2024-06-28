

#[derive(Debug)]
pub struct Node {
    val: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {

    pub fn new () -> Self {
        BinaryTree {root: None}
    }

    pub fn add(&mut self, val: u32) {
        let new_node = Box::new(Node {
            val,
            left: None,
            right: None,
        });

        match self.root {
            // insert into tree if root exist
            Some(ref mut node) => {
                BinaryTree::insert(node, new_node);
            }
            // assign new node as root
            None => {
                self.root = Some(new_node);
            }
        }
    }

    pub fn delete(&mut self, val: u32){
        match self.root {
            Some(ref mut node) => {
                BinaryTree::remove(node, val); // Assuming this method removes the node with value `val`
            }
            None => {
               panic!("Tree doesn't have root!")
            }
        }
    }
    
    fn remove(node: &mut Box<Node>, val: u32) {
        println!("node val {}",node.val);
        if node.val == val {
            println!("Deleted value {val} from the tree!")  
        }
        else {
            if node.val > val {
               match node.left {
                Some(ref mut left) => {
                    BinaryTree::remove(left, val)
                }
                None => {
                    println!("Value {val} not fount in the tree!")
                }
               }
            }else {
                match node.right {
                    Some(ref mut right) => {
                        BinaryTree::remove(right, val)
                    }
                    None => {
                        println!("Value {val} not found in the tree!")           
                    }
                }
            }
        }
    }

    fn insert(node: &mut Box<Node>, new_node: Box<Node>) {
        // try placin new node in left side of the node
        if new_node.val < node.val {
            match node.left {
                Some(ref mut left_node) => {
                    BinaryTree::insert(left_node, new_node);
                }
                None => {
                    node.left = Some(new_node);
                }
            }
        } else {
            // insert into right side
            match node.right {
                Some(ref mut right_node) => {
                    BinaryTree::insert(right_node, new_node);
                }
                None => {
                    node.right = Some(new_node);
                }
            }
        }
    }

}