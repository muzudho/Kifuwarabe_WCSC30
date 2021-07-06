//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!
pub mod evaluator;
pub mod move_ex;

use crate::entities::cosmic::playing::Game;
use crate::entities::cosmic::recording::{PLY_LEN, SENNTITE_NUM};
use crate::entities::cosmic::smart::features::PieceType;
use crate::entities::cosmic::universe::Universe;
use crate::entities::spaceship::equipment::{Beam, PvString};
use crate::movegen::{PieceEx, PseudoLegalMoves};
use crate::position::destructure_move;
use crate::position::is_board_square;
use crate::position::to_move_code;
use crate::record::RESIGN_MOVE;
use crate::search::evaluator::Evaluation;
use crate::search::evaluator::REPITITION_VALUE;
use crate::take1base::Move;
use crate::view::print_info;
use rand::Rng;
use std::fmt;
use std::time::Instant;

#[derive(Clone)]
pub struct MoveEx {
    pub value: Value,
    pub move_: Move,
    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: Reason,
}

pub struct Tree {
    // この木を生成したと同時にストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,
    // 状態ノード数☆（＾～＾）
    pub state_nodes: u64,

    // Principal variation(読み筋)☆（＾～＾）
    pv: PrincipalVariation,

    // 思考時間（秒）をランダムにすることで、指し手を変えるぜ☆（＾～＾）
    think_sec: u64,

    /// 局面評価値の計算
    pub evaluation: Evaluation,

