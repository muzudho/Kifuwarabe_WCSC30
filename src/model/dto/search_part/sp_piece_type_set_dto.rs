//!
//! 駒種類集合
//!

use super::super::super::super::controller::common_use::cu_conv_controller::*;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use std::collections::HashSet;

pub struct SPPieceTypeSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceTypeSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> SPPieceTypeSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for piece_type in KMS_ARRAY.iter() {
            num_syugo1.insert(piece_type_to_num(*piece_type));
        }
        SPPieceTypeSetDto {
            num_syugo: num_syugo1,
        }
    }
    pub fn remove(&mut self, piece_type: GPPieceTypeVo) {
        self.num_syugo.remove(&piece_type_to_num(piece_type));
    }
}
