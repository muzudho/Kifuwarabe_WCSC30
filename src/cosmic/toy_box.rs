//!
//! 駒 と 盤
//!
use crate::cosmic::playing::Game;
use crate::cosmic::recording::{Person, Phase, Phases};
use crate::cosmic::smart::features::{HandPieces, PieceMeaning, PieceType, HAND_MAX};
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, BOARD_MEMORY_AREA, FILE_0, FILE_11, RANK_0, RANK_1, RANK_10, RANK_11,
};
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::Beam;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// 背番号付きの駒の数。
pub const PIECE_NUM_LEN: usize = 40;

/// 駒に背番号を付けたものだぜ☆（＾～＾）
#[derive(Clone, Copy, FromPrimitive)]
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

#[derive(Clone, Copy)]
pub enum HandAddress {
    _King1,
    Rook1,
    Bishop1,
    Gold1,
    Silver1,
    Knight1,
    Lance1,
    Pawn1,
    _King2,
    Rook2,
    Bishop2,
    Gold2,
    Silver2,
    Knight2,
    Lance2,
    Pawn2,
}
#[derive(Clone, Copy)]
pub enum Location {
    Board(AbsoluteAddress),
    Hand(HandAddress),
    // 作業中のときは、これだぜ☆（＾～＾）
    Busy,
}

