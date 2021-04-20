use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Self>>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Self>>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.as_ref();
        let mut len = 0;
        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }
        len
    }

    pub fn push(&mut self, element: T) {
        let node = Node::new(element, self.head.take());
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        while let Some(data) = self.pop() {
            reversed.push(data);
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        iter.into_iter().for_each(|i| list.push(i));
        list
    }
}

impl<T> Iterator for SimpleLinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(data) = self.pop() {
            vec.push(data);
        }
        vec.reverse();
        vec
    }
}
