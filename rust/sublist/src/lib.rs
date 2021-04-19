use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

trait Sublist<T> {
    fn is_sublist_of(&self, other: &[T]) -> bool;
}

impl<T: PartialEq> Sublist<T> for &[T] {
    fn is_sublist_of(&self, other: &[T]) -> bool {
        self.is_empty() || other.windows(self.len()).any(|sub| sub == *self)
    }
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match a.len().cmp(&b.len()) {
        Ordering::Equal if a.eq(b) => Comparison::Equal,
        Ordering::Less if a.is_sublist_of(b) => Comparison::Sublist,
        Ordering::Greater if b.is_sublist_of(a) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
