pub mod bitboard;
pub mod board;
pub mod game;
pub mod piece;
pub mod position;
mod repl;

use crate::repl::run;

fn main() {
    run();
}
