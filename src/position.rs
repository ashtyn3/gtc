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

    fn encode(&self) -> Result<String, &str> {
        return Ok("".to_string());
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

    fn encode(&self) -> Result<String, &str> {
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
            _ => return Err("invalid position"),
        };

        coords[1] = self.0.to_string();

        Ok(coords.join("").to_string())
    }
}
pub fn decode_position(p: String) -> Result<Position, &'static str> {
    let parts = p.chars().collect::<Vec<char>>();
    if parts.len() < 2 {
        return Err("invalid position string");
    }
    let row = parts[0];
    let column = parts[1];
    let column_dig = column.to_digit(10);
    if column_dig.is_none() {
        return Err("invalid position string");
    }
    let mut pos = (column_dig.unwrap() as u64, 0);

    pos.1 = match row.to_string().as_str() {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        _ => 0,
    } as u64;

    Ok(pos)
}
