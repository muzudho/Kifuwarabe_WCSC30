//!
//! 変換
//!
#![allow(dead_code)]
use super::super::super::controller::consoles::asserts::*;
use super::super::super::controller::geometries::geo_teigi::*;
use super::super::super::model::master::direction::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::Phase;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece_direction::PieceDirection;
use super::super::super::model::master::piece_type::PieceType;
use super::super::super::model::master::place::*;

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

/**************
 * 先後付き駒 *
 **************/

/**
 * 先後付き駒の数値化
 */
pub fn km_to_num(km: &Piece) -> usize {
    use super::super::super::model::master::piece::Piece::*;
    match *km {
        King1 => 0,
        Rook1 => 1,
        Bishop1 => 2,
        Gold1 => 3,
        Silver1 => 4,
        Knight1 => 5,
        Lance1 => 6,
        Pawn1 => 7,
        PromotedRook1 => 8,
        PromotedBishop1 => 9,
        PromotedSilver1 => 10,
        PromotedKnight1 => 11,
        PromotedLance1 => 12,
        PromotedPawn1 => 13,
        King2 => 14,
        Rook2 => 15,
        Bishop2 => 16,
        Gold2 => 17,
        Silver2 => 18,
        Knight2 => 19,
        Lance2 => 20,
        Pawn2 => 21,
        PromotedRook2 => 22,
        PromotedBishop2 => 23,
        PromotedSilver2 => 24,
        PromotedKnight2 => 25,
        PromotedLance2 => 26,
        PromotedPawn2 => 27,
        Kara => 28,
        Owari => 29,
    }
}
pub fn num_to_km(km_num: usize) -> Piece {
    use super::super::super::model::master::piece::Piece::*;
    match km_num {
        0 => King1,
        1 => Rook1,
        2 => Bishop1,
        3 => Gold1,
        4 => Silver1,
        5 => Knight1,
        6 => Lance1,
        7 => Pawn1,
        8 => PromotedRook1,
        9 => PromotedBishop1,
        10 => PromotedSilver1,
        11 => PromotedKnight1,
        12 => PromotedLance1,
        13 => PromotedPawn1,
        14 => King2,
        15 => Rook2,
        16 => Bishop2,
        17 => Gold2,
        18 => Silver2,
        19 => Knight2,
        20 => Lance2,
        21 => Pawn2,
        22 => PromotedRook2,
        23 => PromotedBishop2,
        24 => PromotedSilver2,
        25 => PromotedKnight2,
        26 => PromotedLance2,
        27 => PromotedPawn2,
        28 => Kara,
        _ => Owari,
    }
}
/**
 * ハッシュ値を作る
 */
pub fn push_km_to_hash(hash: u64, km: &Piece) -> u64 {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    (hash << 5) + km_to_num(km) as u64
}
/**
 * ハッシュ値から作る
 */
pub fn pop_km_from_hash(hash: u64) -> (u64, Piece) {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    let km_num = num_to_km((hash & 0b11111) as usize);
    (hash >> 5, km_num)
}
/**
 * 駒→成駒　（成れない駒は、そのまま）
 */
pub fn km_to_prokm(km: &Piece) -> Piece {
    use super::super::super::model::master::piece::Piece::*;
    match *km {
        King1 => King1,
        Rook1 => PromotedRook1,
        Bishop1 => PromotedBishop1,
        Gold1 => Gold1,
        Silver1 => PromotedSilver1,
        Knight1 => PromotedKnight1,
        Lance1 => PromotedLance1,
        Pawn1 => PromotedPawn1,
        PromotedRook1 => PromotedRook1,
        PromotedBishop1 => PromotedBishop1,
        PromotedSilver1 => PromotedSilver1,
        PromotedKnight1 => PromotedKnight1,
        PromotedLance1 => PromotedLance1,
        PromotedPawn1 => PromotedPawn1,
        King2 => King2,
        Rook2 => PromotedRook2,
        Bishop2 => PromotedBishop2,
        Gold2 => Gold2,
        Silver2 => PromotedSilver2,
        Knight2 => PromotedKnight2,
        Lance2 => PromotedLance2,
        Pawn2 => PromotedPawn2,
        PromotedRook2 => PromotedRook2,
        PromotedBishop2 => PromotedBishop2,
        PromotedSilver2 => PromotedSilver2,
        PromotedKnight2 => PromotedKnight2,
        PromotedLance2 => PromotedLance2,
        PromotedPawn2 => PromotedPawn2,
        Kara => Kara,
        Owari => Owari,
    }
}
/**
 * 成駒→駒
 */
