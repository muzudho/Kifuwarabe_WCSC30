//!
//! ランダムムーブ
//!
extern crate rand;
use rand::Rng;

use crate::model::univ::gam::piece_type::PieceType;
use crate::model::univ::gam::piece_type::*;
use crate::model::univ::gam::square::*;

/**
 * ランダムに真偽を返す。
 */
#[allow(dead_code)]
pub fn random_bool() -> bool {
    rand::thread_rng().gen_range(0, 2) == 0
}

/// (筋1～9,段1～9)の範囲で、ランダムに マス座標を返す
pub fn random_square() -> Square {
    Square::from_file_rank(
        rand::thread_rng().gen_range(1, 10),
        rand::thread_rng().gen_range(1, 10),
    )
}

/**
 * ランダムに 駒の種類を返す
 */
pub fn random_piece_type() -> &'static PieceType {
    &PIECE_TYPE_ARRAY[rand::thread_rng().gen_range(0, KMS_ARRAY_LN)]
}
