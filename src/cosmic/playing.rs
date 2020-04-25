use crate::cosmic::recording::{History, Movement, Person, PHASE_FIRST, PHASE_LN, PHASE_SECOND};
use crate::cosmic::smart::features::HAND_PIECE_LN;
use crate::cosmic::smart::features::{PieceMeaning, HAND_MAX, PIECE_LN};
use crate::cosmic::smart::square::{BOARD_MEMORY_AREA, SQUARE_NONE};
use crate::cosmic::toy_box::Board;
use crate::cosmic::toy_box::PieceNum;
use crate::law::speed_of_light::SpeedOfLight;
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
    pub piece: [[u64; PIECE_LN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒
    pub hands: [[u64; HAND_MAX]; HAND_PIECE_LN],
    // 先後
    pub phase: [u64; PHASE_LN],
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
                piece: [[0; PIECE_LN]; BOARD_MEMORY_AREA as usize],
                // 持ち駒
                hands: [[0; HAND_MAX]; HAND_PIECE_LN],
                // 先後
                phase: [0; PHASE_LN],
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
            for i_km in 0..PIECE_LN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.hash_seed.piece[i_square as usize][i_km] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_piece in 0..HAND_PIECE_LN {
            for i_count in 0..HAND_MAX {
                self.hash_seed.hands[i_piece][i_count] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LN {
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
    /// 駒を取ったぜ☆（＾～＾）
    pub fn caputure_piece(&mut self, ply1: usize, pc: Option<(PieceMeaning, PieceNum)>) {
        self.history.captured_pieces[ply1] = pc
    }

    pub fn get_board(&self, num: &PosNums) -> &Board {
        match *num {
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
    pub fn create_starting_position_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = self.starting_board.create_hash(&self, speed_of_light);

        // 手番ハッシュ（後手固定）
        hash ^= self.hash_seed.phase[PHASE_SECOND];

        hash
    }

    /// 局面ハッシュを作り直す
    pub fn create_current_position_hash(&self, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash = self.board.create_hash(&self, speed_of_light);

        // 手番ハッシュ
        use crate::cosmic::recording::Phase::*;
        match self.history.get_phase(Person::Friend) {
            First => hash ^= self.hash_seed.phase[PHASE_FIRST],
            Second => hash ^= self.hash_seed.phase[PHASE_SECOND],
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_position(&self) -> i8 {
        if self.history.ply < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.ply - 1;
        let new_ply = self.history.ply;
        // g_writeln( &format!( "Ｃount_same_ky last_ply={} new_ply={}", last_ply ,new_ply ) );
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            // g_writeln( &format!( "i_ply={} t={}", i_ply, t ) );
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
    pub fn do_move(
        &mut self,
        movement: &Movement,
        speed_of_light: &SpeedOfLight,
    ) -> Option<(PieceMeaning, PieceNum)> {
        // もう入っているかも知れないが、棋譜に入れる☆
        self.set_move(movement);
        let friend = self.history.get_phase(Person::Friend);

        // 取った駒
        let cap: Option<(PieceMeaning, PieceNum)>;
        {
            // 動かす駒
            let moveing_piece: Option<(PieceMeaning, PieceNum)> = if movement.source.is_drop() {
                if let Some(drp) = movement.drop {
                    // 打ったのなら、駒台から取り出すぜ☆（＾～＾）
                    let piece_meaning = PieceMeaning::from_phase_and_piece_type(friend, drp);
                    self.board.pop_hand(piece_meaning)
                } else {
                    panic!(Beam::trouble(
                        "(Err.236) 打なのに駒を指定してないぜ☆（＾～＾）"
                    ));
                }
            } else {
                // 打でなければ、元の升に駒はあるので、それを消す。
                if movement.promote {
                    if let Some(piece) = self.board.pop_board(&movement.source) {
                        // 成ったのなら、元のマスの駒を成らすぜ☆（＾～＾）
                        Some((piece.0.promoted(speed_of_light), piece.1))
                    } else {
                        panic!(Beam::trouble(
                            "(Err.248) 成ったのに、元の升に駒がなかった☆（＾～＾）"
                        ));
                    }
                } else {
                    // 移動元の駒をクローン。
                    self.board.pop_board(&movement.source)
                }
            };

            // 移動先升に駒があるかどうか
            cap = if let Some(captured_piece) = self.board.pop_board(&movement.destination) {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                self.board
                    .push_hand(&(captured_piece.0.captured(speed_of_light), captured_piece.1));
                Some(captured_piece)
            } else {
                None
            };

            // 移動先升に駒を置く
            self.board.push_piece(&movement.destination, moveing_piece);
        }
        self.caputure_piece(self.history.ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_current_position_hash(speed_of_light);
        self.set_position_hash(ky_hash);

        self.history.ply += 1;
        cap
    }

    pub fn undo_move(&mut self, speed_of_light: &SpeedOfLight) -> bool {
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            let movement = &self.get_move().clone();
            {
                // 取った駒が有ったか。
                let captured_piece: Option<(PieceMeaning, PieceNum)> =
                    self.history.captured_pieces[self.history.ply as usize];
                // 動いた駒
                let moving_piece: Option<(PieceMeaning, PieceNum)> = if movement.source.is_drop() {
                    // 打なら
                    if let Some(_drp) = movement.drop {
                        // 打った場所に駒があるはずだぜ☆（＾～＾）
                        let piece = self.board.pop_board(&movement.destination).unwrap();
                        // 自分の持ち駒を増やそうぜ☆（＾～＾）！
                        self.board.push_hand(&piece);
                        Some(piece)
                    } else {
                        panic!(Beam::trouble(
                            "(Err.311) 打なのに駒を指定していないぜ☆（＾～＾）！"
                        ))
                    }
                } else {
                    // 打でなければ
                    if movement.promote {
                        // 成ったなら、成る前へ
                        if let Some(source_piece) = self.board.pop_board(&movement.destination) {
                            Some((source_piece.0.demoted(speed_of_light), source_piece.1))
                        } else {
                            panic!(Beam::trouble(
                                "(Err.322) 成ったのに移動先に駒が無いぜ☆（＾～＾）！"
                            ))
                        }
                    } else {
                        self.board.pop_board(&movement.destination)
                    }
                };

                if let Some(captured_piece_val) = captured_piece {
                    // 自分の持ち駒を減らす
                    self.board
                        .pop_hand(captured_piece_val.0.captured(speed_of_light));
                    // 移動先に取った駒を戻すぜ☆（＾～＾）
                    self.board.push_piece(&movement.destination, captured_piece);
                }

                // 移動元升に、動かした駒を置く
                self.board.push_piece(&movement.source, moving_piece);
            }
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}
