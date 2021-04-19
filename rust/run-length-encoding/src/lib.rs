use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .dedup_with_count()
        .map(|(count, c)| match count {
            1 => format!("{}", c),
            _ => format!("{}{}", count, c),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold((0_usize, String::new()), |(n, text), c| {
            if c.is_digit(10) {
                let repeat = n * 10 + c.to_digit(10).expect("broken encoding") as usize;
                (repeat, text)
            } else {
                let repeat = n.max(1);
                let unfolded = format!("{}", c.to_string().repeat(repeat));
                (0, text + &unfolded)
            }
        })
        .1
}
