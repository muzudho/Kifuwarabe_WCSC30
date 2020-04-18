//!
//! 駒 と 盤
//!

use crate::cosmic::game::game::Game;
use crate::cosmic::game::position::phase::*;
use crate::cosmic::smart::piece_type::*;
use crate::cosmic::smart::square::*;
use crate::law::speed_of_light::*;
use std::fmt;

pub enum ThingsInTheSquare {
    Space,
    Friend,
    Opponent,
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
    /// らいおんの位置
    /// [先後]
    square_of_king: [Square; PHASE_LN],
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
            square_of_king: [Square::from_address(0), Square::from_address(0)],
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

    /*
    /// らいおんの位置
    pub fn get_sq_r(&self, phase_number: usize) -> &Square {
        &self.square_of_king[phase_number]
    }
    */

    /// 歩が置いてあるか確認
    pub fn exists_fu_by_phase_suji(
        &self,
        phase: Phase,
        suji: i8,
        speed_of_light: &SpeedOfLight,
    ) -> bool {
        for dan in RANK_1..RANK_10 {
            let sq = Square::from_file_rank(suji, dan);
            if let Some(piece99) = self.get_piece_by_square(&sq) {
                let ps100 = speed_of_light.get_piece_struct(&piece99);
                let (phase_piece, piece_type) = &ps100.phase_piece_type;
                if *phase_piece == phase && *piece_type == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn get_piece_by_square(&self, sq: &Square) -> Option<Piece> {
        self.board[sq.address as usize]
    }
    /// 升で指定して駒を置く
    pub fn set_piece_by_square(&mut self, sq: &Square, piece_o: Option<Piece>) {
        if let Some(piece) = piece_o {
            self.board[sq.address as usize] = piece_o;

            // 玉の位置を覚え直します。
            use crate::cosmic::game::position::phase::Phase::*;
            match piece {
                Piece::King1 => self.square_of_king[First as usize] = sq.clone(),
                Piece::King2 => self.square_of_king[Second as usize] = sq.clone(),
                _ => {}
            }
        } else {
            self.board[sq.address as usize] = None;
        }
    }
    /**
     * 持ち駒の枚数を加算
     */
    pub fn add_hand(&mut self, hand: &Piece, maisu: i8, speed_of_light: &SpeedOfLight) {
        self.hand[speed_of_light.get_piece_struct(hand).serial_piece_number] += maisu;
    }
    pub fn get_hand(&self, hand: &Piece, speed_of_light: &SpeedOfLight) -> i8 {
        self.hand[speed_of_light.get_piece_struct(hand).serial_piece_number]
    }

    /// 升には何がありますか？
    pub fn what_is_in_the_square(
        &self,
        phase: Phase,
        sq: &Square,
        speed_of_light: &SpeedOfLight,
    ) -> ThingsInTheSquare {
        // TODO 範囲外チェックは？行わない？
        if let Some(piece) = self.get_piece_by_square(&sq) {
            let piece_struct = speed_of_light.get_piece_struct(&piece);
            if piece_struct.phase() == phase {
                return ThingsInTheSquare::Friend;
            }
            ThingsInTheSquare::Opponent
        } else {
            ThingsInTheSquare::Space
        }
    }

    /*
    /// 指定の升に駒があれば真
    pub fn exists_km(&self, sq: &Square) -> bool {
        if let Some(_piece) = self.get_piece_by_square(&sq) {
            true
        } else {
            false
        }
    }

    /// 指定の升に指定の駒があれば真
    pub fn has_sq_km(&self, sq: &Square, piece: &Piece, speed_of_light: &SpeedOfLight) -> bool {
        if let Some(piece2) = self.get_piece_by_square(&sq) {
            return speed_of_light
                .get_piece_struct(&piece)
                .equals_piece(&speed_of_light.get_piece_struct(&piece2));
        }
        false
    }
    */

    /*
    /// 指定の升にある駒の先後
    pub fn get_phase_by_sq(&self, sq: &Square, speed_of_light: &SpeedOfLight) -> Option<Phase> {
        if let Some(piece) = self.get_piece_by_square(sq) {
            return Some(speed_of_light.get_piece_struct(&piece).phase());
        }
        None
    }

    /// 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
    pub fn is_natta(
        &self,
        sq_src: &Square,
        sq_dst: &Square,
        speed_of_light: &SpeedOfLight,
    ) -> bool {
        if let Some(km_src) = self.get_piece_by_square(&sq_src) {
            let ps_src = speed_of_light.get_piece_struct(&km_src);
            let pro_src = ps_src.is_promoted();

            if let Some(km_dst) = self.get_piece_by_square(&sq_dst) {
                let ps_dst = speed_of_light.get_piece_struct(&km_dst);
                // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
                let pro_dst = ps_dst.is_promoted();
                // 成り
                pro_dst && !pro_src
            } else {
                // 空升には成れない☆（＾～＾）
                false
            }
        } else {
            // 空升は成れない☆（＾～＾）
            false
        }
    }
    */

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, game: &Game, speed_of_light: &SpeedOfLight) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for i_address in SQUARE_NONE..BOARD_MEMORY_AREA {
            let i_sq = Square::from_address(i_address as isquare);
            if let Some(km) = self.get_piece_by_square(&i_sq) {
                let num_km = speed_of_light.get_piece_struct(&km).serial_piece_number;
                hash ^= game.hash_seed.km[i_address as usize][num_km];
            }
        }

        // 持ち駒ハッシュ
        GPPieces::for_all(&mut |any_piece| {
            let num_km = speed_of_light
                .get_piece_struct(&any_piece)
                .serial_piece_number;

            let maisu = self.get_hand(&any_piece, &speed_of_light);
            debug_assert!(
                -1 < maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}",
                &any_piece,
                maisu,
                MG_MAX
            );

            hash ^= game.hash_seed.mg[num_km][maisu as usize];
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
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
            // NonePiece => write!(f, "    "),
            // OwariPiece => write!(f, " ×× "),
        }
    }
}
impl Piece {
    /// TODO これを宇宙に移動したいぜ☆（＾～＾）
    /// 先後＆駒種類→先後付き駒
    pub fn from_phase_and_piece_type(phase: Phase, piece_type: PieceType) -> Self {
        use crate::cosmic::smart::piece_type::PieceType::*;
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
/*
pub const KM_ARRAY_HALF_LN: usize = 14;
pub const PHASE_KM_ARRAY: [[Piece; KM_ARRAY_HALF_LN]; PHASE_LN] = [
    [
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
    ],
    [
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
    ],
    [
        Piece::OwariPiece, // らいおん
        Piece::OwariPiece, // きりん
        Piece::OwariPiece, // ぞう
        Piece::OwariPiece, // いぬ
        Piece::OwariPiece, // ねこ
        Piece::OwariPiece, // うさぎ
        Piece::OwariPiece, // いのしし
        Piece::OwariPiece, // ひよこ
        Piece::OwariPiece, // ぱわーあっぷきりん
        Piece::OwariPiece, // ぱわーあっぷぞう
        Piece::OwariPiece, // ぱわーあっぷねこ
        Piece::OwariPiece, // ぱわーあっぷうさぎ
        Piece::OwariPiece, // ぱわーあっぷいのしし
        Piece::OwariPiece, // ぱわーあっぷひよこ
    ],
];
*/