pub fn prokm_to_km(km: &Piece) -> Piece {
    use super::super::super::model::master::piece::Piece::*;
    match *km {
        King1 => King1,
        Rook1 => Rook1,
        Bishop1 => Bishop1,
        Gold1 => Gold1,
        Silver1 => Silver1,
        Knight1 => Knight1,
        Lance1 => Lance1,
        Pawn1 => Pawn1,
        PromotedRook1 => Rook1,
        PromotedBishop1 => Bishop1,
        PromotedSilver1 => Silver1,
        PromotedKnight1 => Knight1,
        PromotedLance1 => Lance1,
        PromotedPawn1 => Pawn1,
        King2 => King2,
        Rook2 => Rook2,
        Bishop2 => Bishop2,
        Gold2 => Gold2,
        Silver2 => Silver2,
        Knight2 => Knight2,
        Lance2 => Lance2,
        Pawn2 => Pawn2,
        PromotedRook2 => Rook2,
        PromotedBishop2 => Bishop2,
        PromotedSilver2 => Silver2,
        PromotedKnight2 => Knight2,
        PromotedLance2 => Lance2,
        PromotedPawn2 => Pawn2,
        Kara => Kara,
        Owari => Owari,
    }
}
/**
 * 駒→長い利きの有無
 */
pub fn km_is_nagaikiki(km: &Piece) -> bool {
    kms_is_nagaikiki(&km_to_kms(km))
}
/**
 * 先後付き駒→駒種類
 */
pub fn km_to_sn_kms(km: &Piece) -> (Phase, PieceType) {
    use super::super::super::model::master::phase::Phase::*;
    use super::super::super::model::master::piece::Piece::*;
    use super::super::super::model::master::piece_type::PieceType::*;
    match *km {
        King1 => (Sen, R),
        Rook1 => (Sen, K),
        Bishop1 => (Sen, Z),
        Gold1 => (Sen, I),
        Silver1 => (Sen, N),
        Knight1 => (Sen, U),
        Lance1 => (Sen, S),
        Pawn1 => (Sen, H),
        PromotedRook1 => (Sen, PK),
        PromotedBishop1 => (Sen, PZ),
        PromotedSilver1 => (Sen, PN),
        PromotedKnight1 => (Sen, PU),
        PromotedLance1 => (Sen, PS),
        PromotedPawn1 => (Sen, PH),
        King2 => (Go, R),
        Rook2 => (Go, K),
        Bishop2 => (Go, Z),
        Gold2 => (Go, I),
        Silver2 => (Go, N),
        Knight2 => (Go, U),
        Lance2 => (Go, S),
        Pawn2 => (Go, H),
        PromotedRook2 => (Go, PK),
        PromotedBishop2 => (Go, PZ),
        PromotedSilver2 => (Go, PN),
        PromotedKnight2 => (Go, PU),
        PromotedLance2 => (Go, PS),
        PromotedPawn2 => (Go, PH),
        Piece::Kara => (Phase::Owari, PieceType::Kara),
        Piece::Owari => (Phase::Owari, PieceType::Owari),
    }
}
/**
 * 先後付き駒　を　先後　へ変換。
 */
#[allow(dead_code)]
pub fn km_to_sn(km: &Piece) -> Phase {
    use super::super::super::model::master::phase::Phase::*;
    use super::super::super::model::master::piece::Piece::*;
    match *km {
        King1 => Sen,
        Rook1 => Sen,
        Bishop1 => Sen,
        Gold1 => Sen,
        Silver1 => Sen,
        Knight1 => Sen,
        Lance1 => Sen,
        Pawn1 => Sen,
        PromotedRook1 => Sen,
        PromotedBishop1 => Sen,
        PromotedSilver1 => Sen,
        PromotedKnight1 => Sen,
        PromotedLance1 => Sen,
        PromotedPawn1 => Sen,
        King2 => Go,
        Rook2 => Go,
        Bishop2 => Go,
        Gold2 => Go,
        Silver2 => Go,
        Knight2 => Go,
        Lance2 => Go,
        Pawn2 => Go,
        PromotedRook2 => Go,
        PromotedBishop2 => Go,
        PromotedSilver2 => Go,
        PromotedKnight2 => Go,
        PromotedLance2 => Go,
        PromotedPawn2 => Go,
        Kara => Phase::Owari,
        Piece::Owari => Phase::Owari,
    }
}
/**
 * 先後付き駒→駒種類
 */
