//!
//! 駒集合
//!

use super::super::super::super::model::dto::search_part::sp_main_dto::*;
use super::super::super::super::model::vo::person::Person;
use super::super::super::super::model::vo::phase::*;
use super::super::super::super::model::vo::piece::Piece;
use super::super::super::super::model::vo::piece::*;
use super::super::super::super::model::vo::speed_of_light::*;
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
    pub fn remove(&mut self, piece: &Piece, speed_of_light: &SpeedOfLight) {
        self.num_syugo.remove(
            &speed_of_light
                .piece_vo_master
                .get_piece_vo(piece)
                .serial_piece_number(),
        );
    }
}
