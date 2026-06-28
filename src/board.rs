use crate::{
    chess_move::Move,
    move_gen::{gen_legal_moves, gen_legal_moves_for_color},
    piece::{Color, Piece, PieceType},
    position::Position,
};

#[derive(Clone, Copy)]
pub struct Board {
    board: [Option<Piece>; 64],
    side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [None; 64],
            side_to_move: Color::White,
        }
    }

    pub fn starting_position(&mut self) {
        self.board = [None; 64];
        self.side_to_move = Color::White;

        let back_row = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (i, typ) in back_row.iter().enumerate() {
            self.board[i] = Some(Piece {
                typ: (*typ),
                color: (Color::White),
            });
        }

        for i in 8..16 {
            self.board[i] = Some(Piece {
                typ: (PieceType::Pawn),
                color: (Color::White),
            });
        }

        for i in 48..56 {
            self.board[i] = Some(Piece {
                typ: (PieceType::Pawn),
                color: (Color::Black),
            });
        }

        for i in 56..64 {
            self.board[i] = Some(Piece {
                typ: (back_row[i - 56]),
                color: (Color::Black),
            });
        }
    }

    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let i = rank * 8 + file;

                if let Some(val) = self.board[i] {
                    print!("{}", val);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn piece_at(&self, index: usize) -> Option<Piece> {
        self.board[index]
    }

    pub fn set_piece(&mut self, index: usize, piece: Option<Piece>) {
        self.board[index] = piece;
    }

    pub fn make_move(&mut self, chess_move: Move) -> bool {
        let Some(piece) = self.piece_at(chess_move.from.index()) else {
            return false;
        };

        if piece.color != self.side_to_move {
            return false;
        }

        let possible_moves = gen_legal_moves(self, chess_move.from.index());

        if possible_moves.contains(&chess_move) {
            self.apply_move(chess_move);
            self.switch_color();
            return true;
        }
        false
    }

    pub fn apply_move(&mut self, chess_move: Move) {
        let from_index = chess_move.from.index();
        let to_index = chess_move.to.index();

        self.board[to_index] = self.board[from_index];
        self.board[from_index] = None;

        if let Some(typ) = chess_move.promotion
            && let Some(piece) = &mut self.board[to_index]
        {
            piece.typ = typ;
        }
    }

    fn find_king(&self, color: Color) -> usize {
        for (index, square) in self.board.iter().enumerate() {
            if let Some(piece) = square
                && piece.typ == PieceType::King
                && piece.color == color
            {
                return index;
            }
        }
        panic!("No {:?} king found on the board", color);
    }

    fn is_square_attacked(&self, square: usize, attacker: Color) -> bool {
        let Some(target_square) = Position::from_index(square) else {
            return false;
        };

        for i in 0..self.board.len() {
            let Some(piece) = self.piece_at(i) else {
                continue;
            };
            if piece.color != attacker {
                continue;
            }

            let Some(attacker_square) = Position::from_index(i) else {
                continue;
            };

            match piece.typ {
                PieceType::Pawn => {
                    let direction: i32 = match piece.color {
                        Color::White => 1,
                        Color::Black => -1,
                    };

                    let capture_offsets = [(direction, 1), (direction, -1)];

                    for (dr, df) in capture_offsets {
                        if (0..8).contains(&(attacker_square.rank as i32 + dr))
                            && (0..8).contains(&(attacker_square.file as i32 + df))
                        {
                            let capture_square = Position {
                                rank: (attacker_square.rank as i32 + dr) as usize,
                                file: (attacker_square.file as i32 + df) as usize,
                            };

                            if capture_square == target_square {
                                return true;
                            }
                        }
                    }
                }
                PieceType::Knight => {
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
                        if (0..8).contains(&(attacker_square.rank as i32 + offset.0))
                            && (0..8).contains(&(attacker_square.file as i32 + offset.1))
                        {
                            let capture_square = Position {
                                rank: (attacker_square.rank as i32 + offset.0) as usize,
                                file: (attacker_square.file as i32 + offset.1) as usize,
                            };

                            if capture_square == target_square {
                                return true;
                            }
                        }
                    }
                }
                PieceType::Bishop => {
                    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    let rank = attacker_square.rank as i32;
                    let file = attacker_square.file as i32;

                    for direction in directions {
                        let mut current_rank = rank;
                        let mut current_file = file;
                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if (0..8).contains(&current_rank) && (0..8).contains(&current_file) {
                                let current_pos = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                if current_pos == target_square {
                                    return true;
                                }

                                if self.piece_at(current_pos.index()).is_some() {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                PieceType::Rook => {
                    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
                    let rank = attacker_square.rank as i32;
                    let file = attacker_square.file as i32;

                    for direction in directions {
                        let mut current_rank = rank;
                        let mut current_file = file;

                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if (0..8).contains(&current_rank) && (0..8).contains(&current_file) {
                                let current_pos = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                if current_pos == target_square {
                                    return true;
                                }

                                if self.piece_at(current_pos.index()).is_some() {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                PieceType::Queen => {
                    let directions: [(i32, i32); 8] = [
                        (0, 1),
                        (1, 0),
                        (-1, 0),
                        (0, -1),
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                    ];
                    let rank = attacker_square.rank as i32;
                    let file = attacker_square.file as i32;

                    for direction in directions {
                        let mut current_rank = rank;
                        let mut current_file = file;

                        loop {
                            current_rank += direction.0;
                            current_file += direction.1;

                            if (0..8).contains(&current_rank) && (0..8).contains(&current_file) {
                                let current_pos = Position {
                                    rank: current_rank as usize,
                                    file: current_file as usize,
                                };

                                if current_pos == target_square {
                                    return true;
                                }

                                if self.piece_at(current_pos.index()).is_some() {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                PieceType::King => {
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
                        if (0..8).contains(&(attacker_square.rank as i32 + offset.0))
                            && (0..8).contains(&(attacker_square.file as i32 + offset.1))
                        {
                            let capture_square = Position {
                                rank: (attacker_square.rank as i32 + offset.0) as usize,
                                file: (attacker_square.file as i32 + offset.1) as usize,
                            };

                            if capture_square == target_square {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let king_pos = self.find_king(color);
        match color {
            Color::White => {
                if self.is_square_attacked(king_pos, Color::Black) {
                    return true;
                }
            }
            Color::Black => {
                if self.is_square_attacked(king_pos, Color::White) {
                    return true;
                }
            }
        }
        false
    }
    pub fn is_checkmate(&self, color: Color) -> bool {
        let moves = gen_legal_moves_for_color(self, color);
        if moves.is_empty() && self.is_in_check(color) {
            return true;
        }
        false
    }

    pub fn is_stalemate(&self, color: Color) -> bool {
        let moves = gen_legal_moves_for_color(self, color);
        if moves.is_empty() && !self.is_in_check(color) {
            return true;
        }
        false
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    fn switch_color(&mut self) {
        if self.side_to_move == Color::Black {
            self.side_to_move = Color::White;
        } else {
            self.side_to_move = Color::Black;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{Color, Piece, PieceType};

    fn add_kings(board: &mut Board) {
        board.set_piece(
            Position { rank: 0, file: 4 }.index(),
            Some(Piece {
                typ: PieceType::King,
                color: Color::White,
            }),
        );

        board.set_piece(
            Position { rank: 7, file: 4 }.index(),
            Some(Piece {
                typ: PieceType::King,
                color: Color::Black,
            }),
        );
    }

    #[test]
    fn make_move_moves_piece_to_new_square() {
        let mut board = Board::new();
        add_kings(&mut board);
        let from = Position { rank: 1, file: 0 };
        let to = Position { rank: 2, file: 0 };

        board.set_piece(
            from.index(),
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            }),
        );

        assert!(board.make_move(Move {
            from,
            to,
            promotion: None
        }));

        assert!(board.piece_at(from.index()).is_none());

        assert_eq!(
            board.piece_at(to.index()),
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            })
        );
    }

    #[test]
    fn make_move_does_not_move_illegal_move() {
        let mut board = Board::new();
        add_kings(&mut board);

        let from = Position { rank: 1, file: 0 };
        let illegal = Position { rank: 4, file: 4 };

        board.set_piece(
            from.index(),
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            }),
        );

        assert!(!board.make_move(Move {
            from,
            to: illegal,
            promotion: None
        }));

        assert!(board.piece_at(illegal.index()).is_none());
        assert!(board.piece_at(from.index()).is_some());
    }

    #[test]
    fn make_move_captures_enemy_piece() {
        let mut board = Board::new();
        add_kings(&mut board);

        let from = Position { rank: 3, file: 3 };
        let to = Position { rank: 4, file: 4 };

        board.set_piece(
            from.index(),
            Some(Piece {
                typ: PieceType::Bishop,
                color: Color::White,
            }),
        );

        board.set_piece(
            to.index(),
            Some(Piece {
                typ: PieceType::Knight,
                color: Color::Black,
            }),
        );

        assert!(board.make_move(Move {
            from,
            to,
            promotion: None
        }));

        assert!(board.piece_at(from.index()).is_none());

        assert_eq!(
            board.piece_at(to.index()),
            Some(Piece {
                typ: PieceType::Bishop,
                color: Color::White,
            })
        );
    }

    #[test]
    fn make_move_from_empty_square_does_nothing() {
        let mut board = Board::new();
        add_kings(&mut board);

        let from = Position { rank: 3, file: 3 };
        let to = Position { rank: 4, file: 3 };

        assert!(!board.make_move(Move {
            from,
            to,
            promotion: None
        }));

        assert!(board.piece_at(from.index()).is_none());
        assert!(board.piece_at(to.index()).is_none());
    }

    #[test]
    fn square_attacked_by_rook() {
        let mut board = Board::new();

        let rook_square = Position { rank: 0, file: 0 }.index();
        let target = Position { rank: 0, file: 5 }.index();

        board.set_piece(
            rook_square,
            Some(Piece {
                typ: PieceType::Rook,
                color: Color::Black,
            }),
        );

        assert!(board.is_square_attacked(target, Color::Black));
        assert!(!board.is_square_attacked(target, Color::White));
    }

    #[test]
    fn square_not_attacked_when_piece_blocked() {
        let mut board = Board::new();

        let rook_square = Position { rank: 0, file: 0 }.index();
        let blocker = Position { rank: 0, file: 3 }.index();
        let target = Position { rank: 0, file: 5 }.index();

        board.set_piece(
            rook_square,
            Some(Piece {
                typ: PieceType::Rook,
                color: Color::Black,
            }),
        );

        board.set_piece(
            blocker,
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::Black,
            }),
        );

        assert!(!board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn square_attacked_by_knight() {
        let mut board = Board::new();

        let knight_square = Position { rank: 3, file: 3 }.index();
        let target = Position { rank: 5, file: 4 }.index();

        board.set_piece(
            knight_square,
            Some(Piece {
                typ: PieceType::Knight,
                color: Color::White,
            }),
        );

        assert!(board.is_square_attacked(target, Color::White));
    }

    #[test]
    fn empty_square_attacked_by_pawn_diagonal() {
        let mut board = Board::new();

        let pawn_square = Position { rank: 1, file: 3 }.index();
        let target = Position { rank: 2, file: 4 }.index();

        board.set_piece(
            pawn_square,
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            }),
        );

        assert!(board.is_square_attacked(target, Color::White));
    }

    #[test]
    fn pawn_does_not_attack_forward_square() {
        let mut board = Board::new();

        let pawn_square = Position { rank: 1, file: 3 }.index();
        let target = Position { rank: 2, file: 3 }.index();

        board.set_piece(
            pawn_square,
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            }),
        );

        assert!(!board.is_square_attacked(target, Color::White));
    }

    #[test]
    fn black_pawn_attacks_downward_diagonal() {
        let mut board = Board::new();

        let pawn_square = Position { rank: 6, file: 3 }.index();
        let target = Position { rank: 5, file: 4 }.index();

        board.set_piece(
            pawn_square,
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::Black,
            }),
        );

        assert!(board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn white_king_in_check_by_black_rook() {
        let mut board = Board::new();

        let king_square = Position { rank: 0, file: 4 }.index();
        let rook_square = Position { rank: 0, file: 0 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::White,
            }),
        );

        board.set_piece(
            rook_square,
            Some(Piece {
                typ: PieceType::Rook,
                color: Color::Black,
            }),
        );

        assert!(board.is_in_check(Color::White));
    }

    #[test]
    fn white_king_not_in_check_when_rook_blocked() {
        let mut board = Board::new();

        let king_square = Position { rank: 0, file: 4 }.index();
        let rook_square = Position { rank: 0, file: 0 }.index();
        let blocker_square = Position { rank: 0, file: 2 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::White,
            }),
        );

        board.set_piece(
            rook_square,
            Some(Piece {
                typ: PieceType::Rook,
                color: Color::Black,
            }),
        );

        board.set_piece(
            blocker_square,
            Some(Piece {
                typ: PieceType::Knight,
                color: Color::White,
            }),
        );

        assert!(!board.is_in_check(Color::White));
    }

    #[test]
    fn white_king_in_check_by_black_knight() {
        let mut board = Board::new();

        let king_square = Position { rank: 3, file: 3 }.index();
        let knight_square = Position { rank: 5, file: 4 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::White,
            }),
        );

        board.set_piece(
            knight_square,
            Some(Piece {
                typ: PieceType::Knight,
                color: Color::Black,
            }),
        );

        assert!(board.is_in_check(Color::White));
    }

    #[test]
    fn black_king_in_check_by_white_bishop() {
        let mut board = Board::new();

        let king_square = Position { rank: 5, file: 5 }.index();
        let bishop_square = Position { rank: 2, file: 2 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::Black,
            }),
        );

        board.set_piece(
            bishop_square,
            Some(Piece {
                typ: PieceType::Bishop,
                color: Color::White,
            }),
        );

        assert!(board.is_in_check(Color::Black));
    }

    #[test]
    fn white_king_in_check_by_black_pawn() {
        let mut board = Board::new();

        let king_square = Position { rank: 4, file: 4 }.index();
        let pawn_square = Position { rank: 5, file: 3 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::White,
            }),
        );

        board.set_piece(
            pawn_square,
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::Black,
            }),
        );

        assert!(board.is_in_check(Color::White));
    }

    #[test]
    fn make_move_rejects_wrong_color() {
        let mut board = Board::new();

        let from = Position { rank: 6, file: 0 };
        let to = Position { rank: 5, file: 0 };

        board.set_piece(
            from.index(),
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::Black,
            }),
        );

        assert!(!board.make_move(Move {
            from,
            to,
            promotion: None
        }));
        assert!(board.piece_at(from.index()).is_some());
        assert!(board.piece_at(to.index()).is_none());
    }

    #[test]
    fn successful_move_switches_side_to_move() {
        let mut board = Board::new();
        add_kings(&mut board);
        let from = Position { rank: 1, file: 0 };
        let to = Position { rank: 2, file: 0 };

        board.set_piece(
            from.index(),
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            }),
        );

        assert!(board.make_move(Move {
            from,
            to,
            promotion: None
        }));
        assert_eq!(board.side_to_move(), Color::Black);
    }

    #[test]
    fn bishop_does_not_attack_through_piece() {
        let mut board = Board::new();

        let bishop_square = Position { rank: 2, file: 2 }.index();
        let blocker = Position { rank: 3, file: 3 }.index();
        let target = Position { rank: 5, file: 5 }.index();

        board.set_piece(
            bishop_square,
            Some(Piece {
                typ: PieceType::Bishop,
                color: Color::Black,
            }),
        );

        board.set_piece(
            blocker,
            Some(Piece {
                typ: PieceType::Pawn,
                color: Color::White,
            }),
        );

        assert!(!board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn bishop_does_not_attack_straight_square() {
        let mut board = Board::new();

        let bishop_square = Position { rank: 2, file: 2 }.index();
        let target = Position { rank: 2, file: 5 }.index();

        board.set_piece(
            bishop_square,
            Some(Piece {
                typ: PieceType::Bishop,
                color: Color::Black,
            }),
        );

        assert!(!board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn queen_attacks_like_rook() {
        let mut board = Board::new();

        let queen_square = Position { rank: 2, file: 2 }.index();
        let target = Position { rank: 2, file: 6 }.index();

        board.set_piece(
            queen_square,
            Some(Piece {
                typ: PieceType::Queen,
                color: Color::Black,
            }),
        );

        assert!(board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn queen_attacks_like_bishop() {
        let mut board = Board::new();

        let queen_square = Position { rank: 2, file: 2 }.index();
        let target = Position { rank: 5, file: 5 }.index();

        board.set_piece(
            queen_square,
            Some(Piece {
                typ: PieceType::Queen,
                color: Color::Black,
            }),
        );

        assert!(board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn queen_does_not_attack_through_piece() {
        let mut board = Board::new();

        let queen_square = Position { rank: 2, file: 2 }.index();
        let blocker = Position { rank: 4, file: 4 }.index();
        let target = Position { rank: 6, file: 6 }.index();

        board.set_piece(
            queen_square,
            Some(Piece {
                typ: PieceType::Queen,
                color: Color::Black,
            }),
        );

        board.set_piece(
            blocker,
            Some(Piece {
                typ: PieceType::Knight,
                color: Color::White,
            }),
        );

        assert!(!board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn square_attacked_by_king() {
        let mut board = Board::new();

        let king_square = Position { rank: 3, file: 3 }.index();
        let target = Position { rank: 4, file: 4 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::Black,
            }),
        );

        assert!(board.is_square_attacked(target, Color::Black));
    }

    #[test]
    fn king_does_not_attack_two_squares_away() {
        let mut board = Board::new();

        let king_square = Position { rank: 3, file: 3 }.index();
        let target = Position { rank: 5, file: 5 }.index();

        board.set_piece(
            king_square,
            Some(Piece {
                typ: PieceType::King,
                color: Color::Black,
            }),
        );

        assert!(!board.is_square_attacked(target, Color::Black));
    }
}
