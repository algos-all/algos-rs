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

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| &*node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    node: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { node: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_deref_mut();
            &mut node.elem
        })
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

    #[test]
    fn list_iter_00() {
        let mut xs = List::new();

        xs.push(1);
        xs.push(2);
        xs.push(3);

        let mut it = xs.into_iter();
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn list_iter_01() {
        let mut xs = List::new();

        xs.push(1);
        xs.push(2);
        xs.push(3);

        let mut it = xs.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);

        let mut it = xs.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn list_iter_02() {
        let mut xs = List::new();

        xs.push(1);
        xs.push(2);
        xs.push(3);

        let mut it = xs.iter_mut();
        assert_eq!(it.next(), Some(&mut 3));
        assert_eq!(it.next(), Some(&mut 2));
        assert_eq!(it.next(), Some(&mut 1));
        assert_eq!(it.next(), None);
    }
}
