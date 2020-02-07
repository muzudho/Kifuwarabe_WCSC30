//!
//! 先後
//!

use super::super::super::controller::common::conv::*;
use std::fmt;

pub const SN_LN: usize = 3;
/**
 * 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
 * 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
 */
#[derive(Clone)]
pub enum Phase {
    Sen,
    Go,
    // 空升の先後を調べようとした場合等
    Owari,
}
pub const SN_SEN: usize = 0;
pub const SN_GO: usize = 1;
/**
 * 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
 */
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::Phase::*;
        match *self {
            Sen => write!(f, "▼"),
            Go => write!(f, "△"),
            Owari => write!(f, "×"),
        }
    }
}

/**
 * 先後の一致比較
 */
pub fn match_sn(a: &Phase, b: &Phase) -> bool {
    sn_to_num(a) == sn_to_num(b)
}

pub const SN_ARRAY_LN: usize = 2;
pub const SN_ARRAY: [Phase; SN_ARRAY_LN] = [Phase::Sen, Phase::Go];
