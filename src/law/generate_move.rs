//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::cosmic::recording::{Movement, Phase};
use crate::cosmic::smart::features::HandAddress;
use crate::cosmic::smart::features::PieceMeaning;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::{
    AbsoluteAddress, Angle, RelAdr, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4,
    RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::cosmic::toy_box::PieceNum;
use crate::cosmic::toy_box::{Board, Location};
use crate::spaceship::equipment::Beam;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    /// Stockfish系コンピューター将棋ソフトが言う Piece は、きふわらべでは PieceMeaning に名前を変えているぜ☆（＾～＾）
    pub meaning: PieceMeaning,
    /// 将棋の駒の背番号だぜ☆（＾～＾）
    pub num: PieceNum,
}
impl Piece {
    pub fn new(meaning: PieceMeaning, num: PieceNum) -> Self {
        Piece {
            meaning: meaning,
            num: num,
        }
    }
}
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Piece({} {:?})", self.meaning, self.num)
    }
}

/// ソートを高速にするためのものだぜ☆（＾～＾）
pub struct Ways {
    /// スワップしても割と速いだろ☆（＾～＾）
    pub indexes: Vec<usize>,
    /// こいつをスワップすると遅くなるぜ☆（＾～＾）
    body: Vec<Way>,
}
impl Ways {
    /// この初期化が遅いかどうかだな☆（＾～＾）
    pub fn new() -> Self {
        Ways {
            indexes: Vec::<usize>::new(),
            body: Vec::<Way>::new(),
        }
    }
    pub fn push(&mut self, way: &Way) {
        self.indexes.push(self.indexes.len());
        self.body.push(*way);
    }
    /// usize型のコピーなら、オブジェクトのコピーより少しは速いだろ☆（＾～＾）
    pub fn swap(&mut self, a: usize, b: usize) {
        let temp = self.indexes[a];
        self.indexes[a] = self.indexes[b];
        self.indexes[b] = temp;
    }
    pub fn get(&self, index: usize) -> Way {
        self.body[self.indexes[index]]
    }
    pub fn len(&self) -> usize {
        self.body.len()
    }
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

/// 局面の差分だぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub struct Way {
    /// 指し手☆（＾～＾）
    pub movement: Movement,
    /// 取った駒☆（＾～＾）
    pub captured: Option<Piece>,
}
impl Default for Way {
    /// ゴミ値☆（＾～＾）
    fn default() -> Self {
        Way {
            movement: Movement::default(),
            captured: None,
        }
    }
}
impl Way {
    pub fn new(mov: Movement, cap: Option<Piece>) -> Self {
        Way {
            movement: mov,
            captured: cap,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Mobility {
    pub angle: Angle,
    pub agility: Agility,
}
impl Mobility {
    pub fn new(angle: Angle, agility: Agility) -> Self {
        Mobility {
            angle: angle,
            agility: agility,
        }
    }
}

/// Pseudo legal move(疑似合法手)☆（＾～＾）
///
/// 先手の連続王手の千日手とか、空き王手とか、駒を見ただけでは調べられないだろ☆（＾～＾）
/// 棋譜や盤面を見ず、駒だけで調べる合法手が Pseudo legal move だぜ☆（＾～＾）
///
/// 二歩とか、打った後で調べた方が高速になるはずだが、探索部がまだできてないので、指し手生成の中でチェックしているぜ☆（＾～＾）
/// 香を２段目に打たないとか強い将棋を目指すことは　まだやってないぜ☆（＾～＾）
pub struct PseudoLegalMoves {}
impl PseudoLegalMoves {
    ///
    /// 現局面の、任意の移動先升の、
    /// - 盤上の駒の移動
    /// - 打
    /// の指し手を生成。
    ///
    /// 王手回避漏れや、千日手などのチェックは行っていない
    ///
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    pub fn make_move<F1>(friend: Phase, board: &Board, listen_move: &mut F1)
    where
        // TODO F1: FnMut(Option<Way>, &AbsoluteAddress),
        F1: FnMut(Way),
    {
        board.for_some_pieces_on_list40(friend, &mut |location, piece| match location {
            Location::Board(source) => {
                PseudoLegalMoves::start_on_board(friend, &source, &piece, board, listen_move)
            }
            Location::Hand(adr) => {
                PseudoLegalMoves::make_drop(friend, adr, board, listen_move);
            }
            Location::Busy => panic!(Beam::trouble(
                "(Err.94) なんで駒が作業中なんだぜ☆（＾～＾）！"
            )),
        });
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `piece` - 駒だぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    fn start_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        piece: &Piece,
        board: &Board,
        listen_move: &mut F1,
    ) where
        // TODO F1: FnMut(Option<Way>, &AbsoluteAddress),
        F1: FnMut(Way),
    {
        let moving =
            &mut |destination, promotability, _agility, move_permission: Option<MovePermission>| {
                let pseudo_captured = board.piece_at(&destination);

                let (ok, space) = if let Some(pseudo_captured_val) = pseudo_captured {
                    if pseudo_captured_val.meaning.phase() == friend {
                        // 味方の駒を取った☆（＾～＾）なしだぜ☆（＾～＾）！
                        (false, false)
                    } else {
                        (true, false)
                    }
                } else {
                    (true, true)
                };

                if ok {
                    // 成れるかどうかの判定☆（＾ｑ＾）
                    use crate::law::generate_move::Promotability::*;
                    let promotion = match &promotability {
                        Forced => true,
                        _ => false,
                    };

                    // 成りじゃない場合は、行き先のない動きを制限されるぜ☆（＾～＾）
                    let forbidden = if let Some(move_permission_val) = move_permission {
                        if move_permission_val.check(&destination) {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    };

                    match &promotability {
                        Any => {
                            // 成ったり、成れなかったりできるとき。
                            if !forbidden {
                                /* TODO
                                listen_move(
                                    Some(Way::new(
                                        Movement::new(Some(*source), destination, false, None),
                                        pseudo_captured,
                                    )),
                                    &destination,
                                );
                                */
                                listen_move(Way::new(
                                    Movement::new(Some(*source), destination, false, None),
                                    pseudo_captured,
                                ));
                            }
                            /* TODO
                            listen_move(
                                Some(Way::new(
                                    Movement::new(Some(*source), destination, true, None),
                                    pseudo_captured,
                                )),
                                &destination,
                            );
                            */
                            listen_move(Way::new(
                                Movement::new(Some(*source), destination, true, None),
                                pseudo_captured,
                            ));
                        }
                        _ => {
                            // 成れるか、成れないかのどちらかのとき。
                            if promotion || !forbidden {
                                /* TODO
                                listen_move(
                                    Some(Way::new(
                                        Movement::new(Some(*source), destination, promotion, None),
                                        pseudo_captured,
                                    )),
                                    &destination,
                                );
                                */
                                listen_move(Way::new(
                                    Movement::new(Some(*source), destination, promotion, None),
                                    pseudo_captured,
                                ));
                            }
                        }
                    };
                    // } else {
                    // TODO listen_move(None, &destination);
                }

                !space
            };

        Area::piece_of(piece.meaning.r#type(), friend, &source, moving);
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    /// * `listen_control` - 利きを受け取れだぜ☆（＾～＾）
    fn make_drop<F1>(friend: Phase, adr: HandAddress, board: &Board, listen_move: &mut F1)
    where
        // TODO F1: FnMut(Option<Way>, &AbsoluteAddress),
        F1: FnMut(Way),
    {
        if let Some(piece) = board.last_hand(adr) {
            // 打つぜ☆（＾～＾）
            let drop = &mut |destination| {
                if let None = board.piece_at(&destination) {
                    // 駒が無いところに打つ
                    use crate::cosmic::smart::features::PieceMeaning::*;
                    match piece.meaning {
                        Pawn1 | Pawn2 => {
                            // ひよこ　は２歩できない☆（＾～＾）
                            if board.exists_pawn_on_file(friend, destination.file()) {
                                return;
                            }
                        }
                        _ => {}
                    }
                    /* TOTO
                    listen_move(
                        Some(Way::new(
                            Movement::new(
                                None,                                        // 駒台
                                destination,                                 // どの升へ行きたいか
                                false,                                       // 打に成りは無し
                                Some(piece.meaning.hand_address().r#type()), // 打った駒種類
                            ),
                            None,
                        )),
                        &destination,
                    );
                    */
                    listen_move(Way::new(
                        Movement::new(
                            None,                                        // 駒台
                            destination,                                 // どの升へ行きたいか
                            false,                                       // 打に成りは無し
                            Some(piece.meaning.hand_address().r#type()), // 打った駒種類
                        ),
                        None,
                    ));
                }
            };

            // 駒を持っていれば
            let ty = adr.r#type();
            use crate::cosmic::smart::features::HandAddressType::*;
            match ty {
                // 歩、香
                Pawn | Lance => Area::drop_pawn_lance(friend, drop),
                // 桂
                Knight => Area::drop_knight(friend, drop),
                // それ以外の駒が打てる範囲は盤面全体。
                _ => Area::for_all(drop),
            }
        }
    }
}

/// 次の升☆（＾～＾）
pub struct Area {}
impl Area {
    /// 全升の面積だぜ☆（＾～＾）駒を打つときに使うぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                callback(AbsoluteAddress::new(file, rank));
            }
        }
    }

    /// 先手から見た盤上の駒の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `piece_type` - 駒の種類だぜ☆（＾～＾）
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `hopping` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    /// * `sliding` -
    fn piece_of<F1>(piece_type: PieceType, friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => Area::pawn(friend, source, moving),
            PieceType::Lance => Area::lance(friend, source, moving),
            PieceType::Knight => Area::knight(friend, source, moving),
            PieceType::Silver => Area::silver(friend, source, moving),
            PieceType::Gold => Area::gold(friend, source, moving),
            PieceType::King => Area::king(source, moving),
            PieceType::Bishop => Area::bishop(friend, source, moving),
            PieceType::Rook => Area::rook(friend, source, moving),
            PieceType::PromotedPawn => Area::gold(friend, source, moving),
            PieceType::PromotedLance => Area::gold(friend, source, moving),
            PieceType::PromotedKnight => Area::gold(friend, source, moving),
            PieceType::PromotedSilver => Area::gold(friend, source, moving),
            PieceType::Horse => Area::horse(source, moving),
            PieceType::Dragon => Area::dragon(source, moving),
        }
    }

    /// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn pawn<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::pawn_lance(
                friend,
                &destination,
                moving,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        for mobility in PieceType::Pawn.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn lance<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::pawn_lance(
                friend,
                &destination,
                moving,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        for mobility in PieceType::Lance.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn knight<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::knight(
                friend,
                &destination,
                moving,
                Some(MovePermission::from_knight(friend)),
            )
        };

        for mobility in PieceType::Knight.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving =
            &mut |destination, _agility| Promoting::silver(friend, &source, &destination, moving);

        for mobility in PieceType::Silver.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn gold<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::Gold.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 盤上の玉の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn king<F1>(source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::King.mobility().iter() {
            Area::r#move(&None, source, *mobility, moving);
        }
    }

    /// 盤上の角の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn bishop<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::bishop_rook(friend, &source, &destination, moving)
        };
        for mobility in PieceType::Bishop.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(friend: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::bishop_rook(friend, &source, &destination, moving)
        };
        for mobility in PieceType::Rook.mobility().iter() {
            Area::r#move(&Some(friend), source, *mobility, moving);
        }
    }

    /// 盤上の馬の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn horse<F1>(source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving =
            &mut |destination, agility| moving(destination, Promotability::Deny, agility, None);

        for mobility in PieceType::Horse.mobility().iter() {
            Area::r#move(&None, source, *mobility, moving);
        }
    }

