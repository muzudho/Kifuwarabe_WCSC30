//!
//! 値チェック
//!
use crate::model::univ::gam::misc::square::isquare;

///
/// 打はテストできない
///
pub fn assert_in_board(adr: isquare, hint: &str) {
    debug_assert!(
        (10 < adr && adr < 20)
            || (20 < adr && adr < 30)
            || (30 < adr && adr < 40)
            || (40 < adr && adr < 50)
            || (50 < adr && adr < 60)
            || (60 < adr && adr < 70)
            || (70 < adr && adr < 80)
            || (80 < adr && adr < 90)
            || (90 < adr && adr < 100),
        "adr=|{}| hint={}",
        adr,
        hint
    );
}

///
/// 打はテストできない
///
pub fn assert_in_board_with_frame(adr: isquare, hint: &str) {
    debug_assert!((-1 < adr && adr < 111), "adr=|{}| hint={}", adr, hint);
}
