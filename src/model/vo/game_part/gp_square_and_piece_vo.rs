use super::super::super::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::vo::other_part::op_square_vo::*;

/// 升 × 駒
pub struct GPSquareAndPieceVo {
    pub square: Square,
    pub piece: GPPieceVo,
}
impl GPSquareAndPieceVo {
    pub fn new(square1: &Square, piece1: &GPPieceVo) -> Self {
        GPSquareAndPieceVo {
            square: square1.clone(),
            piece: piece1.clone(),
        }
    }
}
