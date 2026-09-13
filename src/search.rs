use crate::{Board, chess_move::Move, evaluation::evaluate, move_gen::gen_legal_moves_for_color};

pub fn negamax(board: &Board, depth: i32, color: i32) -> (i32, Option<Move>) {
    let moves = gen_legal_moves_for_color(board, board.side_to_move());

    if moves.is_empty() {
        if board.is_in_check(board.side_to_move()) {
            return (-100_000, None);
        } else {
            return (0, None);
        }
    }

    if depth == 0 {
        return (color * evaluate(board), None);
    }

    let mut max = i32::MIN;
    let mut best_move = None;

    for mv in moves {
        let mut board_copy = *board;
        board_copy.make_move(mv);

        let score = -(negamax(&board_copy, depth - 1, -color).0);

        if score > max {
            max = score;
            best_move = Some(mv);
        }
    }

    (max, best_move)
}
