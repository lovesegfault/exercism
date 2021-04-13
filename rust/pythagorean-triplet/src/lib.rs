use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum)
        .flat_map(|c| (((sum - c) / 2 + 1)..u32::min(c, sum - c)).map(move |b| (sum - b - c, b, c)))
        .filter(|&(a, b, c)| a.pow(2) + b.pow(2) == c.pow(2))
        .map(|(a, b, c)| [a, b, c])
        .collect()
}
