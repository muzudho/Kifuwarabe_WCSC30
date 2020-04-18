//!
//! Position (局面),
//! Phase (先後。手番,相手番),
//! Person (先手,後手).
//!

use crate::cosmic::shogi::recording::Movement;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::cosmic::toy_box::Board;
use std::fmt;

/// ミュータブルなオブジェクト☆（＾～＾）
pub struct Position {
    /// 現局面
    pub current_board: Board,

    /// 現在の指し手を作成中。
    pub current_movement_builder: Movement,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            // 現局面
            current_board: Board::default(),
            /// 現在の指し手を作成中。
            current_movement_builder: Movement::default(),
        }
    }
}
impl Position {
    pub fn set_current_movement_source_temporary(&mut self, src: &AbsoluteAddress) {
        self.current_movement_builder.source = src.clone()
    }
    pub fn set_current_movement_destination_temporary(&mut self, dst: &AbsoluteAddress) {
        self.current_movement_builder.destination = dst.clone()
    }
    pub fn set_current_movement_promote_temporary(&mut self, pro: bool) {
        self.current_movement_builder.promote = pro
    }
    pub fn set_current_movement_drop_temporary(&mut self, piece_type: Option<PieceType>) {
        self.current_movement_builder.drop = piece_type
    }
}

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    First,
    Second,
}
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
/*
impl Phase {
    pub fn turn(&self) -> Phase {
        use self::Phase::*;
        match self {
            First => Second,
            Second => First,
        }
    }
}
*/

/*
pub const PHASE_ARRAY_LN: usize = 2;
pub const PHASE_ARRAY: [Phase; PHASE_ARRAY_LN] = [Phase::First, Phase::Second];

pub fn phase_to_num(phase: Phase) -> usize {
    use self::Phase::*;
    match phase {
        First => PHASE_FIRST,
        Second => PHASE_SECOND,
    }
}
*/

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

/*
pub fn turn_person(person: &Person) -> Person {
    use self::Person::*;
    match *person {
        Friend => _Opponent,
        _Opponent => Friend,
    }
}
*/
