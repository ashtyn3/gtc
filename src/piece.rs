use crate::{bitboard::BitBoard, position::Position};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Side {
    Orange,
    White,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Piece {
    None,
    Goat(Side),
    Horse(Side),
    Sloth(Side),
    Bird(Side),
    Tiger(Side),
    Bear(Side),
    Snake(Side),
    MantisShrimp(Side),
}

impl Piece {
    pub fn side_encode(self, p: char, s: Side) -> String {
        match s {
            Side::Orange => p.to_uppercase().to_string(),
            Side::White => p.to_string(),
        }
    }
    pub fn encode(self) -> String {
        match self.clone() {
            Piece::None => '#'.to_string(),
            Piece::Goat(side) => self.side_encode('g', side),
            Piece::Horse(side) => self.side_encode('h', side),
            Piece::Sloth(side) => self.side_encode('s', side),
            Piece::Bird(side) => self.side_encode('i', side),
            Piece::Tiger(side) => self.side_encode('t', side),
            Piece::Bear(side) => self.side_encode('b', side),
            Piece::Snake(side) => self.side_encode('l', side),
            Piece::MantisShrimp(side) => self.side_encode('m', side),
        }
    }
    pub fn decode(code: String) -> (Piece, Side) {
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
            'b' => Piece::Bear(side),
            'l' => Piece::Snake(side),
            'm' => Piece::MantisShrimp(side),
            c => panic!("bad decode {}", c),
        };

        (p, side)
    }

    pub fn move_mask_raw(self, pos: Position, is_take: bool) -> BitBoard {
        let mut bitb = BitBoard::new();

        match self {
            Piece::None => bitb,
            Piece::Goat(_) | Piece::Horse(_) | Piece::Bird(_) => bitb
                .set((pos.0 + 1, pos.1))
                .set((pos.0 - 1, pos.1))
                .set((pos.0, pos.1 + 1))
                .set((pos.0, pos.1 - 1))
                .set((pos.0 + 1, pos.1 + 1))
                .set((pos.0 - 1, pos.1 + 1))
                .set((pos.0 - 1, pos.1 - 1))
                .set((pos.0 + 1, pos.1 - 1)),
            Piece::Sloth(_) => bitb
                .set((pos.0 + 1, pos.1))
                .set((pos.0 - 1, pos.1))
                .set((pos.0, pos.1 + 1))
                .set((pos.0, pos.1 - 1)),
            // TODO: Add consumption moves and account for forward direction depennding on side
            // (white forward = y+1 and orange forward = y-1)
            _ => bitb
                .set((pos.0 + 1, pos.1 + 1))
                .set((pos.0 - 1, pos.1 + 1))
                .set((pos.0 - 1, pos.1 - 1))
                .set((pos.0 + 1, pos.1 - 1)),
        }
    }
}
