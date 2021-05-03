#[derive(Debug, PartialEq, Eq)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_delta = (self.0.rank - other.0.rank).abs();
        let file_delta = (self.0.file - other.0.file).abs();
        rank_delta == 0 || file_delta == 0 || rank_delta == file_delta
    }
}
