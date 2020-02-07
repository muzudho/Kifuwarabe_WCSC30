#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::super::vo::other_part::op_piece_type_vo::PieceType;
use super::super::super::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::vo::other_part::op_square_vo::*;

/// 升 × 駒
pub struct SqKm {
    sq: Square,
    km: OPPieceVo,
}

/// 升 × 駒種類
pub struct SqKms {
    sq: Square,
    kms: PieceType,
}
