//!
//! コマンド一覧
//!

use super::super::super::model::dto::main_loop::ap_universe_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_phase_vo::Phase;
use super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::model::vo::other_part::piece::Piece;
use super::super::super::model::vo::other_part::piece::*;

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
