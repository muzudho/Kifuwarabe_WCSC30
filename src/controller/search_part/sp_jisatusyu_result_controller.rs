#![allow(dead_code)]
//!
//! 結果：自殺手。移動先が敵の利き
//!

use crate::model::univ::gam::movement_builder::*;
use crate::model::univ::gam::phase::phase_to_num;
use crate::model::univ::gam::phase::turn_phase;
use crate::model::univ::speed_of_light::*;
use crate::model::universe::*;

/// 動かした先が、敵の利きに飛び込んでいれば、自殺手
/// TODO 利きを再計算したい
pub fn is_jisatusyu(
    ml_universe_dto: &Universe,
    ss: &MovementBuilder,
    speed_of_light: &MLSpeedOfLightVo,
) -> bool {
    // 移動元升、動かした駒の先後、駒種類、
    let km_src = ml_universe_dto
        .game
        .position
        .current_board
        .get_piece_by_square(&ss.src);
    let ps_src = speed_of_light.get_piece_struct(km_src);
    let (phase_teban, _piece_type) = ps_src.phase_piece_type();
    // 相手番の先後
    let phase_aite = turn_phase(&phase_teban);

    // 升の利き数だが、指した後で再計算が要るはず
    let control_count = ml_universe_dto.game.position.control_count_by_phase
        [phase_to_num(&phase_aite)]
    .get_number_by_square(&ss.dst);
    0 < control_count
    // g_writeln(&format!(
    //     "info string is_jisatusyu={} km_src={} phase_teban={} piece_type={} phase_aite={} ss.dst={} control_count={}"
    //     ,result ,km_src ,phase_teban ,piece_type ,phase_aite ,ss.dst ,control_count
    // ));
}
