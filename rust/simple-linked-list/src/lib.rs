use std::cell::RefCell;
use std::iter::FromIterator;
use std::rc::Rc;

pub struct SimpleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let mut length = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            length += 1;
            current = node.borrow().next.clone();
        }
        length
    }

    pub fn push(&mut self, _element: T) {
        if self.is_empty() {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: _element,
                next: None,
            })));
        } else {
            let mut current = self.head.clone();
            while let Some(node) = current {
                if node.borrow().next.is_none() {
                    node.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                        data: _element,
                        next: None,
                    })));
                    break;
                }
                current = node.borrow().next.clone();
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
