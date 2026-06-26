mod board;
mod chess_move;
mod move_gen;
mod piece;
mod position;

use board::Board;

use crate::move_gen::gen_moves;

fn main() {
    let mut board = Board::new();
    board.starting_position();

    board.print_board();
    gen_moves(&board, 0);
}
