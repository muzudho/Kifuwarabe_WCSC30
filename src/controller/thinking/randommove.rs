//!
//! ランダムムーブ
//!
extern crate rand;
use rand::Rng;

use super::super::super::controller::common::conv::*;
use super::super::super::model::master::piece_type::PieceType;
use super::super::super::model::master::piece_type::*;
use super::super::super::model::master::place::*;
use super::super::super::model::master::square::*;

/**
 * ランダムに真偽を返す。
 */
#[allow(dead_code)]
pub fn rnd_bool() -> bool {
    rand::thread_rng().gen_range(0, 2) == 0
}

/// (筋1～9,段1～9)の範囲で、ランダムに マス座標を返す
pub fn random_square() -> Square {
    Square::from_umasu(rnd_ms())
}
/// (筋1～9,段1～9)の範囲で、ランダムに マス座標を返す
pub fn rnd_ms() -> umasu {
    suji_dan_to_ms(
        rand::thread_rng().gen_range(1, 10),
        rand::thread_rng().gen_range(1, 10),
    )
}

/**
 * ランダムに 駒の種類を返す
 */
pub fn rnd_kms() -> &'static PieceType {
    &KMS_ARRAY[rand::thread_rng().gen_range(0, KMS_ARRAY_LN)]
}
