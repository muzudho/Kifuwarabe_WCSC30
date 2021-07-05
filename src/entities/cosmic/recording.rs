//!
//! * History (棋譜)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::movegen::PieceEx;
use crate::record::RESIGN_MOVE;
use crate::take1base::Move;
use std::fmt;

/// 手目数。何手目まで指せるか。
/// 棋譜を残す配列のサイズでもある。
/// 大会ルールで 320手が上限なので、終端子として投了を１個入れておけるように +1 する。
pub const PLY_LEN: usize = 321;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: isize = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    pub ply: isize,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub moves: [Move; PLY_LEN],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_LEN],
    /// 取った駒
    pub captured_pieces: [Option<PieceEx>; PLY_LEN],
}
impl Default for History {
    fn default() -> History {
        History {
            ply: 0,
            moves: [RESIGN_MOVE; PLY_LEN],
            position_hashs: [0; PLY_LEN],
            /// 取った駒
            captured_pieces: [None; PLY_LEN],
        }
    }
}
impl History {
    pub fn clear(&mut self) {
        self.ply = 0;
        self.moves = [RESIGN_MOVE; PLY_LEN];
        self.position_hashs = [0; PLY_LEN];
        // 取った駒
        self.captured_pieces = [None; PLY_LEN];
    }

    /// 手番
    pub fn get_phase(&self) -> Phase {
        // 手番
        if self.ply % 2 == 0 {
            Phase::First
        } else {
            Phase::Second
        }
    }
}

/*
///
/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
///
/// #[derive(PartialEq)]
pub enum Person {
    Friend,
    _Opponent,
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Person::*;
        match *self {
            Friend => write!(f, "Fr"),
            _Opponent => write!(f, "Op"),
        }
    }
}
*/
/*
pub fn turn_person(person: &Person) -> Person {
    use self::Person::*;
    match *person {
        Friend => Opponent,
        Opponent => Friend,
    }
}
*/

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LEN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    First,
    Second,
}
// impl Phase {
//     pub fn turn(self) -> Phase {
//         use self::Phase::*;
//         match self {
//             First => Second,
//             Second => First,
//         }
//     }
// }
/// 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // Windows Terminal では ▲、▽が半角サイズで表示されるので、それに合わせている☆（＾～＾） Microsoft 製品に最適化していいのか知らないが……☆（＾～＾）
        use self::Phase::*;
        match *self {
            First => write!(f, " ▲"),
            Second => write!(f, " ▽"),
        }
    }
}
