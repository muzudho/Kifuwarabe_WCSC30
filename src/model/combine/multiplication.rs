#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::master::piece::Piece;
use super::super::master::piece_type::PieceType;
use super::super::master::place::*;

/************
 * 升 × 駒 *
 ************/

pub struct MsKm {
    ms: umasu,
    km: Piece,
}

/****************
 * 升 × 駒種類 *
 ****************/

pub struct MsKms {
    ms: umasu,
    kms: PieceType,
}
