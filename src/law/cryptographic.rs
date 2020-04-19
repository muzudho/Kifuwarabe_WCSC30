//!
//! なんか難しいやつはここだぜ☆（＾～＾）
//!
use crate::cosmic::smart::square::{AbsoluteAddress, Address};

/// 0 なら偽、それ以外は真☆（＾～＾）
pub fn num_to_bool(n: usize) -> bool {
    match n {
        0 => false,
        _ => true,
    }
}
/// ハッシュ値を作る
pub fn push_bool_to_hash(hash: u64, b: bool) -> u64 {
    // bool は i32 だが、hash は u64 なので u64 に合わせるぜ☆（*＾～＾*）
    (hash << 7) + b as u64
}
/// ハッシュ値から作る
pub fn pop_bool_from_hash(hash: u64) -> (u64, bool) {
    let b_num = num_to_bool((hash & 0b1) as usize);
    (hash >> 7, b_num)
}

/// ハッシュ値を作る
pub fn push_sq_to_hash(hash: u64, square: &AbsoluteAddress) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    (hash << 7) + square.address() as u64
}
/// ハッシュ値から作る
pub fn pop_sq_from_hash(hash: u64) -> (u64, AbsoluteAddress) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let sq_num = Address::from_absolute_address((hash & 0b111_1111) as i8);
    (hash >> 7, sq_num)
}

/// 指し手のために、段をアルファベットにすることを想定
pub fn num_to_lower_case(num: i8) -> &'static str {
    match num {
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        9 => "i",
        _ => "?", // 返却型が &'static str なので、エラー値を動的に作れない
    }
}
