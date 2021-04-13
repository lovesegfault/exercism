pub fn build_proverb(list: &[&str]) -> String {
    // It's a shame that I need this special case here, but unfortunately the .chain call in the
    // iterator means its necessary, lest we attempt to accest list[0] on an empty list.
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|elems| format!("For want of a {} the {} was lost.\n", elems[0], elems[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect::<String>()
}
