use crate::bitboard::BitBoard;
use crate::piece::*;
use crate::position::{Normalizable, Position};
use bitvec::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub goats: BitBoard,
    pub horses: BitBoard,
    pub sloths: BitBoard,
    pub birds: BitBoard,

    pub tigers: BitBoard,
    pub otters: BitBoard,
    pub snakes: BitBoard,
    pub mantis_shrimps: BitBoard,

    pub white: BitBoard,
    pub orange: BitBoard,
}

impl Board {
    /// Sets up game board in starting positions
    pub fn new() -> Self {
        let g = Board {
            goats: BitBoard::new().set((1, 1)).set((1, 8)),
            horses: BitBoard::new().set((2, 1)).set((2, 8)),
            tigers: BitBoard::new().set((3, 1)).set((3, 8)),
            otters: BitBoard::new().set((4, 1)).set((4, 8)),
            snakes: BitBoard::new().set((5, 1)).set((5, 8)),
            mantis_shrimps: BitBoard::new().set((6, 1)).set((6, 8)),
            sloths: BitBoard::new().set((7, 1)).set((7, 8)),
            birds: BitBoard::new().set((8, 1)).set((8, 8)),
            white: BitBoard::new().fill_range(0..8),
            orange: BitBoard::new().fill_range(56..64),
        };

        g
    }

    /// Returns blank board
    pub fn blank() -> Self {
        Board {
            goats: BitBoard::new(),
            horses: BitBoard::new(),
            tigers: BitBoard::new(),
            otters: BitBoard::new(),
            snakes: BitBoard::new(),
            mantis_shrimps: BitBoard::new(),
            sloths: BitBoard::new(),
            birds: BitBoard::new(),
            white: BitBoard::new(),
            orange: BitBoard::new(),
        }
    }

    /// Decodes board string into a Game instance
    pub fn decode(code: String) -> Result<Self, &'static str> {
        let rows = code.split("/");
        let mut g = Board {
            goats: BitBoard::new(),
            horses: BitBoard::new(),
            tigers: BitBoard::new(),
            otters: BitBoard::new(),
            snakes: BitBoard::new(),
            mantis_shrimps: BitBoard::new(),
            sloths: BitBoard::new(),
            birds: BitBoard::new(),
            white: BitBoard::new(),
            orange: BitBoard::new(),
        };

