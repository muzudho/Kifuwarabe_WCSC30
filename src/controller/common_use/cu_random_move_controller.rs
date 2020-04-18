//!
//! ランダムムーブ
//!
extern crate rand;

use crate::model::univ::gam::misc::square::*;
use crate::universe::game::piece::piece_type::PieceType;
use crate::universe::game::piece::piece_type::*;
use rand::Rng;

/**
 * ランダムに真偽を返す。
 */
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
