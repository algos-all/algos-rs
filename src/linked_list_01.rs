use std::cmp::PartialEq;
use std::mem;

pub struct List01 {
    head: Link,
}

#[derive(Debug, PartialEq)]
enum Link {
    None,
    Node(Box<Node>),
}

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

impl List01 {
    pub fn new() -> Self {
        List01 { head: Link::None }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::None) {
            Link::None => Option::None,
            Link::Node(node) => {
                self.head = node.next;
                Option::Some(node.elem)
            }
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::None),
        };

        self.head = Link::Node(Box::new(new_node))
    }
}

#[cfg(test)]
mod tests {
    use super::Link;
    use super::List01;
    use super::Node;

    #[test]
    fn list_new_00() {
        let list = List01::new();

        assert_eq!(list.head, Link::None);
    }

    #[test]
    fn list_new_01() {
        let mut list = List01::new();

        list.head = Link::Node(Box::new(Node {
            elem: 42,
            next: Link::None,
        }));

        assert_ne!(list.head, Link::None);
    }

    #[test]
    fn list_00() {
        let mut list = List01::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }
}
