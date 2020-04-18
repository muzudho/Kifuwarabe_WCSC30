//!
//! 自分相手
//!
//! 先後とは別物
//!

use std::fmt;

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
