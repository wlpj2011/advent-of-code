// This is implementing a tree where each node has a unique element/label of type T. 
// This is not the only thing one might implement and call a tree, but is useful for the purpose I want.
pub struct Tree<T> {
    root: Link<T>
}
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    children: Vec<Link<T>>
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None,
        }
    }

    pub fn insert(&mut self, elem: T, parent: T) {
        // Given an elem and parent, find the parent in the tree, and make elem a new child of it
        let new_node = Node { elem, children: Vec::new()};

    }


}