//!
//! 変換
//!
#![allow(dead_code)]
use super::super::super::controller::consoles::asserts::*;
use super::super::super::controller::geometries::geo_teigi::*;
use super::super::super::model::master::direction::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::Phase;
use super::super::super::model::master::piece_direction::PieceDirection;
use super::super::super::model::master::piece_type::PieceType;
use super::super::super::model::master::place::*;
use super::super::super::model::master::square::*;

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
    use super::super::super::model::master::direction::Dir8::*;
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
    use super::super::super::model::master::direction::Dir8::*;
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
    use super::super::super::model::master::phase::Phase::*;
    match *sn {
        Sen => 0,
        Go => 1,
        Owari => 2,
    }
}
pub fn hanten_sn(sn: &Phase) -> Phase {
    use super::super::super::model::master::phase::Phase::*;
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
    use super::super::super::model::master::person::Person::*;
    match *jiai {
        Ji => 0,
        Ai => 1,
        Owari => 2,
    }
}
pub fn hanten_jiai(jiai: &Person) -> Person {
    use super::super::super::model::master::person::Person::*;
    match *jiai {
        Ji => Ai,
        Ai => Ji,
        Owari => Owari,
    }
}

/******************
 * 盤、升、筋、段 *
 ******************/

/**
 * umasu は 将棋盤座標
 *
 * 91 81 71 ...
 * 92 82 72
 * 93 83 73
 * ...
 */

pub fn ms_to_suji_dan(ms: umasu) -> (i8, i8) {
    assert_banjo_ms(ms, "(203)Ｍs_to_suji_dan");
    ((ms / 10) as i8, (ms % 10) as i8)
}
pub fn ms_to_p(ms: umasu) -> Point {
    assert_banjo_ms(ms, "(203b)ms_to_p");
    Point {
        x: (ms / 10) as i8,
        y: (ms % 10) as i8,
    }
}
pub fn suji_dan_to_ms(suji: i8, dan: i8) -> umasu {
    debug_assert!(
        (SUJI_0 < suji && suji < SUJI_10) && (DAN_0 < dan && dan < DAN_10),
        "(204)suji_dan_to_ms suji={},dan={}",
        suji,
        dan
    );

    (suji * 10 + dan) as umasu
}
pub fn p_in_ban(p: &Point) -> bool {
    (SUJI_0 < p.x && p.x < SUJI_10) && (DAN_0 < p.y && p.y < DAN_10)
}
pub fn p_to_sq(p: &Point) -> Square {
    debug_assert!(p_in_ban(&p), "(204b)p_to_ms x={},y={}", p.x, p.y);

    Square::from_umasu((p.x * 10 + p.y) as umasu)
}
pub fn p_to_ms(p: &Point) -> umasu {
    debug_assert!(p_in_ban(&p), "(204b)p_to_ms x={},y={}", p.x, p.y);

    (p.x * 10 + p.y) as umasu
}
/**
 * ハッシュ値を作る
 */
pub fn push_ms_to_hash(hash: u64, ms: umasu) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    (hash << 7) + ms as u64
}
/**
 * ハッシュ値から作る
 */
pub fn pop_ms_from_hash(hash: u64) -> (u64, umasu) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let ms_num = (hash & 0b1111111) as umasu;
    (hash >> 7, ms_num)
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
pub fn kaiten180_ms_by_ms_sn(ms: umasu, sn: &Phase) -> umasu {
    use super::super::super::model::master::phase::Phase::*;
    match *sn {
        Sen => BAN_MAX - ms + BAN_MIN,
        _ => ms,
    }
}

/**********
 * 駒種類 *
 **********/

/**
 * 駒種類の数値化
 */
pub fn kms_to_num(kms: &PieceType) -> usize {
    use super::super::super::model::master::piece_type::PieceType::*;
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
    use super::super::super::model::master::piece_type::PieceType::*;
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
    use super::super::super::model::master::piece_type::PieceType::*;
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
    use super::super::super::model::master::piece_type::PieceType::*;
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
    use super::super::super::model::master::piece_direction::PieceDirection::*;
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
