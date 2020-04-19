//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Movement, Person, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::Evaluation;
use crate::cosmic::smart::evaluator::REPITITION_VALUE;
use crate::cosmic::smart::features::PieceType::King;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::PseudoLegalMoves;
use crate::law::speed_of_light::SpeedOfLight;
use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

pub struct Tree {
    // この木を生成したと同時にストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,

    // Principal variation(読み筋)☆（＾～＾）
    pv: PrincipalVariation,
}
impl Default for Tree {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        Tree {
            stopwatch: stopwatch1,
            pv: PrincipalVariation::default(),
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
        universe.game.info.clear();
        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        let mut best_ts = self.search(0, 0, &mut universe.game, speed_of_light);

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for max_depth in 1..universe.option_max_depth {
            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            let movement = best_ts.to_movement();
            universe.game.info.print(
                Some(max_depth),
                Some(best_ts.get_sum_state()),
                Some(best_ts.value()),
                Some(movement),
                Some(format!("{}", movement,)), // この指し手を選んだ時の pv の読み筋が欲しいぜ☆（＾～＾）
                None,
            );

            if movement.resign() {
                // すでに投了が見えているのなら探索終了だぜ☆（＾～＾）
                break;
            }

            // 横線で仕切るぜ☆（＾～＾）
            universe.game.info.print(
                None,
                None,
                None,
                None,
                None,
                Some(format!("----------Iteration deeping----------")),
            );

            // リセット
            self.pv.clear();

            // 探索局面数は引き継ぐぜ☆（＾～＾）積み上げていった方が見てて面白いだろ☆（＾～＾）
            let ts = self.search(
                max_depth,
                best_ts.get_sum_state(),
                &mut universe.game,
                speed_of_light,
            );
            if ts.timeout {
                // 時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }

            // 無条件に更新だぜ☆（＾～＾）初手の高得点を引きずられて王手回避漏れされたら嫌だしな☆（＾～＾）
            let temp = best_ts.get_sum_state();
            best_ts = ts.clone();
            best_ts.add_state(temp);
        }

        best_ts
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
        max_depth: u8,
        parent_sum_state: u64,
        game: &mut Game,
        speed_of_light: &SpeedOfLight,
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
            } else if 15 < self.stopwatch.elapsed().as_secs() {
                // とりあえず 15 秒で探索を打ち切ろうぜ☆（＾～＾）？
                ts.timeout = true;
                break;
            }

            // 1手進めるぜ☆（＾～＾）
            ts.add_state(1);
            let movement = Movement::from_hash(*movement_hash);
            let captured_piece = game.do_move(&movement, speed_of_light);
            self.pv.push(&movement);
            /*
            IO::debugln(&format!("n={} do.", sum_nodes));
            Commands::pos(&game);
            */

            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.r#type(speed_of_light) == King {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！
                    ts.catch_king(*movement_hash);

                    // 1手戻すぜ☆（＾～＾）
                    self.pv.pop();
                    game.undo_move(speed_of_light);
                    break;
                }
            }

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_position() {
                // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）
                ts.repetition_movement_hash = *movement_hash;
            } else if max_depth < self.pv.len() as u8 {
                // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
                let evaluation = Evaluation::from_caputured_piece(
                    self.pv.len() as u8,
                    captured_piece,
                    speed_of_light,
                );
                ts.check_leaf(&evaluation, *movement_hash);

                if game.info.is_printable() {
                    // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                    // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                    let movement = ts.to_movement();
                    game.info.print(
                        Some(self.pv.len() as u8),
                        Some(ts.get_sum_state() + parent_sum_state),
                        Some(ts.value()),
                        Some(movement),
                        Some(format!("{}", self.pv)),
                        None,
                    );
                }
            // IO::debugln(&format!("n={} Value={}.", sum_nodes, evaluation.value));
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let opponent_ts = self.search(
                    max_depth,
                    ts.get_sum_state() + parent_sum_state,
                    game,
                    speed_of_light,
                );

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                ts.add_turn_over(&opponent_ts, *movement_hash);
            }

            // 1手戻すぜ☆（＾～＾）
            self.pv.pop();
            game.undo_move(speed_of_light);
            /*
            IO::debugln(&format!("n={} undo.", sum_nodes));
            Commands::pos(&game);
            */
        }

        if ts.get_movement_hash() == 0 && ts.repetition_movement_hash != 0 {
            // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
            ts.movement_hash = ts.repetition_movement_hash;
            ts.value = Value::CentiPawn(REPITITION_VALUE);
            ts.reason = "repetition better than resign".to_string();
        };

        ts
    }
}

#[derive(Clone)]
pub struct TreeState {
    sum_state: u64,
    pub value: Value,

