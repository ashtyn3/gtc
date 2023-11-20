/*!
Positions have two types:

Normalized: a single unsigned integer repersenting an index of a bit in a bitboard.
regular or non-normalized: a tuple containing a non-zero inclusive x and y (or column and row) pair of board position
*/
pub type Position = (u64, u64);

pub trait Normalizable {
    fn normal(&self) -> usize {
        return 0;
    }
}

impl Normalizable for Position {
    /// Returns normalized position from regular position
    fn normal(&self) -> usize {
        let (x, y) = self;
        (((y - 1) * 8) + (x - 1)) as usize
    }
}
