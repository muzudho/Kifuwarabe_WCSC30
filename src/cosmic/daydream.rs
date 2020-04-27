//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::PLY_LEN;
use crate::cosmic::recording::{Movement, Person, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::{Evaluation, REPITITION_VALUE};
use crate::cosmic::smart::features::PieceMeaning;
use crate::cosmic::smart::features::PieceType::King;
use crate::cosmic::toy_box::PieceNum;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::PseudoLegalMoves;
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::{Beam, PvString};
use rand::Rng;
use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

pub struct Tree {
    // この木を生成したと同時にストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,
    // 状態ノード数☆（＾～＾）
    pub state_nodes: u64,

    // Principal variation(読み筋)☆（＾～＾）
    pv: PrincipalVariation,

    // 思考時間（秒）をランダムにすることで、指し手を変えるぜ☆（＾～＾）
    think_sec: u64,

    pub evaluation: Evaluation,

    // 反復深化探索の１回目だけ真☆（＾～＾）
    pub first_iteration_deeping: bool,
}
impl Tree {
    pub fn new(board_coverage_weight: i32, komawari_weight: i32, promotion_weight: i32) -> Self {
        Tree {
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            evaluation: Evaluation::new(board_coverage_weight, komawari_weight, promotion_weight),
            first_iteration_deeping: true,
        }
    }
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(
        &mut self,
        universe: &mut Universe,
        speed_of_light: &SpeedOfLight,
    ) -> TreeState {
        universe.game.info.clear();
        self.think_sec = rand::thread_rng()
            .gen_range(universe.option_min_think_sec, universe.option_max_think_sec);

        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        self.evaluation.before_search();
        let mut best_ts = self.node(0, &mut universe.game, std::i16::MAX, speed_of_light);
        self.evaluation.after_search();

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for max_depth in universe.option_depth_not_to_give_up..universe.option_max_depth {
            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            let movement = best_ts.bestmove.to_movement();
            universe.game.info.print(
                Some(max_depth),
                Some((self.state_nodes, self.nps())),
                Some(best_ts.bestmove.value),
                Some(movement),
                &Some(PvString::PV(self.msec(), format!("{}", movement,))), // この指し手を選んだ時の pv の読み筋が欲しいぜ☆（＾～＾）
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
                &Some(PvString::String(format!(
                    "----------Iteration deeping----------"
                ))),
            );

            // 探索局面数は引き継ぐぜ☆（＾～＾）積み上げていった方が見てて面白いだろ☆（＾～＾）
            self.evaluation.before_search();
            let ts = self.node(max_depth, &mut universe.game, std::i16::MAX, speed_of_light);
            self.evaluation.after_search();
            if ts.timeout {
                // 時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }

            // 無条件に更新だぜ☆（＾～＾）初手の高得点を引きずられて王手回避漏れされたら嫌だしな☆（＾～＾）
            best_ts = ts.clone();
            self.first_iteration_deeping = false;
        }

        best_ts
    }

    /// 先手の気持ちで、勝てだぜ☆（*＾～＾*）
    ///
    /// # Arguments
    ///
    /// * `max_depth` - 読みの深さ☆（＾～＾）
    /// * `game` - 対局。
    /// * `sibling_best` - アルファベータ探索のベータ値。兄弟で一番良い評価値。
    /// * `speed_of_light` - (光速)
    ///
    /// # Returns
    ///
    /// Best movement, Value, Sum nodes
    fn node(
        &mut self,
        max_depth: u8,
        game: &mut Game,
        sibling_best: i16,
        speed_of_light: &SpeedOfLight,
    ) -> TreeState {
        let mut ts = TreeState::default();
        // 指し手の一覧を作るぜ☆（＾～＾） 指し手はハッシュ値で入っている☆（＾～＾）
        let mut movement_set = HashSet::<u64>::new();

        // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
        PseudoLegalMoves::make_move(
            game.history.get_phase(Person::Friend),
            &game.board,
            &speed_of_light,
            &mut |movement| {
                &movement_set.insert(movement);
            },
        );

        // 指せる手が無ければ投了☆（＾～＾）
        if movement_set.is_empty() {
            return ts;
        }

        let coverage_sign: i16 = if self.pv.len() % 2 == 0 {
            // 先手が指すところだぜ☆（＾～＾）
            1
        } else {
            // 後手が指すところだぜ☆（＾～＾）
            -1
        };
        self.add_control(coverage_sign, game, &movement_set);
        for movement_hash in movement_set.iter() {
            // 時間を見ようぜ☆（＾～＾）？
            if ts.timeout && !self.first_iteration_deeping {
                // 時間切れなら この探索結果は使わないぜ☆（＾～＾）
                // 反復深化探索の１回目はタイムアウトしない☆（＾～＾）考え続けるぜ☆（＾～＾）
                break;
            } else if self.think_sec < self.sec() {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                ts.timeout = true;
                break;
            }

            // 1手進めるぜ☆（＾～＾）
            self.state_nodes += 1;
            let movement = Movement::from_hash(*movement_hash);
            let source_piece = if let Some(source_val) = &movement.source {
                game.board.piece_at(source_val)
            } else {
                // 打
                None
            };
            let captured_piece: Option<(PieceMeaning, PieceNum)> =
                game.do_move(&movement, speed_of_light);
            self.pv.push(&movement);
            let (captured_piece_centi_pawn, delta_promotion_bonus) = self.evaluation.after_do_move(
                &source_piece,
                &captured_piece,
                movement.promote,
                speed_of_light,
            );

            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.0.r#type(speed_of_light) == King {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！
                    ts.bestmove.catch_king(*movement_hash);

                    self.evaluation
                        .before_undo_move(captured_piece_centi_pawn, delta_promotion_bonus);
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
                // 葉で評価しようぜ☆（＾～＾）

                // 利きを集計するぜ☆（＾～＾）自分が後手なら符号を逆さにして見ろだぜ☆（＾～＾）
                let board_coverage_value: i16 = coverage_sign * game.board.coverage_value();
                ts.choice_friend(
                    &Value::CentiPawn(self.evaluation.centi_pawn(board_coverage_value)),
                    *movement_hash,
                );

                if game.info.is_printable() {
                    // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                    // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                    let movement = ts.bestmove.to_movement();
                    game.info.print(
                        None,
                        None,
                        None,
                        None,
                        &Some(PvString::String(format!(
                            "board coverage={} | {} {} {} | komawari={} | promotion={}",
                            self.evaluation.board_coverage(board_coverage_value),
                            game.board.control[68],
                            game.board.control[58],
                            game.board.control[48],
                            self.evaluation.komawari(),
                            self.evaluation.promotion()
                        ))),
                    );
                    game.info.print(
                        Some(self.pv.len() as u8),
                        Some((self.state_nodes, self.nps())),
                        Some(ts.bestmove.value),
                        Some(movement),
                        &Some(PvString::PV(self.msec(), format!("{}", self.pv))),
                    );
                }
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                self.evaluation.before_search();
                let opponent_ts = self.node(
                    max_depth,
                    game,
                    if let Value::CentiPawn(centi_pawn) = ts.bestmove.value {
                        -centi_pawn
                    } else {
                        std::i16::MAX
                    },
                    speed_of_light,
                );
                self.evaluation.after_search();

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                ts.turn_over_and_choice(
                    &opponent_ts,
                    *movement_hash,
                    self.evaluation.centi_pawn(0),
                );
            }

            self.evaluation
                .before_undo_move(captured_piece_centi_pawn, delta_promotion_bonus);
            self.pv.pop();
            game.undo_move(speed_of_light);

            match ts.bestmove.value {
                Value::CentiPawn(centi_pawn) => {
                    if sibling_best <= centi_pawn {
                        // 兄弟局面より良い手を見つけたのなら、相手から見ればこの手は選ばないから、もう探索しなくていいぜ☆（＾～＾）
                        // これが　いわゆるベータカットだぜ☆（＾～＾）
                        break;
                    }
                }
                Value::Win => {
                    // 勝った手を見つけたのなら、もう探索しなくていいぜ☆（＾～＾）
                    break;
                }
                _ => {
                    // 続行☆（＾～＾）
                }
            }
        }
        self.add_control(-1 * coverage_sign, game, &movement_set);

        if ts.get_movement_hash() == 0 && ts.repetition_movement_hash != 0 {
            // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
            ts.bestmove.update(
                ts.repetition_movement_hash,
                &Value::CentiPawn(REPITITION_VALUE),
                "repetition better than resign".to_string(),
            );
        };

        ts
    }

    pub fn add_control(&mut self, sign: i16, game: &mut Game, movement_set: &HashSet<u64>) {
        for movement_hash in movement_set.iter() {
            // 駒を動かせたんなら、利きが広いと考えるぜ☆（＾～＾）
            game.board.control[Movement::from_hash(*movement_hash)
                .destination
                .unwrap()
                .address() as usize] += sign;
        }
    }

    pub fn sec(&self) -> u64 {
        self.stopwatch.elapsed().as_secs()
    }

    pub fn msec(&self) -> u128 {
        self.stopwatch.elapsed().as_millis()
    }

    pub fn nps(&self) -> u64 {
        let sec = self.sec();
        if 0 < sec {
            self.state_nodes / sec
        } else {
            0
        }
    }
}

#[derive(Clone)]
pub struct Bestmove {
    pub value: Value,
    movement_hash: u64,
    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: String,
}
impl Default for Bestmove {
    fn default() -> Self {
        Bestmove {
            value: Value::Lose,
            movement_hash: 0u64,
            reason: "no update".to_string(),
        }
    }
}
impl Bestmove {
    pub fn to_movement(&self) -> Movement {
        Movement::from_hash(self.movement_hash)
    }
    pub fn catch_king(&mut self, movement_hash: u64) {
        // 玉を取る手より強い手はないぜ☆（＾～＾）！
        self.movement_hash = movement_hash;
        self.value = Value::Win;
        self.reason = "king catch is strongest".to_string();
    }
    pub fn update(&mut self, hash: u64, value: &Value, reason: String) {
        self.movement_hash = hash;
        self.value = *value;
        self.reason = reason;
    }
}
#[derive(Clone)]
pub struct TreeState {
    pub bestmove: Bestmove,
    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_movement_hash: u64,
    pub timeout: bool,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            bestmove: Bestmove::default(),
            repetition_movement_hash: 0u64,
            timeout: false,
        }
    }
}
impl TreeState {
    pub fn turn_over_and_choice(
        &mut self,
        opponent_ts: &TreeState,
        friend_movement_hash: u64,
        friend_centi_pawn1: i16,
    ) {
        // TODO 玉を取られてたら、ここは投了すべき☆（＾～＾）？

        // TODO 相手が投了してたら、必ず選ぶべき☆（＾～＾）？

        let (update, reason, friend_value) = match opponent_ts.bestmove.value {
            Value::Win => {
                // 相手が勝ったので、自分は負けてるぜ☆（＾～＾）
                (false, "opponent win".to_string(), Value::Lose)
            }
            Value::Lose => {
                // 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
                (true, "friend win".to_string(), Value::Win)
            }
            Value::CentiPawn(num) => {
                // 評価値は ひっくり返します。この指し手の駒の交換値も足します。
                let friend_centi_pawn2 = -num + friend_centi_pawn1;
                if self.bestmove.movement_hash == 0 {
                    // どんな手も 投了より良いだろ☆（＾～＾）
                    (
                        true,
                        "this better than resign".to_string(),
                        Value::CentiPawn(friend_centi_pawn2),
                    )
                } else {
                    match self.bestmove.value {
                        Value::Win => {
                            panic!(Beam::trouble(
                                "(Err.405) 自分が勝つ手を既に読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）"
                            ))
                        }
                        Value::Lose => {
                            // 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
                            (
                                true,
                                "any move more than lose".to_string(),
                                Value::CentiPawn(friend_centi_pawn2),
                            )
                        }
                        Value::CentiPawn(best_centi_pawn) => {
                            if best_centi_pawn < friend_centi_pawn2 {
                                // 上方修正
                                (
                                    true,
                                    "update value".to_string(),
                                    Value::CentiPawn(friend_centi_pawn2),
                                )
                            } else {
                                (false, "".to_string(), self.bestmove.value)
                            }
                        }
                    }
                }
            }
        };

        if update {
            self.bestmove
                .update(friend_movement_hash, &friend_value, reason);
        }
    }

