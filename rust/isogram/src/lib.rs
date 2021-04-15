pub fn check(candidate: &str) -> bool {
    let candidate = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    let mut unique = candidate.clone();
    unique.sort();
    unique.dedup();

    unique.len() == candidate.len()
}
