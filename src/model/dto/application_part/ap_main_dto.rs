//! アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）

use super::super::super::super::model::dto::search_part::sp_position_dto::*;
use super::super::super::super::model::vo::phase::*;
use super::super::super::super::model::vo::piece::*;
use super::super::super::super::model::vo::square::*;

/// 局面ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct PositionHashSeed {
    // 盤上の駒
    pub km: [[u64; KM_LN]; BAN_SIZE],
    // 持ち駒
    pub mg: [[u64; MG_MAX]; KM_LN],
    // 先後
    pub sn: [u64; SN_LN],
}

pub struct ApMainDto {
    /// 局面ハッシュ種☆（＾～＾）
    position_hash_seed: PositionHashSeed,
    /// 初期局面
    starting_position: SPPositionDto,
    /// 初期局面ハッシュ
    starting_position_hash: u64,
}
impl ApMainDto {
    pub fn new() -> Self {
        ApMainDto {
            position_hash_seed: PositionHashSeed {
                // 盤上の駒
                km: [[0; KM_LN]; BAN_SIZE],
                // 持ち駒
                mg: [[0; MG_MAX]; KM_LN],
                // 先後
                sn: [0; SN_LN],
            },
            starting_position: SPPositionDto::new(),
            starting_position_hash: 0,
        }
    }

    pub fn get_position_hash_seed(&self) -> &PositionHashSeed {
        &self.position_hash_seed
    }
    pub fn get_position_hash_seed_mut(&mut self) -> &mut PositionHashSeed {
        &mut self.position_hash_seed
    }

    pub fn get_starting_position(&self) -> &SPPositionDto {
        &self.starting_position
    }
    pub fn get_starting_position_mut(&mut self) -> &mut SPPositionDto {
        &mut self.starting_position
    }

    pub fn get_starting_position_hash(&self) -> &u64 {
        &self.starting_position_hash
    }
    pub fn get_starting_position_hash_mut(&mut self) -> &mut u64 {
        &mut self.starting_position_hash
    }
    pub fn set_starting_position_hash(&mut self, val: u64) {
        self.starting_position_hash = val;
    }
}
