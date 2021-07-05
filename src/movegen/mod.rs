//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::features::HandAddress;
use crate::entities::cosmic::smart::features::PieceType;
use crate::entities::cosmic::smart::square::{
    AbsoluteAddress, Angle, RelAdr, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4,
    RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::entities::move_::new_move;
use crate::entities::spaceship::equipment::Beam;
use crate::position::is_board_square;
use crate::position::position::{PieceNum, Position};
use crate::position::is_hand_square;
use crate::position::square_to_hand_address;
use crate::take1base::Move;
use crate::take1base::Piece;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct PieceEx {
    /// 深い意味は無く Stockfish の Piece（＾～＾）
    pub meaning: Piece,
    /// 将棋の駒の背番号だぜ☆（＾～＾）
    pub num: PieceNum,
}
impl PieceEx {
    pub fn new(meaning: Piece, num: PieceNum) -> Self {
        PieceEx {
            meaning: meaning,
            num: num,
        }
    }
}
impl fmt::Debug for PieceEx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PieceEx({} {:?})", self.meaning, self.num)
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
    /// * `us` - どちらの手番か☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// # Returns
    ///
    /// 指し手の一覧
    pub fn generate(us: Phase, position: &Position) -> Vec<Move> {
        // TODO 自玉の位置検索
        let ksq = match us {
            Phase::First => position.location_at(PieceNum::King1),
            Phase::Second => position.location_at(PieceNum::King2),
        };
        if is_board_square(ksq) {
            // TODO 合い駒(Pinned)検索

            // TODO 右方向
            // TODO 右上方向
            // TODO 上方向
            // TODO 左上方向
            // TODO 左方向
            // TODO 左下方向
            // TODO 下方向
            // TODO 右下方向

            // TODO チェッカー(Checker)検索
        } else {
            panic!("(Err.93) ksq fail")
        }

        // TODO チェッカーがいたら、王手回避(Evasions)モードへ

        // TODO チェッカーがいなかったら、非回避(Non-evasions)モードへ
        PseudoLegalMoves::generate_non_evasion(us, position)
    }

    fn generate_non_evasion(us: Phase, position: &Position) -> Vec<Move> {
        let mut move_list = Vec::<Move>::new();
        let listen_move = &mut |move_| {
            move_list.push(move_);
        };

        // 座標ではなく、駒の背番号で検索
        position.for_some_pieces_on_list40(us, &mut |sq, piece| {
            if is_board_square(sq) {
                PseudoLegalMoves::start_on_board(
                    us,
                    &AbsoluteAddress::from_absolute_address(sq as usize).unwrap(),
                    &piece,
                    position,
                    listen_move,
                )
            } else if is_hand_square(sq) {
                PseudoLegalMoves::make_drop(us, square_to_hand_address(sq), position, listen_move);
            } else {
                std::panic::panic_any(Beam::trouble(
                    "(Err.94) なんで駒が作業中なんだぜ☆（＾～＾）！",
                ))
            }
        });

        move_list
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `piece` - 駒だぜ☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    fn start_on_board<F1>(
        us: Phase,
        source: &AbsoluteAddress,
        piece: &PieceEx,
        position: &Position,
        listen_move: &mut F1,
    ) where
        // TODO F1: FnMut(Option<MoveCap>, &AbsoluteAddress),
        F1: FnMut(Move),
    {
        let moving = &mut |destination: AbsoluteAddress,
                           promotability,
                           _agility,
                           move_permission: Option<MovePermission>| {
            let pseudo_captured = position.piece_at(destination.square_number());

            let (ok, space) = if let Some(pseudo_captured_val) = pseudo_captured {
                if pseudo_captured_val.meaning.phase() == us {
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
                use crate::movegen::Promotability::*;
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
                            listen_move(new_move(
                                us,
                                Some(source.square_number() as u16),
                                destination.square_number() as u16,
                                false,
                                None,
                            ));
                        }
                        listen_move(new_move(
                            us,
                            Some(source.square_number() as u16),
                            destination.square_number() as u16,
                            true,
                            None,
                        ));
                    }
                    _ => {
                        // 成れるか、成れないかのどちらかのとき。
                        if promotion || !forbidden {
                            listen_move(new_move(
                                us,
                                Some(source.square_number() as u16),
                                destination.square_number() as u16,
                                promotion,
                                None,
                            ));
                        }
                    }
                };
                // } else {
                // TODO listen_move(None, &destination);
            }

            !space
        };

        Area::piece_of(piece.meaning.r#type(), us, &source, moving);
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    /// * `listen_control` - 利きを受け取れだぜ☆（＾～＾）
    fn make_drop<F1>(us: Phase, adr: HandAddress, position: &Position, listen_move: &mut F1)
    where
        // TODO F1: FnMut(Option<MoveCap>, &AbsoluteAddress),
        F1: FnMut(Move),
    {
        if let Some(piece) = position.last_hand(adr) {
            // 打つぜ☆（＾～＾）
            let drop = &mut |destination: AbsoluteAddress| {
                if let None = position.piece_at(destination.square_number()) {
                    // 駒が無いところに打つ
                    use crate::take1base::Piece::*;
                    match piece.meaning {
                        P1 | P2 => {
                            // ひよこ　は２歩できない☆（＾～＾）
                            if position.exists_pawn_on_file(us, destination.file()) {
                                return;
                            }
                        }
                        _ => {}
                    }
                    listen_move(new_move(
                        us,
                        None,                                        // 駒台
                        destination.square_number() as u16,          // どの升へ行きたいか
                        false,                                       // 打に成りは無し
                        Some(piece.meaning.hand_address().r#type()), // 打った駒種類
                    ));
                }
            };

            // 駒を持っていれば
            let ty = adr.r#type();
            use crate::entities::cosmic::smart::features::HandAddressType::*;
            match ty {
                // 歩、香
                Pawn | Lance => Area::drop_pawn_lance(us, drop),
                // 桂
                Knight => Area::drop_knight(us, drop),
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
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `hopping` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    /// * `sliding` -
    fn piece_of<F1>(piece_type: PieceType, us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => Area::pawn(us, source, moving),
            PieceType::Lance => Area::lance(us, source, moving),
            PieceType::Knight => Area::knight(us, source, moving),
            PieceType::Silver => Area::silver(us, source, moving),
            PieceType::Gold => Area::gold(us, source, moving),
            PieceType::King => Area::king(source, moving),
            PieceType::Bishop => Area::bishop(us, source, moving),
            PieceType::Rook => Area::rook(us, source, moving),
            PieceType::PromotedPawn => Area::gold(us, source, moving),
            PieceType::PromotedLance => Area::gold(us, source, moving),
            PieceType::PromotedKnight => Area::gold(us, source, moving),
            PieceType::PromotedSilver => Area::gold(us, source, moving),
            PieceType::Horse => Area::horse(source, moving),
            PieceType::Dragon => Area::dragon(source, moving),
        }
    }

    /// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn pawn<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::pawn_lance(
                us,
                &destination,
                moving,
                Some(MovePermission::from_pawn_or_lance(us)),
            )
        };

        for mobility in PieceType::Pawn.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn lance<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::pawn_lance(
                us,
                &destination,
                moving,
                Some(MovePermission::from_pawn_or_lance(us)),
            )
        };

        for mobility in PieceType::Lance.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn knight<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::knight(
                us,
                &destination,
                moving,
                Some(MovePermission::from_knight(us)),
            )
        };

        for mobility in PieceType::Knight.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving =
            &mut |destination, _agility| Promoting::silver(us, &source, &destination, moving);

        for mobility in PieceType::Silver.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn gold<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::Gold.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
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
    fn bishop<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving =
            &mut |destination, _agility| Promoting::bishop_rook(us, &source, &destination, moving);
        for mobility in PieceType::Bishop.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
        }
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(us: Phase, source: &AbsoluteAddress, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving =
            &mut |destination, _agility| Promoting::bishop_rook(us, &source, &destination, moving);
        for mobility in PieceType::Rook.mobility().iter() {
            Area::r#move(&Some(us), source, *mobility, moving);
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
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn drop_pawn_lance<F1>(us: Phase, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        // 180°回転とかするより、for文の方を変えた方が高速だろ……☆（＾～＾）
        let (min_rank, max_rank) = if us == Phase::First {
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
    /// * `us` - 手番☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn drop_knight<F1>(us: Phase, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        for rank in RANK_3..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let mut ab_adr = AbsoluteAddress::new(file, rank);
                if us == Phase::Second {
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
    /// * `us` - 先手か後手か、関係ないか☆（＾～＾）先後同型なら関係ないしな☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn r#move<F1>(us: &Option<Phase>, start: &AbsoluteAddress, mobility: Mobility, moving: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Agility) -> bool,
    {
        let angle = if let Some(friend_val) = us {
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
    fn from_pawn_or_lance(us: Phase) -> Self {
        // ▲P,▲L　は１段目(▽P,▽L　は９段目)には進めない
        match us {
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
    fn from_knight(us: Phase) -> Self {
        // ▲N　は１、２段目(▽N　は８、９段目)には進めない
        match us {
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
    /// * `us` -
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに一番奥の段に移動することはできません。
    fn pawn_lance<F1>(
        us: Phase,
        destinaion: &AbsoluteAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(us, &destinaion) {
            // 自陣から見て一番奥の段
            callback(
                *destinaion,
                Promotability::Forced,
                Agility::Hopping,
                move_permission,
            )
        } else if Promoting::is_second_third_farthest_rank_from_friend(us, &destinaion) {
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
    /// * `us` -
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに奥から２番目の段に移動することはできません。
    fn knight<F1>(
        us: Phase,
        destination: &AbsoluteAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_first_second_farthest_rank_from_friend(us, &destination) {
            callback(
                *destination,
                Promotability::Forced,
                Agility::Knight,
                move_permission,
            )
        } else if Promoting::is_third_farthest_rank_from_friend(us, &destination) {
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
    /// * `us` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn silver<F1>(
        us: Phase,
        source: &AbsoluteAddress,
        destination: &AbsoluteAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(us, &source) {
            callback(*destination, Promotability::Any, Agility::Hopping, None)
        } else if Promoting::is_opponent_region(us, &destination) {
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
    /// * `us` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn bishop_rook<F1>(
        us: Phase,
        source: &AbsoluteAddress,
        destination: &AbsoluteAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_opponent_region(us, &source)
            || Promoting::is_opponent_region(us, &destination)
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
    /// * `us` -
    /// * `destination` -
    fn is_farthest_rank_from_friend(us: Phase, destination: &AbsoluteAddress) -> bool {
        (us == Phase::First && destination.rank() < RANK_2)
            || (us == Phase::Second && RANK_8 < destination.rank())
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `destination` -
    fn is_first_second_farthest_rank_from_friend(us: Phase, destination: &AbsoluteAddress) -> bool {
        (us == Phase::First && destination.rank() < RANK_3)
            || (us == Phase::Second && RANK_7 < destination.rank())
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `destination` -
    fn is_second_third_farthest_rank_from_friend(us: Phase, destination: &AbsoluteAddress) -> bool {
        (us == Phase::First && RANK_1 < destination.rank() && destination.rank() < RANK_4)
            || (us == Phase::Second && RANK_6 < destination.rank() && destination.rank() < RANK_9)
    }
    /// 自陣から見て、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `destination` -
    fn is_third_farthest_rank_from_friend(us: Phase, destination: &AbsoluteAddress) -> bool {
        (us == Phase::First && destination.rank() == RANK_3)
            || (us == Phase::Second && RANK_7 == destination.rank())
    }
    /// 敵陣
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `destination` -
    fn is_opponent_region(us: Phase, destination: &AbsoluteAddress) -> bool {
        (us == Phase::First && destination.rank() < RANK_4)
            || (us == Phase::Second && RANK_6 < destination.rank())
    }
}
