//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!
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
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::time::Instant;
/// 評価値（＾～＾）
pub type CentiPawn = i16;

/// TODO 千日手の価値☆（＾～＾） ENGIN OPTIONにしたいぜ☆（＾～＾）
pub const REPITITION_VALUE: CentiPawn = -300;

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
    // 反復深化探索（Iteration deeping）を使うときの、読みの深さの上限☆（＾～＾）
    id_depth: usize,
    id_max_depth: usize,
    // 時間切れ（＾～＾）
    pub timeout: bool,
    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_move: Move,
}
impl Tree {
    pub fn new(depth_not_to_give_up: usize) -> Self {
        Tree {
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            depth_not_to_give_up: depth_not_to_give_up,
            id_depth: 0,
            id_max_depth: 0,
            timeout: false,
            repetition_move: RESIGN_MOVE,
        }
    }
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(&mut self, universe: &mut Universe) -> (CentiPawn, Move) {
        universe.game.info.clear();
        self.think_sec = rand::thread_rng().gen_range(
            universe.option_min_think_sec as u64,
            universe.option_max_think_sec as u64,
        );

        // alpha値を上げていきたいが、beta値を超えたくない（＾～＾）
        // -32768 を - しても +32768 は無いので + 1 して調整（＾～＾）
        let mut alpha = i16::MIN + 1;
        // beta値は 相手の alpha値の正負を反対にしたもの
        let beta = i16::MAX;
        let mut bestmove = RESIGN_MOVE;

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for depth in 0..(universe.option_max_depth + 1) {
            self.id_max_depth = depth;
            self.id_depth = depth;
            // 探索（＾～＾）
            let (mut value, move_) = self.search(&mut universe.game, alpha, beta);
            value = -value;
            if self.timeout {
                // 思考時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }
            // 最後まで指せだぜ（＾～＾）
            // if move_ == RESIGN_MOVE {
            //     // すでに投了が見えているのなら探索終了だぜ☆（＾～＾）
            //     break;
            // }
            if value < alpha || beta < value {
                // 無視
            } else if bestmove <= RESIGN_MOVE || alpha <= value {
                alpha = value;
                bestmove = move_;
            }

            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            print_info(
                &mut universe.game.info,
                Some(depth),
                Some((self.state_nodes, self.nps())),
                Some(alpha),
                Some(bestmove),
                &Some(PvString::PV(
                    self.msec(),
                    format!("{}", format!("{}", to_move_code(bestmove))),
                )), // この指し手を選んだ時の pv の読み筋が欲しいぜ☆（＾～＾）
            );
        }

        (alpha, bestmove)
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
    fn search(&mut self, game: &mut Game, mut alpha: i16, beta: i16) -> (CentiPawn, Move) {
        let mut bestmove = RESIGN_MOVE;

        // TODO 葉ノードなら、評価値を返して終了（＾～＾）
        if self.id_depth < 1 {
            // 葉だぜ☆（＾～＾）

            // if let Some(_captured) = move_.captured {
            //     // TODO SEEやろうぜ☆（＾～＾）
            //     SEE::go(game, &movement.destination);
            // }

            // 現局面（は相手の手番）の駒割り評価値をひっくり返したもの☆（＾～＾）
            let leaf_value: CentiPawn = -game.position.material_advantage(game.history.get_phase());

            // 局面を評価するだけ（＾～＾） 指し手は返さないぜ（＾～＾）
            return (leaf_value, RESIGN_MOVE);
        }
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
            let move_list =
                PseudoLegalMoves::generate(game.history.get_phase(), &game.position, false);

            move_list
            //}
        };

        // 指せる手が無ければ投了☆（＾～＾）
        if move_list.is_empty() {
            return (alpha, RESIGN_MOVE);
        }

        // TODO 指し手のオーダリングをしたいが、難しいのでシャッフルしたろ（＾～＾）
        move_list.shuffle(&mut thread_rng());

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
            if self.think_sec < self.sec()
                && self.depth_not_to_give_up <= self.id_max_depth - self.id_depth
            {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                // タイムアウトしたんだったら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                self.timeout = true;
                return (alpha, bestmove);
            }

            let captured_piece: Option<PieceEx> = game.do_move(*move_);
            // 1手進めるぜ☆（＾～＾）
            self.state_nodes += 1;

            self.pv.push(*move_);

            // TODO 廃止方針☆（＾～＾）
            if let Some(captured_piece_val) = captured_piece {
                if captured_piece_val.piece.type_() == PieceType::K {
                    // 玉を取る手より強い手はないぜ☆（＾～＾）！探索終了～☆（＾～＾）！この手を選べだぜ☆（＾～＾）！
                    bestmove = *move_;
                    alpha = i16::MAX;

                    self.pv.pop();
                    game.undo_move();
                    break;
                }
            }

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_position() {
                // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）何もせず次へ（＾～＾）
                self.repetition_move = *move_;
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                self.id_depth -= 1;
                let (mut node_value, _) = self.search(game, -beta, -alpha);
                node_value = -node_value;
                self.id_depth += 1;

                // if self.timeout {
                //     // TODO すでにタイムアウトしていたのなら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                //     return (alpha, bestmove);
                // }

                // 初期状態が 投了なので、更新したい（＾～＾）
                if bestmove == RESIGN_MOVE || alpha <= node_value {
                    // (1) どんな悪手も、投了より良いだろ☆（＾～＾）
                    // (2) アルファー・アップデート
                    alpha = node_value;
                    bestmove = *move_;
                }
            }

            self.pv.pop();
            game.undo_move();

            // すべての弟ノードのベータカット判定☆（＾～＾）
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

        if bestmove == RESIGN_MOVE {
            // 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
            bestmove = self.repetition_move;
            alpha = REPITITION_VALUE;
        }

        (alpha, bestmove)
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
