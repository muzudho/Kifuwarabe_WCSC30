//!
//! 駒集合
//!

use super::super::super::super::model::dto::search_part::sp_main_dto::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::super::model::vo::other_part::op_piece_vo::*;
use std::collections::HashSet;

pub struct SPPieceSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceSetDto {
    /**
     * 全ての元を含む
     */
    pub fn new_all(speed_of_light: &SpeedOfLight) -> SPPieceSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for piece in KM_ARRAY.iter() {
            let ps = speed_of_light.piece_vo_master.get_piece_vo(piece);
            num_syugo1.insert(ps.serial_piece_number());
        }
        let km_syugo = SPPieceSetDto {
            num_syugo: num_syugo1,
        };
        km_syugo
    }
    /**
     * 自分相手
     */
    pub fn new_jiai(
        &self,
        jiai: &Person,
        search_part: &SPMainDto,
        speed_of_light: &SpeedOfLight,
    ) -> SPPieceSetDto {
        let sn0 = search_part.get_phase(&jiai);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let ps = speed_of_light.piece_vo_master.get_piece_vo(km);
            let (sn1, _kms) = ps.phase_piece_type();
            if match_sn(&sn0, &sn1) {
                num_syugo1.insert(ps.serial_piece_number());
            }
        }
        let km_syugo = SPPieceSetDto {
            num_syugo: num_syugo1,
        };
        km_syugo
    }
    pub fn remove(&mut self, piece: &OPPieceVo, speed_of_light: &SpeedOfLight) {
        self.num_syugo.remove(
            &speed_of_light
                .piece_vo_master
                .get_piece_vo(piece)
                .serial_piece_number(),
        );
    }
}
