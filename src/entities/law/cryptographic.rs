//!
//! なんか難しいやつはここだぜ☆（＾～＾）
//!
use crate::position::Square;

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
pub fn push_sq_to_hash(hash: u64, sq: Square) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    if sq.is_square() {
        (hash << 7) + (sq.number() as u64)
    } else {
        panic!("push_sq_to_hash fail")
    }
}
/// ハッシュ値から作る
pub fn pop_sq_from_hash(hash: u64) -> (u64, Square) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let sq = Square::new((hash & 0b111_1111) as u8);
    if sq.is_square() {
        (hash >> 7, sq)
    } else {
        panic!("pop_sq_from_hash fail")
    }
}

/// 指し手のために、段をアルファベットにすることを想定
pub fn num_to_lower_case(num: usize) -> &'static str {
    const ALPHABETS: [&str; 9] = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
    // 配列の範囲外は強制終了だぜ☆（＾～＾）
    ALPHABETS[num - 1]
}
