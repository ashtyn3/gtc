use crate::position::{Normalizable, Position};
use bitvec::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BitBoard {
    pub num: BitArray<u64, Msb0>,
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            num: 0.into_bitarray::<Msb0>(),
        }
    }
    pub fn position(&mut self, pos: Position) -> bool {
        let actual_pos = pos.normal();

        *self.num.get(actual_pos).unwrap()
    }
    pub fn set(&mut self, pos: Position) -> BitBoard {
        let actual_pos = pos.normal();
        self.num.set(actual_pos, true);

        self.clone()
    }

    pub fn fill_range(&mut self, r: std::ops::Range<u8>) -> BitBoard {
        for i in r {
            self.num.set(i.into(), true);
        }

        self.clone()
    }
    pub fn to_string(self) -> String {
        return self.num.to_string();
    }
}
