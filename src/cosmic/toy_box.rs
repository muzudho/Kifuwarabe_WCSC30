//!
//! 駒 と 盤
//!
use crate::cosmic::playing::Game;
use crate::cosmic::recording::Phase;
use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::smart::features::ControlBoard;
use crate::cosmic::smart::features::HAND_ADDRESS_LEN;
use crate::cosmic::smart::features::HAND_ADDRESS_TYPE_LEN;
use crate::cosmic::smart::features::{HandAddress, PieceMeaning, PieceType, HAND_MAX};
use crate::cosmic::smart::square::{
    AbsoluteAddress, RelAdr, BOARD_MEMORY_AREA, FILE_0, FILE_1, FILE_10, RANK_0, RANK_1, RANK_10,
};
use crate::law::generate_move::{Agility, Area, Piece};
use crate::law::speed_of_light::{HandAddresses, Nine299792458};
use crate::spaceship::equipment::Beam;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

/// 背番号付きの駒の数。
pub const PIECE_NUM_LEN: usize = 40;

/// 駒に背番号を付けたものだぜ☆（＾～＾）
#[derive(Clone, Copy, FromPrimitive, Debug, PartialEq)]
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
    // いわゆる盤☆（＾～＾）
    pieces: [Option<Piece>; BOARD_MEMORY_AREA as usize],
    /// 駒の居場所☆（＾～＾）
    location: [Location; PIECE_NUM_LEN],
    hand_index: [usize; HAND_ADDRESS_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hands: [HandAddressTypeStack; HAND_ADDRESS_LEN],
    /* TODO
    /// 利きの数☆（＾～＾）
    controls: [ControlBoard; PHASE_LEN],
    */
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
            hand_index: [
                PieceNum::King1 as usize,
                PieceNum::Rook21 as usize,
                PieceNum::Bishop19 as usize,
                PieceNum::Gold3 as usize,
                PieceNum::Silver7 as usize,
                PieceNum::Knight11 as usize,
                PieceNum::Lance15 as usize,
                PieceNum::Pawn23 as usize,
            ],
            // 持ち駒
            hands: [
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
            ],
            // TODO controls: [ControlBoard::default(); PHASE_LEN],
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
        self.hand_index = [
            PieceNum::King1 as usize,
            PieceNum::Rook21 as usize,
            PieceNum::Bishop19 as usize,
            PieceNum::Gold3 as usize,
            PieceNum::Silver7 as usize,
            PieceNum::Knight11 as usize,
            PieceNum::Lance15 as usize,
            PieceNum::Pawn23 as usize,
        ];
        // 持ち駒☆（＾～＾）
        self.hands = [
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
        ];
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, board: &Board) {
        self.pieces = board.pieces.clone();
        self.location = board.location.clone();
        self.hand_index = board.hand_index.clone();
        self.hands = board.hands.clone();
        // TODO self.controls = board.controls.clone();
    }

    /* TODO
    pub fn add_control(&mut self, phase: Phase, adr: &AbsoluteAddress, offset: isize) {
        self.controls[phase as usize].add(adr.address(), offset);
    }

    pub fn get_control(&self, phase: Phase, adr: &AbsoluteAddress) -> isize {
        self.controls[phase as usize].get(adr.address())
    }
    */

    /* TODO
    /// TODO 初期局面の利きを数えようぜ☆（＾～＾）？
    pub fn init_controls(&mut self) {
        Area::for_all(&mut |source| {
            // そこに置いてある駒を調べようぜ☆（＾～＾）？
            if let Some(piece) = self.piece_at(&source) {
                // 駒の利きを調べようぜ☆（＾～＾）？
                for mobility in piece.meaning.r#type().mobility() {
                    match mobility.agility {
                        Agility::Hopping => {
                            let mut cur = source.clone();
                            let mut rel = RelAdr::new(1, 0);
                            rel.rotate(mobility.angle);
                            if piece.meaning.phase() == Phase::Second {
                                rel.rotate_180();
                            }
                            if !cur.offset(&rel).wall() {
                                self.add_control(piece.meaning.phase(), &cur, 1);
                            }
                        }
                        Agility::Sliding => {
                            let mut cur = source.clone();
                            let mut rel = RelAdr::new(1, 0);
                            rel.rotate(mobility.angle);
                            if piece.meaning.phase() == Phase::Second {
                                rel.rotate_180();
                            }
                            for _i in 0..8 {
                                if !cur.offset(&rel).wall() {
                                    // とりあえず盤の上なら隣に利きは通るぜ☆（＾～＾）
                                    self.add_control(piece.meaning.phase(), &cur, 1);

                                    // 利きを調べたいだけなんで、味方／敵問わず駒が有れば終了だぜ☆（＾～＾）
                                    if let Some(_collision_piece) = self.piece_at(&cur) {
                                        break;
                                    }
                                } else {
                                    // 壁に利きは通らないぜ☆（＾～＾）
                                    break;
                                }
                            }
                        }
                        Agility::Knight => {
                            let mut cur = source.clone();
                            let mut rel = RelAdr::new(1, 0);
                            rel.rotate(mobility.angle).double_rank();
                            if piece.meaning.phase() == Phase::Second {
                                rel.rotate_180();
                            }
                            if !cur.offset(&rel).wall() {
                                self.add_control(piece.meaning.phase(), &cur, 1);
                            }
                        }
                    }
                }
            }
        });
    }
    */

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(&self, phase: Phase, file: usize) -> bool {
        for rank in RANK_1..RANK_10 {
            let adr = AbsoluteAddress::new(file, rank);
            if let Some(piece) = self.piece_at(&adr) {
                if piece.meaning.phase() == phase && piece.meaning.r#type() == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn piece_at(&self, adr: &AbsoluteAddress) -> Option<Piece> {
        self.pieces[adr.address() as usize]
    }
    /// 駒の背番号で指定して場所を取得
    pub fn location_at(&self, adr: PieceNum) -> Location {
        self.location[adr as usize]
    }

    /// 升で指定して駒を置く
    pub fn push_to_board(&mut self, adr: &AbsoluteAddress, piece: Option<Piece>) {
        if let Some(piece_val) = piece {
            self.pieces[adr.address() as usize] = piece;
            self.location[piece_val.num as usize] = Location::Board(*adr);
        } else {
            self.pieces[adr.address() as usize] = None;
        }
    }
    /// 盤上から駒を無くし、その駒を返り値で返すぜ☆（＾～＾）
    pub fn pop_from_board(&mut self, adr: &AbsoluteAddress) -> Option<Piece> {
        // 取り出すピースは複製するぜ☆（＾～＾）
        let piece = self.pieces[adr.address() as usize].clone();
        if let Some(piece_val) = piece {
            self.pieces[adr.address() as usize] = None;
            self.location[piece_val.num as usize] = Location::Busy;
        }
        piece
    }
    /// 盤に駒か空升を置いていきます。
    pub fn push_piece_on_init(&mut self, file: usize, rank: usize, piece: Option<PieceMeaning>) {
        if !(FILE_0 < file && file < FILE_10 && RANK_0 < rank && rank < RANK_10) {
            panic!(Beam::trouble(&format!(
                "(Err.323) 盤上の初期化で盤の外を指定するのは止めろだぜ☆（＾～＾）！ ({}, {})",
                file, rank
            )))
        }

        if let Some(piece_meaning) = piece {
            let source = AbsoluteAddress::new(file, rank);
            let piece_num = match piece_meaning {
                // 玉だけ、先後を確定させようぜ☆（＾～＾）
                PieceMeaning::King1 => {
                    self.location[PieceNum::King1 as usize] = Location::Board(source);
                    PieceNum::King1
                }
                PieceMeaning::King2 => {
                    self.location[PieceNum::King2 as usize] = Location::Board(source);
                    PieceNum::King2
                }
                _ => {
                    let hand_type = piece_meaning.hand_address().r#type();
                    self.location[self.hand_index[hand_type as usize]] = Location::Board(source);
                    let pn = PieceNum::from_usize(self.hand_index[hand_type as usize]).unwrap();
                    self.hand_index[hand_type as usize] += 1;
                    pn
                }
            };
            self.push_to_board(
                &AbsoluteAddress::new(file, rank),
                Some(Piece::new(piece_meaning, piece_num)),
            );
        }
    }
    /// 駒台に置く
    pub fn push_hand_on_init(&mut self, piece_meaning: PieceMeaning, number: isize) {
        for _i in 0..number {
            let adr = piece_meaning.hand_address();
            let hand = piece_meaning.hand_address();
            let hand_type = hand.r#type();
            let cursor = self.hand_index[hand_type as usize];
            self.location[cursor] = Location::Hand(adr);
            self.hands[hand as usize].push(&Piece::new(
                piece_meaning,
                PieceNum::from_usize(cursor).unwrap(),
            ));
            self.hand_index[hand_type as usize] += 1;
        }
    }
    pub fn push_hand(&mut self, hand: &Piece) {
        let adr = hand.meaning.hand_address();
        self.hands[adr as usize].push(hand);
        self.location[hand.num as usize] = Location::Hand(adr);
    }
    pub fn pop_hand(&mut self, adr: HandAddress) -> Piece {
        let piece = self.hands[adr as usize].pop();
        self.location[piece.num as usize] = Location::Busy;
        piece
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, adr: HandAddress) -> Option<&Piece> {
        self.hands[adr as usize].last()
    }
    pub fn count_hand(&self, adr: HandAddress) -> usize {
        self.hands[adr as usize].len()
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, game: &Game) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let ab_adr = &AbsoluteAddress::new(file, rank);
                if let Some(piece) = self.piece_at(ab_adr) {
                    hash ^= game.hash_seed.piece[ab_adr.address() as usize][piece.meaning as usize];
                }
            }
        }

        // 持ち駒ハッシュ
        HandAddresses::for_all(&mut |adr| {
            let count = self.count_hand(adr);
            debug_assert!(
                count <= HAND_MAX,
                "持ち駒 {:?} の枚数 {} <= {}",
                adr,
                count,
                HAND_MAX
            );
            hash ^= game.hash_seed.hands[adr as usize][count as usize];
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_board<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress>, Option<Piece>),
    {
        for (i, location) in self.location.iter().enumerate() {
            match location {
                Location::Board(adr) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(adr).unwrap();
                    piece_get(i, Some(adr), Some(piece));
                }
                Location::Hand(_adr) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.624) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_some_pieces_on_list40<F>(&self, friend: Phase, piece_get: &mut F)
    where
        F: FnMut(Location, Piece),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            let location = self.location[*piece_num as usize];
            match location {
                Location::Board(adr) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(&adr).unwrap();
                    if piece.meaning.phase() == friend {
                        piece_get(location, piece);
                    }
                }
                Location::Hand(_adr) => {
                    // 持ち駒はここで調べるのは無駄な気がするよな☆（＾～＾）持ち駒に歩が１８個とか☆（＾～＾）
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.650) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        }

        const FIRST_SECOND: [[HandAddress; HAND_ADDRESS_TYPE_LEN - 1]; 2] = [
            [
                // King なし
                HandAddress::Rook1,
                HandAddress::Bishop1,
                HandAddress::Gold1,
                HandAddress::Silver1,
                HandAddress::Knight1,
                HandAddress::Lance1,
                HandAddress::Pawn1,
            ],
            [
                // King なし
                HandAddress::Rook2,
                HandAddress::Bishop2,
                HandAddress::Gold2,
                HandAddress::Silver2,
                HandAddress::Knight2,
                HandAddress::Lance2,
                HandAddress::Pawn2,
            ],
        ];
        for adr in &FIRST_SECOND[friend as usize] {
            if let Some(piece) = self.last_hand(*adr) {
                piece_get(Location::Hand(*adr), *piece);
            }
        }
    }
}

#[derive(Clone)]
pub struct HandAddressTypeStack {
    items: [Piece; HAND_MAX],
    count: usize,
}
impl Default for HandAddressTypeStack {
    fn default() -> Self {
        HandAddressTypeStack {
            // ゴミ値で埋めるぜ☆（＾～＾）
            items: [Piece::new(PieceMeaning::King1, PieceNum::King1); HAND_MAX],
            count: 0,
        }
    }
}
impl HandAddressTypeStack {
    fn push(&mut self, piece: &Piece) {
        self.items[self.count] = *piece;
        self.count += 1;
    }

    fn pop(&mut self) -> Piece {
        self.count -= 1;
        let piece = self.items[self.count];
        // ゴミ値は消さないぜ☆（＾～＾）
        piece
    }

    fn last(&self) -> Option<&Piece> {
        if 0 < self.count {
            Some(&self.items[self.count - 1])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.count
    }
}
impl fmt::Display for HandAddressTypeStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for i in 0..=self.count {
            buffer.push_str(&format!(
                "({}, {:?}) ",
                self.items[i].meaning, self.items[i].num
            ));
        }
        write!(f, "{}", buffer.trim_end())
    }
}
