//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use super::super::super::super::controller::common_use::cu_conv_controller::*;
use std::fmt;

pub const KMS_LN: usize = 16;
/// USIでCopyするので、Copyが要る。
#[derive(Copy, Clone)]
pub enum GPPieceTypeVo {
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
    // 空マス
    Kara,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari,
}
impl fmt::Display for GPPieceTypeVo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::GPPieceTypeVo::*;
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
            Kara => write!(f, "　"),
            Owari => write!(f, "×"),
        }
    }
}

// 駒の動ける方向数、終端子込み
pub const KM_UGOKI_LN: usize = 9;
/**
 * 駒種類の一致比較
 */
pub fn match_piece_type(a: GPPieceTypeVo, b: GPPieceTypeVo) -> bool {
    piece_type_to_num(a) == piece_type_to_num(b)
}

// 駒種類数
pub const KMS_ARRAY_LN: usize = 14;
// 駒種類
pub const PIECE_TYPE_ARRAY: [GPPieceTypeVo; KMS_ARRAY_LN] = [
    GPPieceTypeVo::King,           // らいおん
    GPPieceTypeVo::Rook,           // きりん
    GPPieceTypeVo::Bishop,         // ぞう
    GPPieceTypeVo::Gold,           // いぬ
    GPPieceTypeVo::Silver,         // ねこ
    GPPieceTypeVo::Knight,         // うさぎ
    GPPieceTypeVo::Lance,          // いのしし
    GPPieceTypeVo::Pawn,           // ひよこ
    GPPieceTypeVo::Dragon,         // ぱわーあっぷきりん
    GPPieceTypeVo::Horse,          // ぱわーあっぷぞう
    GPPieceTypeVo::PromotedSilver, // ぱわーあっぷねこ
    GPPieceTypeVo::PromotedKnight, // ぱわーあっぷうさぎ
    GPPieceTypeVo::PromotedLance,  // ぱわーあっぷいのしし
    GPPieceTypeVo::PromotedPawn,   // ぱわーあっぷひよこ
];

// 非成 駒種類数
pub const KMS_NPRO_ARRAY_LN: usize = 8;
// 非成 駒種類
pub const KMS_NPRO_ARRAY: [GPPieceTypeVo; KMS_NPRO_ARRAY_LN] = [
    GPPieceTypeVo::King,   // らいおん
    GPPieceTypeVo::Rook,   // きりん
    GPPieceTypeVo::Bishop, // ぞう
    GPPieceTypeVo::Gold,   // いぬ
    GPPieceTypeVo::Silver, // ねこ
    GPPieceTypeVo::Knight, // うさぎ
    GPPieceTypeVo::Lance,  // いのしし
    GPPieceTypeVo::Pawn,   // ひよこ
];

// 成 駒種類数
pub const KMS_PRO_ARRAY_LN: usize = 6;
// 成 駒種類
pub const KMS_PRO_ARRAY: [GPPieceTypeVo; KMS_PRO_ARRAY_LN] = [
    GPPieceTypeVo::Dragon,         // ぱわーあっぷきりん
    GPPieceTypeVo::Horse,          // ぱわーあっぷぞう
    GPPieceTypeVo::PromotedSilver, // ぱわーあっぷねこ
    GPPieceTypeVo::PromotedKnight, // ぱわーあっぷうさぎ
    GPPieceTypeVo::PromotedLance,  // ぱわーあっぷいのしし
    GPPieceTypeVo::PromotedPawn,   // ぱわーあっぷひよこ
];

// 持駒種類数
pub const MGS_ARRAY_LN: usize = 7;
// 持駒種類
pub const MGS_ARRAY: [GPPieceTypeVo; MGS_ARRAY_LN] = [
    GPPieceTypeVo::Rook,
    GPPieceTypeVo::Bishop,
    GPPieceTypeVo::Gold,
    GPPieceTypeVo::Silver,
    GPPieceTypeVo::Knight,
    GPPieceTypeVo::Lance,
    GPPieceTypeVo::Pawn,
];
