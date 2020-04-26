//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use crate::cosmic::recording::Phase;
use crate::law::speed_of_light::SpeedOfLight;
use std::fmt;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq)]
pub enum PieceMeaning {
    // ▲玉
    King1,
    // ▲きりん
    Rook1,
    // ▲ぞう
    Bishop1,
    // ▲いぬ
    Gold1,
    // ▲ねこ
    Silver1,
    // ▲うさぎ
    Knight1,
    // ▲いのしし
    Lance1,
    // ▲ひよこ
    Pawn1,
    // ▲ぱわーあっぷきりん
    Dragon1,
    // ▲ぱわーあっぷぞう
    Horse1,
    // ▲ぱわーあっぷねこ
    PromotedSilver1,
    // ▲ぱわーあっぷうさぎ
    PromotedKnight1,
    // ▲ぱわーあっぷいのしし
    PromotedLance1,
    // ▲ぱわーあっぷひよこ
    PromotedPawn1,
    // ▽ライオン
    King2,
    // ▽キリン
    Rook2,
    // ▽ゾウ
    Bishop2,
    // ▽イヌ
    Gold2,
    // ▽ネコ
    Silver2,
    // ▽ウサギ
    Knight2,
    // ▽イノシシ
    Lance2,
    // ▽ヒヨコ
    Pawn2,
    // ▽パワーアップキリン
    Dragon2,
    // ▽パワーアップゾウ
    Horse2,
    // ▽パワーアップネコ
    PromotedSilver2,
    // ▽パワーアップウサギ
    PromotedKnight2,
    // ▽パワーアップイノシシ
    PromotedLance2,
    // ▽パワーアップヒヨコ
    PromotedPawn2,
}

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const HAND_MAX: usize = 18;
pub const PIECE_LN: usize = 30;
pub const HAND_PIECE_LN: usize = 14;
pub static PIECE_WHITE_SPACE: &str = "    ";
impl fmt::Display for PieceMeaning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲、▽ が半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::cosmic::smart::features::PieceMeaning::*;
        match *self {
            King1 => write!(f, " ▲K "),
            Rook1 => write!(f, " ▲R "),
            Bishop1 => write!(f, " ▲B "),
            Gold1 => write!(f, " ▲G "),
            Silver1 => write!(f, " ▲S "),
            Knight1 => write!(f, " ▲N "),
            Lance1 => write!(f, " ▲L "),
            Pawn1 => write!(f, " ▲P "),
            Dragon1 => write!(f, " ▲PR"),
            Horse1 => write!(f, " ▲PB"),
            PromotedSilver1 => write!(f, " ▲PS"),
            PromotedKnight1 => write!(f, " ▲PN"),
            PromotedLance1 => write!(f, " ▲PL"),
            PromotedPawn1 => write!(f, " ▲PP"),
            King2 => write!(f, " ▽k "),
            Rook2 => write!(f, " ▽r "),
            Bishop2 => write!(f, " ▽b "),
            Gold2 => write!(f, " ▽g "),
            Silver2 => write!(f, " ▽s "),
            Knight2 => write!(f, " ▽n "),
            Lance2 => write!(f, " ▽l "),
            Pawn2 => write!(f, " ▽p "),
            Dragon2 => write!(f, " ▽pr"),
            Horse2 => write!(f, " ▽pb"),
            PromotedSilver2 => write!(f, " ▽ps"),
            PromotedKnight2 => write!(f, " ▽pn"),
            PromotedLance2 => write!(f, " ▽pl"),
            PromotedPawn2 => write!(f, " ▽pp"),
        }
    }
}
impl PieceMeaning {
    /// TODO これを宇宙に移動したいぜ☆（＾～＾）
    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_and_piece_type(phase: Phase, piece_type: PieceType) -> Self {
        use crate::cosmic::smart::features::PieceMeaning::*;
        use crate::cosmic::smart::features::PieceType::*;
        match phase {
            Phase::First => match piece_type {
                King => King1,
                Rook => Rook1,
                Bishop => Bishop1,
                Gold => Gold1,
                Silver => Silver1,
                Knight => Knight1,
                Lance => Lance1,
                Pawn => Pawn1,
                Dragon => Dragon1,
                Horse => Horse1,
                PromotedSilver => PromotedSilver1,
                PromotedKnight => PromotedKnight1,
                PromotedLance => PromotedLance1,
                PromotedPawn => PromotedPawn1,
            },
            Phase::Second => match piece_type {
                King => King2,
                Rook => Rook2,
                Bishop => Bishop2,
                Gold => Gold2,
                Silver => Silver2,
                Knight => Knight2,
                Lance => Lance2,
                Pawn => Pawn2,
                Dragon => Dragon2,
                Horse => Horse2,
                PromotedSilver => PromotedSilver2,
                PromotedKnight => PromotedKnight2,
                PromotedLance => PromotedLance2,
                PromotedPawn => PromotedPawn2,
            },
        }
    }
}