    // 反復深化探索の１回目だけ真☆（＾～＾）
    pub depth_not_to_give_up: usize,
    // 読みの深さの上限☆（＾～＾）１手を読み切るなら 0 を指定しろだぜ☆（＾～＾）
    max_depth0: usize,
}
impl Tree {
    pub fn new(
        many_ways_weight: isize,
        material_advantage_weight: isize,
        promotion_weight: isize,
        depth_not_to_give_up: usize,
    ) -> Self {
        Tree {
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            evaluation: Evaluation::new(
                many_ways_weight,
                material_advantage_weight,
                promotion_weight,
            ),
            depth_not_to_give_up: depth_not_to_give_up,
            max_depth0: 0,
        }
    }
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(&mut self, universe: &mut Universe) -> (MoveEx, TreeState) {
        universe.game.info.clear();
        self.think_sec = rand::thread_rng().gen_range(
            universe.option_min_think_sec as u64,
            universe.option_max_think_sec as u64,
        );

        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        self.evaluation.before_search();
        self.max_depth0 = 0;
        let (mut bestmove, mut best_ts) = self.search(&mut universe.game, Value::Win);
        self.evaluation.after_search();

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for id in 1..universe.option_max_depth {
            self.max_depth0 = id;
            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            print_info(
                &mut universe.game.info,
                Some(self.max_depth0),
                Some((self.state_nodes, self.nps())),
                Some(bestmove.value),
                Some(bestmove.move_),
                &Some(PvString::PV(
                    self.msec(),
                    format!("{}", format!("{}", to_move_code(bestmove.move_))),
                )), // この指し手を選んだ時の pv の読み筋が欲しいぜ☆（＾～＾）
            );

            if bestmove.move_ == RESIGN_MOVE {
                // すでに投了が見えているのなら探索終了だぜ☆（＾～＾）
                break;
            }

            // 横線で仕切るぜ☆（＾～＾）
            print_info(
                &mut universe.game.info,
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
            let (bestmove_tmp, ts) = self.search(&mut universe.game, Value::Win);
            bestmove = bestmove_tmp;
            self.evaluation.after_search();
            if ts.timeout {
                // 思考時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }

            // 無条件に更新だぜ☆（＾～＾）初手の高得点を引きずられて王手回避漏れされたら嫌だしな☆（＾～＾）
            best_ts = ts.clone();
        }

        (bestmove, best_ts)
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
    fn search(&mut self, game: &mut Game, another_branch_best: Value) -> (MoveEx, TreeState) {
        let mut ts = TreeState::default();
        let mut bestmove = MoveEx::default();

        // この手を指すと負けてしまう、という手が見えていたら、このフラグを立てろだぜ☆（＾～＾）
        let mut exists_lose = false;

        // TODO let mut controls = Vec::<Square>::new();

        // 指し手の一覧を作るぜ☆（＾～＾） 指し手はハッシュ値で入っている☆（＾～＾）
        let mut move_list = {
            /*
            // TODO 1手詰めは必ず仕留めなければいけないぜ☆（＾～＾）？
            let mut lioncatch = Lioncatch::new(game);
            lioncatch.init(game).pinned_pieces(game).checkers(game);
            if !lioncatch.checks.is_empty() {
                lioncatch.checks
            } else {
                //   */
            let move_list = PseudoLegalMoves::generate(game.history.get_phase(), &game.position);

            move_list
            //}
        };

        // 指せる手が無ければ投了☆（＾～＾）
        if move_list.is_empty() {
            return (bestmove, ts);
        }

        // TODO この利きは、この１手を指すまえの利き（１年前の夜空を見ていることを１光年と言うだろ）をキープしているということに注意しろだぜ☆（＾～＾）
        // いわば、１光手 利きカウントボードだぜ☆（＾～＾）
        // for destination in &controls {
        //     game.position
        //         .add_control(game.history.get_phase(), destination, 1);
        // }

        // 指し手のオーダリングをしたいぜ☆（＾～＾） 取った駒は指し手生成の段階で調べているし☆（＾～＾）
        let mut cap = 0;
        if 1 < move_list.len() {
            for i in 0..move_list.len() {
                let (_, to, _) = destructure_move(move_list[i]);
                if let Some(_captured) = game.position.piece_at(to) {
                    // 駒を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                    // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                    move_list.swap(cap, i);
                    cap += 1;
                }
            }
            // 次は駒を取ったグループの中で、玉を取った手をグループの先頭に集めるぜ☆（＾～＾）
            let mut king = 0;
            for i in 0..cap {
                let (_, to, _) = destructure_move(move_list[i]);
                if let Some(captured) = game.position.piece_at(to) {
                    match captured.piece.type_() {
                        PieceType::K => {
                            // 玉を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                            // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                            move_list.swap(king, i);
                            king += 1;
                        }
                        _ => {}
                    }
                } else {
                    panic!("captured fail")
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
        self.evaluation.add_control(coverage_sign, &move_list);
        for move_ in move_list.iter() {
            // 時間を見ようぜ☆（＾～＾）？
            if self.think_sec < self.sec() && self.depth_not_to_give_up <= self.max_depth0 {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                // タイムアウトしたんだったら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                ts.timeout = true;
                return (bestmove, ts);
            }

            // 1手進めるぜ☆（＾～＾）
            self.state_nodes += 1;

            let (from, _, promote) = destructure_move(*move_);

            let from_pc = if is_board_square(from) {
                game.position.piece_at(from)
            } else {
                // 打
                None
            };
            let captured_piece: Option<PieceEx> = game.do_move(*move_);
            self.pv.push(*move_);
            let (captured_piece_centi_pawn, delta_promotion_bonus) =
                self.evaluation
                    .after_do_move(&from_pc, &captured_piece, promote);

            // TODO 廃止方針☆（＾～＾）
            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.piece.type_() == PieceType::K {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！探索終了～☆（＾～＾）！この手を選べだぜ☆（＾～＾）！
                    bestmove.catch_king(*move_);

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
                ts.repetition_move = *move_;
            } else if self.max_depth0 < self.pv.len() {
                // 葉だぜ☆（＾～＾）

                // if let Some(_captured) = move_.captured {
                //     // TODO SEEやろうぜ☆（＾～＾）
                //     SEE::go(game, &movement.destination);
                // }

                // 評価を集計するぜ☆（＾～＾）
                ts.choice_friend(
                    &mut bestmove,
                    Value::CentiPawn(self.evaluation.centi_pawn()),
                    *move_,
                );

                if game.info.is_printable() {
                    // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                    // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                    print_info(
                        &mut game.info,
                        None,
                        None,
                        None,
                        None,
                        &Some(PvString::String(format!(
                            "move_list={} | material={} | promotion={}", //  | {} {} {} |
                            self.evaluation.move_list(),
                            self.evaluation.material_advantage(),
                            self.evaluation.promotion(),
                            /* TODO
                            // サンプルを見ているだけだぜ☆（＾～＾）
                            game.position.get_control(
                                game.history.get_phase(),
                                (6, 8)
                            ),
                            game.position.get_control(
                                game.history.get_phase(),
                                (5, 8)
                            ),
                            game.position.get_control(
                                game.history.get_phase(),
                                (4, 8)
                            ),
                            */
                        ))),
                    );
                    print_info(
                        &mut game.info,
                        Some(self.pv.len()),
                        Some((self.state_nodes, self.nps())),
                        Some(bestmove.value),
                        Some(bestmove.move_),
                        &Some(PvString::PV(self.msec(), format!("{}", self.pv))),
                    );
                }
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                self.evaluation.before_search();
                let (child_move, _) = self.search(
                    game,
                    match bestmove.value {
                        Value::CentiPawn(centi_pawn) => Value::CentiPawn(-centi_pawn),
                        Value::Win => Value::Lose,
                        Value::Lose => Value::Win,
                    },
                );

                if ts.timeout {
                    // すでにタイムアウトしていたのなら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                    return (bestmove, ts);
                }
                self.evaluation.after_search();

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                let friend_move = *move_;
                let friend_centi_pawn = self.evaluation.centi_pawn();
                exists_lose = {
                    // TODO 玉を取られてたら、ここは投了すべき☆（＾～＾）？

                    // TODO 相手が投了してたら、必ず選ぶべき☆（＾～＾）？

                    match child_move.value {
                        Value::Win => {
                            // 相手が勝ったので、自分は負けてるぜ☆（＾～＾）この手は選んではいけないぜ☆（＾～＾）
                            true
                        }
                        Value::Lose => {
                            // 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
                            bestmove.update(friend_move, Value::Win, Reason::FriendWin);
                            false
                        }
                        Value::CentiPawn(num) => {
                            // 評価値は ひっくり返します。この指し手の駒の交換値も足します。
                            let friend_centi_pawn2 = -num + friend_centi_pawn;
                            if bestmove.move_ != RESIGN_MOVE {
                                // どんな悪手も、詰みでなければ 投了より良いだろ☆（＾～＾）
                                bestmove.update(
                                    friend_move,
                                    Value::CentiPawn(friend_centi_pawn2),
                                    Reason::ThisBetterThanResign,
                                );
                            } else {
                                match bestmove.value {
                        Value::Win => {
                            std::panic::panic_any(Beam::trouble(
                                "(Err.405) 自分が勝つ手を既に読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）"
                            ))
                        }
                        Value::Lose => {
                            // 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
                            bestmove.update(
                                friend_move,
                                Value::CentiPawn(friend_centi_pawn2),
                                Reason::AnyMoveMoreThanLose,
                            );
                        }
                        Value::CentiPawn(best_centi_pawn) => {
                            if best_centi_pawn < friend_centi_pawn2 {
                                // 上方修正
                                bestmove.update(
                                    friend_move,
                                    Value::CentiPawn(friend_centi_pawn2),
                                    Reason::ValueUp,
                                );
                            }
                        }
                    }
                            }
                            false
                        }
                    }
                };
            }

            self.evaluation
                .before_undo_move(captured_piece_centi_pawn, delta_promotion_bonus);
            self.pv.pop();
            game.undo_move();

            match bestmove.value {
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
        self.evaluation.add_control(-1 * coverage_sign, &move_list);

        // TODO 利き削除☆（＾～＾）
        // for destination in &controls {
        //     game.position
        //         .add_control(game.history.get_phase(), destination, -1);
        // }

        if !exists_lose {
            if bestmove.move_ == RESIGN_MOVE {
                // 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
                bestmove.update(
                    ts.repetition_move,
                    Value::CentiPawn(REPITITION_VALUE),
                    Reason::RepetitionBetterThanResign,
                );
            }
        }

        (bestmove, ts)
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
pub struct TreeState {
    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_move: Move,
    pub timeout: bool,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            repetition_move: RESIGN_MOVE,
            timeout: false,
        }
    }
}
impl TreeState {
    /// 指し手のベストを選ぶぜ☆（＾～＾）
    pub fn choice_friend(&mut self, bestmove: &mut MoveEx, value: Value, move_: Move) {
        if bestmove.move_ == RESIGN_MOVE {
            // どんな葉も 投了より良いだろ☆（＾～＾）
            // TODO でも、王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
            bestmove.update(move_, value, Reason::AnyLeafBetterThanResign);
            return;
        } else {
            match bestmove.value {
                Value::Win => std::panic::panic_any(Beam::trouble(
                    "(Err.397) 自分が勝つ手を読んでるなら、ここに来るのはおかしいぜ☆（＾～＾）",
                )),
                Value::Lose => {
                    // どんな評価値でも、負けるよりマシだろ☆（＾～＾）
                    bestmove.update(move_, value, Reason::AnyLeafMoreThanLose);
                    return;
                }
                Value::CentiPawn(best_centi_pawn) => {
                    match value {
                        Value::Win => {
                            // 勝つんだから更新するぜ☆（＾～＾）
                            bestmove.update(move_, value, Reason::Win);
                            return;
                        }
                        Value::Lose => {
                            // TODO ここは通らないぜ☆（＾～＾）要対応☆（＾～＾）
                        }
                        Value::CentiPawn(leaf_centi_pawn) => {
                            if best_centi_pawn < leaf_centi_pawn {
                                // 評価値が良かったから更新☆（＾～＾）
                                bestmove.update(move_, value, Reason::GoodPosition);
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
    moves: [Move; PLY_LEN],
    ply: usize,
}
impl Default for PrincipalVariation {
    fn default() -> Self {
        PrincipalVariation {
            // ゴミの値で埋めるぜ☆（＾～＾）
            moves: [RESIGN_MOVE; PLY_LEN],
            ply: 0,
        }
    }
}
impl PrincipalVariation {
    fn push(&mut self, move_: Move) {
        self.moves[self.ply] = move_;
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
            buffer.push_str(&format!("{} ", to_move_code(self.moves[i])));
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
