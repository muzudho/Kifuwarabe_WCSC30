//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!
use crate::entities::cosmic::playing::Game;
use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::recording::{PLY_LEN, SENNTITE_NUM};
use crate::entities::cosmic::smart::features::PieceType;
use crate::entities::cosmic::universe::Universe;
use crate::entities::spaceship::equipment::PvString;
use crate::movegen::{PieceEx, PseudoLegalMoves};
use crate::position::to_move_code;
use crate::record::RESIGN_MOVE;
use crate::take1base::Move;
use crate::view::print_info;
use std::fmt;
use std::time::Instant;
//use rand::Rng;
/// 評価値（＾～＾）
pub type CentiPawn = i16;

/// 正、負の符号を付けた時に、どちらでも使える最大の数
pub const VALUE_INFINITE: i16 = 32767;

/// TODO 千日手の価値☆（＾～＾） ENGIN OPTIONにしたいぜ☆（＾～＾）
pub const REPITITION_VALUE: CentiPawn = -300;

pub struct SearchStack {
    // 自分
    us: Phase,
    // この木を生成したと同時にストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,
    // 状態ノード数☆（＾～＾）
    pub state_nodes: u64,

    // 読み筋(Principal variation)☆（＾～＾）
    pv: PrincipalVariation,

    // 1手に費やす思考時間（秒）
    think_sec: u64,

