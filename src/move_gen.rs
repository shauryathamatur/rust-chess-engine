use crate::piece::Color;
use crate::position::Position;
use crate::{board::Board, chess_move::Move, piece::PieceType};

fn gen_moves(board: &Board, from: usize) -> Vec<Move> {
    let mut moves = Vec::new();

    if let Some(piece) = board.piece_at(from) {
        let color = piece.color;
        match piece.typ {
            PieceType::Knight => {
                if let Some(position) = Position::from_index(from) {
                    let offsets: [(i32, i32); 8] = [
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ];
                    for offset in offsets {
                        let new_rank = position.rank as i32 + offset.0;
                        let new_file = position.file as i32 + offset.1;

                        if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                            let to = Position {
                                rank: new_rank as usize,
                                file: new_file as usize,
                            };

                            moves.push(Move {
                                from: position,
                                to,
                                promotion: None,
                            });
                        }
                    }
                }
            }
            PieceType::Bishop => {
                if let Some(position) = Position::from_index(from) {
                    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    let rank = position.rank;
                    let file = position.file;
                    for direction in directions {
                        let mut current_rank = rank as i32;
                        let mut current_file = file as i32;

                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if !(0..=7).contains(&current_rank) || !(0..=7).contains(&current_file)
                            {
                                break;
                            } else {
                                let target_square = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                match board.piece_at(target_square.index()) {
                                    Some(val) => {
                                        if val.color != color {
                                            moves.push(Move {
                                                from: position,
                                                to: target_square,
                                                promotion: None,
                                            });
                                        }
                                        break;
                                    }
                                    None => moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: None,
                                    }),
                                }
                            }
                        }
                    }
                }
            }
            PieceType::King => {
                if let Some(position) = Position::from_index(from) {
                    let offsets: [(i32, i32); 8] = [
                        (1, 0),
                        (1, 1),
                        (0, 1),
                        (-1, 1),
                        (-1, 0),
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                    ];

                    for offset in offsets {
                        let new_rank = position.rank as i32 + offset.0;
                        let new_file = position.file as i32 + offset.1;

                        if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                            moves.push(Move {
                                from: position,
                                to: Position {
                                    rank: new_rank as usize,
                                    file: new_file as usize,
                                },
                                promotion: None,
                            })
                        }
                    }
                }
            }
            PieceType::Pawn => {
                if let Some(position) = Position::from_index(from) {
                    let direction: i32 = match piece.color {
                        Color::White => 1,
                        Color::Black => -1,
                    };
                    let one_ahead = position.rank as i32 + direction;
                    let two_ahead = position.rank as i32 + direction * 2;

                    if (0..8).contains(&one_ahead) {
                        let target_square = Position {
                            rank: one_ahead as usize,
                            file: position.file,
                        };

                        if board.piece_at(target_square.index()).is_none() {
                            let starting_rank = match piece.color {
                                Color::White => 1,
                                Color::Black => 6,
                            };

                            let final_rank = match piece.color {
                                Color::White => 7,
                                Color::Black => 0,
                            };

                            if target_square.rank == final_rank {
                                let promotion_pieces = [
                                    PieceType::Queen,
                                    PieceType::Rook,
                                    PieceType::Bishop,
                                    PieceType::Knight,
                                ];

                                for typ in promotion_pieces {
                                    moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: Some(typ),
                                    })
                                }
                            } else {
                                moves.push(Move {
                                    from: position,
                                    to: target_square,
                                    promotion: None,
                                })
                            }

                            if position.rank == starting_rank && (0..8).contains(&two_ahead) {
                                let target_square = Position {
                                    rank: two_ahead as usize,
                                    file: position.file,
                                };

                                if board.piece_at(target_square.index()).is_none() {
                                    moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: None,
                                    })
                                }
                            }
                        }
                    }
                    let capture_offsets = [(direction, 1), (direction, -1)];

                    for (dr, df) in capture_offsets {
                        if (0..8).contains(&(position.rank as i32 + dr))
                            && (0..8).contains(&(position.file as i32 + df))
                        {
                            let target_square = Position {
                                rank: (position.rank as i32 + dr) as usize,
                                file: (position.file as i32 + df) as usize,
                            };

                            if let Some(val) = board.piece_at(target_square.index())
                                && piece.color != val.color
                            {
                                let final_rank = match piece.color {
                                    Color::White => 7,
                                    Color::Black => 0,
                                };

                                if target_square.rank == final_rank {
                                    let promotion_pieces = [
                                        PieceType::Queen,
                                        PieceType::Rook,
                                        PieceType::Bishop,
                                        PieceType::Knight,
                                    ];

                                    for typ in promotion_pieces {
                                        moves.push(Move {
                                            from: position,
                                            to: target_square,
                                            promotion: Some(typ),
                                        })
                                    }
                                } else {
                                    moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: None,
                                    })
                                }
                            }
                        }
                    }
                }
            }
            PieceType::Queen => {
                if let Some(position) = Position::from_index(from) {
                    let diag_directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    let lat_directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                    let rank = position.rank;
                    let file = position.file;
                    for direction in diag_directions {
                        let mut current_rank = rank as i32;
                        let mut current_file = file as i32;

                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if !(0..=7).contains(&current_rank) || !(0..=7).contains(&current_file)
                            {
                                break;
                            } else {
                                let target_square = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                match board.piece_at(target_square.index()) {
                                    Some(val) => {
                                        if val.color != color {
                                            moves.push(Move {
                                                from: position,
                                                to: target_square,
                                                promotion: None,
                                            });
                                            break;
                                        } else {
                                            break;
                                        }
                                    }
                                    None => moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: None,
                                    }),
                                }
                            }
                        }
                    }

                    for direction in lat_directions {
                        let mut current_rank = rank as i32;
                        let mut current_file = file as i32;

                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if !(0..=7).contains(&current_rank) || !(0..=7).contains(&current_file)
                            {
                                break;
                            } else {
                                let target_square = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                match board.piece_at(target_square.index()) {
                                    Some(val) => {
                                        if val.color != color {
                                            moves.push(Move {
                                                from: position,
                                                to: target_square,
                                                promotion: None,
                                            });
                                            break;
                                        } else {
                                            break;
                                        }
                                    }
                                    None => moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: None,
                                    }),
                                }
                            }
                        }
                    }
                }
            }
            PieceType::Rook => {
                if let Some(position) = Position::from_index(from) {
                    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                    let rank = position.rank;
                    let file = position.file;
                    for direction in directions {
                        let mut current_rank = rank as i32;
                        let mut current_file = file as i32;

                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if !(0..=7).contains(&current_rank) || !(0..=7).contains(&current_file)
                            {
                                break;
                            } else {
                                let target_square = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                match board.piece_at(target_square.index()) {
                                    Some(val) => {
                                        if val.color != color {
                                            moves.push(Move {
                                                from: position,
                                                to: target_square,
                                                promotion: None,
                                            });
                                            break;
                                        } else {
                                            break;
                                        }
                                    }
                                    None => moves.push(Move {
                                        from: position,
                                        to: target_square,
                                        promotion: None,
                                    }),
                                }
                            }
                        }
                    }
                }
            }
        }
        filter_moves(board, &mut moves, color);
    }
    moves
}

