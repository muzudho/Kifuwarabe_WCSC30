//!
//! Phase. (先後)
//!

use std::fmt;

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, PartialEq)]
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

pub const PHASE_ARRAY_LN: usize = 2;
pub const PHASE_ARRAY: [Phase; PHASE_ARRAY_LN] = [Phase::First, Phase::Second];

pub fn phase_to_num(phase: &Phase) -> usize {
    use self::Phase::*;
    match *phase {
        First => PHASE_FIRST,
        Second => PHASE_SECOND,
    }
}
pub fn turn_phase(phase: &Phase) -> Phase {
    use self::Phase::*;
    match *phase {
        First => Second,
        Second => First,
    }
}
