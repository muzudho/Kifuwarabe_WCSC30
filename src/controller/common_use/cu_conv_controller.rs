//!
//! 変換
//!
#![allow(dead_code)]
use super::super::super::controller::common_use::cu_geo_teigi_controller::*;
use super::super::super::model::vo::other_part::op_direction_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::model::vo::other_part::op_phase_vo::Phase;
use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection;
use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType;
use super::super::super::model::vo::other_part::op_square_vo::*;

/**********
 * 論理値 *
 **********/
/**
 * false => 0
 * true => 1
 *
 * bool は i32 だが、_to_num 系は usize を返すように合わせるぜ☆（*＾～＾*）
 */
#[allow(dead_code)]
pub fn bool_to_num(b: bool) -> usize {
    b as usize
}
/**
 * 0 なら偽、それ以外は真☆（＾～＾）
 */
#[allow(dead_code)]
pub fn num_to_bool(n: usize) -> bool {
    match n {
        0 => false,
        _ => true,
    }
}
/**
 * ハッシュ値を作る
 */
#[allow(dead_code)]
pub fn push_bool_to_hash(hash: u64, b: bool) -> u64 {
    // bool は i32 だが、hash は u64 なので u64 に合わせるぜ☆（*＾～＾*）
    (hash << 7) + b as u64
}
/**
 * ハッシュ値から作る
 */
#[allow(dead_code)]
pub fn pop_bool_from_hash(hash: u64) -> (u64, bool) {
    let b_num = num_to_bool((hash & 0b1) as usize);
    (hash >> 7, b_num)
}

/*********
 * 4角度 *
 *********/

/*********
 * 8方向 *
 *********/
#[allow(dead_code)]
pub fn dir8_to_num(dir: &Dir8) -> usize {
    use super::super::super::model::vo::other_part::op_direction_vo::Dir8::*;
    match *dir {
        E => 0,
        NE => 1,
        N => 2,
        NW => 3,
        W => 4,
        SW => 5,
        S => 6,
        SE => 7,
        Owari => 8,
    }
}
#[allow(dead_code)]
pub fn num_to_dir8(n: usize) -> Dir8 {
    use super::super::super::model::vo::other_part::op_direction_vo::Dir8::*;
    match n {
        0 => E,
        1 => NE,
        2 => N,
        3 => NW,
        4 => W,
        5 => SW,
        6 => S,
        7 => SE,
        _ => Owari,
    }
}
/**
 * ハッシュ値を作る
 */
#[allow(dead_code)]
pub fn push_dir8_to_hash(hash: u64, dir: &Dir8) -> u64 {
    // エラー値含めて 9bit あるので 2^5
    (hash << 5) + dir8_to_num(dir) as u64
}
/**
 * ハッシュ値から作る
 */
#[allow(dead_code)]
pub fn pop_dir8_from_hash(hash: u64) -> (u64, Dir8) {
    // エラー値含めて 9bit あるので 2^5
    let dir = num_to_dir8((hash & 0b11111) as usize);
    (hash >> 5, dir)
}

/********
 * 先後 *
 ********/
pub fn sn_to_num(sn: &Phase) -> usize {
    use super::super::super::model::vo::other_part::op_phase_vo::Phase::*;
    match *sn {
        Sen => 0,
        Go => 1,
        Owari => 2,
    }
}
pub fn hanten_sn(sn: &Phase) -> Phase {
    use super::super::super::model::vo::other_part::op_phase_vo::Phase::*;
    match *sn {
        Sen => Go,
        Go => Sen,
        Owari => Owari,
    }
}

/************
 * 自分相手 *
 ************/
pub fn jiai_to_num(jiai: &Person) -> usize {
    use super::super::super::model::vo::other_part::op_person_vo::Person::*;
    match *jiai {
        Ji => 0,
        Ai => 1,
        Owari => 2,
    }
}
pub fn hanten_jiai(jiai: &Person) -> Person {
    use super::super::super::model::vo::other_part::op_person_vo::Person::*;
    match *jiai {
        Ji => Ai,
        Ai => Ji,
        Owari => Owari,
    }
}

/******************
 * 盤、升、筋、段 *
 ******************/

pub fn p_in_ban(p: &Point) -> bool {
    (SUJI_0 < p.x && p.x < SUJI_10) && (DAN_0 < p.y && p.y < DAN_10)
}
/// ハッシュ値を作る
pub fn push_sq_to_hash(hash: u64, sq: &Square) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    (hash << 7) + sq.to_umasu() as u64
}
/// ハッシュ値から作る
pub fn pop_sq_from_hash(hash: u64) -> (u64, Square) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let sq_num = Square::from_umasu((hash & 0b1111111) as umasu);
    (hash >> 7, sq_num)
}