/*
pub struct Pieces {}
impl Pieces {
    /// すべての駒☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(PieceMeaning),
    {
        const KM_ARRAY: [PieceMeaning; 28] = [
            PieceMeaning::King1,           // らいおん
            PieceMeaning::Rook1,           // きりん
            PieceMeaning::Bishop1,         // ぞう
            PieceMeaning::Gold1,           // いぬ
            PieceMeaning::Silver1,         // ねこ
            PieceMeaning::Knight1,         // うさぎ
            PieceMeaning::Lance1,          // いのしし
            PieceMeaning::Pawn1,           // ひよこ
            PieceMeaning::Dragon1,         // ぱわーあっぷきりん
            PieceMeaning::Horse1,          // ぱわーあっぷぞう
            PieceMeaning::PromotedSilver1, // ぱわーあっぷねこ
            PieceMeaning::PromotedKnight1, // ぱわーあっぷうさぎ
            PieceMeaning::PromotedLance1,  // ぱわーあっぷいのしし
            PieceMeaning::PromotedPawn1,   // ぱわーあっぷひよこ
            PieceMeaning::King2,           // らいおん
            PieceMeaning::Rook2,           // きりん
            PieceMeaning::Bishop2,         // ぞう
            PieceMeaning::Gold2,           // いぬ
            PieceMeaning::Silver2,         // ねこ
            PieceMeaning::Knight2,         // うさぎ
            PieceMeaning::Lance2,          // いのしし
            PieceMeaning::Pawn2,           // ひよこ
            PieceMeaning::Dragon2,         // ぱわーあっぷきりん
            PieceMeaning::Horse2,          // ぱわーあっぷぞう
            PieceMeaning::PromotedSilver2, // ぱわーあっぷねこ
            PieceMeaning::PromotedKnight2, // ぱわーあっぷうさぎ
            PieceMeaning::PromotedLance2,  // ぱわーあっぷいのしし
            PieceMeaning::PromotedPawn2,   // ぱわーあっぷひよこ
        ];
        for piece in KM_ARRAY.iter() {
            callback(*piece);
        }
    }
}
*/

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

#[derive(Clone, Copy, Debug)]
pub enum HandAddress {
    King1,
    Rook1,
    Bishop1,
    Gold1,
    Silver1,
    Knight1,
    Lance1,
    Pawn1,
    King2,
    Rook2,
    Bishop2,
    Gold2,
    Silver2,
    Knight2,
    Lance2,
    Pawn2,
}
pub struct HandAddresses {}
impl HandAddresses {
    /// 持駒種類
    pub fn for_phase<F1>(phase: Phase, callback: &mut F1)
    where
        F1: FnMut(HandAddress),
    {
        let list = match phase {
            Phase::First => [
                HandAddress::Rook1,
                HandAddress::Bishop1,
                HandAddress::Gold1,
                HandAddress::Silver1,
                HandAddress::Knight1,
                HandAddress::Lance1,
                HandAddress::Pawn1,
            ],
            Phase::Second => [
                HandAddress::Rook2,
                HandAddress::Bishop2,
                HandAddress::Gold2,
                HandAddress::Silver2,
                HandAddress::Knight2,
                HandAddress::Lance2,
                HandAddress::Pawn2,
            ],
        };

        for adr in &list {
            callback(*adr);
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

/// 数値から駒種類を作るぜ☆（＾～＾）ハッシュを使うときに使うぜ☆（＾～＾）
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
