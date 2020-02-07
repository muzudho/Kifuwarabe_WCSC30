#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::vo::piece::Piece;
use super::super::vo::piece_type::PieceType;
use super::super::vo::square::*;

/************
 * 升 × 駒 *
 ************/

pub struct SqKm {
    sq: Square,
    km: Piece,
}

/****************
 * 升 × 駒種類 *
 ****************/

pub struct SqKms {
    sq: Square,
    kms: PieceType,
}
