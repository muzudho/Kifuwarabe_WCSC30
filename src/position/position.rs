//!
//! 駒 と 盤
//!
use crate::entities::cosmic::playing::Game;
use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::features::HAND_ADDRESS_LEN;
use crate::entities::cosmic::smart::features::HAND_ADDRESS_TYPE_LEN;
use crate::entities::cosmic::smart::features::{HandPiece, PieceType, HAND_MAX};
use crate::entities::cosmic::smart::square::{
    BOARD_MEMORY_AREA, FILE_0, FILE_1, FILE_10, RANK_0, RANK_1, RANK_10,
};
use crate::entities::law::speed_of_light::{HandPieces, Nine299792458};
use crate::entities::spaceship::equipment::Beam;
use crate::movegen::PieceEx;
use crate::position::hand_type_to_square;
use crate::position::is_board_square;
use crate::position::is_hand_square;
use crate::position::square_from;
use crate::position::square_to_hand_piece;
use crate::position::Square;
use crate::position::SQUARE_NONE;
use crate::search::CentiPawn;
use crate::take1base::Piece;
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

/// 現局面、または初期局面☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct Position {
    // いわゆる盤☆（＾～＾）
    board: [Option<PieceEx>; BOARD_MEMORY_AREA as usize],
    /// 背番号 to 駒の居場所☆（＾～＾）
    pc_num_to_location: [Square; PIECE_NUM_LEN],
    hand_index: [usize; HAND_ADDRESS_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hands: [HandTypeStack; HAND_ADDRESS_LEN],
    /* TODO
    /// 利きの数☆（＾～＾）
    controls: [ControlBoard; PHASE_LEN],
    */
}
impl Default for Position {
    fn default() -> Self {
        Position {
            // 盤上
            board: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
            pc_num_to_location: [SQUARE_NONE; PIECE_NUM_LEN],
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
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
                HandTypeStack::default(),
            ],
            // TODO controls: [ControlBoard::default(); PHASE_LEN],
        }
    }
}
impl Position {
    pub fn clear(&mut self) {
        self.board = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        self.pc_num_to_location = [SQUARE_NONE; PIECE_NUM_LEN];
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
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
            HandTypeStack::default(),
        ];
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, position: &Position) {
        self.board = position.board.clone();
        self.pc_num_to_location = position.pc_num_to_location.clone();
        self.hand_index = position.hand_index.clone();
        self.hands = position.hands.clone();
        // TODO self.controls = position.controls.clone();
    }

    /* TODO
    pub fn add_control(&mut self, phase: Phase, adr: Square, offset: isize) {
        self.controls[phase as usize].add(adr.address(), offset);
    }

    pub fn get_control(&self, phase: Phase, adr: Square) -> isize {
        self.controls[phase as usize].get(adr.address())
    }
    */

    /* TODO
    /// TODO 初期局面の利きを数えようぜ☆（＾～＾）？
    pub fn init_controls(&mut self) {
        Area::for_all(&mut |source| {
            // そこに置いてある駒を調べようぜ☆（＾～＾）？
            if let Some(pc_ex) = self.piece_at_board(&source) {
                // 駒の利きを調べようぜ☆（＾～＾）？
                for mobility in pc_ex.piece.type_().mobility() {
                    match mobility.move_range {
                        MoveRange::Adjacent => {
                            let mut cur = source.clone();
                            let mut rel = RelAdr::new(1, 0);
                            rel.rotate(mobility.angle);
                            if pc_ex.piece.phase() == Phase::Second {
                                rel.rotate_180();
                            }
                            if !cur.offset(&rel).wall() {
                                self.add_control(pc_ex.piece.phase(), &cur, 1);
                            }
                        }
                        MoveRange::Sliding => {
                            let mut cur = source.clone();
                            let mut rel = RelAdr::new(1, 0);
                            rel.rotate(mobility.angle);
                            if pc_ex.piece.phase() == Phase::Second {
                                rel.rotate_180();
                            }
                            for _i in 0..8 {
                                if !cur.offset(&rel).wall() {
                                    // とりあえず盤の上なら隣に利きは通るぜ☆（＾～＾）
                                    self.add_control(pc_ex.piece.phase(), &cur, 1);

                                    // 利きを調べたいだけなんで、味方／敵問わず駒が有れば終了だぜ☆（＾～＾）
                                    if let Some(_collision_piece) = self.piece_at_board(&cur) {
                                        break;
                                    }
                                } else {
                                    // 壁に利きは通らないぜ☆（＾～＾）
                                    break;
                                }
                            }
                        }
                        MoveRange::Knight => {
                            let mut cur = source.clone();
                            let mut rel = RelAdr::new(1, 0);
                            rel.rotate(mobility.angle).double_rank();
                            if pc_ex.piece.phase() == Phase::Second {
                                rel.rotate_180();
                            }
                            if !cur.offset(&rel).wall() {
                                self.add_control(pc_ex.piece.phase(), &cur, 1);
                            }
                        }
                    }
                }
            }
        });
    }
    */

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(&self, phase: Phase, file: u8) -> bool {
        for rank in RANK_1..RANK_10 {
            let sq = square_from(file, rank);
            if let Some(pc_ex) = self.piece_at_board(sq) {
                if pc_ex.piece.phase() == phase && pc_ex.piece.type_() == PieceType::P {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn piece_at_board(&self, sq: Square) -> Option<PieceEx> {
        self.board[sq as usize]
    }
    /// 駒の背番号で指定して場所を取得
    pub fn location_at(&self, adr: PieceNum) -> Square {
        self.pc_num_to_location[adr as usize]
    }

    /// 升で指定して駒を置く
    pub fn push_to_board(&mut self, sq: Square, pc_ex: Option<PieceEx>) {
        if let Some(piece_val) = pc_ex {
            self.board[sq as usize] = pc_ex;
            self.pc_num_to_location[piece_val.num as usize] = sq;
        } else {
            self.board[sq as usize] = None;
        }
    }
    /// 盤上から駒を無くし、その駒を返り値で返すぜ☆（＾～＾）
    pub fn pop_from_board(&mut self, sq: Square) -> Option<PieceEx> {
        // 取り出すピースは複製するぜ☆（＾～＾）
        let pc_ex = self.board[sq as usize].clone();
        if let Some(piece_val) = pc_ex {
            self.board[sq as usize] = None;
            self.pc_num_to_location[piece_val.num as usize] = SQUARE_NONE;
        }
        pc_ex
    }
    /// 盤に駒か空升を置いていきます。
    pub fn push_piece_on_init(&mut self, file: u8, rank: u8, pc_ex: Option<Piece>) {
        if !(FILE_0 < file && file < FILE_10 && RANK_0 < rank && rank < RANK_10) {
            std::panic::panic_any(Beam::trouble(&format!(
                "(Err.323) 盤上の初期化で盤の外を指定するのは止めろだぜ☆（＾～＾）！ ({}, {})",
                file, rank
            )))
        }

        if let Some(piece) = pc_ex {
            let from = square_from(file, rank);
            let pc_num = match piece {
                // 玉だけ、先後を確定させようぜ☆（＾～＾）
                Piece::K1 => {
                    self.pc_num_to_location[PieceNum::King1 as usize] = from;
                    PieceNum::King1
                }
                Piece::K2 => {
                    self.pc_num_to_location[PieceNum::King2 as usize] = from;
                    PieceNum::King2
                }
                _ => {
                    let hand_piece = piece.hand_piece().type_();
                    self.pc_num_to_location[self.hand_index[hand_piece as usize]] = from;
                    if let Some(pn) = PieceNum::from_usize(self.hand_index[hand_piece as usize]) {
                        self.hand_index[hand_piece as usize] += 1;
                        pn
                    } else {
                        panic!("hand_index={}", self.hand_index[hand_piece as usize])
                    }
                }
            };
            self.push_to_board(square_from(file, rank), Some(PieceEx::new(piece, pc_num)));
        }
    }
    /// 駒台に置く
    pub fn push_hand_on_init(&mut self, piece: Piece, number: u8) {
        for _i in 0..number {
            let ha = piece.hand_piece();
            let hand_piece = ha.type_();
            let pc_num = self.hand_index[hand_piece as usize];
            self.pc_num_to_location[pc_num] = hand_type_to_square(ha);
            if let Some(pc_num) = PieceNum::from_usize(pc_num) {
                self.hands[ha as usize].push(&PieceEx::new(piece, pc_num));
            } else {
                panic!("pc_num={}", pc_num)
            }
            self.hand_index[hand_piece as usize] += 1;

            // Beam::shoot(&format!("# hand[{}]{} pc_num={}", i, piece, pc_num));
        }
    }
    pub fn push_hand(&mut self, hand: &PieceEx) {
        let adr = hand.piece.hand_piece();
        self.hands[adr as usize].push(hand);
        self.pc_num_to_location[hand.num as usize] = hand_type_to_square(adr);
    }
    pub fn pop_hand(&mut self, ha: HandPiece) -> PieceEx {
        let pc_ex = self.hands[ha as usize].pop();
        self.pc_num_to_location[pc_ex.num as usize] = SQUARE_NONE;
        pc_ex
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, adr: HandPiece) -> Option<&PieceEx> {
        self.hands[adr as usize].last()
    }
    /// 持駒の枚数
    pub fn count_hand(&self, adr: HandPiece) -> usize {
        self.hands[adr as usize].len()
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, game: &Game) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let sq = square_from(file, rank);
                if let Some(pc_ex) = self.piece_at_board(sq) {
                    hash ^= game.hash_seed.piece_hash[sq as usize][pc_ex.piece as usize];
                }
            }
        }

        // 持ち駒ハッシュ
        HandPieces::for_all(&mut |hand_pc| {
            let count = self.count_hand(hand_pc);
            debug_assert!(
                count <= HAND_MAX,
                "持ち駒 {:?} の枚数 {} <= {}",
                hand_pc,
                count,
                HAND_MAX
            );
            hash ^= game.hash_seed.hand_hash[hand_pc as usize][count as usize];
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_board<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<Square>, Option<PieceEx>),
    {
        for (i, sq) in self.pc_num_to_location.iter().enumerate() {
            if is_board_square(*sq) {
                // 盤上の駒☆（＾～＾）
                if let Some(pc_ex) = self.piece_at_board(*sq) {
                    piece_get(i, Some(*sq), Some(pc_ex));
                } else {
                    panic!("sq={:?}", sq)
                }
            } else if is_hand_square(*sq) {
                // TODO 持ち駒☆（＾～＾）
                piece_get(i, None, None);
            } else {
                std::panic::panic_any(Beam::trouble(
                    "(Err.624) なんで駒が作業中なんだぜ☆（＾～＾）！",
                ))
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_some_pieces_on_list40<F>(&self, us: Phase, piece_get: &mut F)
    where
        F: FnMut(Square, PieceEx),
    {
        // 駒の背番号
        for pc_num in Nine299792458::piece_numbers().iter() {
            let sq = self.pc_num_to_location[*pc_num as usize];
            if is_board_square(sq) {
                // 盤上の駒☆（＾～＾）
                if let Some(pc_ex) = self.piece_at_board(sq) {
                    if pc_ex.piece.phase() == us {
                        piece_get(sq, pc_ex);
                    }
                } else {
                    panic!("sq={:?}", sq)
                }
            } else if is_hand_square(sq) {
                // 持ち駒はここで調べるのは無駄な気がするよな☆（＾～＾）持ち駒に歩が１８個とか☆（＾～＾）
            } else {
                std::panic::panic_any(Beam::trouble(&format!(
                    "(Err.650) 駒{:?} が盤にも駒台にも無いぜ☆（＾～＾）！",
                    pc_num
                )))
            }
        }

        const FIRST_SECOND: [[HandPiece; HAND_ADDRESS_TYPE_LEN - 1]; 2] = [
            [
                // King なし
                HandPiece::Rook1,
                HandPiece::Bishop1,
                HandPiece::Gold1,
                HandPiece::Silver1,
                HandPiece::Knight1,
                HandPiece::Lance1,
                HandPiece::Pawn1,
            ],
            [
                // King なし
                HandPiece::Rook2,
                HandPiece::Bishop2,
                HandPiece::Gold2,
                HandPiece::Silver2,
                HandPiece::Knight2,
                HandPiece::Lance2,
                HandPiece::Pawn2,
            ],
        ];
        for ha in &FIRST_SECOND[us as usize] {
            if let Some(pc_ex) = self.last_hand(*ha) {
                piece_get(hand_type_to_square(*ha), *pc_ex);
            }
        }
    }

    /// 手番側の駒割評価値
    pub fn material_advantage(&self, us: Phase) -> CentiPawn {
        let mut value = 0;
        for pc_num in 0..PIECE_NUM_LEN {
            let sq = self.pc_num_to_location[pc_num];
            if is_board_square(sq) {
                if let Some(pc_ex) = self.piece_at_board(sq) {
                    if us == pc_ex.piece.phase() {
                        value += pc_ex.piece.hand_type().captured_value();
                    } else {
                        value -= pc_ex.piece.hand_type().captured_value();
                    }
                }
            } else if is_hand_square(sq) {
                let hand_piece = square_to_hand_piece(sq);
                if us == hand_piece.phase() {
                    value += hand_piece.type_().captured_value();
                } else {
                    value -= hand_piece.type_().captured_value();
                }
            }
        }
        value
    }
}

#[derive(Clone)]
pub struct HandTypeStack {
    items: [PieceEx; HAND_MAX],
    count: usize,
}
impl Default for HandTypeStack {
    fn default() -> Self {
        HandTypeStack {
            // ゴミ値で埋めるぜ☆（＾～＾）
            items: [PieceEx::new(Piece::K1, PieceNum::King1); HAND_MAX],
            count: 0,
        }
    }
}
impl HandTypeStack {
    fn push(&mut self, pc_ex: &PieceEx) {
        self.items[self.count] = *pc_ex;
        self.count += 1;
    }

    fn pop(&mut self) -> PieceEx {
        self.count -= 1;
        let pc_ex = self.items[self.count];
        // ゴミ値は消さないぜ☆（＾～＾）
        pc_ex
    }

    fn last(&self) -> Option<&PieceEx> {
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
impl fmt::Display for HandTypeStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for i in 0..=self.count {
            buffer.push_str(&format!(
                "({}, {:?}) ",
                self.items[i].piece, self.items[i].num
            ));
        }
        write!(f, "{}", buffer.trim_end())
    }
}
