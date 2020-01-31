//!
//! 駒集合
//!

use super::super::super::controller::common::conv::*;
use super::super::super::jotai::uchu::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece::*;
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
            num_syugo1.insert(km_to_num(km));
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
            let (sn1, _kms) = km_to_sn_kms(km);
            if match_sn(&sn0, &sn1) {
                num_syugo1.insert(km_to_num(km));
            }
        }
        let km_syugo = PieceSet {
            num_syugo: num_syugo1,
        };
        km_syugo
    }
    pub fn remove(&mut self, km: &Piece) {
        self.num_syugo.remove(&km_to_num(km));
    }
}