    /// 盤上の竜の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn dragon<F1>(source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let moving =
                &mut |destination, agility| moving(destination, Promotability::Deny, agility, None);

            for mobility in PieceType::Dragon.mobility().iter() {
                Area::r#move(&None, source, *mobility, moving);
            }
        }
    }

    /// 先手から見た歩、香車の打てる面積だぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `phase` - 後手視点にしたけりゃ phase.turn() しろだぜ☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn drop_pawn_lance<F1>(phase: Phase, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        // 180°回転とかするより、for文の方を変えた方が高速だろ……☆（＾～＾）
        let (min_rank, max_rank) = if phase == Phase::First {
            (RANK_2, RANK_10)
        } else {
            (RANK_1, RANK_9)
        };

        for rank in min_rank..max_rank {
            for file in (FILE_1..FILE_10).rev() {
                callback(AbsoluteAddress::new(file, rank));
            }
        }
    }

    /// 先手から見た桂馬の打てる面積だぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `phase` - 後手視点にしたけりゃ phase.turn() しろだぜ☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn drop_knight<F1>(phase: Phase, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        for rank in RANK_3..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let mut ab_adr = AbsoluteAddress::new(file, rank);
                if phase == Phase::Second {
                    ab_adr = ab_adr.rotate_180();
                }

                callback(ab_adr);
            }
        }
    }

    /// 盤上の駒を指すぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 先手か後手か、関係ないか☆（＾～＾）先後同型なら関係ないしな☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn r#move<F1>(
        friend: &Option<Phase>,
        start: &AbsoluteAddress,
        mobility: Mobility,
        moving: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Agility) -> bool,
    {
        let angle = if let Some(friend_val) = friend {
            // 先後同型でない駒は、後手なら１８０°回転だぜ☆（＾～＾）
            if *friend_val == Phase::Second {
                mobility.angle.rotate180()
            } else {
                mobility.angle
            }
        } else {
            // 先後同型だからそのままだぜ☆（＾～＾）
            mobility.angle
        };

        match mobility.agility {
            Agility::Sliding => {
                let mut cur = start.clone();
                let r = RelAdr::new(1, 0).rotate(mobility.angle).clone();

                loop {
                    // 西隣から反時計回りだぜ☆（＾～＾）
                    if cur.offset(&r).wall() {
                        break;
                    }

                    if moving(cur, mobility.agility) {
                        break;
                    }
                }
            }
            // 桂馬専用☆（＾～＾）行き先の無いところに置いてないはずだぜ☆（＾～＾）
            Agility::Knight => {
                let mut cur = start.clone();

                // 西隣から反時計回りだぜ☆（＾～＾）
                if !cur.offset(&angle.west_ccw_double_rank()).wall() {
                    moving(cur, mobility.agility);
                }
            }
            Agility::Hopping => {
                let mut cur = start.clone();

                // 西隣から反時計回りだぜ☆（＾～＾）
                if !cur.offset(&angle.west_ccw()).wall() {
                    moving(cur, mobility.agility);
                }
            }
        }
    }
}

