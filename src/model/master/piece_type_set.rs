//!
//! 駒種類集合
//!

use super::super::super::controller::common::conv::*;
use super::super::super::model::master::piece_type::PieceType;
use super::super::super::model::master::piece_type::*;
use std::collections::HashSet;

pub struct PieceTypeSet {
    num_syugo: HashSet<usize>,
}
impl PieceTypeSet {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> PieceTypeSet {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for kms in KMS_ARRAY.iter() {
            num_syugo1.insert(kms_to_num(kms));
        }
        let kms_syugo = PieceTypeSet {
            num_syugo: num_syugo1,
        };
        kms_syugo
    }
    pub fn remove(&mut self, kms: &PieceType) {
        self.num_syugo.remove(&kms_to_num(kms));
    }
}
