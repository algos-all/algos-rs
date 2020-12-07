use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

pub struct Tree<K, V>
where
    K: Ord,
{
    pub root: NodeRef<K, V>,
}

impl<K, V> Tree<K, V>
where
    K: Ord,
{
    pub fn new(key: K, val: V) -> Self {
        Tree {
            root: Some(Rc::new(RefCell::new(Node::new(key, val)))),
        }
    }

    pub fn empty() -> Self {
        Tree { root: None }
    }

    pub fn find_node_with_parent(
        &self,
        key: &K,
    ) -> (NodeRef<K, V>, NodeRef<K, V>) {
        let mut node = match &self.root {
            Some(rc) => Some(Rc::clone(rc)),
            None => None,
        };

        let mut parent: NodeRef<K, V> = None;

        while let Some(rc) = node {
            let cell = (*rc).borrow();

            if *key == cell.key {
                return (parent, Some(Rc::clone(&rc)));
            }

            parent = Some(Rc::clone(&rc));

            node = if *key < cell.key {
                match &cell.lft {
                    Some(lft) => Some(Rc::clone(lft)),
                    None => None,
                }
            } else {
                match &cell.rgt {
                    Some(rgt) => Some(Rc::clone(rgt)),
                    None => None,
                }
            }
        }

        (parent, node)
    }

    pub fn find(&self, key: &K) -> NodeRef<K, V> {
        let (_, node) = self.find_node_with_parent(key);

        node
    }

    pub fn push(&mut self, key: K, val: V) {
        let (parent, node) = self.find_node_with_parent(&key);

        match (parent, node) {
            (None, None) => {
                self.root = Some(Rc::new(RefCell::new(Node::new(key, val))))
            }
            (Some(rc), None) => {
                let mut cell = rc.borrow_mut();
                if key < cell.key {
                    cell.lft =
                        Some(Rc::new(RefCell::new(Node::new(key, val))));
                } else {
                    cell.rgt =
                        Some(Rc::new(RefCell::new(Node::new(key, val))));
                }
            }
            (_, Some(rc)) => {
                rc.borrow_mut().val = val;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub lft: NodeRef<K, V>,
    pub rgt: NodeRef<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Node {
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn bst_new() {
        let tree = bst::Tree::new(42, 42);

        assert_eq!(tree.root.is_some(), true);
    }

    #[test]
    fn bst_find_node_with_parent_one() {
        let tree = bst::Tree::new(42, 42);

        let (parent, node) = tree.find_node_with_parent(&42);

        assert_eq!(parent, None);
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().borrow().val, 42)
    }

    #[test]
    fn bst_find_node_with_parent_two() {
        let tree = &bst::Tree::new(42, 42);

        tree.root.as_ref().unwrap().borrow_mut().rgt =
            Some(Rc::new(RefCell::new(bst::Node::new(43, 43))));

        let (parent, node) = tree.find_node_with_parent(&43);

        assert_eq!(parent.is_some(), true);
        assert_eq!(node.is_some(), true);

        assert_eq!(parent.unwrap().borrow().val, 42);
        assert_eq!(node.unwrap().borrow().val, 43);
    }

    #[test]
    fn bst_find_one() {
        let tree = bst::Tree::new(42, 42);

        let node = tree.find(&42);

        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().borrow().val, 42)
    }

    #[test]
    fn bst_find_none() {
        let tree = bst::Tree::new(42, 42);

        let node = tree.find(&43);

        assert_eq!(node, None);
    }

    #[test]
    fn bst_push_one() {
        let mut tree = bst::Tree::<i32, i32>::empty();

        tree.push(42, 42);

        assert_eq!(tree.root.is_some(), true);
        assert_eq!(tree.root.as_ref().unwrap().borrow().key, 42);
        assert_eq!(tree.root.as_ref().unwrap().borrow().val, 42);
    }

    #[test]
    fn bst_push_two() {
        let mut tree = bst::Tree::<i32, i32>::empty();

        tree.push(42, 42);
        tree.push(43, 43);
        tree.push(13, 13);

        let (parent13, node13) = tree.find_node_with_parent(&13);

        assert_eq!(parent13.is_some(), true);
        assert_eq!(parent13.as_ref().unwrap().borrow().key, 42);
        assert_eq!(parent13.as_ref().unwrap().borrow().val, 42);

        assert_eq!(node13.is_some(), true);
        assert_eq!(node13.as_ref().unwrap().borrow().key, 13);
        assert_eq!(node13.as_ref().unwrap().borrow().val, 13);

        let (parent43, node43) = tree.find_node_with_parent(&43);

        assert_eq!(parent43.is_some(), true);
        assert_eq!(parent43.as_ref().unwrap().borrow().key, 42);
        assert_eq!(parent43.as_ref().unwrap().borrow().val, 42);

        assert_eq!(node43.is_some(), true);
        assert_eq!(node43.as_ref().unwrap().borrow().key, 43);
        assert_eq!(node43.as_ref().unwrap().borrow().val, 43);
    }
}
