//!
//! 駒
//!
//! 先後付き駒
//!

use crate::model::univ::gam::misc::phase::*;
use crate::model::univ::gam::misc::piece_type::*;
use std::fmt;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    // ▼玉
    King1,
    // ▼きりん
    Rook1,
    // ▼ぞう
    Bishop1,
    // ▼いぬ
    Gold1,
    // ▼ねこ
    Silver1,
    // ▼うさぎ
    Knight1,
    // ▼いのしし
    Lance1,
    // ▼ひよこ
    Pawn1,
    // ▼ぱわーあっぷきりん
    Dragon1,
    // ▼ぱわーあっぷぞう
    Horse1,
    // ▼ぱわーあっぷねこ
    PromotedSilver1,
    // ▼ぱわーあっぷうさぎ
    PromotedKnight1,
    // ▼ぱわーあっぷいのしし
    PromotedLance1,
    // ▼ぱわーあっぷひよこ
    PromotedPawn1,
    // △ライオン
    King2,
    // △キリン
    Rook2,
    // △ゾウ
    Bishop2,
    // △イヌ
    Gold2,
    // △ネコ
    Silver2,
    // △ウサギ
    Knight2,
    // △イノシシ
    Lance2,
    // △ヒヨコ
    Pawn2,
    // △パワーアップキリン
    Dragon2,
    // △パワーアップゾウ
    Horse2,
    // △パワーアップネコ
    PromotedSilver2,
    // △パワーアップウサギ
    PromotedKnight2,
    // △パワーアップイノシシ
    PromotedLance2,
    // △パワーアップヒヨコ
    PromotedPawn2,
    // 空マス
    NonePiece,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    OwariPiece,
}

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX: usize = 18;
pub const PIECE_LN: usize = 30;
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▼、△ が半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::model::univ::gam::misc::piece::Piece::*;
        match *self {
            King1 => write!(f, " ▼K "),
            Rook1 => write!(f, " ▼R "),
            Bishop1 => write!(f, " ▼B "),
            Gold1 => write!(f, " ▼G "),
            Silver1 => write!(f, " ▼S "),
            Knight1 => write!(f, " ▼N "),
            Lance1 => write!(f, " ▼L "),
            Pawn1 => write!(f, " ▼P "),
            Dragon1 => write!(f, " ▼PR"),
            Horse1 => write!(f, " ▼PB"),
            PromotedSilver1 => write!(f, " ▼PS"),
            PromotedKnight1 => write!(f, " ▼PN"),
            PromotedLance1 => write!(f, " ▼PL"),
            PromotedPawn1 => write!(f, " ▼PP"),
            King2 => write!(f, " △k "),
            Rook2 => write!(f, " △r "),
            Bishop2 => write!(f, " △b "),
            Gold2 => write!(f, " △g "),
            Silver2 => write!(f, " △s "),
            Knight2 => write!(f, " △n "),
            Lance2 => write!(f, " △l "),
            Pawn2 => write!(f, " △p "),
            Dragon2 => write!(f, " △pr"),
            Horse2 => write!(f, " △pb"),
            PromotedSilver2 => write!(f, " △ps"),
            PromotedKnight2 => write!(f, " △pn"),
            PromotedLance2 => write!(f, " △pl"),
            PromotedPawn2 => write!(f, " △pp"),
            NonePiece => write!(f, "    "),
            OwariPiece => write!(f, " ×× "),
        }
    }
}
impl Piece {
    /// TODO これを宇宙に移動したいぜ☆（＾～＾）
    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_and_piece_type(phase: &Phase, piece_type: PieceType) -> Self {
        use crate::model::univ::gam::misc::piece::Piece::*;
        use crate::model::univ::gam::misc::piece_type::PieceType::*;
        match *phase {
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
                _ => Piece::OwariPiece,
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
                _ => Piece::OwariPiece,
            },
            Phase::None => Piece::OwariPiece,
        }
    }
}

