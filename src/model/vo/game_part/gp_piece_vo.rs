//!
//! 駒
//!
//! 先後付き駒
//!

use crate::model::univ::gam::phase::*;
use crate::model::univ::gam::piece_type::*;
use std::fmt;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq)]
pub enum GPPieceVo {
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
impl fmt::Display for GPPieceVo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▼、△ が半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use super::gp_piece_vo::GPPieceVo::*;
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
impl GPPieceVo {
    /// TODO これを宇宙に移動したいぜ☆（＾～＾）
    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_and_piece_type(phase: &Phase, piece_type: GPPieceTypeVo) -> Self {
        use super::gp_piece_vo::GPPieceVo::*;
        use crate::model::univ::gam::piece_type::GPPieceTypeVo::*;
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
                _ => GPPieceVo::OwariPiece,
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
                _ => GPPieceVo::OwariPiece,
            },
            Phase::None => GPPieceVo::OwariPiece,
        }
    }
}

pub struct GPPieces {}
impl GPPieces {
    /// すべての駒☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(GPPieceVo),
    {
        const KM_ARRAY: [GPPieceVo; 28] = [
            GPPieceVo::King1,           // らいおん
            GPPieceVo::Rook1,           // きりん
            GPPieceVo::Bishop1,         // ぞう
            GPPieceVo::Gold1,           // いぬ
            GPPieceVo::Silver1,         // ねこ
            GPPieceVo::Knight1,         // うさぎ
            GPPieceVo::Lance1,          // いのしし
            GPPieceVo::Pawn1,           // ひよこ
            GPPieceVo::Dragon1,         // ぱわーあっぷきりん
            GPPieceVo::Horse1,          // ぱわーあっぷぞう
            GPPieceVo::PromotedSilver1, // ぱわーあっぷねこ
            GPPieceVo::PromotedKnight1, // ぱわーあっぷうさぎ
            GPPieceVo::PromotedLance1,  // ぱわーあっぷいのしし
            GPPieceVo::PromotedPawn1,   // ぱわーあっぷひよこ
            GPPieceVo::King2,           // らいおん
            GPPieceVo::Rook2,           // きりん
            GPPieceVo::Bishop2,         // ぞう
            GPPieceVo::Gold2,           // いぬ
            GPPieceVo::Silver2,         // ねこ
            GPPieceVo::Knight2,         // うさぎ
            GPPieceVo::Lance2,          // いのしし
            GPPieceVo::Pawn2,           // ひよこ
            GPPieceVo::Dragon2,         // ぱわーあっぷきりん
            GPPieceVo::Horse2,          // ぱわーあっぷぞう
            GPPieceVo::PromotedSilver2, // ぱわーあっぷねこ
            GPPieceVo::PromotedKnight2, // ぱわーあっぷうさぎ
            GPPieceVo::PromotedLance2,  // ぱわーあっぷいのしし
            GPPieceVo::PromotedPawn2,   // ぱわーあっぷひよこ
        ];
        for piece in KM_ARRAY.iter() {
            callback(*piece);
        }
    }
}
/*
pub const KM_ARRAY_HALF_LN: usize = 14;
pub const PHASE_KM_ARRAY: [[GPPieceVo; KM_ARRAY_HALF_LN]; PHASE_LN] = [
    [
        GPPieceVo::King1,           // らいおん
        GPPieceVo::Rook1,           // きりん
        GPPieceVo::Bishop1,         // ぞう
        GPPieceVo::Gold1,           // いぬ
        GPPieceVo::Silver1,         // ねこ
        GPPieceVo::Knight1,         // うさぎ
        GPPieceVo::Lance1,          // いのしし
        GPPieceVo::Pawn1,           // ひよこ
        GPPieceVo::Dragon1,         // ぱわーあっぷきりん
        GPPieceVo::Horse1,          // ぱわーあっぷぞう
        GPPieceVo::PromotedSilver1, // ぱわーあっぷねこ
        GPPieceVo::PromotedKnight1, // ぱわーあっぷうさぎ
        GPPieceVo::PromotedLance1,  // ぱわーあっぷいのしし
        GPPieceVo::PromotedPawn1,   // ぱわーあっぷひよこ
    ],
    [
        GPPieceVo::King2,           // らいおん
        GPPieceVo::Rook2,           // きりん
        GPPieceVo::Bishop2,         // ぞう
        GPPieceVo::Gold2,           // いぬ
        GPPieceVo::Silver2,         // ねこ
        GPPieceVo::Knight2,         // うさぎ
        GPPieceVo::Lance2,          // いのしし
        GPPieceVo::Pawn2,           // ひよこ
        GPPieceVo::Dragon2,         // ぱわーあっぷきりん
        GPPieceVo::Horse2,          // ぱわーあっぷぞう
        GPPieceVo::PromotedSilver2, // ぱわーあっぷねこ
        GPPieceVo::PromotedKnight2, // ぱわーあっぷうさぎ
        GPPieceVo::PromotedLance2,  // ぱわーあっぷいのしし
        GPPieceVo::PromotedPawn2,   // ぱわーあっぷひよこ
    ],
    [
        GPPieceVo::OwariPiece, // らいおん
        GPPieceVo::OwariPiece, // きりん
        GPPieceVo::OwariPiece, // ぞう
        GPPieceVo::OwariPiece, // いぬ
        GPPieceVo::OwariPiece, // ねこ
        GPPieceVo::OwariPiece, // うさぎ
        GPPieceVo::OwariPiece, // いのしし
        GPPieceVo::OwariPiece, // ひよこ
        GPPieceVo::OwariPiece, // ぱわーあっぷきりん
        GPPieceVo::OwariPiece, // ぱわーあっぷぞう
        GPPieceVo::OwariPiece, // ぱわーあっぷねこ
        GPPieceVo::OwariPiece, // ぱわーあっぷうさぎ
        GPPieceVo::OwariPiece, // ぱわーあっぷいのしし
        GPPieceVo::OwariPiece, // ぱわーあっぷひよこ
    ],
];
*/