        for (ri, row) in rows.enumerate() {
            let mut norm = ri * 8;
            for (_i, c) in row.chars().enumerate() {
                if c.is_ascii_digit() {
                    let modif = c.to_digit(10);
                    norm += modif.unwrap() as usize;
                } else {
                    let dec = Piece::decode(c.to_string());

                    if dec.is_err() {
                        return Err(dec.unwrap_err());
                    }
                    let (p, side) = dec.unwrap();

                    g.side_bitboard(side).num.set(norm, true);
                    g.piece_bitboard(p).unwrap().num.set(norm, true);
                    norm += 1;
                }
            }
        }
        Ok(g)
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
        } else if temp_bitboard.num.data & self.otters.num.data > 0 {
            return Piece::Otter(side);
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
    pub fn side_bitboard(&mut self, s: Side) -> &mut BitBoard {
        match s {
            Side::Orange => &mut self.orange,
            Side::White => &mut self.white,
        }
    }
    /// gets corresponding bitboard for piece type
    fn piece_bitboard(&mut self, s: Piece) -> Result<&mut BitBoard, &'static str> {
        match s {
            Piece::None => Err("Invalid piece"),
            Piece::Goat(_) => Ok(&mut self.goats),
            Piece::Horse(_) => Ok(&mut self.horses),
            Piece::Sloth(_) => Ok(&mut self.sloths),
            Piece::Bird(_) => Ok(&mut self.birds),
            Piece::Tiger(_) => Ok(&mut self.tigers),
            Piece::Otter(_) => Ok(&mut self.otters),
            Piece::Snake(_) => Ok(&mut self.snakes),
            Piece::MantisShrimp(_) => Ok(&mut self.mantis_shrimps),
        }
    }

    /// Finds piece non-normalized position on board
    pub fn pos_from_piece(&mut self, p: Piece) -> Result<Position, &'static str> {
        let p_dec = Piece::decode(p.encode());
        if p_dec.is_err() {
            return Err(p_dec.unwrap_err());
        }
        let p_bitb = self.piece_bitboard(p);
        if p_bitb.is_err() {
            return Err(p_bitb.err().unwrap());
        }
        let bits = p_bitb.unwrap().num.data & self.clone().side_bitboard(p_dec.unwrap().1).num.data;

        let mut bitb = BitBoard::new();
        bitb.num = bits.into_bitarray();
        let i = bitb.num.first_one().unwrap();

        return Ok(Board::normal_to_pos(i as u64));
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
            let pos = Board::normal_to_pos(i);
            if pos.0 == 8 && pos.1 != 8 {
                if none_count != 0 {
                    fen.push(none_count.to_string());
                }
                fen.push("/".to_string());
                none_count = 0;
            }
        }
        fen.join("")
    }

    /// prints ascii representation of board to stdout
    pub fn print_board(self) {
        print!("  ");
        for i in 0..8 {
            print!("  {} ", i + 1)
        }
        println!();
        print!("  +");
        for _ in 0..8 {
            print!("---+")
        }
        println!();
        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                println!();
                print!("  +");
                for _ in 0..8 {
                    print!("---+")
                }
                println!();
            }
            if i % 8 == 0 {
                print!("{} |", char::from_u32(65 + (i / 8)).unwrap())
            }
            print!(" {} |", self.piece_from_norm(i.into()).encode());
        }
        println!();
        print!("  +");
        for _ in 0..8 {
            print!("---+")
        }
        println!();
    }

    /// Generates bitboard of possible moves from given position
    pub fn move_mask_raw(&mut self, p: Piece) -> Result<BitBoard, &'static str> {
        let mut bitb = BitBoard::new();
        let pos = self.pos_from_piece(p);

        if pos.is_err() {
            return Err(pos.unwrap_err());
        }
        let pos = pos.unwrap();

        match p {
            Piece::None => Ok(bitb),
            Piece::Bird(s) => {
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
                        let mask = bitb.set((pos.0, pos.1 + 1)).set((pos.0 - 1, pos.1 + 1));
                        if self.orange.num.data & mask.num.data != 0 {
                            consume_mask = BitBoard::from_bitarray(
                                (self.board_state().data & mask.num.data).into_bitarray::<Msb0>(),
                            );
                        }
                    }
                };
                let mut base = BitBoard::new();

                if pos.0 < 8 {
                    base.set((pos.0 + 1, pos.1));
                }
                if pos.0 < 8 && pos.1 < 8 {
                    base.set((pos.0 + 1, pos.1 + 1));
                }
                if pos.0 < 8 && pos.1 > 1 {
                    base.set((pos.0 + 1, pos.1 - 1));
                }
                let base_mask = base
                    .set((pos.0 - 1, pos.1 + 1))
                    .set((pos.0 - 1, pos.1 - 1))
                    .set((pos.0, pos.1 + 1))
                    .set((pos.0, pos.1 - 1))
                    .set((pos.0 - 1, pos.1));

                Ok(BitBoard::from_bitarray(
                    ((!self.board_state().data & base_mask.num.data) | consume_mask.num.data)
                        .into_bitarray(),
                ))
            }
            Piece::Goat(_) | Piece::Horse(_) => {
                if pos.0 < 8 {
                    bitb.set((pos.0 + 1, pos.1));
                }
                if pos.0 < 8 && pos.1 < 8 {
                    bitb.set((pos.0 + 1, pos.1 + 1));
                }
                if pos.0 < 8 && pos.1 > 1 {
                    bitb.set((pos.0 + 1, pos.1 - 1));
                }
                Ok(BitBoard::from_bitarray(
                    (!self.board_state().data
                        & bitb
                            .set((pos.0 - 1, pos.1))
                            .set((pos.0, pos.1 + 1))
                            .set((pos.0, pos.1 - 1))
                            .set((pos.0 - 1, pos.1 + 1))
                            .set((pos.0 - 1, pos.1 - 1))
                            .num
                            .data)
                        .into_bitarray(),
                ))
            }
            Piece::Sloth(_) => Ok(BitBoard::from_bitarray(
                (!self.board_state().data
                    & bitb
                        .set((pos.0 + 1, pos.1))
                        .set((pos.0 - 1, pos.1))
                        .set((pos.0, pos.1 + 1))
                        .set((pos.0, pos.1 - 1))
                        .num
                        .data)
                    .into_bitarray(),
            )),
            Piece::Tiger(s) | Piece::Otter(s) | Piece::MantisShrimp(s) | Piece::Snake(s) => {
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
                        let mask = bitb.set((pos.0, pos.1 + 1)).set((pos.0 - 1, pos.1 + 1));
                        if self.orange.num.data & mask.num.data != 0 {
                            consume_mask = BitBoard::from_bitarray(
                                (self.board_state().data & mask.num.data).into_bitarray::<Msb0>(),
                            );
                        }
                    }
                };
                let mut base = BitBoard::new();
                let base_mask = base
                    .set((pos.0 - 1, pos.1 + 1))
                    .set((pos.0 + 1, pos.1 + 1))
                    .set((pos.0 + 1, pos.1 - 1))
                    .set((pos.0 - 1, pos.1 - 1));

                Ok(BitBoard::from_bitarray(
                    ((!self.board_state().data & base_mask.num.data) | consume_mask.num.data)
                        .into_bitarray(),
                ))
            }
        }
    }

    /// DO NOT USE UNLESS YOU KNOW WHAT YOU'RE DOING.
    /// Modifies game state by moving a piece to a given position without verifying with game
    /// rules.
    // Possibly make more efficient use bitwise operations and less array operations
    fn new_position_unsafe(&mut self, p: Piece, to: Position) {
        let mut binding = BitBoard::new();
        let target_bitb = binding.set(to);

        let target_is_piece = self.board_state().data & target_bitb.num.data;
        if target_is_piece > 0 {
            let target_norm = target_bitb.num.first_one().unwrap();
            let norm_enc = self.piece_from_norm(target_norm as u64).encode();
            let target_enc = Piece::decode(norm_enc);
            if target_enc.is_err() {
                println!("{}", target_enc.unwrap_err());
                return;
            }
            let (target_piece, side) = target_enc.unwrap();

            self.side_bitboard(side).set(to);
            self.piece_bitboard(target_piece).unwrap().set(to);
        }
        let p_enc = Piece::decode(p.encode());
        if p_enc.is_err() {
            println!("{}", p_enc.unwrap_err());
            return;
        }
        let (_, act_side) = p_enc.unwrap();

        let act_pos = self.pos_from_piece(p);
        if act_pos.is_err() {
            println!("{}", act_pos.unwrap_err());
            return;
        }
        self.side_bitboard(act_side).set(act_pos.unwrap());
        self.piece_bitboard(p).unwrap().set(act_pos.unwrap());

        self.side_bitboard(act_side).set(to);
        self.piece_bitboard(p).unwrap().set(to);
    }

    /// Checks if move is valid and executes if valid. Returns false if not valid.
    pub fn new_position(&mut self, p: Piece, to: Position) -> bool {
        let mut target_bitb = BitBoard::new();
        target_bitb.set(to);
        let mmask = self.move_mask_raw(p);
        if mmask.is_err() {
            println!("{}", mmask.unwrap_err());
            return false;
        }

        if mmask.unwrap().num.data & target_bitb.num.data == 0 {
            if to.encode().is_err() {
                println!("{}", to.encode().unwrap_err());
                return false;
            }
            let p_place = self.pos_from_piece(p);
            if p_place.is_err() {
                println!("{}", p_place.unwrap_err());
                return false;
            }
            if p_place.unwrap().encode().is_err() {
                println!("{}", p_place.unwrap().encode().unwrap_err());
                return false;
            }
            println!(
                "Not valid move: {}{}-{}{}",
                p.encode(),
                p_place.unwrap().encode().unwrap(),
                p.encode(),
                to.encode().unwrap()
            );
            return false;
        } else {
            self.new_position_unsafe(p, to);
            return true;
        }
    }
}
