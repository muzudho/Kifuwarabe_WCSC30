//!
//! 駒
//!
//! 先後付き駒
//!

use super::super::game_part::gp_piece_type_vo::*;
use super::op_phase_vo::*;
use std::fmt;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
// #[derive(Copy, Clone)]
#[derive(Clone, PartialEq)]
pub enum OPPieceVo {
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
    PromotedRook1,
    // ▼ぱわーあっぷぞう
    PromotedBishop1,
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
    PromotedRook2,
    // △パワーアップゾウ
    PromotedBishop2,
    // △パワーアップネコ
    PromotedSilver2,
    // △パワーアップウサギ
    PromotedKnight2,
    // △パワーアップイノシシ
    PromotedLance2,
    // △パワーアップヒヨコ
    PromotedPawn2,
    // 空マス
    Kara,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Owari,
}

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX: usize = 18;
pub const KM_LN: usize = 30;
impl fmt::Display for OPPieceVo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::op_piece_vo::OPPieceVo::*;
        match *self {
            King1 => write!(f, "▼ら"),
            Rook1 => write!(f, "▼き"),
            Bishop1 => write!(f, "▼ぞ"),
            Gold1 => write!(f, "▼い"),
            Silver1 => write!(f, "▼ね"),
            Knight1 => write!(f, "▼う"),
            Lance1 => write!(f, "▼し"),
            Pawn1 => write!(f, "▼ひ"),
            PromotedRook1 => write!(f, "▼PK"),
            PromotedBishop1 => write!(f, "▼PZ"),
            PromotedSilver1 => write!(f, "▼PN"),
            PromotedKnight1 => write!(f, "▼PU"),
            PromotedLance1 => write!(f, "▼PS"),
            PromotedPawn1 => write!(f, "▼PH"),
            King2 => write!(f, "△ラ"),
            Rook2 => write!(f, "△キ"),
            Bishop2 => write!(f, "△ゾ"),
            Gold2 => write!(f, "△イ"),
            Silver2 => write!(f, "△ネ"),
            Knight2 => write!(f, "△ウ"),
            Lance2 => write!(f, "△シ"),
            Pawn2 => write!(f, "△ヒ"),
            PromotedRook2 => write!(f, "△pk"),
            PromotedBishop2 => write!(f, "△pz"),
            PromotedSilver2 => write!(f, "△pn"),
            PromotedKnight2 => write!(f, "△pu"),
            PromotedLance2 => write!(f, "△ps"),
            PromotedPawn2 => write!(f, "△ph"),
            Kara => write!(f, "　　"),
            Owari => write!(f, "××"),
        }
    }
}
impl OPPieceVo {
    /// TODO これを宇宙に移動したいぜ☆（＾～＾）
    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_piece_type(phase: &Phase, piece_type: GPPieceTypeVo) -> Self {
        use super::super::game_part::gp_piece_type_vo::GPPieceTypeVo::*;
        use super::op_piece_vo::OPPieceVo::*;
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
                Dragon => PromotedRook1,
                Horse => PromotedBishop1,
                PromotedSilver => PromotedSilver1,
                PromotedKnight => PromotedKnight1,
                PromotedLance => PromotedLance1,
                PromotedPawn => PromotedPawn1,
                _ => OPPieceVo::Owari,
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
                Dragon => PromotedRook2,
                Horse => PromotedBishop2,
                PromotedSilver => PromotedSilver2,
                PromotedKnight => PromotedKnight2,
                PromotedLance => PromotedLance2,
                PromotedPawn => PromotedPawn2,
                _ => OPPieceVo::Owari,
            },
            Phase::None => OPPieceVo::Owari,
        }
    }
}

