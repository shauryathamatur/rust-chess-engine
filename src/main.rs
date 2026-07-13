use crate::board::Board;
use crate::move_gen::perft;

mod board;
mod chess_move;
mod move_gen;
mod piece;
mod position;

fn main() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    board.print_board();

    for depth in 0..5 {
        println!("{}", perft(&board, depth));
    }
}
