use std::{collections::HashMap, sync::Arc};

use bitvec::{prelude::Msb0, view::BitViewSized};

use crate::{
    bitboard::BitBoard,
    board::Board,
    piece::{Piece, Side},
    position::Position,
};

#[derive(Clone)]
pub struct Instance {
    pub board: Board,
    pub side: Side,
    pub states: Arc<String>,
}

impl std::ops::Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Orange => Side::White,
            Side::White => Side::Orange,
        }
    }
}
const WHITE_TEMPLATE_NUMBER: u64 =
    0b00000000_10000001_10000001_10000001_10000001_10000001_10000001_10000001;
const ORANGE_TEMPLATE_NUMBER: u64 =
    0b10000001_10000001_10000001_10000001_10000001_10000001_10000001_00000000;

const GOAT_E_NUMBER: u64 =
    0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

const SLOTH_E_NUMBER: u64 =
    0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;

impl Instance {
    pub fn active_edges(&self, side: Side) -> BitBoard {
        match side {
            Side::White => {
                let edge_data = self.board.white.num.data & WHITE_TEMPLATE_NUMBER as u64;
                BitBoard::from_bitarray(edge_data.into_bitarray::<Msb0>())
            }
            Side::Orange => {
                let edge_data = self.board.orange.num.data & ORANGE_TEMPLATE_NUMBER as u64;
                BitBoard::from_bitarray(edge_data.into_bitarray::<Msb0>())
            }
        }
    }

    pub fn make_move(&mut self, p: Piece, pos: Position) {
        let (_, side) = Piece::decode(p.encode()).unwrap();
        if side != self.side {
            println!("{}'s turn", self.side);
            return;
        }
        if self.board.new_position(p, pos) == true {
            self.side = !self.side;
        }
    }

    pub fn passive_tiles(&self) -> BitBoard {
        let b = self.board.board_state().data;

        return BitBoard::from_bitarray(
            (b & self.board.goats.num.data
                | b & self.board.horses.num.data
                | b & self.board.sloths.num.data
                | b & self.board.birds.num.data)
                .into(),
        );
    }
    pub fn has_passiveless(&self) -> HashMap<Side, bool> {
        let mut hmap = HashMap::from([(Side::White, false), (Side::Orange, false)]);

        if self.passive_tiles().num.data & self.board.orange.num.data == 0 {
            hmap.entry(Side::Orange).and_modify(|e| *e = true);
        }

        if self.passive_tiles().num.data & self.board.white.num.data == 0 {
            hmap.entry(Side::White).and_modify(|e| *e = true);
        }

        return hmap;
    }

    pub fn has_alignment(&self) -> (bool, bool) {
        let curr_s_side = self.active_edges(self.side).num.data & SLOTH_E_NUMBER;
        let curr_g_side = self.active_edges(self.side).num.data & GOAT_E_NUMBER;

        let op_g_side = self.active_edges(!self.side).num.data & GOAT_E_NUMBER;
        let op_s_side = self.active_edges(!self.side).num.data & SLOTH_E_NUMBER;

        let (mut has_g_side, mut has_s_side) = (false, false);
        if curr_s_side | op_s_side > curr_s_side {
            has_s_side = false;
        } else {
            let p_on_edge = (self.passive_tiles().num.data & curr_s_side).into_bitarray::<Msb0>();

            if p_on_edge.count_ones() == 4 {
                has_s_side = true;
            }
        }

        if curr_g_side | op_g_side > curr_g_side {
            has_g_side = false;
        } else {
            let p_on_edge = (self.passive_tiles().num.data & curr_g_side)
                .into_bitarray::<Msb0>()
                .count_ones();

            if p_on_edge == 4 {
                has_g_side = true;
            }
        }

        return (has_g_side, has_s_side);
    }

    pub fn has_win(&self) -> bool {
        let align = self.has_alignment();
        let passives = self.has_passiveless();

        if align.0 == true || align.1 == true || *passives.get(&(!self.side)).unwrap() == true {
            return true;
        }

        return false;
    }
}
