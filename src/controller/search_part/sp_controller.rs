//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

extern crate rand;
use crate::model::univ::gam::history::SENNTITE_NUM;
use std::collections::HashSet;

use super::sp_evaluation_controller::*;
use crate::controller::movement_generation::mg_controller::*;
use crate::controller::search_part::sp_control_count_controller::*;
use crate::model::univ::gam::misc::movement::Movement;
use crate::model::univ::gam::misc::movement_builder::*;
use crate::model::univ::speed_of_light::*;
use crate::model::universe::*;

/// 将来の結果を、現在に遡って持ってくる方向の結果。
pub struct SPBestmove {
    pub movement: MovementBuilder,
    pub changed_value: i16,
    pub sum_nodes: u64,
    /// らいおんきゃっち数。玉を取ったら1。
    /// 現局面では、玉を取られた時は偶数、玉を取った時は奇数になる。
    pub raioncatch_number: i16,
}
impl SPBestmove {
    pub fn new(
        movement1: MovementBuilder,
        changed_value1: i16,
        sum_nodes1: u64,
        raioncatch_number1: i16,
    ) -> Self {
        SPBestmove {
            movement: movement1,
            changed_value: changed_value1,
            sum_nodes: sum_nodes1,
            raioncatch_number: raioncatch_number1,
        }
    }
}

/// 兄弟局面（横方向）と比較しての結果。
struct SiblingBestmoveState {
    movement_hash: u64,
    pub value: i16,
    /// らいおんきゃっち数。玉を取ったら1。
    /// 現局面では、玉を取られた時は偶数、玉を取った時は奇数になる。
    raioncatch_number: i16,
}
impl SiblingBestmoveState {
    pub fn new() -> Self {
        SiblingBestmoveState {
            movement_hash: 0u64,
            value: -1,
            raioncatch_number: 32767,
        }
    }

    pub fn get_movement_hash(&self) -> u64 {
        self.movement_hash
    }

    pub fn catch_king(&mut self) {
        self.raioncatch_number = std::cmp::min(self.raioncatch_number, 1);
    }

    pub fn catch_no_king(&mut self) {
        self.raioncatch_number = std::cmp::min(self.raioncatch_number, 0);
    }

    pub fn update_bestmove(&mut self, changed_value: i16, movement_hash1: u64) -> bool {
        if self.value < changed_value {
            self.movement_hash = movement_hash1;
            self.value = changed_value;
            return true;
        }
        false
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
/// * `pv` - 読み筋
///
/// # Returns
///
/// Best movement, Value, Sum nodes
pub fn get_best_movement(
    cur_depth: u16,
    end_depth: u16,
    mut sum_nodes: u64,
    universe: &mut Universe,
    speed_of_light: &MLSpeedOfLightVo,
    pv: &str,
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
        let resign_move = MovementBuilder::default();
        universe.game.info.print(
            cur_depth,
            sum_nodes,
            best_value,
            &resign_move,
            &format!("{} {} EmptyMoves", pv, resign_move),
        );
        return None;
    }

    // TODO その中から１手指して、局面を進めるぜ☆（＾～＾）評価値は差分更新したいぜ☆（＾～＾）
    let mut bestmove_state = SiblingBestmoveState::new();
    // 千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    let mut repetition_move_hash = 0u64;
    for movement_hash in movement_set.iter() {
        // 1手進めるぜ☆（＾～＾）
        let movement = Movement::from_hash(*movement_hash);
        let captured_piece = universe.game.do_move(&movement, speed_of_light);

        // 千日手かどうかを判定する☆（＾～＾）
        if SENNTITE_NUM <= universe.game.count_same_ky() {
            // 千日手なら、この手は戻そうぜ☆（＾～＾）
            repetition_move_hash = *movement_hash;
        } else if end_depth <= cur_depth {
            // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
            let (changed_value, king_catch1) =
                SPEvaluationController::evaluate(captured_piece, speed_of_light);
            sum_nodes += 1;

            if king_catch1 {
                bestmove_state.catch_king();
            } else {
                bestmove_state.catch_no_king();
            }

            if bestmove_state.update_bestmove(changed_value, *movement_hash) {}
            let movement = MovementBuilder::from_hash(*movement_hash);
            universe.game.info.print(
                cur_depth,
                sum_nodes,
                bestmove_state.value,
                &movement,
                &format!("{} {} EndNode", pv, movement),
            );
        } else {
            // 枝局面なら、更に深く進むぜ☆（＾～＾）
            match get_best_movement(
                cur_depth + 1,
                end_depth,
                sum_nodes + 1,
                universe,
                speed_of_light,
                &format!("{} {}", pv, MovementBuilder::from_hash(*movement_hash)),
            ) {
                Some(opponent_best_move) => {
                    sum_nodes = opponent_best_move.sum_nodes;

                    let changed_value: i16 = if opponent_best_move.raioncatch_number == 0 {
                        // 玉を取ったり取られたりしないぜ☆（＾～＾）
                        -opponent_best_move.changed_value
                    } else if opponent_best_move.raioncatch_number == 1 {
                        // 次の相手の番に玉を取られてしまうぜ☆（＾～＾）！王手回避漏れか自殺手になってしまうぜ☆（＾～＾）！
                        // こんな手を指し手はいけないぜ☆（＾～＾）！
                        return None;
                    } else {
                        bestmove_state.raioncatch_number = opponent_best_move.raioncatch_number + 1;
                        if bestmove_state.raioncatch_number % 2 == 0 {
                            // 相手に玉を取られる詰めろだぜ☆（＾～＾）
                            -30000 + bestmove_state.raioncatch_number
                        } else {
                            // 相手の玉を取る詰めろだぜ☆（＾～＾）
                            30000 - bestmove_state.raioncatch_number
                        }
                    };

                    if bestmove_state.update_bestmove(changed_value, *movement_hash) {}
                    let movement = &MovementBuilder::from_hash(*movement_hash);
                    universe.game.info.print(
                        cur_depth,
                        sum_nodes,
                        bestmove_state.value,
                        movement,
                        &format!("{} {} Backward1", pv, movement),
                    );
                }
                None => {}
            }
        }
        // 1手戻すぜ☆（＾～＾）
        universe.game.undo_move(&movement, speed_of_light)
    }

    let best_movement = if bestmove_state.get_movement_hash() != 0 {
        MovementBuilder::from_hash(bestmove_state.get_movement_hash())
    } else {
        // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
        MovementBuilder::from_hash(repetition_move_hash)
    };

    // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
    universe.game.info.print(
        cur_depth,
        sum_nodes,
        bestmove_state.value,
        &best_movement,
        &format!("{} {} Backward2", pv, best_movement),
    );

    Some(SPBestmove::new(
        best_movement,
        bestmove_state.value,
        sum_nodes,
        bestmove_state.raioncatch_number,
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
