//!
//! 自分相手
//!
//! 先後とは別物

use super::super::super::super::controller::common::conv::*;
use std::fmt;

pub const JIAI_LN: usize = 3;
///
/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
///
pub enum Person {
    Ji,
    Ai,
    Owari,
}
pub const JIAI_JI: usize = 0;
pub const JIAI_AI: usize = 1;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Person::*;
        match *self {
            Ji => write!(f, "自"),
            Ai => write!(f, "相"),
            Owari => write!(f, "×"),
        }
    }
}
/**
 * 一致比較
 */
pub fn match_jiai(a: &Person, b: &Person) -> bool {
    jiai_to_num(a) == jiai_to_num(b)
}

pub const JIAI_ARRAY_LN: usize = 2;
pub const JIAI_ARRAY: [Person; JIAI_ARRAY_LN] = [Person::Ji, Person::Ai];
