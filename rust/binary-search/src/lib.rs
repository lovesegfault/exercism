use std::cmp::Ordering;

pub fn find<L, I>(list: L, key: I) -> Option<usize>
where
    L: AsRef<[I]>,
    I: Ord,
{
    let list = list.as_ref();
    let mid = list.len() / 2;
    match key.cmp(list.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&list[0..mid], key),
        Ordering::Greater => find(&list[(mid + 1)..], key).map(|idx| idx + mid + 1),
    }
}
