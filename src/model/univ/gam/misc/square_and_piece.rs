use crate::model::univ::gam::misc::square::*;
use crate::universe::game::piece::piece::Piece;

/// 升 × 駒
pub struct SquareAndPiece {
    pub square: Square,
    pub piece: Piece,
}
impl SquareAndPiece {
    pub fn new(square1: &Square, piece1: &Piece) -> Self {
        SquareAndPiece {
            square: square1.clone(),
            piece: piece1.clone(),
        }
    }
}
