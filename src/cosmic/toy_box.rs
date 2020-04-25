//!
//! 駒 と 盤
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Person, Phase};
use crate::cosmic::smart::features::{Piece, PieceType, Pieces, MG_MAX, PIECE_LN};
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, BOARD_MEMORY_AREA, FILE_0, FILE_11, RANK_0, RANK_1, RANK_10, RANK_11,
};
use crate::law::generate_move::Area;
use crate::law::speed_of_light::SpeedOfLight;

/// 背番号付きの駒の数。
pub const PIECE_NUM_LEN: usize = 40;

/// 駒に背番号を付けたものだぜ☆（＾～＾）
pub enum PieceNum {
    // 1 先手玉
    King1,
    // 2 後手玉
    King2,
    // 3 金
    Gold3,
    // 4 金
    Gold4,
    // 5 金
    Gold5,
    // 6 金
    Gold6,
    // 7 銀
    Silver7,
    // 8 銀
    Silver8,
    // 9 銀
    Silver9,
    // 10 銀
    Silver10,
    // 11 桂
    Knight11,
    // 12 桂
    Knight12,
    // 13 桂
    Knight13,
    // 14 桂
    Knight14,
    // 15 香
    Lance15,
    // 16 香
    Lance16,
    // 17 香
    Lance17,
    // 18 香
    Lance18,
    // 19 角
    Bishop19,
    // 20 角
    Bishop20,
    // 21 飛
    Rook21,
    // 22 飛
    Rook22,
    // 23 歩
    Pawn23,
    // 24 歩
    Pawn24,
    // 25 歩
    Pawn25,
    // 26 歩
    Pawn26,
    // 27 歩
    Pawn27,
    // 28 歩
    Pawn28,
    // 29 歩
    Pawn29,
    // 30 歩
    Pawn30,
    // 31 歩
    Pawn31,
    // 32 歩
    Pawn32,
    // 33 歩
    Pawn33,
    // 34 歩
    Pawn34,
    // 35 歩
    Pawn35,
    // 36 歩
    Pawn36,
    // 37 歩
    Pawn37,
    // 38 歩
    Pawn38,
    // 39 歩
    Pawn39,
    // 40 歩
    Pawn40,
}

