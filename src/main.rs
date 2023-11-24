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

    // let p = g.pos_from_piece(Piece::Goat(Side::White));
    // println!("{:?}: {:?}", p, g.piece_from_norm(p.normal() as u64));

    g.make_move(Piece::Snake(Side::Orange), (6, 7));
    g.make_move(Piece::Snake(Side::Orange), (5, 6));
    g.make_move(Piece::Snake(Side::Orange), (4, 5));
    g.make_move(Piece::Snake(Side::Orange), (3, 4));
    g.make_move(Piece::Snake(Side::Orange), (2, 3));
    g.make_move(Piece::Snake(Side::Orange), (1, 2));
    g.make_move(Piece::Snake(Side::Orange), (1, 1));
    g.make_move(Piece::Snake(Side::Orange), (2, 2));
    println!("{}", g.encode());
    // g.make_move(Piece::Snake(Side::Orange), (3, 3));
    // g.make_move(Piece::Snake(Side::Orange), (4, 2));
    g.make_move(Piece::Tiger(Side::White), (2, 2));
    println!("{}", g.encode());
    // println!(
    //     "{:?}",
    //     (g.move_mask_raw(Piece::Snake(Side::Orange)).num.data
    //         & bitboard::BitBoard::new()
    //             .set(g.pos_from_piece(Piece::Tiger(Side::White)))
    //             .num
    //             .data)
    //         .into_bitarray::<Msb0>()
    // );
}
