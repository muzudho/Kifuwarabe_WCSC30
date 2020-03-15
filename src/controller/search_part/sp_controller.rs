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

/// 探索結果。
pub struct SPBestmove {
    pub movement: MLMovementDto,
    pub changed_value: i16,
    pub sum_nodes: u64,
    /// 相手の玉を捕まえた。
    pub captured_opponent_king: u16,
    /// 自分の玉が捕まった。
    pub my_king_has_been_caught: u16,
}
impl SPBestmove {
    pub fn new(
        movement1: MLMovementDto,
        changed_value1: i16,
        sum_nodes1: u64,
        captured_opponent_king1: u16,
        my_king_has_been_caught1: u16,
    ) -> Self {
        SPBestmove {
            movement: movement1,
            changed_value: changed_value1,
            sum_nodes: sum_nodes1,
            captured_opponent_king: captured_opponent_king1,
            my_king_has_been_caught: my_king_has_been_caught1,
        }
    }
}

struct BestmoveState {
    movement_hash: u64,
    value: i16,
    /// 相手の王を捕まえるまでのターン。0なら未確定。
    captured_opponent_king: u16,
    /// 自分の王が捕まえるまでのターン。0なら未確定。
    my_king_has_been_caught: u16,
}
impl BestmoveState {
    pub fn new() -> Self {
        BestmoveState {
            movement_hash: 0u64,
            value: -1,
            captured_opponent_king: 0,
            my_king_has_been_caught: 0,
        }
    }

    pub fn get_movement_hash(&self) -> u64 {
        self.movement_hash
    }

    pub fn get_value(&self) -> i16 {
        self.value
    }

    /*
    pub fn get_captured_opponent_king(&self) -> u16 {
        self.captured_opponent_king
    }

    pub fn get_my_king_has_been_caught(&self) -> u16 {
        self.my_king_has_been_caught
    }
    */

    pub fn catch_opponent_king(&mut self) {
        self.captured_opponent_king += 1;
    }

    /// 自玉が相手の玉より先に取られるか☆（＾～＾）？
    pub fn is_losing(&self) -> bool {
        0 < self.my_king_has_been_caught
            && self.captured_opponent_king < self.my_king_has_been_caught
    }

    pub fn update(&mut self, changed_value: i16, movement_hash1: u64) {
        if self.is_losing() {
            // 自玉が相手の玉より先に取られる手が、良いわけ無いぜ☆（＾～＾）
            return;
        }

        if self.value < changed_value {
            self.movement_hash = movement_hash1;
            self.value = changed_value;
        }
    }

    pub fn swap_phase(&mut self) {
        let temp = self.my_king_has_been_caught;
        self.my_king_has_been_caught = self.captured_opponent_king;
        self.captured_opponent_king = temp;
    }
}

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
) -> Option<SPBestmove> {
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
        universe
            .get_mut_info()
            .print(cur_depth, sum_nodes, best_value, &resign_move);
        return None;
    }

    // TODO その中から１手指して、局面を進めるぜ☆（＾～＾）評価値は差分更新したいぜ☆（＾～＾）
    let mut bestmove_state = BestmoveState::new();
    // 千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    let mut repetition_move_hash = 0u64;
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
            let (changed_value, king_catch1) =
                SPEvaluationController::evaluate(captured_piece, speed_of_light);
            sum_nodes += 1;

            bestmove_state.update(changed_value, *movement_hash);

            universe.get_mut_info().print(
                cur_depth,
                sum_nodes,
                bestmove_state.get_value(),
                &MLMovementDto::from_hash(bestmove_state.get_movement_hash()),
            );

            if king_catch1 {
                // 王を取る手より良い手は無いので探索終了☆（＾～＾）
                bestmove_state.catch_opponent_king();
                break;
            }
        } else {
            // 枝局面なら、更に深く進むぜ☆（＾～＾）
            match get_best_movement(
                cur_depth + 1,
                end_depth,
                sum_nodes + 1,
                universe,
                speed_of_light,
            ) {
                Some(opponent_best_move) => {
                    let changed_value = -opponent_best_move.changed_value;
                    sum_nodes = opponent_best_move.sum_nodes;
                    bestmove_state.update(changed_value, *movement_hash);

                    universe.get_mut_info().print(
                        cur_depth,
                        sum_nodes,
                        bestmove_state.get_value(),
                        &MLMovementDto::from_hash(bestmove_state.get_movement_hash()),
                    );

                    bestmove_state.swap_phase();
                    if bestmove_state.is_losing() {
                        // 次の手で、相手が王を捕まえているようでは、こんな手を指してはいけないぜ☆（＾～＾）
                        // 探索はさっさと終了だぜ☆（＾～＾）
                        break;
                    }
                }
                None => {}
            }
        }
        // 1手戻すぜ☆（＾～＾）
        universe
            .get_search_part_mut()
            .undo_move(&movement, speed_of_light)
    }

    let best_movement = if bestmove_state.get_movement_hash() != 0 {
        MLMovementDto::from_hash(bestmove_state.get_movement_hash())
    } else {
        // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
        MLMovementDto::from_hash(repetition_move_hash)
    };

    // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
    universe.get_mut_info().print(
        cur_depth,
        sum_nodes,
        bestmove_state.get_value(),
        &best_movement,
    );

    Some(SPBestmove::new(
        best_movement,
        bestmove_state.get_value(),
        sum_nodes,
        bestmove_state.captured_opponent_king,
        bestmove_state.my_king_has_been_caught,
    ))
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
            g_writeln(&format!("info string solution:{}.", best_movement));
            return best_movement;
        }
    }

    // 投了
    MLMovementDto::default()
    */
}
