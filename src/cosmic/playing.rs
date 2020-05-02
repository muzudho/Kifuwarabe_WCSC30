use crate::cosmic::recording::{History, Movement, PHASE_FIRST, PHASE_LEN, PHASE_SECOND};
use crate::cosmic::smart::features::{HandAddress, HAND_ADDRESS_LEN, HAND_MAX, PIECE_MEANING_LEN};
use crate::cosmic::smart::square::{BOARD_MEMORY_AREA, SQUARE_NONE};
use crate::cosmic::toy_box::Board;
use crate::law::generate_move::Piece;
use crate::spaceship::equipment::{Beam, DestinationDisplay};
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
    pub piece: [[u64; PIECE_MEANING_LEN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒
    pub hands: [[u64; HAND_MAX]; HAND_ADDRESS_LEN],
    // 先後
    pub phase: [u64; PHASE_LEN],
}

pub struct Game {
    /// 棋譜
    pub history: History,
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
    /// 初期盤面
    pub starting_board: Board,
    /// 現対局ハッシュ種☆（＾～＾）
    pub hash_seed: GameHashSeed,
    /// 現盤面
    pub board: Board,
    /// 情報表示担当
    pub info: DestinationDisplay,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            history: History::default(),
            starting_position_hash: 0,
            starting_board: Board::default(),
            hash_seed: GameHashSeed {
                // 盤上の駒
                piece: [[0; PIECE_MEANING_LEN]; BOARD_MEMORY_AREA as usize],
                // 持ち駒
                hands: [[0; HAND_MAX]; HAND_ADDRESS_LEN],
                // 先後
                phase: [0; PHASE_LEN],
            },
            board: Board::default(),
            info: DestinationDisplay::default(),
        }
    }
}
impl Game {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_square in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_piece in 0..PIECE_MEANING_LEN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.hash_seed.piece[i_square as usize][i_piece] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_piece in 0..HAND_ADDRESS_LEN {
            for i_count in 0..HAND_MAX {
                self.hash_seed.hands[i_piece][i_count] =
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
    pub fn set_move(&mut self, r#move: &Movement) {
        self.history.movements[self.history.ply as usize] = r#move.clone()
    }
    pub fn get_move(&self) -> &Movement {
        &self.history.movements[self.history.ply as usize]
    }
    /// テスト用に棋譜表示☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.history.ply {
            let movement = &self.history.movements[ply as usize];
            s.push_str(&format!("[{}] {}", ply, movement));
        }
        s
    }

    pub fn set_position_hash(&mut self, hash: u64) {
        self.history.position_hashs[self.history.ply as usize] = hash;
    }
    pub fn set_captured(&mut self, ply1: usize, pc: Option<Piece>) {
        self.history.captured_pieces[ply1] = pc
    }

    pub fn get_board(&self, num: PosNums) -> &Board {
        match num {
            PosNums::Current => &self.board,
            PosNums::Start => &self.starting_board,
        }
    }
    pub fn mut_starting(&mut self) -> &mut Board {
        &mut self.starting_board
    }

    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear(&mut self) {
        self.starting_board.clear();
        self.board.clear();
        self.history.ply = 0;
    }

