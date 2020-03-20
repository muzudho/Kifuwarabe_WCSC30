use crate::model::univ::gam::misc::piece_struct::*;
use crate::model::univ::gam::misc::square::*;

/// イミュータブルとして使われる想定なので、ゲッターもセッターも作らないぜ☆（＾～＾）　直接フィールドにアクセスしろだぜ☆（＾～＾）
pub struct SquareAndPieceStruct {
    pub square: Square,
    pub piece_struct: PieceStruct,
}
impl SquareAndPieceStruct {
    pub fn new(square1: &Square, piece_struct1: &PieceStruct) -> Self {
        SquareAndPieceStruct {
            square: square1.clone(),
            piece_struct: piece_struct1.clone(),
        }
    }
}
