//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

extern crate rand;
use rand::Rng;
use std::collections::HashSet;

use super::super::super::controller::movement_generation::mg_choicing_controller::*;
use super::super::super::controller::movement_generation::mg_controller::*;
use super::super::super::controller::search_part::sp_kikisu_controller::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;

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
    // TODO 指し手の一覧を作るぜ☆（＾～＾）
    // TODO その中から１手指して、局面を進めるぜ☆（＾～＾）
    // TODO 進めた局面に評価値を付けるぜ☆（＾～＾）
    // TODO 繰り返すぜ☆（＾～＾）
    // TODO 一番良い評価値になる１手を選ぶぜ☆（＾～＾）それが最善手だぜ☆（＾～＾）
    get_best_movement(universe, speed_of_light)
}

/// 最善手を返すぜ☆（＾～＾）
fn get_best_movement(
    universe: &mut MLUniverseDto,
    speed_of_light: &MLSpeedOfLightVo,
) -> MLMovementDto {
    // 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）
    // 相手の利き升調べ（自殺手、特に王手放置回避漏れ　防止のため）
    {
        update_effect_count(universe, speed_of_light);
    }

    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
    // 指し手はハッシュ値で入っている☆（＾～＾）
    let mut movement_set = HashSet::<u64>::new();

    // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
    get_up_potential_movement(&universe.get_search_part(), &speed_of_light, |movement| {
        movement_set.insert(movement);
    });
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &ss_hashset );

    select_movement_except_check(
        &mut movement_set,
        &universe.get_search_part(),
        &speed_of_light,
    );

    // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
    select_movement_except_fourfold_repetition(&mut movement_set, universe, speed_of_light);

    // 自殺手は省くぜ☆（＾～＾）
    select_movement_except_suiceid(&mut movement_set, universe, speed_of_light);

    if movement_set.is_empty() {
        // 投了
        MLMovementDto::default()
    } else {
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
    }
}
