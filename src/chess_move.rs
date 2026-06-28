use crate::piece::PieceType;
use crate::position::Position;

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<PieceType>,
}
