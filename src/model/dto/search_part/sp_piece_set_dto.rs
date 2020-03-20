//!
//! 駒集合
//!

use super::super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::super::model::vo::game_part::gp_piece_vo::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_person_vo::Person;
use crate::model::univ::gam::position::*;
use std::collections::HashSet;

pub struct SPPieceSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all(speed_of_light: &MLSpeedOfLightVo) -> SPPieceSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        GPPieces::for_all(&mut |any_piece| {
            let ps = speed_of_light.get_piece_struct_vo(&any_piece);
            num_syugo1.insert(ps.serial_piece_number());
        });
        SPPieceSetDto {
            num_syugo: num_syugo1,
        }
    }
    /**
     * 自分相手
     */
    pub fn new_person(
        &self,
        person: &Person,
        position: &Position,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> SPPieceSetDto {
        let phase0 = position.get_phase(&person);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        GPPieces::for_all(&mut |any_piece| {
            let ps = speed_of_light.get_piece_struct_vo(&any_piece);
            let (phase1, _piece_type) = ps.phase_piece_type();
            if phase0 == *phase1 {
                num_syugo1.insert(ps.serial_piece_number());
            }
        });
        SPPieceSetDto {
            num_syugo: num_syugo1,
        }
    }
    pub fn remove(&mut self, piece: &GPPieceVo, speed_of_light: &MLSpeedOfLightVo) {
        self.num_syugo.remove(
            &speed_of_light
                .get_piece_struct_vo(piece)
                .serial_piece_number(),
        );
    }
}
