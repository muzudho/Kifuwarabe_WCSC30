//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use crate::cosmic::recording::Phase;
use num_derive::FromPrimitive;
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
// Note: 持ち駒には玉も含むぜ☆（＾～＾）
pub const HAND_ADDRESS_LN: usize = 16;
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
/*
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
*/
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

pub const HAND_ADDRESS_TYPE_LEN: usize = 8;
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum HandAddressType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
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
impl HandAddress {
    pub fn from_phase_and_type(phase: Phase, adr: HandAddressType) -> Self {
        match phase {
            Phase::First => match adr {
                HandAddressType::King => HandAddress::King1,
                HandAddressType::Rook => HandAddress::Rook1,
                HandAddressType::Bishop => HandAddress::Bishop1,
                HandAddressType::Gold => HandAddress::Gold1,
                HandAddressType::Silver => HandAddress::Silver1,
                HandAddressType::Knight => HandAddress::Knight1,
                HandAddressType::Lance => HandAddress::Lance1,
                HandAddressType::Pawn => HandAddress::Pawn1,
            },
            Phase::Second => match adr {
                HandAddressType::King => HandAddress::King2,
                HandAddressType::Rook => HandAddress::Rook2,
                HandAddressType::Bishop => HandAddress::Bishop2,
                HandAddressType::Gold => HandAddress::Gold2,
                HandAddressType::Silver => HandAddress::Silver2,
                HandAddressType::Knight => HandAddress::Knight2,
                HandAddressType::Lance => HandAddress::Lance2,
                HandAddressType::Pawn => HandAddress::Pawn2,
            },
        }
    }
}

/// 持駒種類
pub struct HandAddresses {}
impl HandAddresses {
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(HandAddress),
    {
        for adr in &[
            HandAddress::Rook1,
            HandAddress::Bishop1,
            HandAddress::Gold1,
            HandAddress::Silver1,
            HandAddress::Knight1,
            HandAddress::Lance1,
            HandAddress::Pawn1,
            HandAddress::Rook2,
            HandAddress::Bishop2,
            HandAddress::Gold2,
            HandAddress::Silver2,
            HandAddress::Knight2,
            HandAddress::Lance2,
            HandAddress::Pawn2,
        ] {
            callback(*adr);
        }
    }
    /*
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
    */
}

/*
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
*/
