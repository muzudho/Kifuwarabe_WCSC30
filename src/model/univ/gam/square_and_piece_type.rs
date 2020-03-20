use crate::model::univ::gam::piece_type::PieceType;
use crate::model::univ::gam::square::*;

/// 升 × 駒種類
pub struct SquareAndPieceType {
    pub square: Square,
    pub piece_type: PieceType,
}
impl SquareAndPieceType {
    pub fn new(square1: &Square, piece_type1: &PieceType) -> Self {
        SquareAndPieceType {
            square: square1.clone(),
            piece_type: piece_type1.clone(),
        }
    }
}
