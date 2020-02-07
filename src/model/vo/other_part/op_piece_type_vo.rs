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
pub enum PieceType {
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
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceType::*;
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
pub fn match_kms(a: &PieceType, b: &PieceType) -> bool {
    kms_to_num(a) == kms_to_num(b)
}

// 駒種類数
pub const KMS_ARRAY_LN: usize = 14;
// 駒種類
pub const KMS_ARRAY: [PieceType; KMS_ARRAY_LN] = [
    PieceType::R,  // らいおん
    PieceType::K,  // きりん
    PieceType::Z,  // ぞう
    PieceType::I,  // いぬ
    PieceType::N,  // ねこ
    PieceType::U,  // うさぎ
    PieceType::S,  // いのしし
    PieceType::H,  // ひよこ
    PieceType::PK, // ぱわーあっぷきりん
    PieceType::PZ, // ぱわーあっぷぞう
    PieceType::PN, // ぱわーあっぷねこ
    PieceType::PU, // ぱわーあっぷうさぎ
    PieceType::PS, // ぱわーあっぷいのしし
    PieceType::PH, // ぱわーあっぷひよこ
];

// 非成 駒種類数
pub const KMS_NPRO_ARRAY_LN: usize = 8;
// 非成 駒種類
pub const KMS_NPRO_ARRAY: [PieceType; KMS_NPRO_ARRAY_LN] = [
    PieceType::R, // らいおん
    PieceType::K, // きりん
    PieceType::Z, // ぞう
    PieceType::I, // いぬ
    PieceType::N, // ねこ
    PieceType::U, // うさぎ
    PieceType::S, // いのしし
    PieceType::H, // ひよこ
];

// 成 駒種類数
pub const KMS_PRO_ARRAY_LN: usize = 6;
// 成 駒種類
pub const KMS_PRO_ARRAY: [PieceType; KMS_PRO_ARRAY_LN] = [
    PieceType::PK, // ぱわーあっぷきりん
    PieceType::PZ, // ぱわーあっぷぞう
    PieceType::PN, // ぱわーあっぷねこ
    PieceType::PU, // ぱわーあっぷうさぎ
    PieceType::PS, // ぱわーあっぷいのしし
    PieceType::PH, // ぱわーあっぷひよこ
];

// 持駒種類数
pub const MGS_ARRAY_LN: usize = 7;
// 持駒種類
pub const MGS_ARRAY: [PieceType; MGS_ARRAY_LN] = [
    PieceType::K,
    PieceType::Z,
    PieceType::I,
    PieceType::N,
    PieceType::U,
    PieceType::S,
    PieceType::H,
];
