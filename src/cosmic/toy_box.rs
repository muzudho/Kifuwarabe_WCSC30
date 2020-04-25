//!
//! 駒 と 盤
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Person, Phase};
use crate::cosmic::smart::features::{Piece, PieceType, Pieces, MG_MAX, PIECE_LN};
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, BOARD_MEMORY_AREA, FILE_0, FILE_11, RANK_0, RANK_1, RANK_10, RANK_11,
};
use crate::law::speed_of_light::SpeedOfLight;

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
    /// 玉の座標☆（＾～＾）
    pub king_pos: [AbsoluteAddress; 2],
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
            king_pos: [AbsoluteAddress::default(); 2],
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
