use crate::entities::cosmic::recording::{History, PHASE_FIRST, PHASE_LEN, PHASE_SECOND};
use crate::entities::cosmic::smart::features::{HandPiece, HAND_ADDRESS_LEN, HAND_MAX};
use crate::entities::cosmic::smart::square::BOARD_MEMORY_AREA;
use crate::entities::move_::to_move_object;
use crate::entities::spaceship::equipment::{Beam, DestinationDisplay};
use crate::movegen::PieceEx;
use crate::position::destructure_move;
use crate::position::position::Position;
use crate::take1base::Move;
use crate::take1base::PIECE_MEANING_LEN;
use rand::Rng;

/// 局面
pub enum PosNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/// 現対局ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct GameHashSeed {
    // 盤上の駒
    pub piece_hash: [[u64; PIECE_MEANING_LEN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒の枚数 最大で 0～18 の 19サイズ
    pub hand_hash: [[u64; HAND_MAX + 1]; HAND_ADDRESS_LEN],
    // 先後
    pub phase: [u64; PHASE_LEN],
}

pub struct Game {
    /// 棋譜
    pub history: History,
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
    /// 初期盤面
    pub starting_board: Position,
    /// 現対局ハッシュ種☆（＾～＾）
    pub hash_seed: GameHashSeed,
    /// 現盤面
    pub position: Position,
    /// 情報表示担当
    pub info: DestinationDisplay,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            history: History::default(),
            starting_position_hash: 0,
            starting_board: Position::default(),
            hash_seed: GameHashSeed {
                // 盤上の駒
                piece_hash: [[0; PIECE_MEANING_LEN]; BOARD_MEMORY_AREA as usize],
                // 持ち駒
                hand_hash: [[0; HAND_MAX + 1]; HAND_ADDRESS_LEN],
                // 先後
                phase: [0; PHASE_LEN],
            },
            position: Position::default(),
            info: DestinationDisplay::default(),
        }
    }
}
impl Game {
    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear(&mut self) {
        self.history.clear();
        self.starting_position_hash = 0;
        self.starting_board.clear();
        self.position.clear();
    }
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_square in 11..BOARD_MEMORY_AREA {
            for i_piece in 0..PIECE_MEANING_LEN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.hash_seed.piece_hash[i_square as usize][i_piece] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_piece in 0..HAND_ADDRESS_LEN {
            for i_count in 0..HAND_MAX + 1 {
                self.hash_seed.hand_hash[i_piece][i_count] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LEN {
            self.hash_seed.phase[i_phase] =
                rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    /// 棋譜の作成
    pub fn set_move(&mut self, move_: Move) {
        self.history.moves[self.history.moves_num() as usize] = move_
    }
    pub fn get_move(&self) -> Move {
        self.history.moves[self.history.moves_num() as usize]
    }
    /// デバッグ用に棋譜表示☆（＾～＾） 普通に棋譜が欲しいときは sfen 見ろだぜ（＾～＾）
    pub fn get_moves_history_debug_text(&self) -> String {
        let mut s = String::new();
        for moves_num in 0..self.history.moves_num() {
            let m = self.history.moves[moves_num as usize];
            let (from, to, promote) = destructure_move(m);

            s.push_str(&format!(
                "[{}]{} {}{} ",
                moves_num,
                from.number(),
                to.number(),
                if promote { "+" } else { "" }
            ));
        }
        s
    }

    pub fn set_position_hash(&mut self, hash: u64) {
        self.history.position_hashs[self.history.moves_num() as usize] = hash;
    }
    pub fn set_captured(&mut self, ply1: usize, pc: Option<PieceEx>) {
        self.history.captured_pieces[ply1] = pc
    }

    pub fn get_board(&self, num: PosNums) -> &Position {
        match num {
            PosNums::Current => &self.position,
            PosNums::Start => &self.starting_board,
        }
    }
    pub fn mut_starting(&mut self) -> &mut Position {
        &mut self.starting_board
    }

    /// テスト用に局面ハッシュ☆（＾～＾）
    pub fn get_positions_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.starting_position_hash));

        for ply in 0..self.history.moves_num() {
            let hash = &self.history.position_hashs[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /// 初期局面ハッシュを作り直す
    pub fn create_starting_position_hash(&self) -> u64 {
        let mut hash = self.starting_board.create_hash(&self);

        // 手番ハッシュ（後手固定）
        hash ^= self.hash_seed.phase[PHASE_SECOND];

        hash
    }

    /// 局面ハッシュを作り直す
    pub fn create_current_position_hash(&self) -> u64 {
        let mut hash = self.position.create_hash(&self);

        // 手番ハッシュ
        use crate::entities::cosmic::recording::Phase::*;
        match self.history.get_phase() {
            First => hash ^= self.hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.hash_seed.phase[PHASE_SECOND],
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_position(&self) -> isize {
        if self.history.moves_num() < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.moves_num() - 1;
        let new_ply = self.history.moves_num();
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            if self.history.position_hashs[t as usize]
                == self.history.position_hashs[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.starting_position_hash == self.history.position_hashs[last_ply as usize] {
            count += 1;
        }

        count
    }

    /// 入れた指し手の通り指すぜ☆（＾～＾）
    ///
    /// # Returns
    ///
    /// 取った駒
    pub fn do_move(&mut self, move_: Move) -> Option<PieceEx> {
        // もう入っているかも知れないが、棋譜に入れる☆
        self.set_move(move_);
        let us = self.history.get_phase();
        // let (from, to, pro) = destructure_move(move_);
        let (from2, to2, promote2, drop2) = to_move_object(us, move_);

        // TODO 利き
        {
            // game.position.controls[friend_index]
            //     .add(move_list.get(*index).movement.destination.address(), sign);
        }

        // 取った駒
        let cap: Option<PieceEx>;
        {
            // 動かす駒
            let moveing_piece: Option<PieceEx> = if let Some(from) = from2 {
                // 打でなければ、元の升に駒はあるので、それを消す。
                let piece152: Option<PieceEx> = if promote2 {
                    if let Some(pc_ex) = self.position.pop_from_board(from) {
                        // 成ったのなら、元のマスの駒を成らすぜ☆（＾～＾）
                        Some(PieceEx::new(pc_ex.piece.promoted(), pc_ex.num))
                    } else {
                        std::panic::panic_any(Beam::trouble(
                            "(Err.248) 成ったのに、元の升に駒がなかった☆（＾～＾）",
                        ));
                    }
                } else {
                    // 移動元の駒。
                    self.position.pop_from_board(from)
                };

                piece152
            } else {
                // 打なら
                // 自分の持ち駒を減らす
                if let Some(drp) = drop2 {
                    Some(
                        self.position
                            .pop_hand(HandPiece::from_phase_and_type(us, drp)),
                    )
                } else {
                    std::panic::panic_any(Beam::trouble(
                        "(Err.236) 打なのに駒を指定してないぜ☆（＾～＾）",
                    ));
                }
            };
            // 移動先升に駒があるかどうか
            cap = if let Some(collision_piece) = self.position.pop_from_board(to2) {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                let captured_piece =
                    PieceEx::new(collision_piece.piece.captured(), collision_piece.num);
                self.position.push_hand(&captured_piece);
                Some(collision_piece)
            } else {
                None
            };

            // 移動先升に駒を置く
            self.position.push_to_board(to2, moveing_piece);
        }
        self.set_captured(self.history.moves_num() as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_current_position_hash();
        self.set_position_hash(ky_hash);

        self.history.increase_moves_num();
        cap
    }

    pub fn undo_move(&mut self) -> bool {
        if 0 < self.history.moves_num() {
            // まず　手目を戻す
            self.history.decrease_moves_num();
            let us = self.history.get_phase();
            let move_ = self.get_move();
            // let (from, to, pro) = destructure_move(move_);
            let (from2, to2, promote2, drop2) = to_move_object(us, move_);
            // 取った駒が有ったか。
            let captured: Option<PieceEx> =
                self.history.captured_pieces[self.history.moves_num() as usize];
            // 動いた駒
            let moveing_piece: Option<PieceEx> = if let Some(_source_val) = from2 {
                // 打でなければ
                if promote2 {
                    // 成ったなら、成る前へ
                    if let Some(source_piece) = self.position.pop_from_board(to2) {
                        Some(PieceEx::new(source_piece.piece.demoted(), source_piece.num))
                    } else {
                        std::panic::panic_any(Beam::trouble(
                            "(Err.305) 成ったのに移動先に駒が無いぜ☆（＾～＾）！",
                        ))
                    }
                } else {
                    self.position.pop_from_board(to2)
                }
            } else {
                if let Some(_drp) = drop2 {
                    // 打った場所に駒があるはずだぜ☆（＾～＾）
                    if let Some(pc_ex) = self.position.pop_from_board(to2) {
                        // 自分の持ち駒を増やそうぜ☆（＾～＾）！
                        self.position.push_hand(&pc_ex);
                        Some(pc_ex)
                    } else {
                        panic!("dst={:?}", to2.number())
                    }
                } else {
                    std::panic::panic_any(Beam::trouble(
                        "(Err.311) 打なのに駒を指定していないぜ☆（＾～＾）！",
                    ))
                }
            };

            if let Some(captured_piece_val) = captured {
                // 自分の持ち駒を減らす
                self.position
                    .pop_hand(captured_piece_val.piece.captured().hand_piece());
                // 移動先の駒を、取った駒（あるいは空）に戻す
                self.position.push_to_board(to2, captured);
            }

            if let Some(from2) = from2 {
                // 打でなければ、移動元升に、動かした駒を置く☆（＾～＾）打なら何もしないぜ☆（＾～＾）
                self.position.push_to_board(from2, moveing_piece);
            }
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}
