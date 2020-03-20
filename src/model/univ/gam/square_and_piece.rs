use crate::model::univ::gam::piece::Piece;
use crate::model::univ::gam::square::*;

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