    movement_hash: u64,

    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_movement_hash: u64,

    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: String,
    pub timeout: bool,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            sum_state: 0,
            value: Value::Lose,
            movement_hash: 0u64,
            repetition_movement_hash: 0u64,
            reason: "no update".to_string(),
            timeout: false,
        }
    }
}
impl TreeState {
    pub fn get_sum_state(&self) -> u64 {
        self.sum_state
    }

    pub fn value(&self) -> Value {
        self.value
    }

    pub fn add_state(&mut self, val: u64) {
        self.sum_state += val;
    }

    pub fn add_turn_over(&mut self, opponent_ts: &TreeState, friend_movement_hash: u64) {
        self.sum_state += opponent_ts.get_sum_state();

        // TODO 玉を取られてたら、ここは投了すべき☆（＾～＾）？

        // TODO 相手が投了してたら、必ず選ぶべき☆（＾～＾）？

        let (update, reason, friend_value) = match opponent_ts.value {
            Value::Win => {
                // 相手が勝ったので、自分は負けてるぜ☆（＾～＾）
                (false, "opponent win".to_string(), Value::Lose)
            }
            Value::Lose => {
                // 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
                (true, "friend win".to_string(), Value::Win)
            }
            Value::CentiPawn(num) => {
                // 評価値は ひっくり返します。
                let friend_centi_pawn = -num;
                if self.movement_hash == 0 {
                    // どんな手も 投了より良いだろ☆（＾～＾）
                    (
                        true,
                        "this better than resign".to_string(),
                        Value::CentiPawn(friend_centi_pawn),
                    )
                } else {
                    match self.value {
                        Value::Win => {
                            // 自分が勝つ手を既に読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）
                            (false, "".to_string(), self.value)
                        }
                        Value::Lose => {
                            // 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
                            (
                                true,
                                "any move more than lose".to_string(),
                                Value::CentiPawn(friend_centi_pawn),
                            )
                        }
                        Value::CentiPawn(best_centi_pawn) => {
                            if best_centi_pawn < friend_centi_pawn {
                                // 上方修正
                                (
                                    true,
                                    "update value".to_string(),
                                    Value::CentiPawn(friend_centi_pawn),
                                )
                            } else {
                                (false, "".to_string(), self.value)
                            }
                        }
                    }
                }
            }
        };

        if update {
            self.movement_hash = friend_movement_hash;
            self.value = friend_value;
            self.reason = reason;
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
        } else {
            match self.value {
                Value::Win => {
                    // 自分が勝つ手を読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）
                }
                Value::Lose => {
                    // どんな評価値でも、負けるよりマシだろ☆（＾～＾）
                    self.movement_hash = movement_hash;
                    self.value = evaluation.value;
                    self.reason = "any leaf more than lose".to_string();
                    return;
                }
                Value::CentiPawn(best_centi_pawn) => {
                    match evaluation.value {
                        Value::Win => {
                            // 勝つんだから更新するぜ☆（＾～＾）
                            self.movement_hash = movement_hash;
                            self.value = evaluation.value;
                            self.reason = "win".to_string();
                            return;
                        }
                        Value::Lose => {
                            // TODO ここは通らないぜ☆（＾～＾）要対応☆（＾～＾）
                        }
                        Value::CentiPawn(leaf_centi_pawn) => {
                            if best_centi_pawn < leaf_centi_pawn {
                                // 更新☆（＾～＾）
                                self.movement_hash = movement_hash;
                                self.value = evaluation.value;
                                self.reason = "good position".to_string();
                                return;
                            }
                        }
                    }
                }
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
        self.value = Value::Win;
        self.reason = "king catch is strongest".to_string();
    }
}

/// 指し手の評価値だぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub enum Value {
    /// 歩１枚の交換値を 100 とするぜ☆（＾～＾）
    /// 将棋は、相手は駒を取られて損、自分は駒を取って得という風に痛手が２倍広がるので、
    /// 交換値が 100 ということは、200点差が開くということだぜ☆（＾～＾）
    CentiPawn(i16),

    /// 勝ち☆（＾～＾）
    Win,

    /// 負け☆（＾～＾）
    Lose,
}

#[derive(Clone)]
pub struct PrincipalVariation {
    /// 根っこに戻ると、中身が空っぽになっているので困るぜ（＾～＾）
    moves: Vec<Movement>,
}
impl Default for PrincipalVariation {
    fn default() -> Self {
        PrincipalVariation {
            moves: Vec::default(),
        }
    }
}
impl PrincipalVariation {
    fn clear(&mut self) {
        self.moves.clear();
    }
    fn push(&mut self, movement: &Movement) {
        self.moves.push(*movement);
    }

    fn pop(&mut self) {
        self.moves.pop();
    }

    fn len(&self) -> usize {
        self.moves.len()
    }
}
impl fmt::Display for PrincipalVariation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for movement in &self.moves {
            buffer.push_str(&format!("{} ", movement));
        }
        write!(f, "{}", buffer)
    }
}
