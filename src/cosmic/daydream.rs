//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::shogi::playing::Game;
use crate::cosmic::shogi::recording::{Movement, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::Evaluation;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::movement_generator::generate_movement;
use crate::law::speed_of_light::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct NodeCounter {
    sum_state: u64,
}
impl Default for NodeCounter {
    fn default() -> Self {
        NodeCounter { sum_state: 0 }
    }
}
impl NodeCounter {
    pub fn add_state(&mut self) {
        self.sum_state += 1;
    }

    pub fn get_sum_state(&self) -> u64 {
        self.sum_state
    }

    pub fn add_sum(&mut self, nc: &NodeCounter) {
        self.sum_state += nc.get_sum_state();
    }
}

pub struct Tree {}
impl Tree {
    pub fn first_move(speed_of_light: &SpeedOfLight, universe: &mut Universe) -> Bestmove {
        universe.game.info.clear();

        Tree::get_best_movement(
            0,
            universe.option_max_depth - 1,
            &mut universe.game,
            speed_of_light,
            "",
        )
    }

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
    fn get_best_movement(
        cur_depth: u16,
        end_depth: u16,
        game: &mut Game,
        speed_of_light: &SpeedOfLight,
        pv: &str,
    ) -> Bestmove {
        let mut nc = NodeCounter::default();
        /* TODO
        {
            // 指定局面の利き数ボード再計算。
            // 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）
            // 相手の利き升調べ（自殺手、特に王手放置回避漏れ　防止のため）
            recalculate_control_count(game, speed_of_light);
        }
        */
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

        /*
        IO::debugln(&format!(
            "n={} friend={}.",
            sum_nodes,
            game.history.get_phase(&Person::Friend)
        ));
        */
        generate_movement(game, speed_of_light, &mut movement_set);
        // Commands::genmove(&speed_of_light, &game);

        // 指せる手が無ければ投了☆（＾～＾）
        if movement_set.is_empty() {
            let best_value = 0;
            let resign_move = Movement::default();
            game.info.print(
                cur_depth,
                nc.get_sum_state(),
                best_value,
                &resign_move,
                &format!("{} {} EmptyMoves", pv, resign_move),
                false,
            );
            return Bestmove::new(
                resign_move,
                best_value,
                nc,
                0,
                "Saseru te nakatta.".to_string(),
            );
        }

        // TODO その中から１手指して、局面を進めるぜ☆（＾～＾）評価値は差分更新したいぜ☆（＾～＾）
        let mut sibling_bestmove = SiblingBestmove::new();
        // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
        let mut repetition_move_hash = 0u64;
        for movement_hash in movement_set.iter() {
            // 1手進めるぜ☆（＾～＾）
            nc.add_state();
            let movement = Movement::from_hash(*movement_hash);
            let captured_piece = game.do_move(&movement, speed_of_light);
            /*
            IO::debugln(&format!("n={} do.", sum_nodes));
            Commands::pos(&game);
            */

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_ky() {
                // 千日手なら、この手は戻そうぜ☆（＾～＾）
                repetition_move_hash = *movement_hash;
            } else if end_depth <= cur_depth {
                // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
                let evaluation = Evaluation::from_caputured_piece(captured_piece, speed_of_light);

                if sibling_bestmove.update_bestmove(&evaluation, *movement_hash) {}
                let movement = Movement::from_hash(*movement_hash);
                game.info.print(
                    cur_depth,
                    nc.get_sum_state(),
                    sibling_bestmove.value,
                    &movement,
                    &format!("{} {} EndNode", pv, movement),
                    false,
                );
            // IO::debugln(&format!("n={} Value={}.", sum_nodes, evaluation.value));
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let opponent_best_move = Tree::get_best_movement(
                    cur_depth + 1,
                    end_depth,
                    game,
                    speed_of_light,
                    &format!("{} {}", pv, Movement::from_hash(*movement_hash)),
                );

                nc.add_sum(&opponent_best_move.node_counter);

                let changed_value: i16 = if opponent_best_move.movement.resign() {
                    // 相手が投了してるなら、良い手だぜ☆（＾～＾）！
                    30000
                } else if opponent_best_move.lion_catch == 1 {
                    // 次の相手の番に玉を取られてしまうぜ☆（＾～＾）！王手回避漏れか自殺手になってしまうぜ☆（＾～＾）！
                    // こんな手を指し手はいけないぜ☆（＾～＾）！
                    -30000
                } else if opponent_best_move.lion_catch % 2 == 1 {
                    // 将来的に玉を取られてしまう詰めろに入ってるぜ☆（＾～＾）！
                    // こんな手を指し手はいけないぜ☆（＾～＾）！
                    -30000
                } else {
                    // それ以外なら別に☆（＾～＾）相手が得しない手を選ぼうぜ☆（＾～＾）
                    -opponent_best_move.changed_value
                };

                if sibling_bestmove
                    .update_bestmove(&Evaluation::new(changed_value, false), *movement_hash)
                {
                    let movement = &Movement::from_hash(*movement_hash);
                    game.info.print(
                        cur_depth,
                        nc.get_sum_state(),
                        sibling_bestmove.value,
                        movement,
                        &format!("{} {} Backward1", pv, movement),
                        false,
                    );
                }
            }
            // 1手戻すぜ☆（＾～＾）
            game.undo_move(speed_of_light);
            /*
            IO::debugln(&format!("n={} undo.", sum_nodes));
            Commands::pos(&game);
            */
        }

        // メートを調べようぜ☆（＾～＾）
        if sibling_bestmove.lion_catch < 1 {
            if sibling_bestmove.lion_catched {
                // 相手玉を取ることがここで確定するぜ☆（＾～＾）
                sibling_bestmove.lion_catch = 1;
            }
        } else {
            // カウントアップ☆（＾～＾）
            sibling_bestmove.lion_catch += 1;
        }

        let best_movement = if sibling_bestmove.get_movement_hash() != 0 {
            Movement::from_hash(sibling_bestmove.get_movement_hash())
        } else {
            // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
            Movement::from_hash(repetition_move_hash)
        };

        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        game.info.print(
            cur_depth,
            nc.get_sum_state(),
            sibling_bestmove.value,
            &best_movement,
            &format!("{} {} Backward2", pv, best_movement),
            false,
        );

        Bestmove::new(
            best_movement,
            sibling_bestmove.value,
            nc,
            sibling_bestmove.lion_catch,
            "Searching...".to_string(),
        )
    }
}

/// 将来の結果を、現在に遡って持ってくる方向の結果。
pub struct Bestmove {
    pub movement: Movement,
    pub changed_value: i16,

    pub node_counter: NodeCounter,
    /// 玉を取ったり取られたり、取られなかったり、まだ確定していなければ 0 が入っている。
    /// 相手の玉を取ることが確定していたら 1 が入っている。
    /// 相手に玉を取られることが確定していたら 2 が入っている。
    /// 相手に玉を取られたことが確定していたら 奇数の正の数が入っている。
    /// 相手の玉を取ったことが確定していたら 0より大きい偶数の正の数が入っている。
    /// この数を 1 引いたものが、いわゆる mate (メート)。 mate 7 なら 7手あれば相手玉を詰めることができる。
    pub lion_catch: u16,
    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: String,
}
impl Bestmove {
    pub fn new(
        movement1: Movement,
        changed_value1: i16,
        node_counter1: NodeCounter,
        lion_catch1: u16,
        reason1: String,
    ) -> Self {
        Bestmove {
            movement: movement1,
            changed_value: changed_value1,
            node_counter: node_counter1,
            lion_catch: lion_catch1,
            reason: reason1,
        }
    }
}

/// 兄弟局面（横方向）と比較しての結果。
struct SiblingBestmove {
    movement_hash: u64,
    pub value: i16,
    /// 玉を取ったり取られたり、取られなかったり、まだ確定していなければ 0 が入っている。
    /// 相手の玉を取ることが確定していたら 1 が入っている。
    /// 相手に玉を取られることが確定していたら 2 が入っている。
    /// 相手に玉を取られたことが確定していたら 奇数の正の数が入っている。
    /// 相手の玉を取ったことが確定していたら 0より大きい偶数の正の数が入っている。
    /// この数を 1 引いたものが、いわゆる mate (メート)。 mate 7 なら 7手あれば相手玉を詰めることができる。
    pub lion_catch: u16,
    /// 玉を取る手が存在すれば真☆　必ずその指し手を選ぶだろう☆（＾～＾）
    pub lion_catched: bool,
}
impl SiblingBestmove {
    pub fn new() -> Self {
        SiblingBestmove {
            movement_hash: 0u64,
            value: -1,
            lion_catch: 0,
            lion_catched: false,
        }
    }

    pub fn get_movement_hash(&self) -> u64 {
        self.movement_hash
    }

    pub fn update_bestmove(&mut self, evaluation: &Evaluation, movement_hash1: u64) -> bool {
        if self.lion_catched {
            // すでにライオンキャッチする手を見つけているから、更新しないぜ☆（＾～＾）
            return false;
        } else if evaluation.king_catch {
            self.movement_hash = movement_hash1;
            self.value = evaluation.value;
            self.lion_catched = true;
            return true;
        } else if self.value < evaluation.value {
            self.movement_hash = movement_hash1;
            self.value = evaluation.value;
            return true;
        }
        false
    }
}
