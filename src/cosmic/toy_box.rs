//!
//! 駒 と 盤
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::Person;
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, BOARD_MEMORY_AREA, FILE_0, FILE_11, RANK_0, RANK_1, RANK_10, RANK_11,
};
use crate::law::speed_of_light::SpeedOfLight;
use std::fmt;

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
    pub control_board: [i16; BOARD_MEMORY_AREA as usize],
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
            control_board: [0; BOARD_MEMORY_AREA as usize],
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
        GPPieces::for_all(&mut |any_piece| {
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

    pub fn control_value(&self) -> i16 {
        self.control_board.iter().sum()
    }
}

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    // ▲玉
    King1,
    // ▲きりん
    Rook1,
    // ▲ぞう
    Bishop1,
    // ▲いぬ
    Gold1,
    // ▲ねこ
    Silver1,
    // ▲うさぎ
    Knight1,
    // ▲いのしし
    Lance1,
    // ▲ひよこ
    Pawn1,
    // ▲ぱわーあっぷきりん
    Dragon1,
    // ▲ぱわーあっぷぞう
    Horse1,
    // ▲ぱわーあっぷねこ
    PromotedSilver1,
    // ▲ぱわーあっぷうさぎ
    PromotedKnight1,
    // ▲ぱわーあっぷいのしし
    PromotedLance1,
    // ▲ぱわーあっぷひよこ
    PromotedPawn1,
    // ▽ライオン
    King2,
    // ▽キリン
    Rook2,
    // ▽ゾウ
    Bishop2,
    // ▽イヌ
    Gold2,
    // ▽ネコ
    Silver2,
    // ▽ウサギ
    Knight2,
    // ▽イノシシ
    Lance2,
    // ▽ヒヨコ
    Pawn2,
    // ▽パワーアップキリン
    Dragon2,
    // ▽パワーアップゾウ
    Horse2,
    // ▽パワーアップネコ
    PromotedSilver2,
    // ▽パワーアップウサギ
    PromotedKnight2,
    // ▽パワーアップイノシシ
    PromotedLance2,
    // ▽パワーアップヒヨコ
    PromotedPawn2,
}

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX: usize = 18;
pub const PIECE_LN: usize = 30;
pub static PIECE_WHITE_SPACE: &str = "    ";
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲、▽ が半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::cosmic::toy_box::Piece::*;
        match *self {
            King1 => write!(f, " ▲K "),
            Rook1 => write!(f, " ▲R "),
            Bishop1 => write!(f, " ▲B "),
            Gold1 => write!(f, " ▲G "),
            Silver1 => write!(f, " ▲S "),
            Knight1 => write!(f, " ▲N "),
            Lance1 => write!(f, " ▲L "),
            Pawn1 => write!(f, " ▲P "),
            Dragon1 => write!(f, " ▲PR"),
            Horse1 => write!(f, " ▲PB"),
            PromotedSilver1 => write!(f, " ▲PS"),
            PromotedKnight1 => write!(f, " ▲PN"),
            PromotedLance1 => write!(f, " ▲PL"),
            PromotedPawn1 => write!(f, " ▲PP"),
            King2 => write!(f, " ▽k "),
            Rook2 => write!(f, " ▽r "),
            Bishop2 => write!(f, " ▽b "),
            Gold2 => write!(f, " ▽g "),
            Silver2 => write!(f, " ▽s "),
            Knight2 => write!(f, " ▽n "),
            Lance2 => write!(f, " ▽l "),
            Pawn2 => write!(f, " ▽p "),
            Dragon2 => write!(f, " ▽pr"),
            Horse2 => write!(f, " ▽pb"),
            PromotedSilver2 => write!(f, " ▽ps"),
            PromotedKnight2 => write!(f, " ▽pn"),
            PromotedLance2 => write!(f, " ▽pl"),
            PromotedPawn2 => write!(f, " ▽pp"),
        }
    }
}
impl Piece {
    /// TODO これを宇宙に移動したいぜ☆（＾～＾）
    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_and_piece_type(phase: Phase, piece_type: PieceType) -> Self {
        use crate::cosmic::smart::features::PieceType::*;
        use crate::cosmic::toy_box::Piece::*;
        match phase {
            Phase::First => match piece_type {
                King => King1,
                Rook => Rook1,
                Bishop => Bishop1,
                Gold => Gold1,
                Silver => Silver1,
                Knight => Knight1,
                Lance => Lance1,
                Pawn => Pawn1,
                Dragon => Dragon1,
                Horse => Horse1,
                PromotedSilver => PromotedSilver1,
                PromotedKnight => PromotedKnight1,
                PromotedLance => PromotedLance1,
                PromotedPawn => PromotedPawn1,
            },
            Phase::Second => match piece_type {
                King => King2,
                Rook => Rook2,
                Bishop => Bishop2,
                Gold => Gold2,
                Silver => Silver2,
                Knight => Knight2,
                Lance => Lance2,
                Pawn => Pawn2,
                Dragon => Dragon2,
                Horse => Horse2,
                PromotedSilver => PromotedSilver2,
                PromotedKnight => PromotedKnight2,
                PromotedLance => PromotedLance2,
                PromotedPawn => PromotedPawn2,
            },
        }
    }
}

pub struct GPPieces {}
impl GPPieces {
    /// すべての駒☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(Piece),
    {
        const KM_ARRAY: [Piece; 28] = [
            Piece::King1,           // らいおん
            Piece::Rook1,           // きりん
            Piece::Bishop1,         // ぞう
            Piece::Gold1,           // いぬ
            Piece::Silver1,         // ねこ
            Piece::Knight1,         // うさぎ
            Piece::Lance1,          // いのしし
            Piece::Pawn1,           // ひよこ
            Piece::Dragon1,         // ぱわーあっぷきりん
            Piece::Horse1,          // ぱわーあっぷぞう
            Piece::PromotedSilver1, // ぱわーあっぷねこ
            Piece::PromotedKnight1, // ぱわーあっぷうさぎ
            Piece::PromotedLance1,  // ぱわーあっぷいのしし
            Piece::PromotedPawn1,   // ぱわーあっぷひよこ
            Piece::King2,           // らいおん
            Piece::Rook2,           // きりん
            Piece::Bishop2,         // ぞう
            Piece::Gold2,           // いぬ
            Piece::Silver2,         // ねこ
            Piece::Knight2,         // うさぎ
            Piece::Lance2,          // いのしし
            Piece::Pawn2,           // ひよこ
            Piece::Dragon2,         // ぱわーあっぷきりん
            Piece::Horse2,          // ぱわーあっぷぞう
            Piece::PromotedSilver2, // ぱわーあっぷねこ
            Piece::PromotedKnight2, // ぱわーあっぷうさぎ
            Piece::PromotedLance2,  // ぱわーあっぷいのしし
            Piece::PromotedPawn2,   // ぱわーあっぷひよこ
        ];
        for piece in KM_ARRAY.iter() {
            callback(*piece);
        }
    }
}
