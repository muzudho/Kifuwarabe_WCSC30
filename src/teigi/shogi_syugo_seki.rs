#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::model::master::piece::Piece;
use super::super::teigi::shogi_syugo::*;

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
    kms: KmSyurui,
}
