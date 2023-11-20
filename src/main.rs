pub mod bitboard;
pub mod game;
pub mod piece;
pub mod position;

use bitvec::prelude::Msb0;
use bitvec::view::BitViewSized;

use crate::game::*;
use crate::piece::*;
use crate::position::*;

fn main() {
    let mut g = Game::new();
    g.orange = g.orange.set((2, 2)).set((5, 8));
    g.snakes = g.snakes.set((2, 2)).set((5, 8));

    let p = g.pos_from_piece(Piece::Goat(Side::White));
    println!("{:?}: {:?}", p, g.piece_from_norm(p.normal() as u64));
    g.print_board();

    println!();
    println!(
        "{:?}",
        (g.move_mask_raw(Piece::Snake(Side::Orange)).num.data
            & bitboard::BitBoard::new()
                .set(g.pos_from_piece(Piece::Tiger(Side::White)))
                .num
                .data)
            .into_bitarray::<Msb0>()
    );
}
