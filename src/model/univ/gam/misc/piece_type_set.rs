//!
//! 駒種類集合
//!

use crate::model::univ::gam::misc::piece_type::PieceType;
use crate::model::univ::gam::misc::piece_type::*;
use crate::speed_of_light::*;
use std::collections::HashSet;

pub struct SPPieceTypeSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceTypeSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all(speed_of_light: &SpeedOfLight) -> SPPieceTypeSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for piece_type in PIECE_TYPE_ARRAY.iter() {
            num_syugo1.insert(
                speed_of_light
                    .get_piece_type_struct_from_piece_type(piece_type)
                    .serial_piece_number,
            );
        }
        SPPieceTypeSetDto {
            num_syugo: num_syugo1,
        }
    }
    pub fn remove(&mut self, piece_type: PieceType, speed_of_light: &SpeedOfLight) {
        self.num_syugo.remove(
            &speed_of_light
                .get_piece_type_struct_from_piece_type(&piece_type)
                .serial_piece_number,
        );
    }
}
