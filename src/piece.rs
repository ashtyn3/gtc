use strum::{Display, EnumString};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, EnumString, Display, Hash, Default,
)]
pub enum Side {
    #[default]
    Orange = 0,
    White,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, EnumString, Display)]
pub enum Piece {
    None,
    Goat(Side),
    Horse(Side),
    Sloth(Side),
    Bird(Side),
    Tiger(Side),
    Otter(Side),
    Snake(Side),
    MantisShrimp(Side),
}

impl Piece {
    /// Capitalizes encoded piece depending on side
    pub fn side_encode(self, p: char, s: Side) -> String {
        match s {
            Side::Orange => p.to_uppercase().to_string(),
            Side::White => p.to_string(),
        }
    }
    /// Converts piece to tile notation
    pub fn encode(self) -> String {
        match self.clone() {
            Piece::None => '#'.to_string(),
            Piece::Goat(side) => self.side_encode('g', side),
            Piece::Horse(side) => self.side_encode('h', side),
            Piece::Sloth(side) => self.side_encode('s', side),
            Piece::Bird(side) => self.side_encode('i', side),
            Piece::Tiger(side) => self.side_encode('t', side),
            Piece::Otter(side) => self.side_encode('o', side),
            Piece::Snake(side) => self.side_encode('l', side),
            Piece::MantisShrimp(side) => self.side_encode('m', side),
        }
    }
    /// Converts Tile Notation to piece
    pub fn decode(code: String) -> Result<(Piece, Side), &'static str> {
        let mut side: Side = Side::White;
        if code.chars().next().unwrap().is_uppercase() {
            side = Side::Orange;
        }
        let p = match code
            .chars()
            .next()
            .unwrap()
            .to_lowercase()
            .to_string()
            .chars()
            .next()
            .unwrap()
        {
            'g' => Piece::Goat(side),
            'h' => Piece::Horse(side),
            's' => Piece::Sloth(side),
            'i' => Piece::Bird(side),
            't' => Piece::Tiger(side),
            'o' => Piece::Otter(side),
            'l' => Piece::Snake(side),
            'm' => Piece::MantisShrimp(side),
            _ => Piece::None,
        };

        Ok((p, side))
    }
}
