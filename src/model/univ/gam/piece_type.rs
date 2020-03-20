//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use crate::model::vo::main_loop::ml_speed_of_light_vo::MLSpeedOfLightVo;
use std::fmt;

pub const KMS_LN: usize = 16;
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
    // 空マス
    KaraPieceType,
    // 要素数より1小さい数。エラー値用に使っても可
    OwariPieceType,
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
            KaraPieceType => write!(f, "　"),
            OwariPieceType => write!(f, "×"),
        }
    }
}

// 駒種類数
pub const KMS_ARRAY_LN: usize = 14;
// 駒種類
pub const PIECE_TYPE_ARRAY: [PieceType; KMS_ARRAY_LN] = [
    PieceType::King,           // らいおん
    PieceType::Rook,           // きりん
    PieceType::Bishop,         // ぞう
    PieceType::Gold,           // いぬ
    PieceType::Silver,         // ねこ
    PieceType::Knight,         // うさぎ
    PieceType::Lance,          // いのしし
    PieceType::Pawn,           // ひよこ
    PieceType::Dragon,         // ぱわーあっぷきりん
    PieceType::Horse,          // ぱわーあっぷぞう
    PieceType::PromotedSilver, // ぱわーあっぷねこ
    PieceType::PromotedKnight, // ぱわーあっぷうさぎ
    PieceType::PromotedLance,  // ぱわーあっぷいのしし
    PieceType::PromotedPawn,   // ぱわーあっぷひよこ
];

// 非成 駒種類数
pub const KMS_NPRO_ARRAY_LN: usize = 8;
// 非成 駒種類
pub const KMS_NPRO_ARRAY: [PieceType; KMS_NPRO_ARRAY_LN] = [
    PieceType::King,   // らいおん
    PieceType::Rook,   // きりん
    PieceType::Bishop, // ぞう
    PieceType::Gold,   // いぬ
    PieceType::Silver, // ねこ
    PieceType::Knight, // うさぎ
    PieceType::Lance,  // いのしし
    PieceType::Pawn,   // ひよこ
];

// 成 駒種類数
pub const KMS_PRO_ARRAY_LN: usize = 6;
// 成 駒種類
pub const KMS_PRO_ARRAY: [PieceType; KMS_PRO_ARRAY_LN] = [
    PieceType::Dragon,         // ぱわーあっぷきりん
    PieceType::Horse,          // ぱわーあっぷぞう
    PieceType::PromotedSilver, // ぱわーあっぷねこ
    PieceType::PromotedKnight, // ぱわーあっぷうさぎ
    PieceType::PromotedLance,  // ぱわーあっぷいのしし
    PieceType::PromotedPawn,   // ぱわーあっぷひよこ
];

pub struct GPHandPieces {}
impl GPHandPieces {
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
pub fn num_to_piece_type(n: usize) -> PieceType {
    use PieceType::*;
    match n {
        0 => King,
        1 => Rook,
        2 => Bishop,
        3 => Gold,
        4 => Silver,
        5 => Knight,
        6 => Lance,
        7 => Pawn,
        8 => Dragon,
        9 => Horse,
        10 => PromotedSilver,
        11 => PromotedKnight,
        12 => PromotedLance,
        13 => PromotedPawn,
        14 => KaraPieceType,
        _ => OwariPieceType,
    }
}

/// ハッシュ値を作る
pub fn push_piece_type_to_hash(
    hash: u64,
    piece_type: PieceType,
    speed_of_light: &MLSpeedOfLightVo,
) -> u64 {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    (hash << 4)
        + speed_of_light
            .get_piece_type_struct_vo_from_piece_type(&piece_type)
            .serial_piece_number as u64
}

/// ハッシュ値から作る
pub fn pop_piece_type_from_hash(hash: u64) -> (u64, PieceType) {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    let piece_type_num = num_to_piece_type((hash & 0b1111) as usize);
    (hash >> 4, piece_type_num)
}