/// 機敏性。
#[derive(Clone, Copy, Debug)]
pub enum Agility {
    /// 隣へ１つ進む駒。
    Hopping,
    /// 長い利き。
    Sliding,
    /// 桂馬。
    Knight,
}

enum Promotability {
    /// 成ることはできないぜ☆（＾～＾）
    Deny,
    /// 成る、成らない両方あるぜ☆（＾～＾）
    Any,
    /// 必ず成れだぜ☆（＾～＾）
    Forced,
}

/// 行き先があるかないかのチェックに使うぜ☆（＾～＾）
/// 成れるときは使わないぜ☆（＾～＾）
struct MovePermission {
    min_rank: usize,
    max_rank: usize,
}
impl MovePermission {
    fn from_pawn_or_lance(friend: Phase) -> Self {
        // ▲P,▲L　は１段目(▽P,▽L　は９段目)には進めない
        match friend {
            Phase::First => MovePermission {
                min_rank: 2,
                max_rank: 9,
            },
            Phase::Second => MovePermission {
                min_rank: 1,
                max_rank: 8,
            },
        }
    }
    fn from_knight(friend: Phase) -> Self {
        // ▲N　は１、２段目(▽N　は８、９段目)には進めない
        match friend {
            Phase::First => MovePermission {
                min_rank: 3,
                max_rank: 9,
            },
            Phase::Second => MovePermission {
                min_rank: 1,
                max_rank: 7,
            },
        }
    }
    fn check(&self, destination: &AbsoluteAddress) -> bool {
        if destination.rank() < self.min_rank || self.max_rank < destination.rank() {
            return false;
        }
        true
    }
}
impl fmt::Debug for MovePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rank{}~{})", self.min_rank, self.max_rank)
    }
}

