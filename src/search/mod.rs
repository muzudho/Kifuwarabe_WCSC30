//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!
pub mod move_ex;

use crate::entities::cosmic::playing::Game;
use crate::entities::cosmic::recording::{PLY_LEN, SENNTITE_NUM};
use crate::entities::cosmic::smart::features::PieceType;
use crate::entities::cosmic::universe::Universe;
use crate::entities::spaceship::equipment::PvString;
use crate::movegen::{PieceEx, PseudoLegalMoves};
use crate::position::destructure_move;
use crate::position::to_move_code;
use crate::record::RESIGN_MOVE;
use crate::take1base::Move;
use crate::view::print_info;
use rand::Rng;
use std::fmt;
use std::time::Instant;

/// 評価値（＾～＾）
pub type CentiPawn = i16;

/// TODO 千日手の価値☆（＾～＾） ENGIN OPTIONにしたいぜ☆（＾～＾）
pub const REPITITION_VALUE: CentiPawn = -300;

#[derive(Clone)]
pub struct MoveEx {
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

    // 反復深化探索の１回目だけ真☆（＾～＾）
    pub depth_not_to_give_up: usize,
    // 読みの深さの上限☆（＾～＾）１手を読み切るなら 0 を指定しろだぜ☆（＾～＾）
    max_depth0: usize,
}
impl Tree {
    pub fn new(depth_not_to_give_up: usize) -> Self {
        Tree {
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            depth_not_to_give_up: depth_not_to_give_up,
            max_depth0: 0,
        }
    }
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(&mut self, universe: &mut Universe) -> (CentiPawn, MoveEx, TreeState) {
        universe.game.info.clear();
        self.think_sec = rand::thread_rng().gen_range(
            universe.option_min_think_sec as u64,
            universe.option_max_think_sec as u64,
        );

        // alpha値を上げていきたいが、beta値を超えたくない（＾～＾）
        let mut alpha = i16::MIN;
        let beta = i16::MAX;
        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        self.max_depth0 = 0;
        let (node_value, mut bestmove, mut best_ts) = self.search(&mut universe.game, alpha, beta);
        if node_value < alpha || beta < node_value {
            // 無視
        } else if alpha < node_value {
            alpha = node_value
        }

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for id in 1..universe.option_max_depth {
            self.max_depth0 = id;
            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            print_info(
                &mut universe.game.info,
                Some(self.max_depth0),
                Some((self.state_nodes, self.nps())),
                Some(alpha),
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
            let (node_value, bestmove_tmp, ts) = self.search(&mut universe.game, -beta, -alpha);
            if ts.timeout {
                // 思考時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }
            if node_value < alpha || beta < node_value {
                // 無視
            } else if alpha < node_value {
                alpha = node_value;
                bestmove = bestmove_tmp;
            }

            // 無条件に更新だぜ☆（＾～＾）初手の高得点を引きずられて王手回避漏れされたら嫌だしな☆（＾～＾）
            best_ts = ts.clone();
        }

        (-alpha, bestmove, best_ts)
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
    fn search(
        &mut self,
        game: &mut Game,
        mut alpha: i16,
        beta: i16,
    ) -> (CentiPawn, MoveEx, TreeState) {
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
            return (-alpha, bestmove, ts);
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
                if let Some(_captured) = game.position.piece_at_board(to) {
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
                if let Some(captured) = game.position.piece_at_board(to) {
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

        for move_ in move_list.iter() {
            // 時間を見ようぜ☆（＾～＾）？
            if self.think_sec < self.sec() && self.depth_not_to_give_up <= self.max_depth0 {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                // タイムアウトしたんだったら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                ts.timeout = true;
                return (-alpha, bestmove, ts);
            }

            // 1手進めるぜ☆（＾～＾）
            self.state_nodes += 1;

            let captured_piece: Option<PieceEx> = game.do_move(*move_);
            self.pv.push(*move_);

            // TODO 廃止方針☆（＾～＾）
            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.piece.type_() == PieceType::K {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！探索終了～☆（＾～＾）！この手を選べだぜ☆（＾～＾）！
                    bestmove.catch_king(*move_);
                    alpha = i16::MAX;

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

                // 現局面（は相手の手番）の駒割り評価値をひっくり返したもの☆（＾～＾）
                let leaf_value: CentiPawn =
                    -game.position.material_advantage(game.history.get_phase());
                {
                    if bestmove.move_ == RESIGN_MOVE {
                        // どんな葉も 投了よりは更新したいだろ☆（＾～＾）
                        bestmove.update(*move_, Reason::AnyLeafBetterThanResign);
                        alpha = leaf_value;
                    } else {
                        if alpha < leaf_value {
                            // 評価値が良かったから更新☆（＾～＾）
                            bestmove.update(*move_, Reason::GoodPosition);
                            alpha = leaf_value;
                        }
                    }
                }
                if game.info.is_printable() {
                    // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                    // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                    print_info(
                        &mut game.info,
                        Some(self.pv.len()),
                        Some((self.state_nodes, self.nps())),
                        Some(alpha),
                        Some(bestmove.move_),
                        &Some(PvString::PV(self.msec(), format!("{}", self.pv))),
                    );
                }
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let (node_value, child_move, _) = self.search(game, -beta, -alpha);

                if ts.timeout {
                    // すでにタイムアウトしていたのなら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                    return (alpha, bestmove, ts);
                }

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                let friend_move = *move_;
                exists_lose = {
                    // TODO 玉を取られてたら、ここは投了すべき☆（＾～＾）？

                    // TODO 相手が投了してたら、必ず選ぶべき☆（＾～＾）？

                    if bestmove.move_ != RESIGN_MOVE {
                        // どんな悪手も、詰みでなければ 投了より良いだろ☆（＾～＾）
                        bestmove.update(friend_move, Reason::ThisBetterThanResign);
                        alpha = node_value;
                        bestmove = child_move;
                    } else {
                        if alpha < node_value {
                            // 上方修正
                            bestmove.update(friend_move, Reason::ValueUp);
                            alpha = node_value;
                            bestmove = child_move;
                        }
                    }
                    false
                };
            }

            self.pv.pop();
            game.undo_move();

            // ベータカット判定☆（＾～＾）
            if beta < alpha {
                // 兄弟局面より良い手を見つけたのなら、相手から見ればこの手は選ばないから、もう探索しなくていいぜ☆（＾～＾）
                // これが　いわゆるベータカットだぜ☆（＾～＾）
                break;
            }
        }

        // TODO 利き削除☆（＾～＾）
        // for destination in &controls {
        //     game.position
        //         .add_control(game.history.get_phase(), destination, -1);
        // }

        if !exists_lose {
            if bestmove.move_ == RESIGN_MOVE {
                // 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
                bestmove.update(ts.repetition_move, Reason::RepetitionBetterThanResign);
                alpha = REPITITION_VALUE;
            }
        }

        (-alpha, bestmove, ts)
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
