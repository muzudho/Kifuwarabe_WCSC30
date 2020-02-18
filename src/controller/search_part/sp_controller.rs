//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

extern crate rand;
// use rand::Rng;
use std::collections::HashSet;

use super::super::super::controller::movement_generation::mg_controller::*;
use super::super::super::controller::search_part::sp_kikisu_controller::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::model::vo::game_part::gp_movement_vo::GPMovementVo;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::sp_evaluation_controller::*;

/// Let there be light. (光在れ)
/// 現局面での最善手を返すぜ☆（*＾～＾*）
///
/// # Arguments
///
/// * `universe` - (宇宙)
/// * `speed_of_light` - (光速)
///
/// # Returns
///
/// Best movement.
pub fn let_there_be_light(
    universe: &mut MLUniverseDto,
    speed_of_light: &MLSpeedOfLightVo,
) -> MLMovementDto {
    // 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）
    // 相手の利き升調べ（自殺手、特に王手放置回避漏れ　防止のため）
    {
        update_effect_count(universe, speed_of_light);
    }
    // TODO 指し手の一覧を作るぜ☆（＾～＾）
    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
    // 指し手はハッシュ値で入っている☆（＾～＾）
    let mut movement_set = HashSet::<u64>::new();
    get_up_movement(universe, speed_of_light, &mut movement_set);
    // 指せる手が無ければ投了☆（＾～＾）
    if movement_set.is_empty() {
        return MLMovementDto::default();
    }
    // TODO その中から１手指して、局面を進めるぜ☆（＾～＾）評価値は差分更新したいぜ☆（＾～＾）
    let mut best_movement_hash = 0u64;
    let mut best_value = -1;
    for movement_hash in movement_set.iter() {
        let movement = GPMovementVo::from_hash(*movement_hash);
        let captured_piece = universe
            .get_search_part_mut()
            .do_move(&movement, speed_of_light);
        // 変化した評価値
        let changed_value = SPEvaluationController::evaluate(captured_piece, speed_of_light);
        if best_value < changed_value {
            best_movement_hash = *movement_hash;
            best_value = changed_value;
        }
        universe
            .get_search_part_mut()
            .undo_move(&movement, speed_of_light)
    }
    MLMovementDto::from_hash(best_movement_hash)
    /*
    // TODO 進めた局面に評価値を付けるぜ☆（＾～＾）
    // TODO 繰り返すぜ☆（＾～＾）
    // TODO 一番良い評価値になる１手を選ぶぜ☆（＾～＾）それが最善手だぜ☆（＾～＾）
    // 最善手を返すぜ☆（＾～＾）
    let index = rand::thread_rng().gen_range(0, movement_set.len());
    for (i, ss_hash) in movement_set.into_iter().enumerate() {
        if i == index {
            //let result : MLMovementDto = ss.clone();
            let best_movement = MLMovementDto::from_hash(ss_hash);
            g_writeln(&format!("info solution:{}.", best_movement));
            return best_movement;
        }
    }

    // 投了
    MLMovementDto::default()
    */
}
