//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Movement, PLY_LEN, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::{Evaluation, REPITITION_VALUE};
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::mate1::Lioncatch;
use crate::cosmic::smart::see::SEE;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::{Piece, PseudoLegalMoves, Ways};
use crate::spaceship::equipment::{Beam, PvString};
use rand::Rng;
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
    pub depth_not_to_give_up: usize,
    // 読みの深さの上限☆（＾～＾）１手を読み切るなら 0 を指定しろだぜ☆（＾～＾）
    max_depth0: usize,
}
impl Tree {
    pub fn new(
        many_ways_weight: isize,
        komawari_weight: isize,
        promotion_weight: isize,
        depth_not_to_give_up: usize,
    ) -> Self {
        Tree {
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            evaluation: Evaluation::new(many_ways_weight, komawari_weight, promotion_weight),
            depth_not_to_give_up: depth_not_to_give_up,
            max_depth0: 0,
        }
    }
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(&mut self, universe: &mut Universe) -> TreeState {
        universe.game.info.clear();
        self.think_sec = rand::thread_rng().gen_range(
            universe.option_min_think_sec as u64,
            universe.option_max_think_sec as u64,
        );

        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        self.evaluation.before_search();
        self.max_depth0 = 0;
        let mut best_ts = self.node(&mut universe.game, Value::Win);
        self.evaluation.after_search();

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for id in 1..universe.option_max_depth {
            self.max_depth0 = id;
            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            let movement = best_ts.bestmove.movement;
            universe.game.info.print(
                Some(self.max_depth0),
                Some((self.state_nodes, self.nps())),
                Some(best_ts.bestmove.value),
                movement,
                &Some(PvString::PV(
                    self.msec(),
                    format!(
                        "{}",
                        if let Some(movement_val) = movement {
                            format!("{}", movement_val)
                        } else {
                            "resign".to_string()
                        },
                    ),
                )), // この指し手を選んだ時の pv の読み筋が欲しいぜ☆（＾～＾）
            );

            if let None = movement {
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
            let ts = self.node(&mut universe.game, Value::Win);
            self.evaluation.after_search();
            if ts.timeout {
                // 思考時間切れなら この探索結果は使わないぜ☆（＾～＾）
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
    /// * `game` - 対局。
    /// * `sibling_best` - アルファベータ探索のベータ値。兄弟で一番良い評価値。
    ///
    /// # Returns
    ///
    /// Best movement, Value, Sum nodes
    fn node(&mut self, game: &mut Game, another_branch_best: Value) -> TreeState {
        let mut ts = TreeState::default();

        // この手を指すと負けてしまう、という手が見えていたら、このフラグを立てろだぜ☆（＾～＾）
        let mut exists_lose = false;

        // TODO let mut controls = Vec::<AbsoluteAddress>::new();

        // 指し手の一覧を作るぜ☆（＾～＾） 指し手はハッシュ値で入っている☆（＾～＾）
        let mut ways = {
            /*
            // TODO 1手詰めは必ず仕留めなければいけないぜ☆（＾～＾）？
            let mut lioncatch = Lioncatch::new(game);
            lioncatch.init(game).pinned_pieces(game).checkers(game);
            if !lioncatch.checks.is_empty() {
                lioncatch.checks
            } else {
                //   */
            let mut ways = Ways::new();

            // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
            /* TODO
            PseudoLegalMoves::make_move(
                game.history.get_friend(),
                &game.board,
                &mut |way, destination| {
                    if let Some(way_val) = way {
                        ways.push(&way_val);
                    }
                    // TODO 利きを一旦覚えようぜ☆（＾～＾）？
                    // controls.push(*destination);
                },
            );
            */
            PseudoLegalMoves::make_move(game.history.get_friend(), &game.board, &mut |way| {
                ways.push(&way);
            });

            ways
            //}
        };

        // 指せる手が無ければ投了☆（＾～＾）
        if ways.is_empty() {
            return ts;
        }

        // TODO この利きは、この１手を指すまえの利き（１年前の夜空を見ていることを１光年と言うだろ）をキープしているということに注意しろだぜ☆（＾～＾）
        // いわば、１光手 利きカウントボードだぜ☆（＾～＾）
        // for destination in &controls {
        //     game.board
        //         .add_control(game.history.get_friend(), destination, 1);
        // }

        // 指し手のオーダリングをしたいぜ☆（＾～＾） 取った駒は指し手生成の段階で調べているし☆（＾～＾）
        let mut cap = 0;
        if 1 < ways.len() {
            for i in 0..ways.len() {
                if let Some(_captured) = ways.get(i).captured {
                    // 駒を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                    // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                    ways.swap(cap, i);
                    cap += 1;
                }
            }
            // 次は駒を取ったグループの中で、玉を取った手をグループの先頭に集めるぜ☆（＾～＾）
            let mut king = 0;
            for i in 0..cap {
                match ways.get(i).captured.unwrap().meaning.r#type() {
                    PieceType::King => {
                        // 玉を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                        // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                        ways.swap(king, i);
                        king += 1;
                    }
                    _ => {}
                }
            }
        }

        let coverage_sign = if self.pv.len() % 2 == 0 {
            // 先手が指すところだぜ☆（＾～＾）
            1
        } else {
            // 後手が指すところだぜ☆（＾～＾）
            -1
        };
        self.evaluation.add_control(coverage_sign, &ways);
        for index in ways.indexes.iter() {
            let way = ways.get(*index);
            // 時間を見ようぜ☆（＾～＾）？
            if self.think_sec < self.sec() && self.depth_not_to_give_up <= self.max_depth0 {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                // タイムアウトしたんだったら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                ts.timeout = true;
                return ts;
            }

            // 1手進めるぜ☆（＾～＾）
            self.state_nodes += 1;
            let movement = way.movement;
            let source_piece = if let Some(source_val) = &movement.source {
                game.board.piece_at(source_val)
            } else {
                // 打
                None
            };
            let captured_piece: Option<Piece> = game.do_move(&movement);
            self.pv.push(&movement);
            let (captured_piece_centi_pawn, delta_promotion_bonus) =
                self.evaluation
                    .after_do_move(&source_piece, &captured_piece, movement.promote);

            // TODO 廃止方針☆（＾～＾）
            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.meaning.r#type() == PieceType::King {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！探索終了～☆（＾～＾）！この手を選べだぜ☆（＾～＾）！
                    ts.bestmove.catch_king(way.movement);

                    self.evaluation
                        .before_undo_move(captured_piece_centi_pawn, delta_promotion_bonus);
                    self.pv.pop();
                    game.undo_move();
                    break;
                }
            }

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_position() {
                // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）
                ts.repetition_movement = Some(way.movement);
            } else if self.max_depth0 < self.pv.len() {
                // 葉だぜ☆（＾～＾）

                if let Some(_captured) = way.captured {
                    // TODO SEEやろうぜ☆（＾～＾）
                    SEE::go(game, &movement.destination);
                }

                // 評価を集計するぜ☆（＾～＾）
                ts.choice_friend(
                    &Value::CentiPawn(self.evaluation.centi_pawn()),
                    way.movement,
                );

                if game.info.is_printable() {
                    // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                    // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                    let movement = ts.bestmove.movement;
                    game.info.print(
                        None,
                        None,
                        None,
                        None,
                        &Some(PvString::String(format!(
                            "ways={} | komawari={} | promotion={}", //  | {} {} {} |
                            self.evaluation.ways(),
                            self.evaluation.komawari(),
                            self.evaluation.promotion(),
                            /* TODO
                            // サンプルを見ているだけだぜ☆（＾～＾）
                            game.board.get_control(
                                game.history.get_friend(),
                                &AbsoluteAddress::new(6, 8)
                            ),
                            game.board.get_control(
                                game.history.get_friend(),
                                &AbsoluteAddress::new(5, 8)
                            ),
                            game.board.get_control(
                                game.history.get_friend(),
                                &AbsoluteAddress::new(4, 8)
                            ),
                            */
                        ))),
                    );
                    game.info.print(
                        Some(self.pv.len()),
                        Some((self.state_nodes, self.nps())),
                        Some(ts.bestmove.value),
                        movement,
                        &Some(PvString::PV(self.msec(), format!("{}", self.pv))),
                    );
                }
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                self.evaluation.before_search();
                let opponent_ts = self.node(
                    game,
                    match ts.bestmove.value {
                        Value::CentiPawn(centi_pawn) => Value::CentiPawn(-centi_pawn),
                        Value::Win => Value::Lose,
                        Value::Lose => Value::Win,
                    },
                );

                if ts.timeout {
                    // すでにタイムアウトしていたのなら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                    return ts;
                }
                self.evaluation.after_search();

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                exists_lose = ts.turn_over_and_choice(
                    &opponent_ts,
                    way.movement,
                    self.evaluation.centi_pawn(),
                );
            }

            self.evaluation
                .before_undo_move(captured_piece_centi_pawn, delta_promotion_bonus);
            self.pv.pop();
            game.undo_move();

            match ts.bestmove.value {
                Value::Win => {
                    // この手について、次の手ではなく、２手以上先で勝ったんだろ☆（＾～＾）もう探索しなくていいぜ☆（＾～＾）この手にしようぜ☆（＾～＾）！
                    break;
                }
                Value::Lose => {
                    // この手について、次の相手の番で王さまを取られたか、３手先以上の奇数番で詰められたんだろ☆（＾～＾）詰められてない別の手を探すんだぜ、続行☆（＾～＾）！
                }
                Value::CentiPawn(current_centi_pawn) => {
                    // ベータカット判定☆（＾～＾）
                    match another_branch_best {
                        Value::CentiPawn(another_branch_best_centi_pawn) => {
                            // 兄弟局面より良い手を見つけたのなら、相手から見ればこの手は選ばないから、もう探索しなくていいぜ☆（＾～＾）
                            // これが　いわゆるベータカットだぜ☆（＾～＾）
                            if another_branch_best_centi_pawn <= current_centi_pawn {
                                break;
                            }
                        }
                        Value::Win => {
                            // 初手に、ゴミの最大値として入っているか、兄弟局面で勝ちの手があったようだぜ☆（＾～＾）
                            // ベータカットは起こらないぜ☆（＾～＾）
                        }
                        Value::Lose => {
                            // 兄弟局面に負けがあるんだったら、この
                            // 負けに比べればどんな手でも良いぜ☆（＾～＾）ベータカットな☆（＾～＾）！
                            break;
                        }
                    }
                }
            }
        }
        self.evaluation.add_control(-1 * coverage_sign, &ways);

        // TODO 利き削除☆（＾～＾）
        // for destination in &controls {
        //     game.board
        //         .add_control(game.history.get_friend(), destination, -1);
        // }

        if !exists_lose {
            if let None = ts.bestmove.movement {
                if let Some(repetition_movement_val) = ts.repetition_movement {
                    // 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
                    ts.bestmove.update(
                        repetition_movement_val,
                        &Value::CentiPawn(REPITITION_VALUE),
                        Reason::RepetitionBetterThanResign,
                    );
                }
            }
        }

        ts
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
    pub movement: Option<Movement>,
    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: Reason,
}
impl Default for Bestmove {
    fn default() -> Self {
        Bestmove {
            value: Value::Lose,
            movement: None,
            // なんの手も無かったぜ☆（＾～＾）
            reason: Reason::NoUpdate,
        }
    }
}
impl Bestmove {
    /// TODO 廃止予定☆（＾～＾）
    pub fn catch_king(&mut self, movement: Movement) {
        // 玉を取る手より強い手はないぜ☆（＾～＾）！
        self.movement = Some(movement);
        self.value = Value::Win;
        self.reason = Reason::KingCatchIsStrongest;
    }
    pub fn update(&mut self, movement: Movement, value: &Value, reason: Reason) {
        self.movement = Some(movement);
        self.value = *value;
        self.reason = reason;
    }
}
#[derive(Clone)]
pub struct TreeState {
    pub bestmove: Bestmove,
    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_movement: Option<Movement>,
    pub timeout: bool,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            bestmove: Bestmove::default(),
            repetition_movement: None,
            timeout: false,
        }
    }
}
impl TreeState {
    pub fn turn_over_and_choice(
        &mut self,
        opponent_ts: &TreeState,
        friend_movement: Movement,
        friend_centi_pawn1: isize,
    ) -> bool {
        // TODO 玉を取られてたら、ここは投了すべき☆（＾～＾）？

        // TODO 相手が投了してたら、必ず選ぶべき☆（＾～＾）？

        match opponent_ts.bestmove.value {
            Value::Win => {
                // 相手が勝ったので、自分は負けてるぜ☆（＾～＾）この手は選んではいけないぜ☆（＾～＾）
                true
            }
            Value::Lose => {
                // 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
                self.bestmove
                    .update(friend_movement, &Value::Win, Reason::FriendWin);
                false
            }
            Value::CentiPawn(num) => {
                // 評価値は ひっくり返します。この指し手の駒の交換値も足します。
                let friend_centi_pawn2 = -num + friend_centi_pawn1;
                if let None = self.bestmove.movement {
                    // どんな悪手も、詰みでなければ 投了より良いだろ☆（＾～＾）
                    self.bestmove.update(
                        friend_movement,
                        &Value::CentiPawn(friend_centi_pawn2),
                        Reason::ThisBetterThanResign,
                    );
                } else {
                    match self.bestmove.value {
                        Value::Win => {
                            panic!(Beam::trouble(
                                "(Err.405) 自分が勝つ手を既に読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）"
                            ))
                        }
                        Value::Lose => {
                            // 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
                            self.bestmove
                            .update(friend_movement, &Value::CentiPawn(friend_centi_pawn2), Reason::AnyMoveMoreThanLose);
                        }
                        Value::CentiPawn(best_centi_pawn) => {
                            if best_centi_pawn < friend_centi_pawn2 {
                                // 上方修正
                                self.bestmove
                                .update(friend_movement, &Value::CentiPawn(friend_centi_pawn2), Reason::ValueUp);
                            }
                        }
                    }
                }
                false
            }
        }
    }

