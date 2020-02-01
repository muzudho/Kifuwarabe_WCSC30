//! アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）

use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::*;
use super::super::super::model::master::square::*;

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

pub struct ApplicationPart {
    /// 局面ハッシュ種☆（＾～＾）
    pub position_hash_seed: PositionHashSeed,
}
impl ApplicationPart {
    pub fn new() -> Self {
        ApplicationPart {
            position_hash_seed: PositionHashSeed {
                // 盤上の駒
                km: [[0; KM_LN]; BAN_SIZE],
                // 持ち駒
                mg: [[0; MG_MAX]; KM_LN],
                // 先後
                sn: [0; SN_LN],
            },
        }
    }
}
