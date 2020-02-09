#![allow(dead_code)]
//!
//! 結果：自殺手。移動先が敵の利き
//!

use super::super::super::controller::common_use::cu_conv_controller::*;
use super::super::super::model::dto::main_loop::ml_dto::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;

/// 動かした先が、敵の利きに飛び込んでいれば、自殺手
/// TODO 利きを再計算したい
pub fn is_jisatusyu(ml_dto: &MLDto, ss: &MLMovementDto, speed_of_light: &MLSpeedOfLightVo) -> bool {
    // 移動元升、動かした駒の先後、駒種類、
    let km_src = ml_dto
        .get_search_part()
        .get_current_position()
        .get_piece_by_square(&ss.src);
    let ps_src = speed_of_light
        .ml_piece_struct_master_vo
        .get_piece_vo(km_src);
    let (sn_teban, _piece_type) = ps_src.phase_piece_type();
    // 相手番の先後
    let sn_aite = hanten_sn(&sn_teban);

    // 升の利き数だが、指した後で再計算が要るはず
    let kikisu =
        ml_dto.get_search_part().effect_count_by_phase[sn_to_num(&sn_aite)].get_su_by_sq(&ss.dst);
    let result = 0 < kikisu;
    // g_writeln(&format!(
    //     "info is_jisatusyu={} km_src={} sn_teban={} piece_type={} sn_aite={} ss.dst={} kikisu={}"
    //     ,result ,km_src ,sn_teban ,piece_type ,sn_aite ,ss.dst ,kikisu
    // ));

    result
}
