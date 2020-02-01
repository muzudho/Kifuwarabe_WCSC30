//!
//! 駒
//!
//! 先後付き駒
//!

use super::phase::*;
// use super::piece_struct::PieceStruct;
use std::fmt;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
// #[derive(Copy, Clone)]
#[derive(Clone, PartialEq)]
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
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::super::super::model::master::piece::Piece::*;
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

pub const KM_ARRAY_HALF_LN: usize = 14;
pub const KM_ARRAY_LN: usize = 28;
pub const KM_ARRAY: [Piece; KM_ARRAY_LN] = [
    Piece::King1,           // らいおん
    Piece::Rook1,           // きりん
    Piece::Bishop1,         // ぞう
    Piece::Gold1,           // いぬ
    Piece::Silver1,         // ねこ
    Piece::Knight1,         // うさぎ
    Piece::Lance1,          // いのしし
    Piece::Pawn1,           // ひよこ
    Piece::PromotedRook1,   // ぱわーあっぷきりん
    Piece::PromotedBishop1, // ぱわーあっぷぞう
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
    Piece::PromotedRook2,   // ぱわーあっぷきりん
    Piece::PromotedBishop2, // ぱわーあっぷぞう
    Piece::PromotedSilver2, // ぱわーあっぷねこ
    Piece::PromotedKnight2, // ぱわーあっぷうさぎ
    Piece::PromotedLance2,  // ぱわーあっぷいのしし
    Piece::PromotedPawn2,   // ぱわーあっぷひよこ
];
pub const SN_KM_ARRAY: [[Piece; KM_ARRAY_HALF_LN]; SN_LN] = [
    [
        Piece::King1,           // らいおん
        Piece::Rook1,           // きりん
        Piece::Bishop1,         // ぞう
        Piece::Gold1,           // いぬ
        Piece::Silver1,         // ねこ
        Piece::Knight1,         // うさぎ
        Piece::Lance1,          // いのしし
        Piece::Pawn1,           // ひよこ
        Piece::PromotedRook1,   // ぱわーあっぷきりん
        Piece::PromotedBishop1, // ぱわーあっぷぞう
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
        Piece::PromotedRook2,   // ぱわーあっぷきりん
        Piece::PromotedBishop2, // ぱわーあっぷぞう
        Piece::PromotedSilver2, // ぱわーあっぷねこ
        Piece::PromotedKnight2, // ぱわーあっぷうさぎ
        Piece::PromotedLance2,  // ぱわーあっぷいのしし
        Piece::PromotedPawn2,   // ぱわーあっぷひよこ
    ],
    [
        Piece::Owari, // らいおん
        Piece::Owari, // きりん
        Piece::Owari, // ぞう
        Piece::Owari, // いぬ
        Piece::Owari, // ねこ
        Piece::Owari, // うさぎ
        Piece::Owari, // いのしし
        Piece::Owari, // ひよこ
        Piece::Owari, // ぱわーあっぷきりん
        Piece::Owari, // ぱわーあっぷぞう
        Piece::Owari, // ぱわーあっぷねこ
        Piece::Owari, // ぱわーあっぷうさぎ
        Piece::Owari, // ぱわーあっぷいのしし
        Piece::Owari, // ぱわーあっぷひよこ
    ],
];