    /// テスト用に局面ハッシュ☆（＾～＾）
    pub fn get_positions_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.starting_position_hash));

        for ply in 0..self.history.ply {
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
        let mut hash = self.board.create_hash(&self);

        // 手番ハッシュ
        use crate::cosmic::recording::Phase::*;
        match self.history.get_friend() {
            First => hash ^= self.hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.hash_seed.phase[PHASE_SECOND],
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_position(&self) -> isize {
        if self.history.ply < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.ply - 1;
        let new_ply = self.history.ply;
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
    /// Captured piece.
    pub fn do_move(&mut self, movement: &Movement) -> Option<Piece> {
        // もう入っているかも知れないが、棋譜に入れる☆
        self.set_move(movement);
        let friend = self.history.get_friend();

        // TODO 利き
        {
            // game.board.controls[friend_index]
            //     .add(ways.get(*index).movement.destination.address(), sign);
        }

        // 取った駒
        let cap: Option<Piece>;
        {
            // 動かす駒
            let moveing_piece: Option<Piece> = if let Some(source_val) = movement.source {
                // 打でなければ、元の升に駒はあるので、それを消す。
                let piece152: Option<Piece> = if movement.promote {
                    if let Some(piece) = self.board.pop_from_board(&source_val) {
                        // 成ったのなら、元のマスの駒を成らすぜ☆（＾～＾）
                        Some(Piece::new(piece.meaning.promoted(), piece.num))
                    } else {
                        panic!(Beam::trouble(
                            "(Err.248) 成ったのに、元の升に駒がなかった☆（＾～＾）"
                        ));
                    }
                } else {
                    // 移動元の駒。
                    self.board.pop_from_board(&source_val)
                };

                piece152
            } else {
                // 打なら
                // 自分の持ち駒を減らす
                if let Some(drp) = movement.drop {
                    Some(
                        self.board
                            .pop_hand(HandAddress::from_phase_and_type(friend, drp)),
                    )
                } else {
                    panic!(Beam::trouble(
                        "(Err.236) 打なのに駒を指定してないぜ☆（＾～＾）"
                    ));
                }
            };
            // 移動先升に駒があるかどうか
            cap = if let Some(collision_piece) = self.board.pop_from_board(&movement.destination) {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                let captured_piece =
                    Piece::new(collision_piece.meaning.captured(), collision_piece.num);
                self.board.push_hand(&captured_piece);
                Some(collision_piece)
            } else {
                None
            };

            // 移動先升に駒を置く
            self.board
                .push_to_board(&movement.destination, moveing_piece);
        }
        self.set_captured(self.history.ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_current_position_hash();
        self.set_position_hash(ky_hash);

        self.history.ply += 1;
        cap
    }

    pub fn undo_move(&mut self) -> bool {
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            let movement = &self.get_move().clone();
            {
                // 取った駒が有ったか。
                let captured: Option<Piece> =
                    self.history.captured_pieces[self.history.ply as usize];
                // 動いた駒
                let moveing_piece: Option<Piece> = if let Some(_source_val) = movement.source {
                    // 打でなければ
                    if movement.promote {
                        // 成ったなら、成る前へ
                        if let Some(source_piece) = self.board.pop_from_board(&movement.destination)
                        {
                            Some(Piece::new(source_piece.meaning.demoted(), source_piece.num))
                        } else {
                            panic!(Beam::trouble(
                                "(Err.305) 成ったのに移動先に駒が無いぜ☆（＾～＾）！"
                            ))
                        }
                    } else {
                        self.board.pop_from_board(&movement.destination)
                    }
                } else {
                    if let Some(_drp) = movement.drop {
                        // 打った場所に駒があるはずだぜ☆（＾～＾）
                        let piece = self.board.pop_from_board(&movement.destination).unwrap();
                        // 自分の持ち駒を増やそうぜ☆（＾～＾）！
                        self.board.push_hand(&piece);
                        Some(piece)
                    } else {
                        panic!(Beam::trouble(
                            "(Err.311) 打なのに駒を指定していないぜ☆（＾～＾）！"
                        ))
                    }
                };

                if let Some(captured_piece_val) = captured {
                    // 自分の持ち駒を減らす
                    self.board
                        .pop_hand(captured_piece_val.meaning.captured().hand_address());
                    // 移動先の駒を、取った駒（あるいは空）に戻す
                    self.board.push_to_board(&movement.destination, captured);
                }

                if let Some(source_val) = movement.source {
                    // 打でなければ、移動元升に、動かした駒を置く☆（＾～＾）打なら何もしないぜ☆（＾～＾）
                    self.board.push_to_board(&source_val, moveing_piece);
                }
            }
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}