/// 成れるか、成れないか☆（＾～＾）
struct Promoting {}
impl Promoting {
    /// 歩と香のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに一番奥の段に移動することはできません。
    fn pawn_lance<F1>(
        friend: Phase,
        destinaion: &AbsoluteAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(friend, &destinaion) {
            // 自陣から見て一番奥の段
            callback(
                *destinaion,
                Promotability::Forced,
                Agility::Hopping,
                move_permission,
            )
        } else if Promoting::is_second_third_farthest_rank_from_friend(friend, &destinaion) {
            // 自陣から見て二番、三番目の奥の段
            callback(
                *destinaion,
                Promotability::Any,
                Agility::Hopping,
                move_permission,
            )
        } else {
            callback(
                *destinaion,
                Promotability::Deny,
                Agility::Hopping,
                move_permission,
            )
        }
    }

    /// 桂のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに奥から２番目の段に移動することはできません。
    fn knight<F1>(
        friend: Phase,
        destination: &AbsoluteAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_first_second_farthest_rank_from_friend(friend, &destination) {
            callback(
                *destination,
                Promotability::Forced,
                Agility::Knight,
                move_permission,
            )
        } else if Promoting::is_third_farthest_rank_from_friend(friend, &destination) {
            callback(
                *destination,
                Promotability::Any,
                Agility::Knight,
                move_permission,
            )
        } else {
            callback(
                *destination,
                Promotability::Deny,
                Agility::Knight,
                move_permission,
            )
        }
    }

