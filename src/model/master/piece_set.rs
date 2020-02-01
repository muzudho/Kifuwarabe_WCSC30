//!
//! 駒集合
//!

use super::super::super::controller::status::uchu::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece::*;
use super::super::super::model::master::piece_struct::PieceStruct;
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
            num_syugo1.insert(PieceStruct::from_piece(km).serial_piece_number());
        }
        let km_syugo = PieceSet {
            num_syugo: num_syugo1,
        };
        km_syugo
    }
    /**
     * 自分相手
     */
    pub fn new_jiai(&self, jiai: &Person, uchu: &Uchu) -> PieceSet {
        let sn0 = uchu.get_teban(&jiai);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let (sn1, _kms) = PieceStruct::from_piece(km).phase_piece_type();
            if match_sn(&sn0, &sn1) {
                num_syugo1.insert(PieceStruct::from_piece(km).serial_piece_number());
            }
        }
        let km_syugo = PieceSet {
            num_syugo: num_syugo1,
        };
        km_syugo
    }
    pub fn remove(&mut self, km: &Piece) {
        self.num_syugo
            .remove(&PieceStruct::from_piece(km).serial_piece_number());
    }
}
