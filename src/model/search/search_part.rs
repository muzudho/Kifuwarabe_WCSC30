//! 探索部
use super::super::super::model::master::ply::*;

pub struct SearchPart {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    ply: i16,
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hash_history: [u64; TEME_LN],
}
impl SearchPart {
    pub fn new() -> Self {
        SearchPart {
            ply: 0,
            position_hash_history: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, //257手目 TODO 321手に 拡張したいぜ☆（＾～＾）
            ],
        }
    }
    pub fn add_ply(&mut self, ply1: i16) {
        self.ply += ply1
    }
    pub fn get_ply(&self) -> i16 {
        self.ply
    }
    pub fn set_ply(&mut self, ply1: i16) {
        self.ply = ply1
    }

    pub fn get_position_hash_history(&self) -> &[u64; TEME_LN] {
        &self.position_hash_history
    }
    pub fn get_position_hash_history_mut(&mut self) -> &mut [u64; TEME_LN] {
        &mut self.position_hash_history
    }

    pub fn get_current_position_hash(&mut self) -> u64 {
        self.position_hash_history[self.ply as usize]
    }
    pub fn set_current_position_hash(&mut self, hash: u64) {
        self.position_hash_history[self.ply as usize] = hash;
    }
}
