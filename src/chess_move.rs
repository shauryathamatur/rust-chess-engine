use crate::position::Position;

#[derive(Clone, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}
