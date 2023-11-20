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
    /// Sets up game board in starting positions
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

    /// returns bitboard of combined side state (white | orange)
    pub fn board_state(self) -> BitArray<u64, Msb0> {
        (self.white.num.data | self.orange.num.data).into_bitarray()
    }

    /// given normalized position finds the piece in given position
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

    /// gets corresponding bitboard for side
    fn side_bitboard(self, s: Side) -> BitBoard {
        match s {
            Side::Orange => self.orange,
            Side::White => self.white,
        }
    }
    /// gets corresponding bitboard for piece type
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

    /// Finds piece non-normalized position on board
    pub fn pos_from_piece(self, p: Piece) -> Position {
        let bits = self.piece_bitboard(p).num.data
            & self.side_bitboard(Piece::decode(p.encode()).1).num.data;

        let mut bitb = BitBoard::new();
        bitb.num = bits.into_bitarray();
        let i = bitb.num.first_one().unwrap();

        return Game::normal_to_pos(i as u64);
    }

    /// Converts normalized position to position
    pub fn normal_to_pos(i: u64) -> Position {
        let x = i % 8;
        let y = i / 8;

        (x + 1, y + 1)
    }

    /// Encodes piece to tile notation
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

    /// prints ascii representation of board to stdout
    pub fn print_board(self) {
        for i in 0..64 {
            if i % 8 == 0 {
                println!();
            }
            print!(" {} ", self.piece_from_norm(i).encode());
        }
    }

    /// Generates bitboard of possible moves from given position
    pub fn move_mask_raw(self, p: Piece) -> BitBoard {
        let mut bitb = BitBoard::new();
        let pos = self.pos_from_piece(p);

        match p {
            Piece::None => bitb,
            Piece::Goat(_) | Piece::Horse(_) | Piece::Bird(_) => BitBoard::from_bitarray(
                (!self.board_state().data
                    & bitb
                        .set((pos.0 + 1, pos.1))
                        .set((pos.0 - 1, pos.1))
                        .set((pos.0, pos.1 + 1))
                        .set((pos.0, pos.1 - 1))
                        .set((pos.0 + 1, pos.1 + 1))
                        .set((pos.0 - 1, pos.1 + 1))
                        .set((pos.0 - 1, pos.1 - 1))
                        .set((pos.0 + 1, pos.1 - 1))
                        .num
                        .data)
                    .into_bitarray(),
            ),
            Piece::Sloth(_) => BitBoard::from_bitarray(
                (!self.board_state().data
                    & bitb
                        .set((pos.0 + 1, pos.1))
                        .set((pos.0 - 1, pos.1))
                        .set((pos.0, pos.1 + 1))
                        .set((pos.0, pos.1 - 1))
                        .num
                        .data)
                    .into_bitarray(),
            ),
            // TODO: Add consumption moves and account for forward direction depennding on side
            // (white forward = y+1 and orange forward = y-1)
            Piece::Tiger(s) | Piece::Bear(s) | Piece::MantisShrimp(s) | Piece::Snake(s) => {
                let mut consume_mask: BitBoard = BitBoard::new();
                match s {
                    Side::Orange => {
                        let mask = bitb.set((pos.0, pos.1 - 1)).set((pos.0 - 1, pos.1 - 1));
                        if self.white.num.data & mask.num.data != 0 {
                            consume_mask = BitBoard::from_bitarray(
                                (self.board_state().data & mask.num.data).into_bitarray::<Msb0>(),
                            );
                        }
                    }
                    Side::White => {
                        let mask = bitb.set((pos.0, pos.1 + 1)).set((pos.0 + 1, pos.1 + 1));
                        if self.orange.num.data & mask.num.data != 0 {
                            consume_mask = BitBoard::from_bitarray(
                                (self.board_state().data & mask.num.data).into_bitarray::<Msb0>(),
                            );
                        }
                    }
                };
                let base_mask = BitBoard::new()
                    .set((pos.0 - 1, pos.1 + 1))
                    .set((pos.0 + 1, pos.1 + 1))
                    .set((pos.0 + 1, pos.1 - 1))
                    .set((pos.0 - 1, pos.1 - 1));

                BitBoard::from_bitarray(
                    ((!self.board_state().data & base_mask.num.data) | consume_mask.num.data)
                        .into_bitarray(),
                )
            }
        }
    }
}
