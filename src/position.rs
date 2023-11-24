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

    fn encode(&self) -> String {
        return "".to_string();
    }
    fn is_valid(&self) -> bool {
        return true;
    }
}

impl Normalizable for Position {
    /// Returns normalized position from regular position
    fn normal(&self) -> usize {
        let (x, y) = self;
        (((y - 1) * 8) + (x - 1)) as usize
    }

    fn is_valid(&self) -> bool {
        if self.1 < 1 {
            return false;
        }
        if self.0 < 1 {
            return false;
        }
        return true;
    }

    fn encode(&self) -> String {
        let mut coords: [String; 2] = ["".to_string(), "".to_string()];

        match self.1 {
            1 => coords[0] = "a".to_string(),
            2 => coords[0] = "b".to_string(),
            3 => coords[0] = "c".to_string(),
            4 => coords[0] = "d".to_string(),
            5 => coords[0] = "e".to_string(),
            6 => coords[0] = "f".to_string(),
            7 => coords[0] = "g".to_string(),
            8 => coords[0] = "h".to_string(),
            _ => panic!("invalid position"),
        }

        coords[1] = self.0.to_string();

        coords.join("").to_string()
    }
}
