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
use crate::model::vo::game_part::gp_piece_vo::GPPieceVo;

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
        let cap = universe
            .get_search_part_mut()
            .do_move(&movement, speed_of_light);
        let value = match cap {
            GPPieceVo::King1 => 35000,
            GPPieceVo::Rook1 => 1000,
            GPPieceVo::Bishop1 => 900,
            GPPieceVo::Gold1 => 600,
            GPPieceVo::Silver1 => 500,
            GPPieceVo::Knight1 => 300,
            GPPieceVo::Lance1 => 200,
            GPPieceVo::Pawn1 => 100,
            GPPieceVo::Dragon1 => 2000,
            GPPieceVo::Horse1 => 1900,
            GPPieceVo::PromotedSilver1 => 500,
            GPPieceVo::PromotedKnight1 => 300,
            GPPieceVo::PromotedLance1 => 200,
            GPPieceVo::PromotedPawn1 => 100,
            GPPieceVo::King2 => 35000,
            GPPieceVo::Rook2 => 1000,
            GPPieceVo::Bishop2 => 900,
            GPPieceVo::Gold2 => 600,
            GPPieceVo::Silver2 => 500,
            GPPieceVo::Knight2 => 300,
            GPPieceVo::Lance2 => 200,
            GPPieceVo::Pawn2 => 100,
            GPPieceVo::Dragon2 => 2000,
            GPPieceVo::Horse2 => 1900,
            GPPieceVo::PromotedSilver2 => 500,
            GPPieceVo::PromotedKnight2 => 300,
            GPPieceVo::PromotedLance2 => 200,
            GPPieceVo::PromotedPawn2 => 100,
            _ => 0,
        };
        if best_value < value {
            best_movement_hash = *movement_hash;
            best_value = value;
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