fn filter_moves(board: &Board, moves: &mut Vec<Move>, color: Color) {
    moves.retain(|m| match board.piece_at(m.to.index()) {
        Some(piece) => piece.color != color,
        None => true,
    });
}

pub fn gen_legal_moves(board: &Board, from: usize) -> Vec<Move> {
    let mut legal_moves: Vec<Move> = Vec::new();
    let moves = gen_moves(board, from);
    for chess_move in moves {
        let mut board_copy = *board;

        if let Some(piece) = board_copy.piece_at(chess_move.from.index()) {
            let color = piece.color;
            board_copy.apply_move(chess_move);
            if board_copy.is_in_check(color) {
                continue;
            }

            legal_moves.push(chess_move);
        }
    }

    legal_moves
}

pub fn gen_legal_moves_for_color(board: &Board, color: Color) -> Vec<Move> {
    let mut legal_moves: Vec<Move> = Vec::new();
    for i in 0..64 {
        let Some(piece) = board.piece_at(i) else {
            continue;
        };
        if piece.color == color {
            let moves = gen_legal_moves(board, i);
            for chess_move in moves {
                legal_moves.push(chess_move);
            }
        }
    }

    legal_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    fn move_count(piece: PieceType, rank: usize, file: usize) -> usize {
        let mut board = Board::new();

        let from = Position { rank, file };
        board.set_piece(
            from.index(),
            Some(crate::piece::Piece {
                typ: piece,
                color: Color::White,
            }),
        );

        gen_moves(&board, from.index()).len()
    }

    #[test]
    fn knight_moves() {
        assert_eq!(move_count(PieceType::Knight, 3, 3), 8);
        assert_eq!(move_count(PieceType::Knight, 0, 0), 2);
        assert_eq!(move_count(PieceType::Knight, 0, 1), 3);
    }

    #[test]
    fn bishop_moves() {
        assert_eq!(move_count(PieceType::Bishop, 3, 3), 13);
        assert_eq!(move_count(PieceType::Bishop, 0, 0), 7);
        assert_eq!(move_count(PieceType::Bishop, 0, 3), 7);
    }

    #[test]
    fn rook_moves() {
        assert_eq!(move_count(PieceType::Rook, 3, 3), 14);
        assert_eq!(move_count(PieceType::Rook, 0, 0), 14);
    }

    #[test]
    fn queen_moves() {
        assert_eq!(move_count(PieceType::Queen, 3, 3), 27);
        assert_eq!(move_count(PieceType::Queen, 0, 0), 21);
    }

    #[test]
    fn king_moves() {
        assert_eq!(move_count(PieceType::King, 3, 3), 8);
        assert_eq!(move_count(PieceType::King, 0, 0), 3);
        assert_eq!(move_count(PieceType::King, 0, 3), 5);
    }

    #[test]
    fn pawn_moves() {
        // Starting rank: one-step + two-step
        assert_eq!(move_count(PieceType::Pawn, 1, 3), 2);

        // Not on starting rank: only one-step
        assert_eq!(move_count(PieceType::Pawn, 2, 3), 1);
    }
    #[test]
    fn pawn_cannot_move_forward_if_blocked() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };
        let blocker = Position { rank: 2, file: 3 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );
        board.set_piece(
            blocker.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(!has_move(&moves, 2, 3));
        assert!(!has_move(&moves, 3, 3));
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn pawn_can_capture_diagonally() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };
        let enemy_left = Position { rank: 2, file: 2 };
        let enemy_right = Position { rank: 2, file: 4 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );
        board.set_piece(
            enemy_left.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );
        board.set_piece(
            enemy_right.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 2, 2));
        assert!(has_move(&moves, 2, 4));
    }

    #[test]
    fn pawn_cannot_capture_friendly_pieces() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };
        let friendly_left = Position { rank: 2, file: 2 };
        let friendly_right = Position { rank: 2, file: 4 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );
        board.set_piece(
            friendly_left.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );
        board.set_piece(
            friendly_right.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(!has_move(&moves, 2, 2));
        assert!(!has_move(&moves, 2, 4));
    }

    #[test]
    fn pawn_cannot_move_diagonally_without_capture() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(!has_move(&moves, 2, 2));
        assert!(!has_move(&moves, 2, 4));
    }

    #[test]
    fn pawn_double_move_blocked_if_second_square_blocked() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };
        let blocker = Position { rank: 3, file: 3 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );
        board.set_piece(
            blocker.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 2, 3));
        assert!(!has_move(&moves, 3, 3));
    }
    fn has_move(moves: &[Move], rank: usize, file: usize) -> bool {
        moves.iter().any(|m| m.to == Position { rank, file })
    }

    fn test_piece(piece: PieceType, color: Color) -> crate::piece::Piece {
        crate::piece::Piece { typ: piece, color }
    }

    #[test]
    fn knight_does_not_move_to_friendly_piece() {
        let mut board = Board::new();

        let from = Position { rank: 3, file: 3 };
        let friendly = Position { rank: 5, file: 4 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Knight, Color::White)),
        );
        board.set_piece(
            friendly.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(!has_move(&moves, 5, 4));
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn knight_can_capture_enemy_piece() {
        let mut board = Board::new();

        let from = Position { rank: 3, file: 3 };
        let enemy = Position { rank: 5, file: 4 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Knight, Color::White)),
        );
        board.set_piece(
            enemy.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 5, 4));
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn bishop_stops_before_friendly_piece() {
        let mut board = Board::new();

        let from = Position { rank: 3, file: 3 };
        let friendly = Position { rank: 5, file: 5 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Bishop, Color::White)),
        );
        board.set_piece(
            friendly.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 4, 4));
        assert!(!has_move(&moves, 5, 5));
        assert!(!has_move(&moves, 6, 6));
    }

    #[test]
    fn bishop_can_capture_enemy_but_stops_after() {
        let mut board = Board::new();

        let from = Position { rank: 3, file: 3 };
        let enemy = Position { rank: 5, file: 5 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Bishop, Color::White)),
        );
        board.set_piece(
            enemy.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 4, 4));
        assert!(has_move(&moves, 5, 5));
        assert!(!has_move(&moves, 6, 6));
    }

    #[test]
    fn rook_stops_before_friendly_piece() {
        let mut board = Board::new();

        let from = Position { rank: 3, file: 3 };
        let friendly = Position { rank: 3, file: 5 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Rook, Color::White)),
        );
        board.set_piece(
            friendly.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 3, 4));
        assert!(!has_move(&moves, 3, 5));
        assert!(!has_move(&moves, 3, 6));
    }

    #[test]
    fn rook_can_capture_enemy_but_stops_after() {
        let mut board = Board::new();

        let from = Position { rank: 3, file: 3 };
        let enemy = Position { rank: 3, file: 5 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Rook, Color::White)),
        );
        board.set_piece(
            enemy.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_move(&moves, 3, 4));
        assert!(has_move(&moves, 3, 5));
        assert!(!has_move(&moves, 3, 6));
    }

    #[test]
    fn legal_moves_allow_normal_move_when_king_safe() {
        let mut board = Board::new();

        let king = Position { rank: 0, file: 4 };
        let knight = Position { rank: 3, file: 3 };

        board.set_piece(
            king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            knight.index(),
            Some(test_piece(PieceType::Knight, Color::White)),
        );

        let moves = gen_legal_moves(&board, knight.index());

        assert!(has_move(&moves, 5, 4));
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn legal_moves_filter_out_moves_that_leave_king_in_check() {
        let mut board = Board::new();

        let king = Position { rank: 0, file: 4 };
        let rook = Position { rank: 1, file: 4 };
        let enemy_rook = Position { rank: 7, file: 4 };

        board.set_piece(
            king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            rook.index(),
            Some(test_piece(PieceType::Rook, Color::White)),
        );
        board.set_piece(
            enemy_rook.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );

        let moves = gen_legal_moves(&board, rook.index());

        assert!(has_move(&moves, 2, 4));
        assert!(has_move(&moves, 7, 4));

        assert!(!has_move(&moves, 1, 0));
        assert!(!has_move(&moves, 1, 7));
    }

    #[test]
    fn legal_moves_can_capture_checking_piece() {
        let mut board = Board::new();

        let king = Position { rank: 0, file: 4 };
        let rook = Position { rank: 1, file: 7 };
        let enemy_rook = Position { rank: 0, file: 7 };

        board.set_piece(
            king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            rook.index(),
            Some(test_piece(PieceType::Rook, Color::White)),
        );
        board.set_piece(
            enemy_rook.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );

        let moves = gen_legal_moves(&board, rook.index());

        assert!(has_move(&moves, 0, 7));
    }
    #[test]
    fn legal_moves_filter_king_moving_into_check() {
        let mut board = Board::new();

        let king = Position { rank: 0, file: 4 };
        let enemy_rook = Position { rank: 7, file: 5 };

        board.set_piece(
            king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            enemy_rook.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );

        let moves = gen_legal_moves(&board, king.index());

        assert!(!has_move(&moves, 0, 5));
        assert!(!has_move(&moves, 1, 5));
    }

    #[test]
    fn pinned_piece_can_only_move_along_pin_line() {
        let mut board = Board::new();

        let king = Position { rank: 0, file: 4 };
        let bishop = Position { rank: 1, file: 4 };
        let enemy_rook = Position { rank: 7, file: 4 };

        board.set_piece(
            king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            bishop.index(),
            Some(test_piece(PieceType::Bishop, Color::White)),
        );
        board.set_piece(
            enemy_rook.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );

        let moves = gen_legal_moves(&board, bishop.index());

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn starting_position_has_twenty_legal_white_moves() {
        let mut board = Board::new();
        board.starting_position();

        let mut total = 0;

        for i in 0..64 {
            if let Some(piece) = board.piece_at(i)
                && piece.color == Color::White
            {
                total += gen_legal_moves(&board, i).len();
            }
        }

        assert_eq!(total, 20);
    }

    #[test]
    fn starting_position_white_has_20_legal_moves() {
        let mut board = Board::new();
        board.starting_position();

        let moves = gen_legal_moves_for_color(&board, Color::White);

        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn starting_position_black_has_20_legal_moves() {
        let mut board = Board::new();
        board.starting_position();

        let moves = gen_legal_moves_for_color(&board, Color::Black);

        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn legal_moves_for_color_only_returns_that_colors_moves() {
        let mut board = Board::new();

        let white_king = Position { rank: 0, file: 4 };
        let black_king = Position { rank: 7, file: 4 };
        let white_knight = Position { rank: 3, file: 3 };
        let black_knight = Position { rank: 4, file: 4 };

        board.set_piece(
            white_king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            black_king.index(),
            Some(test_piece(PieceType::King, Color::Black)),
        );
        board.set_piece(
            white_knight.index(),
            Some(test_piece(PieceType::Knight, Color::White)),
        );
        board.set_piece(
            black_knight.index(),
            Some(test_piece(PieceType::Knight, Color::Black)),
        );

        let moves = gen_legal_moves_for_color(&board, Color::White);

        assert!(
            moves
                .iter()
                .all(|m| { board.piece_at(m.from.index()).unwrap().color == Color::White })
        );

        assert!(moves.iter().any(|m| m.from == white_knight));
        assert!(!moves.iter().any(|m| m.from == black_knight));
    }

    #[test]
    fn legal_moves_for_color_respects_check() {
        let mut board = Board::new();

        let white_king = Position { rank: 0, file: 4 };
        let black_king = Position { rank: 7, file: 4 };
        let white_rook = Position { rank: 1, file: 4 };
        let black_rook = Position { rank: 7, file: 0 };

        board.set_piece(
            white_king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            black_king.index(),
            Some(test_piece(PieceType::King, Color::Black)),
        );
        board.set_piece(
            white_rook.index(),
            Some(test_piece(PieceType::Rook, Color::White)),
        );
        board.set_piece(
            black_rook.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );

        let moves = gen_legal_moves_for_color(&board, Color::White);

        assert!(moves.iter().all(|m| {
            let mut board_copy = board;
            board_copy.make_move(*m);
            !board_copy.is_in_check(Color::White)
        }));
    }

    #[test]
    fn checkmated_color_has_zero_legal_moves() {
        let mut board = Board::new();

        let white_king = Position { rank: 0, file: 0 };
        let black_king = Position { rank: 7, file: 7 };
        let black_rook = Position { rank: 0, file: 1 };
        let black_queen = Position { rank: 1, file: 1 };

        board.set_piece(
            white_king.index(),
            Some(test_piece(PieceType::King, Color::White)),
        );
        board.set_piece(
            black_king.index(),
            Some(test_piece(PieceType::King, Color::Black)),
        );
        board.set_piece(
            black_rook.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );
        board.set_piece(
            black_queen.index(),
            Some(test_piece(PieceType::Queen, Color::Black)),
        );

        let moves = gen_legal_moves_for_color(&board, Color::White);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn white_pawn_forward_promotion_generates_four_moves() {
        let mut board = Board::new();

        let from = Position { rank: 6, file: 3 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert_eq!(moves.len(), 4);
        assert!(has_promotion(&moves, 7, 3, PieceType::Queen));
        assert!(has_promotion(&moves, 7, 3, PieceType::Rook));
        assert!(has_promotion(&moves, 7, 3, PieceType::Bishop));
        assert!(has_promotion(&moves, 7, 3, PieceType::Knight));
    }

    #[test]
    fn black_pawn_forward_promotion_generates_four_moves() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert_eq!(moves.len(), 4);
        assert!(has_promotion(&moves, 0, 3, PieceType::Queen));
        assert!(has_promotion(&moves, 0, 3, PieceType::Rook));
        assert!(has_promotion(&moves, 0, 3, PieceType::Bishop));
        assert!(has_promotion(&moves, 0, 3, PieceType::Knight));
    }

    #[test]
    fn white_pawn_promotion_capture_generates_four_moves() {
        let mut board = Board::new();

        let from = Position { rank: 6, file: 3 };
        let enemy = Position { rank: 7, file: 4 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );
        board.set_piece(
            enemy.index(),
            Some(test_piece(PieceType::Rook, Color::Black)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(has_promotion(&moves, 7, 4, PieceType::Queen));
        assert!(has_promotion(&moves, 7, 4, PieceType::Rook));
        assert!(has_promotion(&moves, 7, 4, PieceType::Bishop));
        assert!(has_promotion(&moves, 7, 4, PieceType::Knight));
    }

    #[test]
    fn normal_pawn_move_has_no_promotion() {
        let mut board = Board::new();

        let from = Position { rank: 1, file: 3 };

        board.set_piece(
            from.index(),
            Some(test_piece(PieceType::Pawn, Color::White)),
        );

        let moves = gen_moves(&board, from.index());

        assert!(moves.iter().all(|m| m.promotion.is_none()));
    }

    fn has_promotion(moves: &[Move], rank: usize, file: usize, promotion: PieceType) -> bool {
        moves
            .iter()
            .any(|m| m.to == Position { rank, file } && m.promotion == Some(promotion))
    }
}
