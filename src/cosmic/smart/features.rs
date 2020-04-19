//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use crate::law::speed_of_light::SpeedOfLight;
use std::fmt;

pub const NONE_SERIAL_PIECE_TYPE_NUMBER: u64 = 14;

/// USIでCopyするので、Copyが要る。
#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    // 玉
    King,
    // 飛
    Rook,
    // 角
    Bishop,
    // 金
    Gold,
    // 銀
    Silver,
    // 桂
    Knight,
    // 香
    Lance,
    // 歩
    Pawn,
    // 竜
    Dragon,
    // 馬
    Horse,
    // 全
    PromotedSilver,
    // 圭
    PromotedKnight,
    // 杏
    PromotedLance,
    // ぱわーあっぷひよこ
    PromotedPawn,
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceType::*;
        match *self {
            King => write!(f, "ら"),
            Rook => write!(f, "き"),
            Bishop => write!(f, "ぞ"),
            Gold => write!(f, "い"),
            Silver => write!(f, "ね"),
            Knight => write!(f, "う"),
            Lance => write!(f, "い"),
            Pawn => write!(f, "ひ"),
            Dragon => write!(f, "PK"),
            Horse => write!(f, "PZ"),
            PromotedSilver => write!(f, "PN"),
            PromotedKnight => write!(f, "PU"),
            PromotedLance => write!(f, "PS"),
            PromotedPawn => write!(f, "PH"),
        }
    }
}

pub struct HandPieces {}
impl HandPieces {
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(PieceType),
    {
        // 持駒種類
        const MGS_ARRAY: [PieceType; 7] = [
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Gold,
            PieceType::Silver,
            PieceType::Knight,
            PieceType::Lance,
            PieceType::Pawn,
        ];

        for hand_piece_type in MGS_ARRAY.iter() {
            callback(*hand_piece_type);
        }
    }
}

/// 数値の駒種類化
pub fn num_to_piece_type(n: usize) -> Option<PieceType> {
    use PieceType::*;
    match n {
        0 => Some(King),
        1 => Some(Rook),
        2 => Some(Bishop),
        3 => Some(Gold),
        4 => Some(Silver),
        5 => Some(Knight),
        6 => Some(Lance),
        7 => Some(Pawn),
        8 => Some(Dragon),
        9 => Some(Horse),
        10 => Some(PromotedSilver),
        11 => Some(PromotedKnight),
        12 => Some(PromotedLance),
        13 => Some(PromotedPawn),
        _ => None,
    }
}

/// ハッシュ値を作る
pub fn push_piece_type_to_hash(
    hash: u64,
    piece_type_o: Option<PieceType>,
    speed_of_light: &SpeedOfLight,
) -> u64 {
    let num = if let Some(piece_type) = piece_type_o {
        // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
        piece_type.serial_number(speed_of_light) as u64
    } else {
        NONE_SERIAL_PIECE_TYPE_NUMBER
    };
    (hash << 4) + num
}

/// ハッシュ値から作る
pub fn pop_piece_type_from_hash(hash: u64) -> (u64, Option<PieceType>) {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    let piece_type = num_to_piece_type((hash & 0b1111) as usize);
    (hash >> 4, piece_type)
}