//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::shogi::playing::Game;
use crate::cosmic::shogi::recording::{Movement, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::Evaluation;
use crate::cosmic::smart::features::PieceType::King;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::movement_generator::generate_movement;
use crate::law::speed_of_light::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct TreeState {
    sum_state: u64,
    pub value: Option<i16>,

    movement_hash: u64,

    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_movement_hash: u64,
    /// 玉を取ったぜ☆（＾～＾）
    pub king_catched: bool,

    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: String,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            sum_state: 0,
            value: None,
            movement_hash: 0u64,
            repetition_movement_hash: 0u64,
            king_catched: false,
            reason: "no update".to_string(),
        }
    }
}
impl TreeState {
    pub fn get_sum_state(&self) -> u64 {
        self.sum_state
    }

    pub fn get_value(&self) -> Option<i16> {
        self.value
    }

    pub fn was_king_catch(&self) -> bool {
        self.king_catched
    }

    pub fn add_state(&mut self) {
        self.sum_state += 1;
    }

    pub fn add_turn_over(&mut self, opponent_ts: &TreeState, friend_movement_hash: u64) {
        self.sum_state += opponent_ts.get_sum_state();

        if opponent_ts.was_king_catch() {
            // この手を指すと、次に相手に玉を取られるぜ☆（＾～＾）！
            // アップデートせずに終了☆（＾～＾）！
            return;
        }

        if let Some(opponent_value) = opponent_ts.value {
            // 評価値は ひっくり返します。
            let friend_value = -opponent_value;

            if self.movement_hash == 0 {
                // どんな手も 投了より良いだろ☆（＾～＾）
                self.movement_hash = friend_movement_hash;
                self.value = Some(friend_value);
                self.reason = "this better than resign".to_string();
                return;
            } else if let Some(current_value) = self.value {
                if current_value < friend_value {
                    // 更新
                    self.value = Some(friend_value);
                    self.movement_hash = friend_movement_hash;
                    self.reason = "update value".to_string();
                    return;
                }
            }
        } else {
            panic!("評価値が付いてないぜ☆（＾～＾）！")
        }
    }

    pub fn check_leaf(&mut self, evaluation: &Evaluation, movement_hash: u64) {
        if self.movement_hash == 0 {
            // どんな手も 投了より良いだろ☆（＾～＾）
            self.movement_hash = movement_hash;
            self.value = Some(evaluation.value);
            self.reason = "this better than resign".to_string();
            return;
        } else if let Some(current_value) = self.get_value() {
            if current_value < evaluation.value {
                self.movement_hash = movement_hash;
                self.value = Some(evaluation.value);
                self.reason = "good position".to_string();
                return;
            }
        }
    }

    pub fn get_movement_hash(&self) -> u64 {
        self.movement_hash
    }

    pub fn to_movement(&self) -> Movement {
        Movement::from_hash(self.movement_hash)
    }

    pub fn catch_king(&mut self, movement_hash: u64) {
        // 玉を取る手より強い手はないぜ☆（＾～＾）！
        self.movement_hash = movement_hash;
        self.value = None;
        self.king_catched = true;
        self.reason = "king catch is strongest".to_string();
    }
}

pub struct Tree {}
impl Tree {
    pub fn first_move(speed_of_light: &SpeedOfLight, universe: &mut Universe) -> TreeState {
        universe.game.info.clear();

        Tree::search(
            0,
            universe.option_max_depth - 1,
            &mut universe.game,
            speed_of_light,
            "",
            0,
        )
    }

    /// 先手の気持ちで、勝てだぜ☆（*＾～＾*）
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
    fn search(
        cur_depth: u16,
        end_depth: u16,
        game: &mut Game,
        speed_of_light: &SpeedOfLight,
        pv: &str,
        parent_sum_nodes: u64,
    ) -> TreeState {
        let mut ts = TreeState::default();
        // 指し手の一覧を作るぜ☆（＾～＾） 指し手はハッシュ値で入っている☆（＾～＾）
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
            return ts;
        }

        for movement_hash in movement_set.iter() {
            // 1手進めるぜ☆（＾～＾）
            ts.add_state();
            let movement = Movement::from_hash(*movement_hash);
            let captured_piece = game.do_move(&movement, speed_of_light);
            /*
            IO::debugln(&format!("n={} do.", sum_nodes));
            Commands::pos(&game);
            */

            if let Some(cap) = captured_piece {
                if speed_of_light.get_piece_chart(&cap).piece_type() == King {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！
                    ts.catch_king(*movement_hash);

                    // 1手戻すぜ☆（＾～＾）
                    game.undo_move(speed_of_light);
                    break;
                }
            }

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_ky() {
                // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）
                ts.repetition_movement_hash = *movement_hash;
            } else if end_depth <= cur_depth {
                // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
                let evaluation = Evaluation::from_caputured_piece(captured_piece, speed_of_light);
                ts.check_leaf(&evaluation, *movement_hash);

            // IO::debugln(&format!("n={} Value={}.", sum_nodes, evaluation.value));
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let opponent_ts = Tree::search(
                    cur_depth + 1,
                    end_depth,
                    game,
                    speed_of_light,
                    &format!("{} {}", pv, Movement::from_hash(*movement_hash)),
                    ts.get_sum_state() + parent_sum_nodes,
                );

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                ts.add_turn_over(&opponent_ts, *movement_hash);
            }
            // 1手戻すぜ☆（＾～＾）
            game.undo_move(speed_of_light);
            /*
            IO::debugln(&format!("n={} undo.", sum_nodes));
            Commands::pos(&game);
            */
        }

        if ts.get_movement_hash() == 0 && ts.repetition_movement_hash != 0 {
            // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
            ts.movement_hash = ts.repetition_movement_hash;
            ts.value = Some(0);
            ts.reason = "repetition better than resign".to_string();
        };

        if game.info.is_printable() {
            // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
            game.info.print(
                cur_depth,
                ts.get_sum_state() + parent_sum_nodes,
                ts.get_value(),
                ts.movement_hash,
                Some(format!("{} {}", pv, ts.to_movement())),
                None,
            );
        }

        ts
    }
}
