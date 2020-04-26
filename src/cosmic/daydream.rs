//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::playing::Game;
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

    // 玉の安全度の重み☆（＾～＾）
    king_safety_weight: f64,
}
impl Default for Tree {
    fn default() -> Self {
        Tree {
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            king_safety_weight: 0.0,
        }
    }
}
impl Tree {
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(
        &mut self,
        speed_of_light: &SpeedOfLight,
        universe: &mut Universe,
    ) -> TreeState {
        universe.game.info.clear();
        self.think_sec = rand::thread_rng()
            .gen_range(universe.option_min_think_sec, universe.option_max_think_sec);
        self.king_safety_weight = universe.option_king_risk_weight;

        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        let mut best_ts = self.search(0, &mut universe.game, speed_of_light);

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for max_depth in 1..universe.option_max_depth {
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

            // リセット
            self.pv.clear();

            // 探索局面数は引き継ぐぜ☆（＾～＾）積み上げていった方が見てて面白いだろ☆（＾～＾）
            let ts = self.search(max_depth, &mut universe.game, speed_of_light);
            if ts.timeout {
                // 時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }

            // 無条件に更新だぜ☆（＾～＾）初手の高得点を引きずられて王手回避漏れされたら嫌だしな☆（＾～＾）
            best_ts = ts.clone();
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

        let control_sign: i16 = if self.pv.len() % 2 == 0 {
            // 先手が指すところだぜ☆（＾～＾）
            1
        } else {
            // 後手が指すところだぜ☆（＾～＾）
            -1
        };
        self.add_control(control_sign, game, &movement_set);
        for movement_hash in movement_set.iter() {
            // 時間を見ようぜ☆（＾～＾）？
            if ts.timeout {
                break;
            } else if self.think_sec < self.sec() {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                ts.timeout = true;
                break;
            }

            // 1手進めるぜ☆（＾～＾）
            self.state_nodes += 1;
            let movement = Movement::from_hash(*movement_hash);
            let source_piece_on_board = if let Some(source_val) = &movement.source {
                Some(game.board.piece_at(source_val))
            } else {
                // 打
                None
            };
            let captured_piece: Option<(PieceMeaning, PieceNum)> =
                game.do_move(&movement, speed_of_light);
            self.pv.push(&movement);

            // pvリストに１手入れてから評価しろだぜ☆（＾～＾）0除算エラーを防げるぜ☆（＾～＾）
            let promoted_bonus = if let Some(source_piece_on_board_val) = source_piece_on_board {
                if let Some(source_piece_val) = source_piece_on_board_val {
                    Evaluation::from_promotion(
                        self.pv.len(),
                        source_piece_val.0.r#type(&speed_of_light),
                        &movement,
                    )
                } else {
                    0
                }
            } else {
                // 打なら成りは無いぜ☆（＾～＾）
                0
            };
            // 取った駒の価値を評価するぜ☆（＾～＾）
            let captured_piece_centi_pawn =
                Evaluation::from_caputured_piece(self.pv.len(), captured_piece, speed_of_light);

            /*
            IO::debugln(&format!("n={} do.", sum_nodes));
            Commands::pos(&game);
            */

            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.0.r#type(speed_of_light) == King {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！
                    ts.bestmove.catch_king(*movement_hash);

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
                // 葉で評価しようぜ☆（＾～＾）

                // 利きを集計するぜ☆（＾～＾）自分が後手なら符号を逆さにして見ろだぜ☆（＾～＾）
                let control_value: i16 = game.board.control_value();

                // 玉の周囲２４近傍の利きを、重みを付けて集計するぜ☆（＾～＾）
                // 玉のリスクを高くし過ぎると、盤コントロールが無茶苦茶になってしまう☆（＾～＾）
                // かといって 玉のリスクは 歩１枚の価値より重いだろ☆（＾～＾）係数が難しいぜ☆（＾～＾）
                let risk_safety = (self.king_safety_weight
                    * Evaluation::king_safety(game, control_sign))
                    / self.pv.len() as f64;

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
                            "board control={} | king safety={} | {} {} {}",
                            control_sign * control_value,
                            risk_safety,
                            game.board.control[68],
                            game.board.control[58],
                            game.board.control[48],
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

                ts.choice_friend(
                    &Value::CentiPawn(
                        captured_piece_centi_pawn
                            + promoted_bonus
                            + (control_sign * control_value)
                            + risk_safety as i16,
                    ),
                    *movement_hash,
                );

            // IO::debugln(&format!("n={} Value={}.", sum_nodes, evaluation.value));
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let opponent_ts = self.search(max_depth, game, speed_of_light);

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                ts.turn_over_and_choice(
                    &opponent_ts,
                    *movement_hash,
                    captured_piece_centi_pawn + promoted_bonus,
                );
            }

            // 1手戻すぜ☆（＾～＾）
            self.pv.pop();
            game.undo_move(speed_of_light);
            /*
            IO::debugln(&format!("n={} undo.", sum_nodes));
            Commands::pos(&game);
            */
        }
        self.add_control(-1 * control_sign, game, &movement_set);

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
        friend_captured_piece_centi_pawn: i16,
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
                let friend_centi_pawn = -num + friend_captured_piece_centi_pawn;
                if self.bestmove.movement_hash == 0 {
                    // どんな手も 投了より良いだろ☆（＾～＾）
                    (
                        true,
                        "this better than resign".to_string(),
                        Value::CentiPawn(friend_centi_pawn),
                    )
                } else {
                    match self.bestmove.value {
                        Value::Win => {
                            // 自分が勝つ手を既に読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）
                            (false, "".to_string(), self.bestmove.value)
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
        for r#move in &self.moves {
            buffer.push_str(&format!("{} ", r#move));
        }
        write!(f, "{}", buffer.trim_end())
    }
}
