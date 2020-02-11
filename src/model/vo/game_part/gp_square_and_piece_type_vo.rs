use super::gp_piece_type_vo::GPPieceTypeVo;
use super::gp_square_vo::*;

/// 升 × 駒種類
pub struct GPSquareAndPieceTypeVo {
    pub square: Square,
    pub piece_type: GPPieceTypeVo,
}
impl GPSquareAndPieceTypeVo {
    pub fn new(square1: &Square, piece_type1: &GPPieceTypeVo) -> Self {
        GPSquareAndPieceTypeVo {
            square: square1.clone(),
            piece_type: piece_type1.clone(),
        }
    }
}
