use dsa::BinaryTree;



fn main() {
    let mut tree = BinaryTree::new();
    tree.add(5);
    tree.add(2);
    tree.add(1);
    tree.delete(2);
}
