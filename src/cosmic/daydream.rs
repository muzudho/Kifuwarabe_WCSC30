//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Movement, Person, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::Evaluation;
use crate::cosmic::smart::evaluator::LOSE_VALUE;
use crate::cosmic::smart::evaluator::REPITITION_VALUE;
use crate::cosmic::smart::evaluator::WIN_VALUE;
use crate::cosmic::smart::features::PieceType::King;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::PseudoLegalMoves;
use crate::law::speed_of_light::SpeedOfLight;
use std::collections::HashSet;
use std::time::Instant;

pub struct Tree {
    // この木を生成したと同時にストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,
}
impl Default for Tree {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        Tree {
            stopwatch: stopwatch1,
        }
    }
}
impl Tree {
    /// TODO 反復深化探索を入れようと思うんだが、大変だぜ☆（＾～＾）
    pub fn iteration_deeping(
        &mut self,
        speed_of_light: &SpeedOfLight,
        universe: &mut Universe,
    ) -> TreeState {
        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        let mut best_ts = self.root_move(0, speed_of_light, universe);

        // TODO 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for max_depth in 1..universe.option_max_depth {
            let ts = self.root_move(max_depth, speed_of_light, universe);

            if ts.timeout {
                // 時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }

            if best_ts.value <= ts.value {
                best_ts = ts.clone();
            }
        }

        best_ts
    }

    /// 木の根っこで一番良い指し手を返すぜ☆（＾～＾）
    pub fn root_move(
        &mut self,
        max_depth: u8,
        speed_of_light: &SpeedOfLight,
        universe: &mut Universe,
    ) -> TreeState {
        universe.game.info.clear();

        self.search(0, max_depth, &mut universe.game, speed_of_light, "", 0)
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
        &mut self,
        cur_depth: u8,
        end_depth: u8,
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
        // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
        PseudoLegalMoves::make_move(
            game.history.get_phase(Person::Friend),
            &game.board,
            &speed_of_light,
            &mut |movement| {
                &movement_set.insert(movement);
            },
        );

        // Commands::genmove(&speed_of_light, &game);

        // 指せる手が無ければ投了☆（＾～＾）
        if movement_set.is_empty() {
            return ts;
        }

        for movement_hash in movement_set.iter() {
            // 時間を見ようぜ☆（＾～＾）？
            if ts.timeout {
                break;
            } else if 30 < self.stopwatch.elapsed().as_secs() {
                // とりあえず 30 秒で探索を打ち切ろうぜ☆（＾～＾）？
                ts.timeout = true;
                break;
            }

            // 1手進めるぜ☆（＾～＾）
            ts.add_state();
            let movement = Movement::from_hash(*movement_hash);
            let captured_piece = game.do_move(&movement, speed_of_light);
            /*
            IO::debugln(&format!("n={} do.", sum_nodes));
            Commands::pos(&game);
            */

            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.r#type(speed_of_light) == King {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！
                    ts.catch_king(*movement_hash);

                    // 1手戻すぜ☆（＾～＾）
                    game.undo_move(speed_of_light);
                    break;
                }
            }

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_position() {
                // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）
                ts.repetition_movement_hash = *movement_hash;
            } else if end_depth <= cur_depth {
                // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
                let evaluation =
                    Evaluation::from_caputured_piece(cur_depth, captured_piece, speed_of_light);
                ts.check_leaf(&evaluation, *movement_hash);

            // IO::debugln(&format!("n={} Value={}.", sum_nodes, evaluation.value));
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let opponent_ts = self.search(
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
            ts.value = REPITITION_VALUE;
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

#[derive(Clone)]
pub struct TreeState {
    sum_state: u64,
    pub value: i16,

    movement_hash: u64,

    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_movement_hash: u64,
    /// 玉を取ったぜ☆（＾～＾）
    pub king_catched: bool,

    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: String,
    pub timeout: bool,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            sum_state: 0,
            value: LOSE_VALUE,
            movement_hash: 0u64,
            repetition_movement_hash: 0u64,
            king_catched: false,
            reason: "no update".to_string(),
            timeout: false,
        }
    }
}
impl TreeState {
    pub fn get_sum_state(&self) -> u64 {
        self.sum_state
    }

    pub fn get_value(&self) -> i16 {
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

        // 評価値は ひっくり返します。
        let friend_value = -opponent_ts.value;

        if self.movement_hash == 0 {
            // どんな手も 投了より良いだろ☆（＾～＾）
            self.movement_hash = friend_movement_hash;
            self.value = friend_value;
            self.reason = "this better than resign".to_string();
            return;
        } else if self.value < friend_value {
            // 上方修正
            self.value = friend_value;
            self.movement_hash = friend_movement_hash;
            self.reason = "update value".to_string();
            return;
        }
    }

    pub fn check_leaf(&mut self, evaluation: &Evaluation, movement_hash: u64) {
        if self.movement_hash == 0 {
            // どんな葉も 投了より良いだろ☆（＾～＾）
            // TODO 王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
            self.movement_hash = movement_hash;
            self.value = evaluation.value;
            self.reason = "any leaf better than resign".to_string();
            return;
        } else if self.value < evaluation.value {
            self.movement_hash = movement_hash;
            self.value = evaluation.value;
            self.reason = "good position".to_string();
            return;
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
        self.value = WIN_VALUE;
        self.king_catched = true;
        self.reason = "king catch is strongest".to_string();
    }
}
