pub struct PascalsTriangle(u32);

fn factorial(n: u32) -> u32 {
    (2..=n).fold(1, |acc, n| acc * n)
}

fn coefficient(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0)
            .map(|n| (0..=n).map(|k| coefficient(n, k)).collect())
            .collect()
    }
}
