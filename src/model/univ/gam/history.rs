use crate::model::univ::gam::movement::Movement;
use crate::model::univ::gam::piece::Piece;
use crate::model::vo::other_part::op_ply_vo::PLY_LN;

pub struct History {
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_LN],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_LN],
    /// 取った駒
    pub captured_pieces: [Piece; PLY_LN],
}
impl Default for History {
    fn default() -> History {
        History {
            movements: [Movement::default(); PLY_LN],
            position_hashs: [0; PLY_LN],
            /// 取った駒
            captured_pieces: [Piece::NonePiece; PLY_LN],
        }
    }
}
impl History {
    /*
    pub fn get_moves_history(&self) -> &[Movement; PLY_LN] {
        &self.movements
    }
    */
}