/// 現局面、または初期局面☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
pub struct Board {
    /// 10の位を筋、1の位を段とする。
    /// 0筋、0段は未使用
    board: [Option<Piece>; BOARD_MEMORY_AREA as usize],
    /// 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
    /// 増減させたいので、u8 ではなく i8。
    pub hand: [i8; PIECE_LN],
    /// 指し手生成でその升に移動したら、先手なら＋１、後手なら－１しろだぜ☆（＾～＾）葉で得点化するぜ☆（＾～＾）
    pub control: [i16; BOARD_MEMORY_AREA as usize],
    /// 駒の絶対番地☆（＾～＾）
    pub piece_pos: [AbsoluteAddress; PIECE_NUM_LEN],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            // 盤上
            board: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            // 持ち駒数
            hand: [
                // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
                0, 0,
            ],
            control: [0; BOARD_MEMORY_AREA as usize],
            piece_pos: [AbsoluteAddress::default(); PIECE_NUM_LEN],
        }
    }
}
impl Board {
    pub fn clear(&mut self) {
        self.board = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        self.hand = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
            0, 0,
        ];
    }

    /// 盤面の駒に背番号を振っていくぜ☆（＾～＾）
    pub fn init_piece_pos(&mut self) {
        let mut rook_index = PieceNum::Rook21 as usize;
        let mut bishop_index = PieceNum::Bishop19 as usize;
        let mut gold_index = PieceNum::Gold3 as usize;
        let mut silver_index = PieceNum::Silver7 as usize;
        let mut knight_index = PieceNum::Knight11 as usize;
        let mut lance_index = PieceNum::Lance15 as usize;
        let mut pawn_index = PieceNum::Pawn23 as usize;
        Area::for_all(&mut |source| {
            if let Some(piece_val) = self.piece_at(&source) {
                match piece_val {
                    Piece::King1 => {
                        self.piece_pos[PieceNum::King1 as usize] = source;
                    }
                    Piece::King2 => {
                        self.piece_pos[PieceNum::King2 as usize] = source;
                    }
                    Piece::Rook1 | Piece::Rook2 | Piece::Dragon1 | Piece::Dragon2 => {
                        self.piece_pos[rook_index] = source;
                        rook_index += 1;
                    }
                    Piece::Bishop1 | Piece::Bishop2 | Piece::Horse1 | Piece::Horse2 => {
                        self.piece_pos[bishop_index] = source;
                        bishop_index += 1;
                    }
                    Piece::Gold1 | Piece::Gold2 => {
                        self.piece_pos[gold_index] = source;
                        gold_index += 1;
                    }
                    Piece::Silver1
                    | Piece::Silver2
                    | Piece::PromotedSilver1
                    | Piece::PromotedSilver2 => {
                        self.piece_pos[silver_index] = source;
                        silver_index += 1;
                    }
                    Piece::Knight1
                    | Piece::Knight2
                    | Piece::PromotedKnight1
                    | Piece::PromotedKnight2 => {
                        self.piece_pos[knight_index] = source;
                        knight_index += 1;
                    }
                    Piece::Lance1
                    | Piece::Lance2
                    | Piece::PromotedLance1
                    | Piece::PromotedLance2 => {
                        self.piece_pos[lance_index] = source;
                        lance_index += 1;
                    }
                    Piece::Pawn1 | Piece::Pawn2 | Piece::PromotedPawn1 | Piece::PromotedPawn2 => {
                        self.piece_pos[pawn_index] = source;
                        pawn_index += 1;
                    }
                }
            }
        });
    }

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(
        &self,
        phase: Phase,
        file: i8,
        speed_of_light: &SpeedOfLight,
    ) -> bool {
        for rank in RANK_1..RANK_10 {
            let adr = Address::new(file, rank).abs();
            if let Some(piece) = self.piece_at(&adr) {
                if piece.phase(speed_of_light) == phase
                    && piece.r#type(speed_of_light) == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn piece_at(&self, adr: &AbsoluteAddress) -> Option<Piece> {
        self.board[adr.address() as usize]
    }
    pub fn set_piece(&mut self, file: i8, rank: i8, piece_o: Option<Piece>) {
        self.set_piece_at(&Address::new(file, rank).abs(), piece_o);
    }
    /// 升で指定して駒を置く
    pub fn set_piece_at(&mut self, adr: &AbsoluteAddress, piece: Option<Piece>) {
        if let Some(_piece) = piece {
            self.board[adr.address() as usize] = piece;
        } else {
            self.board[adr.address() as usize] = None;
        }
    }
    /// 持ち駒の枚数を加算
    pub fn add_hand(&mut self, hand: &Piece, count: i8, speed_of_light: &SpeedOfLight) {
        self.hand[hand.serial_number(speed_of_light)] += count;
    }
    pub fn get_hand(&self, hand: Piece, speed_of_light: &SpeedOfLight) -> i8 {
        self.hand[hand.serial_number(speed_of_light)]
    }
    pub fn set_hand(&mut self, km: Piece, number: i8) {
        self.hand[km as usize] = number;
    }

    /// 升には何がありますか？
    pub fn what_is_in_the_square(
        &self,
        phase: Phase,
        adr: &AbsoluteAddress,
        speed_of_light: &SpeedOfLight,
    ) -> Option<Person> {
        // TODO 範囲外チェックは？行わない？
        if let Some(piece) = self.piece_at(&adr) {
            if piece.phase(speed_of_light) == phase {
                return Some(Person::Friend);
            }
            Some(Person::Opponent)
        } else {
            None
        }
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, game: &Game, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK_0..RANK_11 {
            for file in (FILE_0..FILE_11).rev() {
                let ab_adr = &Address::new(file, rank).abs();
                if let Some(piece) = self.piece_at(ab_adr) {
                    hash ^= game.hash_seed.piece[ab_adr.address() as usize]
                        [piece.serial_number(speed_of_light)];
                }
            }
        }

        // 持ち駒ハッシュ
        Pieces::for_all(&mut |any_piece| {
            let piece_num = any_piece.serial_number(speed_of_light);

            let maisu = self.get_hand(any_piece, &speed_of_light);
            debug_assert!(
                -1 < maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}",
                &any_piece,
                maisu,
                MG_MAX
            );

            hash ^= game.hash_seed.hand[piece_num][maisu as usize];
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }

    /// 良ければ総量はプラスだぜ☆（＾～＾）
    pub fn control_value(&self) -> i16 {
        self.control.iter().sum()
    }
}
