use crate::cosmic::game::movement::movement::Movement;
use crate::cosmic::game::piece::piece::Piece;
use crate::cosmic::game::position::person::Person;
use crate::cosmic::game::position::phase::Phase;

/// 手目数。何手目まで指せるか。
/// 棋譜を残す配列のサイズでもある。
/// 大会ルールで 320手が上限なので、終端子として投了を１個入れておけるように +1 する。
pub const PLY_LN: usize = 321;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: i8 = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    pub ply: i16,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_LN],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_LN],
    /// 取った駒
    pub captured_pieces: [Option<Piece>; PLY_LN],
}
impl Default for History {
    fn default() -> History {
        History {
            ply: 0,
            movements: [Movement::default(); PLY_LN],
            position_hashs: [0; PLY_LN],
            /// 取った駒
            captured_pieces: [None; PLY_LN],
        }
    }
}
impl History {
    /// 手番
    pub fn get_phase(&self, person: &Person) -> Phase {
        use crate::cosmic::game::position::person::Person::*;
        match *person {
            // None => Phase::None,
            Friend => {
                // 手番
                if self.ply % 2 == 0 {
                    Phase::First
                } else {
                    Phase::Second
                }
            }
            Opponent => {
                // 相手番
                if self.ply % 2 == 0 {
                    Phase::Second
                } else {
                    Phase::First
                }
            }
        }
    }
}
