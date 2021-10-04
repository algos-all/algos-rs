use std::cell::RefCell;
use std::rc::Rc;

pub struct BinarySearchTree<K, V> {
    pub root: Option<Rc<RefCell<BinarySearchNode<K, V>>>>,
}

impl<K, V> BinarySearchTree<K, V> {
    pub fn new(key: K, val: V) -> Self {
        BinarySearchTree {
            root: Some(Rc::new(RefCell::new(BinarySearchNode::new(
                key, val,
            )))),
        }
    }
}

pub struct BinarySearchNode<K, V> {
    pub key: K,
    pub val: V,
    pub lft: Option<Rc<RefCell<BinarySearchNode<K, V>>>>,
    pub rgt: Option<Rc<RefCell<BinarySearchNode<K, V>>>>,
}

impl<K, V> BinarySearchNode<K, V> {
    pub fn new(key: K, val: V) -> Self {
        BinarySearchNode {
            key: key,
            val: val,
            lft: None,
            rgt: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search_tree as bst;

    #[test]
    fn bst_new() {
        let tree = bst::BinarySearchTree::new(42, 42);

        assert_eq!(tree.root.is_some(), true);
    }
}
