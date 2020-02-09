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
    // らいおん
    R,
    // きりん
    K,
    // ぞう
    Z,
    // いぬ
    I,
    // ねこ
    N,
    // うさぎ
    U,
    // いのしし
    S,
    // ひよこ
    H,
    // ぱわーあっぷきりん
    PK,
    // ぱわーあっぷぞう
    PZ,
    // ぱわーあっぷねこ
    PN,
    // ぱわーあっぷうさぎ
    PU,
    // ぱわーあっぷいのしし
    PS,
    // ぱわーあっぷひよこ
    PH,
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
            R => write!(f, "ら"),
            K => write!(f, "き"),
            Z => write!(f, "ぞ"),
            I => write!(f, "い"),
            N => write!(f, "ね"),
            U => write!(f, "う"),
            S => write!(f, "い"),
            H => write!(f, "ひ"),
            PK => write!(f, "PK"),
            PZ => write!(f, "PZ"),
            PN => write!(f, "PN"),
            PU => write!(f, "PU"),
            PS => write!(f, "PS"),
            PH => write!(f, "PH"),
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
pub const KMS_ARRAY: [GPPieceTypeVo; KMS_ARRAY_LN] = [
    GPPieceTypeVo::R,  // らいおん
    GPPieceTypeVo::K,  // きりん
    GPPieceTypeVo::Z,  // ぞう
    GPPieceTypeVo::I,  // いぬ
    GPPieceTypeVo::N,  // ねこ
    GPPieceTypeVo::U,  // うさぎ
    GPPieceTypeVo::S,  // いのしし
    GPPieceTypeVo::H,  // ひよこ
    GPPieceTypeVo::PK, // ぱわーあっぷきりん
    GPPieceTypeVo::PZ, // ぱわーあっぷぞう
    GPPieceTypeVo::PN, // ぱわーあっぷねこ
    GPPieceTypeVo::PU, // ぱわーあっぷうさぎ
    GPPieceTypeVo::PS, // ぱわーあっぷいのしし
    GPPieceTypeVo::PH, // ぱわーあっぷひよこ
];

// 非成 駒種類数
pub const KMS_NPRO_ARRAY_LN: usize = 8;
// 非成 駒種類
pub const KMS_NPRO_ARRAY: [GPPieceTypeVo; KMS_NPRO_ARRAY_LN] = [
    GPPieceTypeVo::R, // らいおん
    GPPieceTypeVo::K, // きりん
    GPPieceTypeVo::Z, // ぞう
    GPPieceTypeVo::I, // いぬ
    GPPieceTypeVo::N, // ねこ
    GPPieceTypeVo::U, // うさぎ
    GPPieceTypeVo::S, // いのしし
    GPPieceTypeVo::H, // ひよこ
];

// 成 駒種類数
pub const KMS_PRO_ARRAY_LN: usize = 6;
// 成 駒種類
pub const KMS_PRO_ARRAY: [GPPieceTypeVo; KMS_PRO_ARRAY_LN] = [
    GPPieceTypeVo::PK, // ぱわーあっぷきりん
    GPPieceTypeVo::PZ, // ぱわーあっぷぞう
    GPPieceTypeVo::PN, // ぱわーあっぷねこ
    GPPieceTypeVo::PU, // ぱわーあっぷうさぎ
    GPPieceTypeVo::PS, // ぱわーあっぷいのしし
    GPPieceTypeVo::PH, // ぱわーあっぷひよこ
];

// 持駒種類数
pub const MGS_ARRAY_LN: usize = 7;
// 持駒種類
pub const MGS_ARRAY: [GPPieceTypeVo; MGS_ARRAY_LN] = [
    GPPieceTypeVo::K,
    GPPieceTypeVo::Z,
    GPPieceTypeVo::I,
    GPPieceTypeVo::N,
    GPPieceTypeVo::U,
    GPPieceTypeVo::S,
    GPPieceTypeVo::H,
];
