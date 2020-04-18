//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::shogi::playing::Game;
use crate::cosmic::shogi::recording::{Movement, SENNTITE_NUM};
use crate::cosmic::shogi::state::Person;
use crate::cosmic::shogi::state::Phase;
use crate::cosmic::smart::evaluator::Evaluation;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::movement_generator::generate_movement;
use crate::law::speed_of_light::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct TreeState {
    sum_state: u64,
    pub value: Option<i16>,

    movement_hash: u64,
    /// 何も起こっていなければ None が入っている。
    /// 相手玉を取ったら 別のフラグが立つ。
    /// その一手前は 0 が入る。
    /// さらに一手前は 1 が入る。
    /// この数は n手詰め に対応する。
    pub king_catch: Option<u16>,
    /// 玉を取る手が存在すれば真☆　必ずその指し手を選ぶだろう☆（＾～＾）
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
            king_catch: None,
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

    pub fn get_king_catch(&self) -> Option<u16> {
        self.king_catch
    }

    pub fn is_king_catch(&self) -> bool {
        self.king_catched
    }

    pub fn add_state(&mut self) {
        self.sum_state += 1;
    }

    pub fn add_turn_over(&mut self, opponent_ts: &TreeState) -> bool {
        self.sum_state += opponent_ts.get_sum_state();

        if let Some(king_catch) = opponent_ts.get_king_catch() {
            // ライオン・キャッチは１足します。
            self.king_catch = Some(king_catch + 1);
        } else if opponent_ts.is_king_catch() {
            // らいおんキャッチした。評価は付けません。
            self.movement_hash = opponent_ts.movement_hash;
            self.value = None;
            self.king_catch = Some(0);
            self.king_catched = true;
            self.reason = "lion catch".to_string();
            return true;
        } else if let Some(opponent_value) = opponent_ts.value {
            // 評価値は ひっくり返します。
            let friend_value = -opponent_value;
            if let Some(current_value) = self.value {
                if current_value < friend_value {
                    // 更新
                    self.value = Some(friend_value);
                    self.movement_hash = opponent_ts.movement_hash;
                    self.reason = "update value".to_string();
                    return true;
                }
            }
        } else {
            panic!("評価値が付いてないぜ☆（＾～＾）！")
        }
        false
    }

    pub fn check_leaf(&mut self, evaluation: &Evaluation, movement_hash: u64) -> bool {
        if self.movement_hash == 0 {
            // どんな手も 投了より良いだろ☆（＾～＾）
            self.movement_hash = movement_hash;
            self.value = Some(evaluation.value);
            self.reason = "this better than resign".to_string();
            return true;
        } else if evaluation.king_catch {
            // 玉を取ったぜ☆（＾～＾）！
            self.movement_hash = movement_hash;
            self.value = None;
            self.king_catch = Some(0);
            self.king_catched = true;
            self.reason = "king catch".to_string();
            return true;
        } else if let Some(current_value) = self.get_value() {
            if current_value < evaluation.value {
                self.movement_hash = movement_hash;
                self.value = Some(evaluation.value);
                self.reason = "good position".to_string();
                return true;
            }
        }
        false
    }

    pub fn get_movement_hash(&self) -> u64 {
        self.movement_hash
    }

    pub fn to_movement(&self) -> Movement {
        Movement::from_hash(self.movement_hash)
    }
}

pub struct Tree {}
impl Tree {
    pub fn first_move(speed_of_light: &SpeedOfLight, universe: &mut Universe) -> TreeState {
        universe.game.info.clear();

        let friend = universe.game.history.get_phase(Person::Friend);
        Tree::get_best_movement(
            friend,
            0,
            universe.option_max_depth - 1,
            &mut universe.game,
            speed_of_light,
            "",
        )
    }

