//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

extern crate rand;
// use rand::Rng;
use crate::model::vo::other_part::op_ply_vo::SENNTITE_NUM;
use std::collections::HashSet;

use super::super::super::controller::movement_generation::mg_controller::*;
use super::super::super::controller::search_part::sp_control_count_controller::*;
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
/// * `depth` - 0 なら末端局面、1 なら末端局面の1歩手前☆（＾～＾）
/// * `universe` - (宇宙)
/// * `speed_of_light` - (光速)
///
/// # Returns
///
/// Best movement, Value, Sum nodes
pub fn get_best_movement(
    cur_depth: u16,
    end_depth: u16,
    mut sum_nodes: u64,
    universe: &mut MLUniverseDto,
    speed_of_light: &MLSpeedOfLightVo,
) -> (MLMovementDto, i16, u64) {
    {
        // 指定局面の利き数ボード再計算。
        // 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）
        // 相手の利き升調べ（自殺手、特に王手放置回避漏れ　防止のため）
        recalculate_control_count(universe, speed_of_light);
    }
    // TODO 利きの差分更新をしたいぜ☆（＾～＾）
    //
    //  ・先後別の２つの利き数ボード、駒別の約３０種類の利き数ボードがあって、全て最新にすること。
    //  ・position で送られてくる指定局面は、一から全再計算☆（＾～＾）
    //  ・指す前の局面でやること。
    //      ・自分の盤上の駒を動かす前に、レイを飛ばして飛角香を逆探知すること☆（＾～＾）
    //      ・取られる駒がある場合、両者の駒を動かす前に、取られる駒の利きをスキャンすること☆（＾～＾）
    //  ・指した後の局面でやること。
    //      ・自分の駒を動かした先で、レイを飛ばして飛角香を逆探知すること☆（＾～＾）
    //      ・動かした駒が　飛角香なら、探知☆（＾～＾）ある程度パターンがあるはず☆（＾～＾）
    //      ・それ以外の駒は、差分のパターンが決まっているので、それに従って増減させること☆（＾～＾）
    //
    // 入力：　手番、移動元、動かした駒種類、移動先の取られる駒。
    // やること：　カウントを引くか、足す。
    //
    // do_move, undo_move. 両方用意すること☆（＾～＾）
    //
    // TODO 指し手の一覧を作るぜ☆（＾～＾）
    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
    // 指し手はハッシュ値で入っている☆（＾～＾）
    let mut movement_set = HashSet::<u64>::new();

    // TODO do_ss とか局面を動かすところで、フリーズしている？
    generate_movement(universe, speed_of_light, &mut movement_set);

    // 指せる手が無ければ投了☆（＾～＾）
    if movement_set.is_empty() {
        let best_value = 0;
        let resign_move = MLMovementDto::default();
        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        g_writeln(&format!(
            "info depth {} nodes {} score cp {} currmove {}",
            cur_depth, sum_nodes, best_value, resign_move
        ));
        return (resign_move, 0, sum_nodes);
    }

    // TODO その中から１手指して、局面を進めるぜ☆（＾～＾）評価値は差分更新したいぜ☆（＾～＾）
    let mut best_movement_hash = 0u64;
    let mut best_value = -1;
    let mut repetition_move_hash = 0u64; // 千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    for movement_hash in movement_set.iter() {
        // 1手進めるぜ☆（＾～＾）
        let movement = GPMovementVo::from_hash(*movement_hash);
        let captured_piece = universe
            .get_search_part_mut()
            .do_move(&movement, speed_of_light);

        // 千日手かどうかを判定する☆（＾～＾）
        if SENNTITE_NUM <= universe.count_same_ky() {
            // 千日手なら、この手は戻そうぜ☆（＾～＾）
            repetition_move_hash = *movement_hash;
        } else if end_depth <= cur_depth {
            // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
            let changed_value = SPEvaluationController::evaluate(captured_piece, speed_of_light);
            sum_nodes += 1;
            if best_value < changed_value {
                best_movement_hash = *movement_hash;
                best_value = changed_value;
            }
            // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
            g_writeln(&format!(
                "info depth {} nodes {} score cp {} currmove {}",
                cur_depth,
                sum_nodes,
                best_value,
                MLMovementDto::from_hash(best_movement_hash)
            ));
        } else {
            // 枝局面なら、更に深く進むぜ☆（＾～＾）
            let opponent_best_move = get_best_movement(
                cur_depth + 1,
                end_depth,
                sum_nodes + 1,
                universe,
                speed_of_light,
            );
            let changed_value = -opponent_best_move.1;
            sum_nodes = opponent_best_move.2;
            if best_value < changed_value {
                best_movement_hash = *movement_hash;
                best_value = changed_value;
            }
            // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
            g_writeln(&format!(
                "info depth {} nodes {} score cp {} currmove {}",
                cur_depth,
                sum_nodes,
                best_value,
                MLMovementDto::from_hash(best_movement_hash)
            ));
        }
        // 1手戻すぜ☆（＾～＾）
        universe
            .get_search_part_mut()
            .undo_move(&movement, speed_of_light)
    }

    let best_movement = if best_movement_hash != 0 {
        MLMovementDto::from_hash(best_movement_hash)
    } else {
        // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
        MLMovementDto::from_hash(repetition_move_hash)
    };

    // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
    g_writeln(&format!(
        "info depth {} nodes {} score cp {} currmove {}",
        cur_depth, sum_nodes, best_value, best_movement
    ));

    (best_movement, best_value, sum_nodes)
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
