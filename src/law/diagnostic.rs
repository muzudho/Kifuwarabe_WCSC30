//!
//! 値チェック
//!
use crate::cosmic::smart::square::AbsoluteAddress;

///
/// 打はテストできない
///
pub fn assert_in_board_as_absolute(ab_adr: &AbsoluteAddress, hint: &str) {
    let adr = ab_adr.address;
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
        "abs-adr=|{}| hint={}",
        adr,
        hint
    );
}

///
/// 打はテストできない
///
pub fn assert_in_board_with_frame_as_absolute(number: i8, hint: &str) {
    debug_assert!(
        (-1 < number && number < 111),
        "abs-adr=|{}| hint={}",
        number,
        hint
    );
}