pub const KM_ARRAY_HALF_LN: usize = 14;
pub const KM_ARRAY_LN: usize = 28;
pub const KM_ARRAY: [OPPieceVo; KM_ARRAY_LN] = [
    OPPieceVo::King1,           // らいおん
    OPPieceVo::Rook1,           // きりん
    OPPieceVo::Bishop1,         // ぞう
    OPPieceVo::Gold1,           // いぬ
    OPPieceVo::Silver1,         // ねこ
    OPPieceVo::Knight1,         // うさぎ
    OPPieceVo::Lance1,          // いのしし
    OPPieceVo::Pawn1,           // ひよこ
    OPPieceVo::PromotedRook1,   // ぱわーあっぷきりん
    OPPieceVo::PromotedBishop1, // ぱわーあっぷぞう
    OPPieceVo::PromotedSilver1, // ぱわーあっぷねこ
    OPPieceVo::PromotedKnight1, // ぱわーあっぷうさぎ
    OPPieceVo::PromotedLance1,  // ぱわーあっぷいのしし
    OPPieceVo::PromotedPawn1,   // ぱわーあっぷひよこ
    OPPieceVo::King2,           // らいおん
    OPPieceVo::Rook2,           // きりん
    OPPieceVo::Bishop2,         // ぞう
    OPPieceVo::Gold2,           // いぬ
    OPPieceVo::Silver2,         // ねこ
    OPPieceVo::Knight2,         // うさぎ
    OPPieceVo::Lance2,          // いのしし
    OPPieceVo::Pawn2,           // ひよこ
    OPPieceVo::PromotedRook2,   // ぱわーあっぷきりん
    OPPieceVo::PromotedBishop2, // ぱわーあっぷぞう
    OPPieceVo::PromotedSilver2, // ぱわーあっぷねこ
    OPPieceVo::PromotedKnight2, // ぱわーあっぷうさぎ
    OPPieceVo::PromotedLance2,  // ぱわーあっぷいのしし
    OPPieceVo::PromotedPawn2,   // ぱわーあっぷひよこ
];
pub const PHASE_KM_ARRAY: [[OPPieceVo; KM_ARRAY_HALF_LN]; PHASE_LN] = [
    [
        OPPieceVo::King1,           // らいおん
        OPPieceVo::Rook1,           // きりん
        OPPieceVo::Bishop1,         // ぞう
        OPPieceVo::Gold1,           // いぬ
        OPPieceVo::Silver1,         // ねこ
        OPPieceVo::Knight1,         // うさぎ
        OPPieceVo::Lance1,          // いのしし
        OPPieceVo::Pawn1,           // ひよこ
        OPPieceVo::PromotedRook1,   // ぱわーあっぷきりん
        OPPieceVo::PromotedBishop1, // ぱわーあっぷぞう
        OPPieceVo::PromotedSilver1, // ぱわーあっぷねこ
        OPPieceVo::PromotedKnight1, // ぱわーあっぷうさぎ
        OPPieceVo::PromotedLance1,  // ぱわーあっぷいのしし
        OPPieceVo::PromotedPawn1,   // ぱわーあっぷひよこ
    ],
    [
        OPPieceVo::King2,           // らいおん
        OPPieceVo::Rook2,           // きりん
        OPPieceVo::Bishop2,         // ぞう
        OPPieceVo::Gold2,           // いぬ
        OPPieceVo::Silver2,         // ねこ
        OPPieceVo::Knight2,         // うさぎ
        OPPieceVo::Lance2,          // いのしし
        OPPieceVo::Pawn2,           // ひよこ
        OPPieceVo::PromotedRook2,   // ぱわーあっぷきりん
        OPPieceVo::PromotedBishop2, // ぱわーあっぷぞう
        OPPieceVo::PromotedSilver2, // ぱわーあっぷねこ
        OPPieceVo::PromotedKnight2, // ぱわーあっぷうさぎ
        OPPieceVo::PromotedLance2,  // ぱわーあっぷいのしし
        OPPieceVo::PromotedPawn2,   // ぱわーあっぷひよこ
    ],
    [
        OPPieceVo::Owari, // らいおん
        OPPieceVo::Owari, // きりん
        OPPieceVo::Owari, // ぞう
        OPPieceVo::Owari, // いぬ
        OPPieceVo::Owari, // ねこ
        OPPieceVo::Owari, // うさぎ
        OPPieceVo::Owari, // いのしし
        OPPieceVo::Owari, // ひよこ
        OPPieceVo::Owari, // ぱわーあっぷきりん
        OPPieceVo::Owari, // ぱわーあっぷぞう
        OPPieceVo::Owari, // ぱわーあっぷねこ
        OPPieceVo::Owari, // ぱわーあっぷうさぎ
        OPPieceVo::Owari, // ぱわーあっぷいのしし
        OPPieceVo::Owari, // ぱわーあっぷひよこ
    ],
];
