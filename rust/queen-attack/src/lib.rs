const BOARD_SIZE: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub struct ChessPosition {
    rank: usize,
    file: usize,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !((0..8).contains(&rank) && (0..8).contains(&file)) {
            return None;
        }
        Some(Self {
            rank: rank as usize,
            file: file as usize,
        })
    }

    pub fn lines(&self) -> impl Iterator<Item = Self> + '_ {
        let vertical = (0..BOARD_SIZE).map(move |f| (self.rank, f));
        let horizontal = (0..BOARD_SIZE).map(move |r| (r, self.file));
        vertical
            .chain(horizontal)
            .map(|(rank, file)| Self { rank, file })
    }

    pub fn diagonals(&self) -> impl Iterator<Item = Self> {
        let (x, y) = (self.file, self.rank);
        let top_left = (0..y).rev().enumerate().map(|(i, y)| (x - i, y));
        let top_right = (0..y).rev().enumerate().map(|(i, y)| (x + i, y));
    }

    pub fn to_bitboard(&self) -> usize {
        (self.rank * 8) + self.file
    }
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0
            .lines()
            .any(|attack_position| attack_position == other.0)
    }
}
