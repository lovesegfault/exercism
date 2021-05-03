#[derive(Debug, PartialEq, Eq)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !((0..8).contains(&rank) && (0..8).contains(&file)) {
            return None;
        }
        Some(Self { rank, file })
    }

    pub fn lines(&self) -> impl Iterator<Item = Self> + '_ {
        let (rank, file) = (self.rank, self.file);
        (0..8)
            .flat_map(move |i| std::array::IntoIter::new([(rank, i), (i, file)]))
            .filter_map(|(rank, file)| Self::new(rank, file))
    }

    pub fn diagonals(&self) -> impl Iterator<Item = Self> {
        let (rank, file) = (self.rank, self.file);
        (0..=rank)
            .enumerate()
            .chain((rank..8).enumerate())
            .map(|(i, rank)| (i as i32, rank))
            .flat_map(move |(i, rank)| {
                std::array::IntoIter::new([(file - i, rank), (file + i, rank)])
            })
            .filter_map(|(rank, file)| Self::new(rank, file))
    }
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    fn attacked_positions(&self) -> impl Iterator<Item = ChessPosition> + '_ {
        let lines = self.0.lines();
        let diagonals = self.0.diagonals();
        lines.chain(diagonals)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.attacked_positions()
            .any(|attack_position| attack_position == other.0)
    }
}
