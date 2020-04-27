//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::cosmic::recording::{Movement, Person, Phase};
use crate::cosmic::smart::features::HandAddress;
use crate::cosmic::smart::features::PieceMeaning;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, Angle, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4,
    RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::cosmic::toy_box::PieceNum;
use crate::cosmic::toy_box::{Board, Location};
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::Beam;
use std::fmt;

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
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    pub fn make_move<F1>(
        friend: Phase,
        board: &Board,
        speed_of_light: &SpeedOfLight,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        board.for_some_pieces_on_list40(friend, speed_of_light, &mut |location, piece| {
            match location {
                Location::Board(source) => PseudoLegalMoves::make_on_board(
                    friend,
                    &source,
                    &piece,
                    board,
                    speed_of_light,
                    callback,
                ),
                Location::Hand(adr) => {
                    PseudoLegalMoves::make_drop(friend, adr, board, speed_of_light, callback);
                }
                Location::Busy => panic!(Beam::trouble(
                    "(Err.94) なんで駒が作業中なんだぜ☆（＾～＾）！"
                )),
            }
        });
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `piece` - 駒だぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    fn make_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        piece: &(PieceMeaning, PieceNum),
        board: &Board,
        speed_of_light: &SpeedOfLight,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        let callback_next =
            &mut |destination, promotability, _agility, move_permission: Option<MovePermission>| {
                let (ok, space) = if let Some(person) =
                    board.what_is_in_the_square(friend, &destination, speed_of_light)
                {
                    match person {
                        Person::Friend => (false, false),
                        Person::Opponent => (true, false),
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
                                callback(
                                    Movement {
                                        source: Some(*source),
                                        destination: destination,
                                        promote: false,
                                        drop: None,
                                    }
                                    .to_hash(),
                                );
                            }
                            callback(
                                Movement {
                                    source: Some(*source),
                                    destination: destination,
                                    promote: true,
                                    drop: None,
                                }
                                .to_hash(),
                            );
                        }
                        _ => {
                            // 成れるか、成れないかのどちらかのとき。
                            if promotion || !forbidden {
                                callback(
                                    Movement {
                                        source: Some(*source),
                                        destination: destination,
                                        promote: promotion,
                                        drop: None,
                                    }
                                    .to_hash(),
                                );
                            }
                        }
                    };
                }

                !space
            };

        Area::piece_of(
            piece.0.r#type(speed_of_light),
            friend,
            &source,
            callback_next,
        );
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    fn make_drop<F1>(
        friend: Phase,
        adr: HandAddress,
        board: &Board,
        speed_of_light: &SpeedOfLight,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        if let Some(piece) = board.last_hand(adr) {
            // 打つぜ☆（＾～＾）
            let drop = &mut |destination| {
                if let None = board.piece_at(&destination) {
                    // 駒が無いところに打つ
                    use crate::cosmic::smart::features::PieceMeaning::*;
                    match piece.0 {
                        Pawn1 | Pawn2 => {
                            // ひよこ　は２歩できない☆（＾～＾）
                            if board.exists_pawn_on_file(friend, destination.file(), speed_of_light)
                            {
                                return;
                            }
                        }
                        _ => {}
                    }
                    callback(
                        Movement {
                            source: None,                                                            // 駒台
                            destination: destination, // どの升へ行きたいか
                            promote: false,           // 打に成りは無し
                            drop: Some(piece.0.hand_address(speed_of_light).r#type(speed_of_light)), // 打った駒種類
                        }
                        .to_hash(),
                    );
                }
            };

            // 駒を持っていれば
            let ty = adr.r#type(speed_of_light);
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
                callback(Address::new(file, rank).abs());
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
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn piece_of<F1>(
        piece_type: PieceType,
        friend: Phase,
        source: &AbsoluteAddress,
        callback: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => Area::pawn(friend, source, callback),
            PieceType::Lance => Area::lance(friend, source, callback),
            PieceType::Knight => Area::knight(friend, source, callback),
            PieceType::Silver => Area::silver(friend, source, callback),
            PieceType::Gold => Area::gold(friend, source, callback),
            PieceType::King => Area::king(source, callback),
            PieceType::Bishop => Area::bishop(friend, source, callback),
            PieceType::Rook => Area::rook(friend, source, callback),
            PieceType::PromotedPawn => Area::gold(friend, source, callback),
            PieceType::PromotedLance => Area::gold(friend, source, callback),
            PieceType::PromotedKnight => Area::gold(friend, source, callback),
            PieceType::PromotedSilver => Area::gold(friend, source, callback),
            PieceType::Horse => Area::horse(source, callback),
            PieceType::Dragon => Area::dragon(source, callback),
        }
    }

    /// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn pawn<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::pawn_lance(
                friend,
                &destination,
                callback,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };

        Area::r#move(source, angle, Agility::Hopping, promoting);
    }

    /// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn lance<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::pawn_lance(
                friend,
                &destination,
                callback,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };

        Area::r#move(source, angle, Agility::Sliding, promoting);
    }

    /// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn knight<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::knight(
                friend,
                &destination,
                callback,
                Some(MovePermission::from_knight(friend)),
            )
        };

        let angle = if friend == Phase::First {
            Angle::Ccw225
        } else {
            Angle::Ccw45
        };

        Area::r#move(source, angle, Agility::Knight, promoting);

        let angle = angle.rotate90ccw();
        Area::r#move(source, angle, Agility::Knight, promoting);
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting =
            &mut |destination| Promoting::silver(friend, &source, &destination, callback);

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };
        Area::r#move(source, angle, Agility::Hopping, promoting);
        Area::r#move(source, angle.rotate45ccw(), Agility::Hopping, promoting);
        Area::r#move(
            source,
            angle.rotate90ccw().rotate45ccw(),
            Agility::Hopping,
            promoting,
        );
        Area::r#move(
            source,
            angle.rotate90cw().rotate45cw(),
            Agility::Hopping,
            promoting,
        );
        Area::r#move(source, angle.rotate45cw(), Agility::Hopping, promoting);
    }

    /// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn gold<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let hopping =
            &mut |destination| callback(destination, Promotability::Deny, Agility::Hopping, None);
        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };
        Area::r#move(source, angle, Agility::Hopping, hopping);
        Area::r#move(source, angle.rotate45ccw(), Agility::Hopping, hopping);
        Area::r#move(source, angle.rotate90ccw(), Agility::Hopping, hopping);
        Area::r#move(source, angle.rotate180(), Agility::Hopping, hopping);
        Area::r#move(source, angle.rotate90cw(), Agility::Hopping, hopping);
        Area::r#move(source, angle.rotate45cw(), Agility::Hopping, hopping);
    }

    /// 盤上の玉の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn king<F1>(source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let hopping =
            &mut |destination| callback(destination, Promotability::Deny, Agility::Hopping, None);
        Area::r#move(source, Angle::Ccw0, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw45, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw90, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw135, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw180, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw225, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw270, Agility::Hopping, hopping);
        Area::r#move(source, Angle::Ccw315, Agility::Hopping, hopping);
    }

    /// 盤上の角の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn bishop<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting =
            &mut |destination| Promoting::bishop_rook(friend, &source, &destination, callback);
        Area::r#move(source, Angle::Ccw45, Agility::Sliding, promoting);
        Area::r#move(source, Angle::Ccw135, Agility::Sliding, promoting);
        Area::r#move(source, Angle::Ccw225, Agility::Sliding, promoting);
        Area::r#move(source, Angle::Ccw315, Agility::Sliding, promoting);
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(friend: Phase, source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting =
            &mut |destination| Promoting::bishop_rook(friend, &source, &destination, callback);
        Area::r#move(source, Angle::Ccw0, Agility::Sliding, promoting);
        Area::r#move(source, Angle::Ccw90, Agility::Sliding, promoting);
        Area::r#move(source, Angle::Ccw180, Agility::Sliding, promoting);
        Area::r#move(source, Angle::Ccw270, Agility::Sliding, promoting);
    }

    /// 盤上の馬の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn horse<F1>(source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let sliding = &mut |destination| {
                callback(destination, Promotability::Deny, Agility::Sliding, None)
            };
            Area::r#move(source, Angle::Ccw45, Agility::Sliding, sliding);
            Area::r#move(source, Angle::Ccw135, Agility::Sliding, sliding);
            Area::r#move(source, Angle::Ccw225, Agility::Sliding, sliding);
            Area::r#move(source, Angle::Ccw315, Agility::Sliding, sliding);
        }
        {
            let hopping = &mut |destination| {
                callback(destination, Promotability::Deny, Agility::Hopping, None)
            };
            Area::r#move(source, Angle::Ccw0, Agility::Hopping, hopping);
            Area::r#move(source, Angle::Ccw90, Agility::Hopping, hopping);
            Area::r#move(source, Angle::Ccw180, Agility::Hopping, hopping);
            Area::r#move(source, Angle::Ccw270, Agility::Hopping, hopping);
        }
    }

    /// 盤上の竜の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn dragon<F1>(source: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let sliding = &mut |destination| {
                callback(destination, Promotability::Deny, Agility::Sliding, None)
            };
            Area::r#move(source, Angle::Ccw0, Agility::Sliding, sliding);
            Area::r#move(source, Angle::Ccw90, Agility::Sliding, sliding);
            Area::r#move(source, Angle::Ccw180, Agility::Sliding, sliding);
            Area::r#move(source, Angle::Ccw270, Agility::Sliding, sliding);
        }
        {
            let hopping = &mut |destination| {
                callback(destination, Promotability::Deny, Agility::Hopping, None)
            };
            Area::r#move(source, Angle::Ccw45, Agility::Hopping, hopping);
            Area::r#move(source, Angle::Ccw135, Agility::Hopping, hopping);
            Area::r#move(source, Angle::Ccw225, Agility::Hopping, hopping);
            Area::r#move(source, Angle::Ccw315, Agility::Hopping, hopping);
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
                callback(Address::new(file, rank).abs());
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
                let mut ab_adr = Address::new(file, rank).abs();
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
    ///
    /// * `start` - 移動元升☆（＾～＾）
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn r#move<F1>(start: &AbsoluteAddress, angle: Angle, agility: Agility, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress) -> bool,
    {
        match agility {
            Agility::Sliding => {
                let mut cur = start.clone();
                let rel = Address::new(1, 0).rel().rotate(angle);
                loop {
                    // 西隣から反時計回りだぜ☆（＾～＾）
                    cur.offset(&rel);
                    if !cur.legal_cur() {
                        break;
                    }

                    if callback(cur) {
                        break;
                    }
                }
            }
            // 桂馬専用☆（＾～＾）行き先の無いところに置いてないはずだぜ☆（＾～＾）
            Agility::Knight => {
                let mut cur = start.clone();

                // 西隣から反時計回りだぜ☆（＾～＾）
                cur.offset(&Address::new(1, 0).rel().rotate(angle).double_rank());
                if cur.legal_cur() {
                    callback(cur);
                }
            }
            Agility::Hopping => {
                let mut cur = start.clone();

                // 西隣から反時計回りだぜ☆（＾～＾）
                cur.offset(&Address::new(1, 0).rel().rotate(angle));
                if cur.legal_cur() {
                    callback(cur);
                }
            }
        }
    }
}

/// 機敏性。
#[derive(Clone, Copy, Debug)]
enum Agility {
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
    min_rank: i8,
    max_rank: i8,
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
