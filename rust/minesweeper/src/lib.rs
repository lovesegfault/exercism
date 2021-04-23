fn count_at((x, y): (usize, usize), minefield: &[Vec<char>]) -> String {
    let (x_min, x_max) = (x.saturating_sub(1), x + 1);
    let (y_min, y_max) = (y.saturating_sub(1), y + 1);
    let count = (x_min..=x_max)
        .flat_map(|x| (y_min..=y_max).map(move |y| (x, y)))
        .filter_map(|(x, y)| minefield.get(x).and_then(|r| r.get(y)))
        .filter(|&&c| c == '*')
        .count();
    match count {
        0 => " ".to_string(),
        x => x.to_string(),
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // I simply refuse to use &[&str], it's a horrible choice for type and just causes headaches
    let minefield: Vec<Vec<char>> = minefield.iter().map(|r| r.chars().collect()).collect();

    minefield
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(move |(j, _)| (i, j))
                .map(|(i, j)| match minefield[i][j] {
                    '*' => "*".to_string(),
                    _ => count_at((i, j), &minefield),
                })
                .collect()
        })
        .collect()
}