/**
 * 指し手のために、段をアルファベットにすることを想定
 */
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
/****************************************************
 * 先手であれば、後手のように番号を振った座標に変換 *
 ****************************************************/
pub fn kaiten180_sq_by_sq_sn(sq: &Square, sn: &Phase) -> Square {
    use super::super::super::model::vo::other_part::op_phase_vo::Phase::*;
    match *sn {
        Sen => Square::from_umasu(BAN_MAX - sq.to_umasu() + BAN_MIN),
        _ => (*sq).clone(),
    }
}

/**********
 * 駒種類 *
 **********/

/**
 * 駒種類の数値化
 */
pub fn kms_to_num(kms: &PieceType) -> usize {
    use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType::*;
    match *kms {
        R => 0,
        K => 1,
        Z => 2,
        I => 3,
        N => 4,
        U => 5,
        S => 6,
        H => 7,
        PK => 8,
        PZ => 9,
        PN => 10,
        PU => 11,
        PS => 12,
        PH => 13,
        Kara => 14,
        Owari => 15,
    }
}
/**
 * 数値の駒種類化
 */
pub fn num_to_kms(n: usize) -> PieceType {
    use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType::*;
    match n {
        0 => R,
        1 => K,
        2 => Z,
        3 => I,
        4 => N,
        5 => U,
        6 => S,
        7 => H,
        8 => PK,
        9 => PZ,
        10 => PN,
        11 => PU,
        12 => PS,
        13 => PH,
        14 => Kara,
        _ => Owari,
    }
}
/**
 * ハッシュ値を作る
 */
pub fn push_kms_to_hash(hash: u64, kms: &PieceType) -> u64 {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    (hash << 4) + kms_to_num(kms) as u64
}
/**
 * ハッシュ値から作る
 */
pub fn pop_kms_from_hash(hash: u64) -> (u64, PieceType) {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    let kms_num = num_to_kms((hash & 0b1111) as usize);
    (hash >> 4, kms_num)
}
/**
 * 成れる駒
 */
pub fn kms_can_pro(kms: &PieceType) -> bool {
    use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType::*;
    match *kms {
        R => false,
        K => true,
        Z => true,
        I => false,
        N => true,
        U => true,
        S => true,
        H => true,
        PK => false,
        PZ => false,
        PN => false,
        PU => false,
        PS => false,
        PH => false,
        Kara => false,
        Owari => false,
    }
}
/**
 * 打てる駒
 */
pub fn kms_can_da(kms: &PieceType) -> bool {
    use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType::*;
    match *kms {
        R => false,
        K => true,
        Z => true,
        I => true,
        N => true,
        U => true,
        S => true,
        H => true,
        PK => false,
        PZ => false,
        PN => false,
        PU => false,
        PS => false,
        PH => false,
        Kara => false,
        Owari => false,
    }
}

/************
 * 駒の動き *
 ************/

/**
 * 上下反転
 */
pub fn hanten_kmdir_joge(kmdir: &PieceDirection) -> PieceDirection {
    use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
    match *kmdir {
        // 東
        E(b) => E(b),
        // 北東
        NE(b) => SE(b),
        // 北北東（桂馬が戻る動き）
        NNE => SSE,
        // 北
        N(b) => S(b),
        // 北北西（桂馬が戻る動き）
        NNW => SSW,
        // 北西
        NW(b) => SW(b),
        // 西
        W(b) => W(b),
        // 南西
        SW(b) => NW(b),
        // 南南西（桂馬の動き）
        SSW => NNW,
        // 南
        S(b) => N(b),
        // 南南東（桂馬の動き）
        SSE => NNE,
        // 南東
        SE(b) => NE(b),
        // 要素数より1小さい数。エラー値用に使っても可
        Owari => Owari,
    }
}
/*
pub fn kmdir_id(kmdir:&PieceDirection) -> usize{
    use teigi::shogi_syugo::PieceDirection::*;
    match *kmdir {
        E  (b)=>if b { 0}else{ 1},
        NE (b)=>if b { 2}else{ 3},
        N  (b)=>if b { 4}else{ 5},
        NW (b)=>if b { 6}else{ 7},
        W  (b)=>if b { 8}else{ 9},
        SW (b)=>if b {10}else{11},
        SSW   =>12,
        S  (b)=>if b {13}else{14},
        SSE   =>15,
        SE (b)=>if b {16}else{17},
        Owari =>18,
    }
}
*/
