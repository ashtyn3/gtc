use crate::position::{Normalizable, Position};
use bitvec::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BitBoard {
    pub num: BitArray<u64, Msb0>,
}

impl BitBoard {
    /// returns zeroed bitboard
    pub fn new() -> Self {
        BitBoard {
            num: 0.into_bitarray::<Msb0>(),
        }
    }

    pub fn from_bitarray(arr: BitArray<u64, Msb0>) -> Self {
        BitBoard { num: arr }
    }

    /// returns if high bit at position
    pub fn position(&mut self, pos: Position) -> bool {
        let actual_pos = pos.normal();

        *self.num.get(actual_pos).unwrap()
    }

    /// set position
    pub fn set(&mut self, pos: Position) -> BitBoard {
        if pos.0 < 1 || pos.1 < 1 {
            return self.clone();
        }
        let actual_pos = pos.normal();
        let binding = self.clone();
        let curr = binding.num.get(actual_pos);
        if curr.is_none() {
            return self.clone();
        }
        self.num.set(actual_pos, !*curr.unwrap());

        self.clone()
    }

    /// fill range of normalized positions
    pub fn fill_range(&mut self, r: std::ops::Range<u8>) -> BitBoard {
        for i in r {
            self.num.set(i.into(), true);
        }

        self.clone()
    }

    /// string of binary array representing bitboard
    pub fn to_string(self) -> String {
        return self.num.to_string();
    }
}