    /// 銀のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 自陣から見て奥から１～３段目に入るときに成れます。元位置が３段目のときは、動けば成るか選べます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn silver<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        destination: &AbsoluteAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(friend, &source) {
            callback(*destination, Promotability::Any, Agility::Hopping, None)
        } else if Promoting::is_opponent_region(friend, &destination) {
            callback(*destination, Promotability::Any, Agility::Hopping, None)
        } else {
            callback(*destination, Promotability::Deny, Agility::Hopping, None)
        }
    }

    /// 角と飛のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn bishop_rook<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        destination: &AbsoluteAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_opponent_region(friend, &source)
            || Promoting::is_opponent_region(friend, &destination)
        {
            callback(*destination, Promotability::Any, Agility::Sliding, None)
        } else {
            callback(*destination, Promotability::Deny, Agility::Sliding, None)
        }
    }

    /// 自陣から見て、一番遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_farthest_rank_from_friend(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.rank() < RANK_2)
            || (friend == Phase::Second && RANK_8 < destination.rank())
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_first_second_farthest_rank_from_friend(
        friend: Phase,
        destination: &AbsoluteAddress,
    ) -> bool {
        (friend == Phase::First && destination.rank() < RANK_3)
            || (friend == Phase::Second && RANK_7 < destination.rank())
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_second_third_farthest_rank_from_friend(
        friend: Phase,
        destination: &AbsoluteAddress,
    ) -> bool {
        (friend == Phase::First && RANK_1 < destination.rank() && destination.rank() < RANK_4)
            || (friend == Phase::Second
                && RANK_6 < destination.rank()
                && destination.rank() < RANK_9)
    }
    /// 自陣から見て、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_third_farthest_rank_from_friend(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.rank() == RANK_3)
            || (friend == Phase::Second && RANK_7 == destination.rank())
    }
    /// 敵陣
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_opponent_region(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.rank() < RANK_4)
            || (friend == Phase::Second && RANK_6 < destination.rank())
    }
}
