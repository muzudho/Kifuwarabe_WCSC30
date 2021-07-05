use crate::genmove::generate_move::PieceEx;
use crate::take1base::Move;

// 投了（＾～＾）
pub const RESIGN_MOVE: Move = 0;

/// 局面の差分だぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub struct MoveCap {
    /// 指し手☆（＾～＾）
    pub move_: Move,
    /// 取った駒☆（＾～＾）
    pub captured: Option<PieceEx>,
}
impl Default for MoveCap {
    /// ゴミ値☆（＾～＾）
    fn default() -> Self {
        MoveCap {
            move_: RESIGN_MOVE,
            captured: None,
        }
    }
}
impl MoveCap {
    pub fn new(mov: Move, cap: Option<PieceEx>) -> Self {
        MoveCap {
            move_: mov,
            captured: cap,
        }
    }
}