    /// 指し手のベストを選ぶぜ☆（＾～＾）
    pub fn choice_friend(&mut self, value: &Value, movement: Movement) {
        if let None = self.bestmove.movement {
            // どんな葉も 投了より良いだろ☆（＾～＾）
            // TODO でも、王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
            self.bestmove
                .update(movement, value, Reason::AnyLeafBetterThanResign);
            return;
        } else {
            match self.bestmove.value {
                Value::Win => panic!(Beam::trouble(
                    "(Err.397) 自分が勝つ手を読んでるなら、ここに来るのはおかしいぜ☆（＾～＾）"
                )),
                Value::Lose => {
                    // どんな評価値でも、負けるよりマシだろ☆（＾～＾）
                    self.bestmove
                        .update(movement, value, Reason::AnyLeafMoreThanLose);
                    return;
                }
                Value::CentiPawn(best_centi_pawn) => {
                    match value {
                        Value::Win => {
                            // 勝つんだから更新するぜ☆（＾～＾）
                            self.bestmove.update(movement, value, Reason::Win);
                            return;
                        }
                        Value::Lose => {
                            // TODO ここは通らないぜ☆（＾～＾）要対応☆（＾～＾）
                        }
                        Value::CentiPawn(leaf_centi_pawn) => {
                            if best_centi_pawn < *leaf_centi_pawn {
                                // 評価値が良かったから更新☆（＾～＾）
                                self.bestmove.update(movement, value, Reason::GoodPosition);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 指し手の評価値だぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub enum Value {
    /// 歩１枚の交換値を 100 とするぜ☆（＾～＾）
    /// 将棋は、相手は駒を取られて損、自分は駒を取って得という風に痛手が２倍広がるので、
    /// 交換値が 100 ということは、200点差が開くということだぜ☆（＾～＾）
    CentiPawn(isize),

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
            // ゴミの値で埋めるぜ☆（＾～＾）
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
        // ゴミの値は消さないぜ☆（＾～＾）
    }

    fn len(&self) -> usize {
        self.ply
    }
}
impl fmt::Display for PrincipalVariation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for i in 0..self.ply {
            buffer.push_str(&format!("{} ", self.moves[i]));
        }
        write!(f, "{}", buffer.trim_end())
    }
}

#[derive(Clone)]
pub enum Reason {
    /// 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
    RepetitionBetterThanResign,
    /// なんの手も無かったぜ☆（＾～＾）
    NoUpdate,
    /// 玉を取る手より強い手はないぜ☆（＾～＾）！
    KingCatchIsStrongest,
    /// 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
    FriendWin,
    /// どんな悪手も、詰みでなければ 投了より良いだろ☆（＾～＾）
    ThisBetterThanResign,
    /// どんな葉も 投了より良いだろ☆（＾～＾）でも、王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
    AnyLeafBetterThanResign,
    /// どんな評価値でも、負けるよりマシだろ☆（＾～＾）
    AnyLeafMoreThanLose,
    /// 勝つんだから更新するぜ☆（＾～＾）
    Win,
    /// 評価値が良かったから更新☆（＾～＾）
    GoodPosition,
    /// 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
    AnyMoveMoreThanLose,
    /// 上方修正
    ValueUp,
}
impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Reason::RepetitionBetterThanResign => "RepetitionBetterThanResign",
                Reason::NoUpdate => "NoUpdate",
                Reason::KingCatchIsStrongest => "KingCatchIsStrongest",
                Reason::FriendWin => "FriendWin",
                Reason::ThisBetterThanResign => "ThisBetterThanResign",
                Reason::AnyLeafBetterThanResign => "AnyLeafBetterThanResign",
                Reason::AnyLeafMoreThanLose => "AnyLeafMoreThanLose",
                Reason::Win => "Win",
                Reason::GoodPosition => "GoodPosition",
                Reason::AnyMoveMoreThanLose => "AnyMoveMoreThanLose",
                Reason::ValueUp => "ValueUp",
            }
        )
    }
}
