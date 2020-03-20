use super::gp_piece_struct_vo::*;
use crate::model::univ::gam::square::*;

/// イミュータブルとして使われる想定なので、ゲッターもセッターも作らないぜ☆（＾～＾）　直接フィールドにアクセスしろだぜ☆（＾～＾）
pub struct GPSquareAndPieceStructVo {
    pub square: Square,
    pub piece_struct: GPPieceStructVo,
}
impl GPSquareAndPieceStructVo {
    pub fn new(square1: &Square, piece_struct1: &GPPieceStructVo) -> Self {
        GPSquareAndPieceStructVo {
            square: square1.clone(),
            piece_struct: piece_struct1.clone(),
        }
    }
}