pub fn km_to_kms(km: &Piece) -> PieceType {
    use super::super::super::model::master::piece::Piece::*;
    use super::super::super::model::master::piece_type::PieceType::*;
    match *km {
        King1 => R,
        Rook1 => K,
        Bishop1 => Z,
        Gold1 => I,
        Silver1 => N,
        Knight1 => U,
        Lance1 => S,
        Pawn1 => H,
        PromotedRook1 => PK,
        PromotedBishop1 => PZ,
        PromotedSilver1 => PN,
        PromotedKnight1 => PU,
        PromotedLance1 => PS,
        PromotedPawn1 => PH,
        King2 => R,
        Rook2 => K,
        Bishop2 => Z,
        Gold2 => I,
        Silver2 => N,
        Knight2 => U,
        Lance2 => S,
        Pawn2 => H,
        PromotedRook2 => PK,
        PromotedBishop2 => PZ,
        PromotedSilver2 => PN,
        PromotedKnight2 => PU,
        PromotedLance2 => PS,
        PromotedPawn2 => PH,
        Piece::Kara => PieceType::Kara,
        Piece::Owari => PieceType::Owari,
    }
}
/**
 * 先後付き駒　を　持ち駒種類　へ変換。
 * 持ち駒にするので、先後は反転するぜ☆（＾～＾）
 */
pub fn km_to_mg(km_cap: Piece) -> Piece {
    use super::super::super::model::master::piece::Piece::*;
    match km_cap {
        King1 => Owari,
        Rook1 => Rook2,
        Bishop1 => Bishop2,
        Gold1 => Gold2,
        Silver1 => Silver2,
        Knight1 => Knight2,
        Lance1 => Lance2,
        Pawn1 => Pawn2,
        PromotedRook1 => Rook2,
        PromotedBishop1 => Bishop2,
        PromotedSilver1 => Silver2,
        PromotedKnight1 => Knight2,
        PromotedLance1 => Lance2,
        PromotedPawn1 => Pawn2,
        King2 => Owari,
        Rook2 => Rook1,
        Bishop2 => Bishop1,
        Gold2 => Gold1,
        Silver2 => Silver1,
        Knight2 => Knight1,
        Lance2 => Lance1,
        Pawn2 => Pawn1,
        PromotedRook2 => Rook1,
        PromotedBishop2 => Bishop1,
        PromotedSilver2 => Silver1,
        PromotedKnight2 => Knight1,
        PromotedLance2 => Lance1,
        PromotedPawn2 => Pawn1,
        Kara => Owari,
        Owari => Owari,
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
// 駒種類→｛　成駒,（不成駒、それ以外）　｝
pub fn kms_is_pro(kms: &PieceType) -> bool {
    use super::super::super::model::master::piece_type::PieceType::*;
    match *kms {
        R => false,
        K => false,
        Z => false,
        I => false,
        N => false,
        U => false,
        S => false,
        H => false,
        PK => true,
        PZ => true,
        PN => true,
        PU => true,
        PS => true,
        PH => true,
        Kara => false,
        Owari => false,
    }
}
// 成り駒種類→成る前の駒種類。成り駒でなければ、空に戻る。
pub fn prokms_to_kms(kms: &PieceType) -> PieceType {
    use super::super::super::model::master::piece_type::PieceType::*;
    match *kms {
        R => Kara,
        K => Kara,
        Z => Kara,
        I => Kara,
        N => Kara,
        U => Kara,
        S => Kara,
        H => Kara,
        PK => K,
        PZ => Z,
        PN => N,
        PU => U,
        PS => S,
        PH => H,
        Kara => Kara,
        Owari => Owari,
    }
}
/**
 * 駒種類→｛　長い利きの駒か否か　｝
 * 合い駒で防ぎえる可能性があれば真
 */
pub fn kms_is_nagaikiki(kms: &PieceType) -> bool {
    use super::super::super::model::master::piece_type::PieceType::*;
    match *kms {
        R => false,
        K => true,
        Z => true,
        I => false,
        N => false,
        U => false,
        S => true,
        H => false,
        PK => true,
        PZ => true,
        PN => false,
        PU => false,
        PS => false,
        PH => false,
        Kara => false,
        Owari => false,
    }
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
// 先後＆駒種類→先後付き駒
pub fn sn_kms_to_km(sn: &Phase, kms: &PieceType) -> Piece {
    use super::super::super::model::master::piece::Piece::*;
    use super::super::super::model::master::piece_type::PieceType::*;
    match *sn {
        Phase::Sen => match *kms {
            R => King1,
            K => Rook1,
            Z => Bishop1,
            I => Gold1,
            N => Silver1,
            U => Knight1,
            S => Lance1,
            H => Pawn1,
            PK => PromotedRook1,
            PZ => PromotedBishop1,
            PN => PromotedSilver1,
            PU => PromotedKnight1,
            PS => PromotedLance1,
            PH => PromotedPawn1,
            _ => Piece::Owari,
        },
        Phase::Go => match *kms {
            R => King2,
            K => Rook2,
            Z => Bishop2,
            I => Gold2,
            N => Silver2,
            U => Knight2,
            S => Lance2,
            H => Pawn2,
            PK => PromotedRook2,
            PZ => PromotedBishop2,
            PN => PromotedSilver2,
            PU => PromotedKnight2,
            PS => PromotedLance2,
            PH => PromotedPawn2,
            _ => Piece::Owari,
        },
        Phase::Owari => Piece::Owari,
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