/// 現局面、または初期局面☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct Board {
    pieces: [Option<(PieceMeaning, PieceNum)>; BOARD_MEMORY_AREA as usize],
    /// 駒の居場所☆（＾～＾）
    location: [Location; PIECE_NUM_LEN],
    rook_index: usize,
    bishop_index: usize,
    gold_index: usize,
    silver_index: usize,
    knight_index: usize,
    lance_index: usize,
    pawn_index: usize,
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hand_king1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_rook1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_bishop1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_gold1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_silver1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_knight1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_lance1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_pawn1: Vec<(PieceMeaning, PieceNum)>,
    pub hand_king2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_rook2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_bishop2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_gold2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_silver2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_knight2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_lance2: Vec<(PieceMeaning, PieceNum)>,
    pub hand_pawn2: Vec<(PieceMeaning, PieceNum)>,
    /// 指し手生成でその升に移動したら、先手なら＋１、後手なら－１しろだぜ☆（＾～＾）葉で得点化するぜ☆（＾～＾）
    pub control: [i16; BOARD_MEMORY_AREA as usize],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            // 盤上
            pieces: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            location: [Location::Busy; PIECE_NUM_LEN],
            rook_index: PieceNum::Rook21 as usize,
            bishop_index: PieceNum::Bishop19 as usize,
            gold_index: PieceNum::Gold3 as usize,
            silver_index: PieceNum::Silver7 as usize,
            knight_index: PieceNum::Knight11 as usize,
            lance_index: PieceNum::Lance15 as usize,
            pawn_index: PieceNum::Pawn23 as usize,
            // 持ち駒
            hand_king1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_rook1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_bishop1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_gold1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_silver1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_knight1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_lance1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_pawn1: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_king2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_rook2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_bishop2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_gold2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_silver2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_knight2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_lance2: Vec::<(PieceMeaning, PieceNum)>::new(),
            hand_pawn2: Vec::<(PieceMeaning, PieceNum)>::new(),
            control: [0; BOARD_MEMORY_AREA as usize],
        }
    }
}
impl Board {
    pub fn clear(&mut self) {
        self.pieces = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        self.location = [Location::Busy; PIECE_NUM_LEN];
        self.rook_index = PieceNum::Rook21 as usize;
        self.bishop_index = PieceNum::Bishop19 as usize;
        self.gold_index = PieceNum::Gold3 as usize;
        self.silver_index = PieceNum::Silver7 as usize;
        self.knight_index = PieceNum::Knight11 as usize;
        self.lance_index = PieceNum::Lance15 as usize;
        self.pawn_index = PieceNum::Pawn23 as usize;
        // 持ち駒☆（＾～＾）
        self.hand_king1.clear();
        self.hand_rook1.clear();
        self.hand_bishop1.clear();
        self.hand_gold1.clear();
        self.hand_silver1.clear();
        self.hand_knight1.clear();
        self.hand_lance1.clear();
        self.hand_pawn1.clear();
        self.hand_king2.clear();
        self.hand_rook2.clear();
        self.hand_bishop2.clear();
        self.hand_gold2.clear();
        self.hand_silver2.clear();
        self.hand_knight2.clear();
        self.hand_lance2.clear();
        self.hand_pawn2.clear();
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, board: &Board) {
        self.pieces = board.pieces.clone();
        self.location = board.location.clone();
        self.rook_index = board.rook_index.clone();
        self.bishop_index = board.bishop_index.clone();
        self.gold_index = board.gold_index.clone();
        self.silver_index = board.silver_index.clone();
        self.knight_index = board.knight_index.clone();
        self.lance_index = board.lance_index.clone();
        self.pawn_index = board.pawn_index.clone();
        self.hand_king1 = board.hand_king1.clone();
        self.hand_rook1 = board.hand_rook1.clone();
        self.hand_bishop1 = board.hand_bishop1.clone();
        self.hand_gold1 = board.hand_gold1.clone();
        self.hand_silver1 = board.hand_silver1.clone();
        self.hand_knight1 = board.hand_knight1.clone();
        self.hand_lance1 = board.hand_lance1.clone();
        self.hand_pawn1 = board.hand_pawn1.clone();
        self.hand_king2 = board.hand_king2.clone();
        self.hand_rook2 = board.hand_rook2.clone();
        self.hand_bishop2 = board.hand_bishop2.clone();
        self.hand_gold2 = board.hand_gold2.clone();
        self.hand_silver2 = board.hand_silver2.clone();
        self.hand_knight2 = board.hand_knight2.clone();
        self.hand_lance2 = board.hand_lance2.clone();
        self.hand_pawn2 = board.hand_pawn2.clone();
        self.control = board.control.clone();
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
                if piece.0.phase(speed_of_light) == phase
                    && piece.0.r#type(speed_of_light) == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn piece_at(&self, adr: &AbsoluteAddress) -> Option<(PieceMeaning, PieceNum)> {
        self.pieces[adr.address() as usize]
    }
    /// 升で指定して駒を置く
    pub fn push_to_board(
        &mut self,
        adr: &AbsoluteAddress,
        piece: Option<(PieceMeaning, PieceNum)>,
    ) {
        if let Some(piece_val) = piece {
            self.pieces[adr.address() as usize] = piece;
            self.location[piece_val.1 as usize] = Location::Board(*adr);
        } else {
            self.pieces[adr.address() as usize] = None;
        }
    }
    /*
    /// TODO push_piece 升で指定して駒を置く
    pub fn set_piece_at(&mut self, adr: &AbsoluteAddress, piece: Option<(PieceMeaning, PieceNum)>) {
        if let Some(_x) = piece {
            self.pieces[adr.address() as usize] = piece;
        } else {
            self.pieces[adr.address() as usize] = None;
        }
    }
    */
    /// 盤上から駒を無くし、その駒を返り値で返すぜ☆（＾～＾）
    pub fn pop_from_board(&mut self, adr: &AbsoluteAddress) -> Option<(PieceMeaning, PieceNum)> {
        // 取り出すピースは複製するぜ☆（＾～＾）
        let piece = self.pieces[adr.address() as usize].clone();
        if let Some(piece_val) = piece {
            self.pieces[adr.address() as usize] = None;
            self.location[piece_val.1 as usize] = Location::Busy;
        }
        piece
    }
    /// 盤に駒を置いていきます。
    pub fn push_piece_on_init(&mut self, file: i8, rank: i8, piece: Option<PieceMeaning>) {
        if let Some(piece_meaning) = piece {
            let source = Address::new(file, rank).abs();
            let piece_num = match piece_meaning {
                PieceMeaning::King1 => {
                    self.location[PieceNum::King1 as usize] = Location::Board(source);
                    PieceNum::King1
                }
                PieceMeaning::King2 => {
                    self.location[PieceNum::King2 as usize] = Location::Board(source);
                    PieceNum::King2
                }
                PieceMeaning::Rook1
                | PieceMeaning::Rook2
                | PieceMeaning::Dragon1
                | PieceMeaning::Dragon2 => {
                    self.location[self.rook_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.rook_index).unwrap();
                    self.rook_index += 1;
                    pn
                }
                PieceMeaning::Bishop1
                | PieceMeaning::Bishop2
                | PieceMeaning::Horse1
                | PieceMeaning::Horse2 => {
                    self.location[self.bishop_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.bishop_index).unwrap();
                    self.bishop_index += 1;
                    pn
                }
                PieceMeaning::Gold1 | PieceMeaning::Gold2 => {
                    self.location[self.gold_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.gold_index).unwrap();
                    self.gold_index += 1;
                    pn
                }
                PieceMeaning::Silver1
                | PieceMeaning::Silver2
                | PieceMeaning::PromotedSilver1
                | PieceMeaning::PromotedSilver2 => {
                    self.location[self.silver_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.silver_index).unwrap();
                    self.silver_index += 1;
                    pn
                }
                PieceMeaning::Knight1
                | PieceMeaning::Knight2
                | PieceMeaning::PromotedKnight1
                | PieceMeaning::PromotedKnight2 => {
                    self.location[self.knight_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.knight_index).unwrap();
                    self.knight_index += 1;
                    pn
                }
                PieceMeaning::Lance1
                | PieceMeaning::Lance2
                | PieceMeaning::PromotedLance1
                | PieceMeaning::PromotedLance2 => {
                    self.location[self.lance_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.lance_index).unwrap();
                    self.lance_index += 1;
                    pn
                }
                PieceMeaning::Pawn1
                | PieceMeaning::Pawn2
                | PieceMeaning::PromotedPawn1
                | PieceMeaning::PromotedPawn2 => {
                    self.location[self.pawn_index] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.pawn_index).unwrap();
                    self.pawn_index += 1;
                    pn
                }
            };
            self.push_to_board(
                &Address::new(file, rank).abs(),
                Some((piece_meaning, piece_num)),
            );
        }
    }
    /// 駒台に置く
    pub fn push_hand_on_init(&mut self, piece_meaning: PieceMeaning, number: i8) {
        for _i in 0..number {
            match piece_meaning {
                PieceMeaning::Rook1 | PieceMeaning::Dragon1 => {
                    self.location[self.rook_index] = Location::Hand(HandAddress::Rook1);
                    self.hand_rook1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.rook_index).unwrap(),
                    ));
                    self.rook_index += 1;
                }
                PieceMeaning::Rook2 | PieceMeaning::Dragon2 => {
                    self.location[self.rook_index] = Location::Hand(HandAddress::Rook2);
                    self.hand_rook2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.rook_index).unwrap(),
                    ));
                    self.rook_index += 1;
                }
                PieceMeaning::Bishop1 | PieceMeaning::Horse1 => {
                    self.location[self.bishop_index] = Location::Hand(HandAddress::Bishop1);
                    self.hand_bishop1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.bishop_index).unwrap(),
                    ));
                    self.bishop_index += 1;
                }
                PieceMeaning::Bishop2 | PieceMeaning::Horse2 => {
                    self.location[self.bishop_index] = Location::Hand(HandAddress::Bishop2);
                    self.hand_bishop2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.bishop_index).unwrap(),
                    ));
                    self.bishop_index += 1;
                }
                PieceMeaning::Gold1 => {
                    self.location[self.gold_index] = Location::Hand(HandAddress::Gold1);
                    self.hand_gold1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.gold_index).unwrap(),
                    ));
                    self.gold_index += 1;
                }
                PieceMeaning::Gold2 => {
                    self.location[self.gold_index] = Location::Hand(HandAddress::Gold2);
                    self.hand_gold2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.gold_index).unwrap(),
                    ));
                    self.gold_index += 1;
                }
                PieceMeaning::Silver1 | PieceMeaning::PromotedSilver1 => {
                    self.location[self.silver_index] = Location::Hand(HandAddress::Silver1);
                    self.hand_silver1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.silver_index).unwrap(),
                    ));
                    self.silver_index += 1;
                }
                PieceMeaning::Silver2 | PieceMeaning::PromotedSilver2 => {
                    self.location[self.silver_index] = Location::Hand(HandAddress::Silver2);
                    self.hand_silver2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.silver_index).unwrap(),
                    ));
                    self.silver_index += 1;
                }
                PieceMeaning::Knight1 | PieceMeaning::PromotedKnight1 => {
                    self.location[self.knight_index] = Location::Hand(HandAddress::Knight1);
                    self.hand_knight1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.knight_index).unwrap(),
                    ));
                    self.knight_index += 1;
                }
                PieceMeaning::Knight2 | PieceMeaning::PromotedKnight2 => {
                    self.location[self.knight_index] = Location::Hand(HandAddress::Knight2);
                    self.hand_knight2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.knight_index).unwrap(),
                    ));
                    self.knight_index += 1;
                }
                PieceMeaning::Lance1 | PieceMeaning::PromotedLance1 => {
                    self.location[self.lance_index] = Location::Hand(HandAddress::Lance1);
                    self.hand_lance1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.lance_index).unwrap(),
                    ));
                    self.lance_index += 1;
                }
                PieceMeaning::Lance2 | PieceMeaning::PromotedLance2 => {
                    self.location[self.lance_index] = Location::Hand(HandAddress::Lance2);
                    self.hand_lance2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.lance_index).unwrap(),
                    ));
                    self.lance_index += 1;
                }
                PieceMeaning::Pawn1 | PieceMeaning::PromotedPawn1 => {
                    self.location[self.pawn_index] = Location::Hand(HandAddress::Pawn1);
                    self.hand_pawn1.push((
                        piece_meaning,
                        PieceNum::from_usize(self.pawn_index).unwrap(),
                    ));
                    self.pawn_index += 1;
                }
                PieceMeaning::Pawn2 | PieceMeaning::PromotedPawn2 => {
                    self.location[self.pawn_index] = Location::Hand(HandAddress::Pawn2);
                    self.hand_pawn2.push((
                        piece_meaning,
                        PieceNum::from_usize(self.pawn_index).unwrap(),
                    ));
                    self.pawn_index += 1;
                }
                _ => panic!(Beam::trouble(&format!(
                    "(Err.447) 持てない駒が指定されたぜ☆（＾～＾）！ {}",
                    piece_meaning
                ))),
            }
        }
    }
    pub fn push_hand(&mut self, hand: &(PieceMeaning, PieceNum)) {
        match hand.0 {
            // 探索中に玉を取ってしまうので、玉も持てるようにするぜ☆（＾～＾）
            PieceMeaning::King1 => self.hand_king1.push(*hand),
            PieceMeaning::King2 => self.hand_king2.push(*hand),
            PieceMeaning::Rook1 | PieceMeaning::Dragon1 => self.hand_rook1.push(*hand),
            PieceMeaning::Rook2 | PieceMeaning::Dragon2 => self.hand_rook2.push(*hand),
            PieceMeaning::Bishop1 | PieceMeaning::Horse1 => self.hand_bishop1.push(*hand),
            PieceMeaning::Bishop2 | PieceMeaning::Horse2 => self.hand_bishop2.push(*hand),
            PieceMeaning::Gold1 => self.hand_gold1.push(*hand),
            PieceMeaning::Gold2 => self.hand_gold2.push(*hand),
            PieceMeaning::Silver1 | PieceMeaning::PromotedSilver1 => self.hand_silver1.push(*hand),
            PieceMeaning::Silver2 | PieceMeaning::PromotedSilver2 => self.hand_silver2.push(*hand),
            PieceMeaning::Knight1 | PieceMeaning::PromotedKnight1 => self.hand_knight1.push(*hand),
            PieceMeaning::Knight2 | PieceMeaning::PromotedKnight2 => self.hand_knight2.push(*hand),
            PieceMeaning::Lance1 | PieceMeaning::PromotedLance1 => self.hand_lance1.push(*hand),
            PieceMeaning::Lance2 | PieceMeaning::PromotedLance2 => self.hand_lance2.push(*hand),
            PieceMeaning::Pawn1 | PieceMeaning::PromotedPawn1 => self.hand_pawn1.push(*hand),
            PieceMeaning::Pawn2 | PieceMeaning::PromotedPawn2 => self.hand_pawn2.push(*hand),
        }
    }
    pub fn pop_hand(&mut self, hand: PieceMeaning) -> Option<(PieceMeaning, PieceNum)> {
        match hand {
            PieceMeaning::King1 => self.hand_king1.pop(),
            PieceMeaning::King2 => self.hand_king2.pop(),
            PieceMeaning::Rook1 | PieceMeaning::Dragon1 => self.hand_rook1.pop(),
            PieceMeaning::Rook2 | PieceMeaning::Dragon2 => self.hand_rook2.pop(),
            PieceMeaning::Bishop1 | PieceMeaning::Horse1 => self.hand_bishop1.pop(),
            PieceMeaning::Bishop2 | PieceMeaning::Horse2 => self.hand_bishop2.pop(),
            PieceMeaning::Gold1 => self.hand_gold1.pop(),
            PieceMeaning::Gold2 => self.hand_gold2.pop(),
            PieceMeaning::Silver1 | PieceMeaning::PromotedSilver1 => self.hand_silver1.pop(),
            PieceMeaning::Silver2 | PieceMeaning::PromotedSilver2 => self.hand_silver2.pop(),
            PieceMeaning::Knight1 | PieceMeaning::PromotedKnight1 => self.hand_knight1.pop(),
            PieceMeaning::Knight2 | PieceMeaning::PromotedKnight2 => self.hand_knight2.pop(),
            PieceMeaning::Lance1 | PieceMeaning::PromotedLance1 => self.hand_lance1.pop(),
            PieceMeaning::Lance2 | PieceMeaning::PromotedLance2 => self.hand_lance2.pop(),
            PieceMeaning::Pawn1 | PieceMeaning::PromotedPawn1 => self.hand_pawn1.pop(),
            PieceMeaning::Pawn2 | PieceMeaning::PromotedPawn2 => self.hand_pawn2.pop(),
        }
    }
    pub fn count_hand(&self, hand: PieceMeaning) -> usize {
        match hand {
            PieceMeaning::King1 => self.hand_king1.len(),
            PieceMeaning::King2 => self.hand_king2.len(),
            PieceMeaning::Rook1 | PieceMeaning::Dragon1 => self.hand_rook1.len(),
            PieceMeaning::Rook2 | PieceMeaning::Dragon2 => self.hand_rook2.len(),
            PieceMeaning::Bishop1 | PieceMeaning::Horse1 => self.hand_bishop1.len(),
            PieceMeaning::Bishop2 | PieceMeaning::Horse2 => self.hand_bishop2.len(),
            PieceMeaning::Gold1 => self.hand_gold1.len(),
            PieceMeaning::Gold2 => self.hand_gold2.len(),
            PieceMeaning::Silver1 | PieceMeaning::PromotedSilver1 => self.hand_silver1.len(),
            PieceMeaning::Silver2 | PieceMeaning::PromotedSilver2 => self.hand_silver2.len(),
            PieceMeaning::Knight1 | PieceMeaning::PromotedKnight1 => self.hand_knight1.len(),
            PieceMeaning::Knight2 | PieceMeaning::PromotedKnight2 => self.hand_knight2.len(),
            PieceMeaning::Lance1 | PieceMeaning::PromotedLance1 => self.hand_lance1.len(),
            PieceMeaning::Lance2 | PieceMeaning::PromotedLance2 => self.hand_lance2.len(),
            PieceMeaning::Pawn1 | PieceMeaning::PromotedPawn1 => self.hand_pawn1.len(),
            PieceMeaning::Pawn2 | PieceMeaning::PromotedPawn2 => self.hand_pawn2.len(),
        }
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
            if piece.0.phase(speed_of_light) == phase {
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
                        [piece.0.serial_number(speed_of_light)];
                }
            }
        }

        // 持ち駒ハッシュ
        Phases::for_all(&mut |any_phase| {
            HandPieces::for_all(&mut |any_piece_type| {
                let hand = any_piece_type.add_phase(any_phase);
                let count = self.count_hand(hand);
                debug_assert!(
                    count <= HAND_MAX,
                    "持ち駒 {} の枚数 {} <= {}",
                    &any_piece_type,
                    count,
                    HAND_MAX
                );
                hash ^= game.hash_seed.hands[hand.hand_index(speed_of_light)][count as usize];
            });
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }

    /// 良ければ総量はプラスだぜ☆（＾～＾）
    pub fn control_value(&self) -> i16 {
        self.control.iter().sum()
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_board<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, &AbsoluteAddress, Option<(PieceMeaning, PieceNum)>),
    {
        for (i, location) in self.location.iter().enumerate() {
            match location {
                Location::Board(adr) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(adr).unwrap();
                    piece_get(i, adr, Some(piece));
                }
                Location::Hand(_adr) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, &AbsoluteAddress::default(), None);
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.624) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        }
    }
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_some_pieces_on_list40<F>(
        &self,
        friend: Phase,
        speed_of_light: &SpeedOfLight,
        piece_get: &mut F,
    ) where
        F: FnMut((PieceMeaning, PieceNum)),
    {
        for location in self.location.iter() {
            match location {
                Location::Board(adr) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(adr).unwrap();
                    if piece.0.phase(speed_of_light) == friend {
                        piece_get(piece);
                    }
                }
                Location::Hand(_adr) => {
                    // 持ち駒なので無視☆（＾～＾）
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.650) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        }
    }

    pub fn location_of(&self, piece_num: PieceNum) -> Location {
        self.location[piece_num as usize]
    }
}
