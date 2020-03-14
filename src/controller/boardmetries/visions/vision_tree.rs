//!
//! 思考部だぜ☆（＾～＾）
//!

use super::super::super::super::model::vo::other_part::op_square_vo::*;
use std::collections::HashSet;

/**
 * 狙いは、この木にぶら下げていくぜ☆（*＾～＾*）
 * 思考で、内容はどんどん変わっていくぜ☆（＾～＾）
 */
pub struct VisionTree {
    // 相手玉の位置
    pub sq_ai_r: Square,
    // 相手玉を取る楽観筋
    pub ss_tume_hashset: HashSet<u64>,
}
impl VisionTree {
    pub fn new() -> VisionTree {
        VisionTree {
            sq_ai_r: Square::from_usquare(0),
            ss_tume_hashset: HashSet::new(),
        }
    }
    pub fn clear(&mut self) {
        self.ss_tume_hashset.clear();
    }
    pub fn set_ai_r(&mut self, sq: &Square) {
        self.sq_ai_r = sq.clone();
    }
}
