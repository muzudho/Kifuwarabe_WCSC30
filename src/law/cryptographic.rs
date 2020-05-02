//!
//! なんか難しいやつはここだぜ☆（＾～＾）
//!
use crate::cosmic::smart::square::AbsoluteAddress;

/*
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
*/

/// ハッシュ値を作る
pub fn push_sq_to_hash(hash: u64, square: Option<&AbsoluteAddress>) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    (hash << 7)
        + if let Some(square_val) = square {
            square_val.address()
        } else {
            0
        } as u64
}
/// ハッシュ値から作る
pub fn pop_sq_from_hash(hash: u64) -> (u64, Option<AbsoluteAddress>) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let adr = AbsoluteAddress::from_absolute_address((hash & 0b111_1111) as usize);
    (hash >> 7, adr)
}

/// 指し手のために、段をアルファベットにすることを想定
pub fn num_to_lower_case(num: usize) -> &'static str {
    const ALPHABETS: [&str; 9] = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
    // 配列の範囲外は強制終了だぜ☆（＾～＾）
    ALPHABETS[num - 1]
}
