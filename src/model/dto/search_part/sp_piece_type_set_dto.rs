//!
//! 駒種類集合
//!

use super::super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use std::collections::HashSet;

pub struct SPPieceTypeSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceTypeSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all(speed_of_light: &MLSpeedOfLightVo) -> SPPieceTypeSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for piece_type in PIECE_TYPE_ARRAY.iter() {
            num_syugo1.insert(
                speed_of_light
                    .get_piece_type_struct_vo_from_piece_type(piece_type)
                    .serial_piece_number,
            );
        }
        SPPieceTypeSetDto {
            num_syugo: num_syugo1,
        }
    }
    pub fn remove(&mut self, piece_type: GPPieceTypeVo, speed_of_light: &MLSpeedOfLightVo) {
        self.num_syugo.remove(
            &speed_of_light
                .get_piece_type_struct_vo_from_piece_type(&piece_type)
                .serial_piece_number,
        );
    }
}
