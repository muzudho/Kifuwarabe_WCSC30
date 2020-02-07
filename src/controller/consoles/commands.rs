//!
//! コマンド一覧
//!

use super::super::super::model::dto::universe::*;
use super::super::super::model::vo::phase::Phase;
use super::super::super::model::vo::phase::*;
use super::super::super::model::vo::piece::Piece;
use super::super::super::model::vo::piece::*;
use super::super::super::model::vo::speed_of_light::*;

/**
 * 利き数表示
 */
pub fn cmd_kikisu(universe: &Universe, speed_of_light: &SpeedOfLight) {
    for pc in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", pc));
        let s = universe.kaku_number_board(&Phase::Owari, pc, speed_of_light);
        g_writeln(&s);
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = universe.kaku_number_board(&sn, &Piece::Owari, speed_of_light);
        g_writeln(&s);
    }
}
