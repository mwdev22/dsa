
#[derive(Debug)]
pub struct Node <T>{
    val: T,
    next: Box<Option<Node<T>>>
}

#[derive(Debug)]
pub struct LinkedList <T> {
    root: Node<T>
}

impl<T> LinkedList<T> {
    fn new(first_val: T) -> Self {
        LinkedList{root: Node::new(first_val)}
    }

    pub fn insert(&mut self, element: T) {
        
    } 
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node{val: val, next: None}
    }
}