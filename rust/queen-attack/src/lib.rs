#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }
        Some(ChessPosition(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let self_pos = &self.0;
        let other_pos = &other.0;
        let (rank1, file1) = (self_pos.0, self_pos.1);
        let (rank2, file2) = (other_pos.0, other_pos.1);
        rank1 == rank2 || file1 == file2 || (rank1 - rank2).abs() == (file1 - file2).abs()
    }
}
