use crate::model::univ::gam::piece::GPPieceVo;
use crate::model::univ::gam::square::*;

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
