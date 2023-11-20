use crate::bitboard::BitBoard;
use crate::piece::*;
use crate::position::Position;
use bitvec::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Game {
    pub goats: BitBoard,
    pub horses: BitBoard,
    pub sloths: BitBoard,
    pub birds: BitBoard,

    pub tigers: BitBoard,
    pub bears: BitBoard,
    pub snakes: BitBoard,
    pub mantis_shrimps: BitBoard,

    pub white: BitBoard,
    pub orange: BitBoard,
}

impl Game {
    pub fn new() -> Self {
        Game {
            goats: BitBoard::new().set((1, 1)).set((1, 8)),
            horses: BitBoard::new().set((2, 1)).set((2, 8)),
            tigers: BitBoard::new().set((3, 1)).set((3, 8)),
            bears: BitBoard::new().set((4, 1)).set((4, 8)),
            snakes: BitBoard::new().set((5, 1)).set((5, 8)),
            mantis_shrimps: BitBoard::new().set((6, 1)).set((6, 8)),
            sloths: BitBoard::new().set((7, 1)).set((7, 8)),
            birds: BitBoard::new().set((8, 1)).set((8, 8)),
            white: BitBoard::new().fill_range(0..8),
            orange: BitBoard::new().fill_range(56..64),
        }
    }
    pub fn board_state(self) -> BitArray<u64, Msb0> {
        (self.white.num.data | self.orange.num.data).into_bitarray()
    }

    pub fn piece_from_norm(self, i: u64) -> Piece {
        let mut temp_bitboard = BitBoard::new();
        temp_bitboard.num.set(i as usize, true);
        let is_piece = self.board_state().data & temp_bitboard.num.data;

        let side: Side;
        if is_piece != 0 {
            if temp_bitboard.num.data & self.white.num.data != 0 {
                side = Side::White;
            } else {
                side = Side::Orange;
            }
        } else {
            return Piece::None;
        }
        if temp_bitboard.num.data & self.goats.num.data > 0 {
            return Piece::Goat(side);
        } else if temp_bitboard.num.data & self.horses.num.data > 0 {
            return Piece::Horse(side);
        } else if temp_bitboard.num.data & self.tigers.num.data > 0 {
            return Piece::Tiger(side);
        } else if temp_bitboard.num.data & self.bears.num.data > 0 {
            return Piece::Bear(side);
        } else if temp_bitboard.num.data & self.snakes.num.data > 0 {
            return Piece::Snake(side);
        } else if temp_bitboard.num.data & self.mantis_shrimps.num.data > 0 {
            return Piece::MantisShrimp(side);
        } else if temp_bitboard.num.data & self.sloths.num.data > 0 {
            return Piece::Sloth(side);
        } else if temp_bitboard.num.data & self.birds.num.data > 0 {
            return Piece::Bird(side);
        }

        return Piece::None;
    }

    fn side_bitboard(self, s: Side) -> BitBoard {
        match s {
            Side::Orange => self.orange,
            Side::White => self.white,
        }
    }
    fn piece_bitboard(self, s: Piece) -> BitBoard {
        match s {
            Piece::None => todo!(),
            Piece::Goat(_) => self.goats,
            Piece::Horse(_) => self.horses,
            Piece::Sloth(_) => self.sloths,
            Piece::Bird(_) => self.birds,
            Piece::Tiger(_) => self.tigers,
            Piece::Bear(_) => self.bears,
            Piece::Snake(_) => self.snakes,
            Piece::MantisShrimp(_) => self.mantis_shrimps,
        }
    }

    pub fn pos_from_piece(self, p: Piece) -> Position {
        let bits = self.piece_bitboard(p).num.data
            & self.side_bitboard(Piece::decode(p.encode()).1).num.data;

        let mut bitb = BitBoard::new();
        bitb.num = bits.into_bitarray();
        let i = bitb.num.first_one().unwrap();

        return Game::normal_to_pos(i as u64);
    }

    pub fn normal_to_pos(i: u64) -> Position {
        let x = i % 8;
        let y = i / 8;

        (x + 1, y + 1)
    }

    pub fn encode(self) -> String {
        let mut none_count = 0;
        let mut fen: Vec<String> = vec![];
        for i in 0..64 {
            let piece = self.piece_from_norm(i);
            if piece != Piece::None {
                if none_count != 0 {
                    fen.push(none_count.to_string());
                }
                none_count = 0;
                fen.push(piece.encode());
            } else {
                none_count += 1;
                if none_count == 8 {
                    fen.push("8".to_string());
                    none_count = 0;
                }
            }
            let pos = Game::normal_to_pos(i);
            if pos.0 == 7 && pos.1 != 7 {
                fen.push("/".to_string())
            }
        }
        fen.join("")
    }

    pub fn print_board(self) {
        for i in 0..64 {
            if i % 8 == 0 {
                println!();
            }
            print!(" {} ", self.piece_from_norm(i).encode());
        }
    }
}
