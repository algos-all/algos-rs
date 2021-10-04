/// The reason to derive `PartialEq` is to be able to use `assert_eq!`
/// in the tests. Otherwise, the binary operator `==` will not work.
#[derive(Debug, PartialEq)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            elem: value,
            next: Link::None,
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Option::None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn push(&mut self, value: T) {
        let new_head = Node {
            elem: value,
            next: self.head.take(),
        };

        self.head = Option::Some(Box::new(new_head));
    }
}

#[cfg(test)]
mod tests {
    use super::Link;
    use super::List;
    use super::Node;

    #[test]
    fn list_new_00() {
        let list: List<i32> = List::new();

        assert_eq!(list.head, Link::None);
    }

    #[test]
    fn list_new_01() {
        let mut list: List<i32> = List::new();

        list.head = Link::Some(Box::new(Node {
            elem: 42,
            next: Link::None,
        }));

        assert_ne!(list.head, Link::None);
    }

    #[test]
    fn list_00() {
        let mut list: List<i32> = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn list_peek_00() {
        let mut list: List<i32> = List::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn list_peek_01() {
        let mut xs: List<i32> = List::new();

        xs.push(42);
        xs.push(43);

        xs.peek_mut().map(|value| {
            *value = 99;
        });

        assert_eq!(xs.peek(), Some(&99));
    }
}