    // 反復深化探索の１回目だけ真☆（＾～＾）
    pub depth_not_to_give_up: usize,
    // 反復深化探索（Iteration deeping）を使うときの、読みの深さの上限☆（＾～＾）
    id_depth: usize,
    id_max_depth: usize,
    // 時間切れ（＾～＾）
    pub is_time_up: bool,
    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_move: Move,
}
impl SearchStack {
    pub fn new(depth_not_to_give_up: usize) -> Self {
        SearchStack {
            us: Phase::First,
            stopwatch: Instant::now(),
            state_nodes: 0,
            pv: PrincipalVariation::default(),
            think_sec: 0,
            depth_not_to_give_up: depth_not_to_give_up,
            id_depth: 0,
            id_max_depth: 0,
            is_time_up: false,
            repetition_move: RESIGN_MOVE,
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

/// 反復深化探索だぜ☆（＾～＾）
pub fn iterative_deepening_search(
    universe: &mut Universe,
    ss: &mut SearchStack,
    think_sec: u64,
) -> (CentiPawn, Move) {
    universe.game.info.clear();
    ss.think_sec = think_sec;
    // ss.think_sec = rand::thread_rng()
    //     .gen_range(universe.option_min_think_sec as u64..universe.option_max_think_sec as u64);

    ss.us = universe.game.history.get_phase();

    // アルファベータ探索
    let mut alpha = -VALUE_INFINITE;
    let beta = VALUE_INFINITE;
    let mut bestmove = RESIGN_MOVE;

    // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
    for depth in 0..(universe.option_max_depth + 1) {
        ss.id_max_depth = depth;
        ss.id_depth = depth;
        // 探索（＾～＾）
        let (node_value, move_) = search(&mut universe.game, ss, alpha, beta);
        if ss.is_time_up {
            // 思考時間切れなら この探索結果は使わないぜ☆（＾～＾）
            break;
        }
        // 最後まで指せだぜ（＾～＾）
        // if move_ == RESIGN_MOVE {
        //     // すでに投了が見えているのなら探索終了だぜ☆（＾～＾）
        //     break;
        // }
        if node_value < alpha || beta < node_value {
            // 無視
        } else if bestmove <= RESIGN_MOVE || alpha <= node_value {
            alpha = node_value;
            bestmove = move_;
        }

        // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
        print_info(
            &mut universe.game.info,
            Some(depth),
            Some((ss.state_nodes, ss.nps())),
            Some(alpha),
            Some(bestmove),
            &Some(PvString::PV(
                ss.msec(),
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
fn search(game: &mut Game, ss: &mut SearchStack, mut alpha: i16, beta: i16) -> (CentiPawn, Move) {
    let mut bestmove = RESIGN_MOVE;

    // TODO 葉ノードなら、評価値を返して終了（＾～＾）
    if ss.id_depth <= 0 {
        // 葉だぜ☆（＾～＾）

        // if let Some(_captured) = move_.captured {
        //     // TODO SEEやろうぜ☆（＾～＾）
        //     SEE::go(game, &movement.destination);
        // }

        // 現局面（は相手の手番）の駒割り評価値をひっくり返したもの☆（＾～＾）
        let leaf_value: CentiPawn = game.position.material_advantage(game.history.get_phase());

        // 局面を評価するだけ（＾～＾） 指し手は返さないぜ（＾～＾）
        return (leaf_value, RESIGN_MOVE);
    }
    // TODO let mut controls = Vec::<Square>::new();

    // 指し手の一覧を作るぜ☆（＾～＾） 指し手はハッシュ値で入っている☆（＾～＾）
    let move_list = {
        /*
        // TODO 1手詰めは必ず仕留めなければいけないぜ☆（＾～＾）？
        let mut lioncatch = Lioncatch::new(game);
        lioncatch.init(game).pinned_pieces(game).checkers(game);
        if !lioncatch.checks.is_empty() {
            lioncatch.checks
        } else {
            //   */
        let move_list = PseudoLegalMoves::generate(game.history.get_phase(), &game.position, false);

        move_list
        //}
    };

    // 指せる手が無ければ投了☆（＾～＾）
    if move_list.is_empty() {
        return (alpha, RESIGN_MOVE);
    }

    // TODO この利きは、この１手を指すまえの利き（１年前の夜空を見ていることを１光年と言うだろ）をキープしているということに注意しろだぜ☆（＾～＾）
    // いわば、１光手 利きカウントボードだぜ☆（＾～＾）
    // for destination in &controls {
    //     game.position
    //         .add_control(game.history.get_phase(), destination, 1);
    // }

    for move_ in move_list.iter() {
        // 時間を見ようぜ☆（＾～＾）？
        if ss.think_sec < ss.sec() && ss.depth_not_to_give_up <= ss.id_max_depth - ss.id_depth {
            // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
            // タイムアウトしたんだったら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
            ss.is_time_up = true;
            // タイムアウトしたときの探索結果は使わないぜ（＾～＾）
            return (0, RESIGN_MOVE);
        }

        let captured_piece: Option<PieceEx> = game.do_move(*move_);
        // 1手進めるぜ☆（＾～＾）
        ss.state_nodes += 1;

        ss.pv.push(*move_);

        // TODO 廃止方針☆（＾～＾）
        if let Some(captured_piece_val) = captured_piece {
            if captured_piece_val.piece.type_() == PieceType::K {
                // 玉を取る手より強い手はないぜ☆（＾～＾）！探索終了～☆（＾～＾）！この手を選べだぜ☆（＾～＾）！
                bestmove = *move_;
                alpha = i16::MAX;

                ss.pv.pop();
                game.undo_move();
                break;
            }
        }

        // 千日手かどうかを判定する☆（＾～＾）
        if SENNTITE_NUM <= game.count_same_position() {
            // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）何もせず次へ（＾～＾）
            ss.repetition_move = *move_;
        } else {
            // 枝局面なら、更に深く進むぜ☆（＾～＾）
            ss.id_depth -= 1;
            let (node_value, _) = search(game, ss, -beta, -alpha);
            let edge_value = -node_value;
            ss.id_depth += 1;

            // TODO ルートノードで、3秒経過していたら info を出力したいぜ（＾～＾）
            // TODO infoの出力は pv でやる（＾～＾）？
            // TODO タイミングによっては、読みの浅い所で表示してしまうが（＾～＾）？
            if game.info.is_printable() {
                // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                // let movement = ts.bestmove.movement;
                // print_info(
                //     None,
                //     None,
                //     None,
                //     None,
                //     &Some(PvString::String(format!(
                //         "ways={} | komawari={} | promotion={}", //  | {} {} {} |
                //         ss.evaluation.ways(),
                //         ss.evaluation.komawari(),
                //         ss.evaluation.promotion(),
                //     ))),
                // );
                print_info(
                    &mut game.info,
                    Some(ss.id_max_depth - ss.id_depth),
                    Some((ss.state_nodes, ss.nps())),
                    Some(if game.history.get_phase() == ss.us {
                        edge_value
                    } else {
                        node_value
                    }),
                    Some(*move_),
                    &Some(PvString::PV(ss.msec(), format!("{}", ss.pv))),
                );
            }

            // 説明変数：何か１つは指し手を選んでおかないと、投了してしまうから、最初の１手は候補に入れておけだぜ（＾～＾）
            let is_any_one_move = bestmove == RESIGN_MOVE;
            if is_any_one_move || alpha <= edge_value {
                // アルファー・アップデート
                alpha = edge_value;
                bestmove = *move_;
            }
        }

        ss.pv.pop();
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

    if bestmove == RESIGN_MOVE {
        // 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
        bestmove = ss.repetition_move;
        alpha = REPITITION_VALUE;
    }

    (alpha, bestmove)
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

    // fn len(&self) -> usize {
    //     self.ply
    // }
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
