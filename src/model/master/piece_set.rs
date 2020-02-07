//!
//! 駒集合
//!

use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece::*;
use super::super::super::model::search::search_part::*;
use super::super::super::model::vo::piece_vo::PieceVo;
use super::super::super::model::vo::speed_of_light::*;
use std::collections::HashSet;

pub struct PieceSet {
    num_syugo: HashSet<usize>,
}
impl PieceSet {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> PieceSet {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let ps = PieceVo::from_piece((*km).clone());
            num_syugo1.insert(ps.serial_piece_number());
        }
        let km_syugo = PieceSet {
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
        search_part: &SearchPart,
        speed_of_light: &SpeedOfLight,
    ) -> PieceSet {
        let sn0 = search_part.get_phase(&jiai);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let ps = speed_of_light.piece_vo_master.get_piece_vo(km);
            let (sn1, _kms) = ps.phase_piece_type();
            if match_sn(&sn0, &sn1) {
                num_syugo1.insert(ps.serial_piece_number());
            }
        }
        let km_syugo = PieceSet {
            num_syugo: num_syugo1,
        };
        km_syugo
    }
    pub fn remove(&mut self, piece: Piece) {
        self.num_syugo
            .remove(&PieceVo::from_piece(piece).serial_piece_number());
    }
}