    /// 現局面での最善手を返すぜ☆（*＾～＾*）
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
    fn get_best_movement(
        friend: Phase,
        cur_depth: u16,
        end_depth: u16,
        game: &mut Game,
        speed_of_light: &SpeedOfLight,
        pv: &str,
    ) -> TreeState {
        let mut ts = TreeState::default();
        /* TODO
        {
            // 指定局面の利き数ボード再計算。
            // 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）
            // 相手の利き升調べ（自殺手、特に王手放置回避漏れ　防止のため）
            recalculate_control_count(game, speed_of_light);
        }
        */
        // TODO 利きの差分更新をしたいぜ☆（＾～＾）
        //
        //  ・先後別の２つの利き数ボード、駒別の約３０種類の利き数ボードがあって、全て最新にすること。
        //  ・position で送られてくる指定局面は、一から全再計算☆（＾～＾）
        //  ・指す前の局面でやること。
        //      ・自分の盤上の駒を動かす前に、レイを飛ばして飛角香を逆探知すること☆（＾～＾）
        //      ・取られる駒がある場合、両者の駒を動かす前に、取られる駒の利きをスキャンすること☆（＾～＾）
        //  ・指した後の局面でやること。
        //      ・自分の駒を動かした先で、レイを飛ばして飛角香を逆探知すること☆（＾～＾）
        //      ・動かした駒が　飛角香なら、探知☆（＾～＾）ある程度パターンがあるはず☆（＾～＾）
        //      ・それ以外の駒は、差分のパターンが決まっているので、それに従って増減させること☆（＾～＾）
        //
        // 入力：　手番、移動元、動かした駒種類、移動先の取られる駒。
        // やること：　カウントを引くか、足す。
        //
        // do_move, undo_move. 両方用意すること☆（＾～＾）
        //
        // TODO 指し手の一覧を作るぜ☆（＾～＾）
        // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
        // 指し手はハッシュ値で入っている☆（＾～＾）
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
            if game.info.is_printable() {
                game.info.print(
                    cur_depth,
                    ts.get_sum_state(),
                    ts.get_value(),
                    ts.get_king_catch(),
                    ts.movement_hash,
                    &format!("{} resign EmptyMoves", pv),
                );
            }
            return ts;
        }

        // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
        let mut repetition_move_hash = 0u64;
        for movement_hash in movement_set.iter() {
            // 1手進めるぜ☆（＾～＾）
            ts.add_state();
            let movement = Movement::from_hash(*movement_hash);
            let captured_piece = game.do_move(&movement, speed_of_light);
            /*
            IO::debugln(&format!("n={} do.", sum_nodes));
            Commands::pos(&game);
            */

            // 千日手かどうかを判定する☆（＾～＾）
            if SENNTITE_NUM <= game.count_same_ky() {
                // 千日手なら、この手は戻そうぜ☆（＾～＾）
                repetition_move_hash = *movement_hash;
            } else if end_depth <= cur_depth {
                // ここを末端局面とするなら、変化した評価値を返すぜ☆（＾～＾）
                let evaluation = Evaluation::from_caputured_piece(captured_piece, speed_of_light);
                if ts.check_leaf(&evaluation, *movement_hash) {
                    if game.info.is_printable() {
                        game.info.print(
                            cur_depth,
                            ts.get_sum_state(),
                            ts.get_value(),
                            ts.get_king_catch(),
                            *movement_hash,
                            &format!("{} {} EndNode", pv, movement),
                        );
                    }
                }

                if ts.is_king_catch() {
                    // 玉を取ったのなら、もう帰るぜ☆（＾～＾）
                    break;
                }

            // IO::debugln(&format!("n={} Value={}.", sum_nodes, evaluation.value));
            } else {
                // 枝局面なら、更に深く進むぜ☆（＾～＾）
                let opponent_ts = Tree::get_best_movement(
                    friend.turn(),
                    cur_depth + 1,
                    end_depth,
                    game,
                    speed_of_light,
                    &format!("{} {}", pv, Movement::from_hash(*movement_hash)),
                );

                // 下の木の結果を、ひっくり返して、引き継ぎます。
                if ts.add_turn_over(&opponent_ts) {
                    // 指し手の更新があれば。
                    if game.info.is_printable() {
                        game.info.print(
                            cur_depth,
                            ts.get_sum_state(),
                            ts.get_value(),
                            ts.get_king_catch(),
                            *movement_hash,
                            &format!("{} {} Backward1", pv, Movement::from_hash(*movement_hash)),
                        );
                    }
                }

                if ts.is_king_catch() {
                    // らいおんキャッチする手を見つけていれば、もう探索しません。
                    break;
                }
            }
            // 1手戻すぜ☆（＾～＾）
            game.undo_move(speed_of_light);
            /*
            IO::debugln(&format!("n={} undo.", sum_nodes));
            Commands::pos(&game);
            */
        }

        if ts.get_movement_hash() == 0 {
            // 投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
            ts.movement_hash = repetition_move_hash
        };

        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        if game.info.is_printable() {
            game.info.print(
                cur_depth,
                ts.get_sum_state(),
                ts.get_value(),
                ts.get_king_catch(),
                ts.movement_hash,
                &format!("{} {}", pv, ts.to_movement()),
            );
        }

        ts
    }
}
