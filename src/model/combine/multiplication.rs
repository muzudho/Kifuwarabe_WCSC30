#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::master::piece::Piece;
use super::super::master::piece_type::PieceType;
use super::super::master::square::*;

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