    /// 指し手のベストを選ぶぜ☆（＾～＾）
    pub fn choice_friend(&mut self, value: &Value, movement_hash: u64) {
        if self.bestmove.movement_hash == 0 {
            // どんな葉も 投了より良いだろ☆（＾～＾）
            // TODO 王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
            self.bestmove.update(
                movement_hash,
                value,
                "any leaf better than resign".to_string(),
            );
            return;
        } else {
            match self.bestmove.value {
                Value::Win => panic!(Beam::trouble(
                    "(Err.397) 自分が勝つ手を読んでるなら、ここに来るのはおかしいぜ☆（＾～＾）"
                )),
                Value::Lose => {
                    // どんな評価値でも、負けるよりマシだろ☆（＾～＾）
                    self.bestmove.update(
                        movement_hash,
                        value,
                        "any leaf more than lose".to_string(),
                    );
                    return;
                }
                Value::CentiPawn(best_centi_pawn) => {
                    match value {
                        Value::Win => {
                            // 勝つんだから更新するぜ☆（＾～＾）
                            self.bestmove
                                .update(movement_hash, value, "win".to_string());
                            return;
                        }
                        Value::Lose => {
                            // TODO ここは通らないぜ☆（＾～＾）要対応☆（＾～＾）
                        }
                        Value::CentiPawn(leaf_centi_pawn) => {
                            if best_centi_pawn < *leaf_centi_pawn {
                                // 更新☆（＾～＾）
                                self.bestmove.update(
                                    movement_hash,
                                    value,
                                    "good position".to_string(),
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_movement_hash(&self) -> u64 {
        self.bestmove.movement_hash
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
    moves: [Movement; PLY_LEN],
    ply: usize,
}
impl Default for PrincipalVariation {
    fn default() -> Self {
        PrincipalVariation {
            // 投了で埋めるぜ☆（＾～＾）
            moves: [Movement::default(); PLY_LEN],
            ply: 0,
        }
    }
}
impl PrincipalVariation {
    fn push(&mut self, movement: &Movement) {
        self.moves[self.ply].set(movement);
        self.ply += 1;
    }

    fn pop(&mut self) {
        self.ply -= 1;
        self.moves[self.ply].clear();
    }

    fn len(&self) -> usize {
        self.ply
    }
}
impl fmt::Display for PrincipalVariation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for i in 0..=self.ply {
            buffer.push_str(&format!("{} ", self.moves[i]));
        }
        write!(f, "{}", buffer.trim_end())
    }
}
