mod bitboard;
mod game;
mod piece;
mod position;

use crate::game::*;
use crate::piece::*;
use crate::position::*;

fn main() {
    let g = Game::new();

    let p = g.pos_from_piece(Piece::MantisShrimp(Side::White));
    println!("{:?}", g.piece_from_norm(p.normal() as u64));
    g.print_board()
    // println!("{}", g.goats.clone().to_string());
    // println!("{:?}", g.goats.position((2, 4)));
    // println!("{:?}", g.goats.set((8, 8)));
    // println!("{:?}", g.goats.position((8, 8)));
    // println!("{}", g.goats.clone().to_string());
}