pub struct GPPieces {}
impl GPPieces {
    /// すべての駒☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(Piece),
    {
        const KM_ARRAY: [Piece; 28] = [
            Piece::King1,           // らいおん
            Piece::Rook1,           // きりん
            Piece::Bishop1,         // ぞう
            Piece::Gold1,           // いぬ
            Piece::Silver1,         // ねこ
            Piece::Knight1,         // うさぎ
            Piece::Lance1,          // いのしし
            Piece::Pawn1,           // ひよこ
            Piece::Dragon1,         // ぱわーあっぷきりん
            Piece::Horse1,          // ぱわーあっぷぞう
            Piece::PromotedSilver1, // ぱわーあっぷねこ
            Piece::PromotedKnight1, // ぱわーあっぷうさぎ
            Piece::PromotedLance1,  // ぱわーあっぷいのしし
            Piece::PromotedPawn1,   // ぱわーあっぷひよこ
            Piece::King2,           // らいおん
            Piece::Rook2,           // きりん
            Piece::Bishop2,         // ぞう
            Piece::Gold2,           // いぬ
            Piece::Silver2,         // ねこ
            Piece::Knight2,         // うさぎ
            Piece::Lance2,          // いのしし
            Piece::Pawn2,           // ひよこ
            Piece::Dragon2,         // ぱわーあっぷきりん
            Piece::Horse2,          // ぱわーあっぷぞう
            Piece::PromotedSilver2, // ぱわーあっぷねこ
            Piece::PromotedKnight2, // ぱわーあっぷうさぎ
            Piece::PromotedLance2,  // ぱわーあっぷいのしし
            Piece::PromotedPawn2,   // ぱわーあっぷひよこ
        ];
        for piece in KM_ARRAY.iter() {
            callback(*piece);
        }
    }
}
/*
pub const KM_ARRAY_HALF_LN: usize = 14;
pub const PHASE_KM_ARRAY: [[Piece; KM_ARRAY_HALF_LN]; PHASE_LN] = [
    [
        Piece::King1,           // らいおん
        Piece::Rook1,           // きりん
        Piece::Bishop1,         // ぞう
        Piece::Gold1,           // いぬ
        Piece::Silver1,         // ねこ
        Piece::Knight1,         // うさぎ
        Piece::Lance1,          // いのしし
        Piece::Pawn1,           // ひよこ
        Piece::Dragon1,         // ぱわーあっぷきりん
        Piece::Horse1,          // ぱわーあっぷぞう
        Piece::PromotedSilver1, // ぱわーあっぷねこ
        Piece::PromotedKnight1, // ぱわーあっぷうさぎ
        Piece::PromotedLance1,  // ぱわーあっぷいのしし
        Piece::PromotedPawn1,   // ぱわーあっぷひよこ
    ],
    [
        Piece::King2,           // らいおん
        Piece::Rook2,           // きりん
        Piece::Bishop2,         // ぞう
        Piece::Gold2,           // いぬ
        Piece::Silver2,         // ねこ
        Piece::Knight2,         // うさぎ
        Piece::Lance2,          // いのしし
        Piece::Pawn2,           // ひよこ
        Piece::Dragon2,         // ぱわーあっぷきりん
        Piece::Horse2,          // ぱわーあっぷぞう
        Piece::PromotedSilver2, // ぱわーあっぷねこ
        Piece::PromotedKnight2, // ぱわーあっぷうさぎ
        Piece::PromotedLance2,  // ぱわーあっぷいのしし
        Piece::PromotedPawn2,   // ぱわーあっぷひよこ
    ],
    [
        Piece::OwariPiece, // らいおん
        Piece::OwariPiece, // きりん
        Piece::OwariPiece, // ぞう
        Piece::OwariPiece, // いぬ
        Piece::OwariPiece, // ねこ
        Piece::OwariPiece, // うさぎ
        Piece::OwariPiece, // いのしし
        Piece::OwariPiece, // ひよこ
        Piece::OwariPiece, // ぱわーあっぷきりん
        Piece::OwariPiece, // ぱわーあっぷぞう
        Piece::OwariPiece, // ぱわーあっぷねこ
        Piece::OwariPiece, // ぱわーあっぷうさぎ
        Piece::OwariPiece, // ぱわーあっぷいのしし
        Piece::OwariPiece, // ぱわーあっぷひよこ
    ],
];
*/
