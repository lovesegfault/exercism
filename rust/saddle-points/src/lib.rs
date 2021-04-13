pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let height = input.len();
    if height > 0 {
        let width = input[0].len();
        for y in 0..height {
            for x in 0..width {
                let v = input[y][x];
                if input[y].iter().all(|i| v >= *i) && (0..height).all(|i| input[i][x] >= v) {
                    res.push((y, x))
                }
            }
        }
    }
    res
}
