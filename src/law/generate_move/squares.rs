use crate::cosmic::shogi::state::Phase;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::{
    AbsoluteAddress, Address, Angle, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4,
    RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::law::diagnostic::{assert_in_board_as_absolute, assert_in_board_with_frame_as_absolute};
use std::fmt;

/// 次の升☆（＾～＾）
pub struct NextSquares {}
impl NextSquares {
    /// 先手から見た盤上の駒の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `piece_type` - 駒の種類だぜ☆（＾～＾）
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `callback` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    pub fn piece_of<F1>(
        piece_type: PieceType,
        friend: Phase,
        source: &AbsoluteAddress,
        callback: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => NextSquares::pawn(friend, source, callback),
            PieceType::Lance => NextSquares::lance(friend, source, callback),
            PieceType::Knight => NextSquares::knight(friend, source, callback),
            PieceType::Silver => NextSquares::silver(friend, source, callback),
            PieceType::Gold => NextSquares::gold(friend, source, callback),
            PieceType::King => NextSquares::king(source, callback),
            PieceType::Bishop => NextSquares::bishop(friend, source, callback),
            PieceType::Rook => NextSquares::rook(friend, source, callback),
            PieceType::PromotedPawn => NextSquares::gold(friend, source, callback),
            PieceType::PromotedLance => NextSquares::gold(friend, source, callback),
            PieceType::PromotedKnight => NextSquares::gold(friend, source, callback),
            PieceType::PromotedSilver => NextSquares::gold(friend, source, callback),
            PieceType::Horse => NextSquares::horse(source, callback),
            PieceType::Dragon => NextSquares::dragon(source, callback),
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
            Promoting::case_of_pawn_lance(
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

        NextSquares::r#move(angle, Agility::Hopping, source, promoting);
        /*
        IO::debugln(&format!(
            "歩の動き source={:?} angle={:?} forbidden={:?}",
            source,
            angle,
            MovePermission::from_pawn_or_lance(friend),
        ));
        */
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
            Promoting::case_of_pawn_lance(
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

        NextSquares::r#move(angle, Agility::Sliding, source, promoting);
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
            Promoting::case_of_knight(
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

        NextSquares::r#move(angle, Agility::Knight, source, promoting);

        let angle = angle.rotate90ccw();
        NextSquares::r#move(angle, Agility::Knight, source, promoting);
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
            &mut |destination| Promoting::case_of_silver(friend, &source, &destination, callback);

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };
        // println!("銀1={:?}", angle);
        NextSquares::r#move(angle, Agility::Hopping, source, promoting);
        // println!("銀2={:?}", angle.rotate45ccw());
        NextSquares::r#move(angle.rotate45ccw(), Agility::Hopping, source, promoting);
        // println!("銀3={:?}", angle.rotate90ccw().rotate45ccw());
        NextSquares::r#move(
            angle.rotate90ccw().rotate45ccw(),
            Agility::Hopping,
            source,
            promoting,
        );
        // println!("銀4={:?}", angle.rotate90cw().rotate45cw());
        NextSquares::r#move(
            angle.rotate90cw().rotate45cw(),
            Agility::Hopping,
            source,
            promoting,
        );
        // println!("銀5={:?}", angle.rotate45cw());
        NextSquares::r#move(angle.rotate45cw(), Agility::Hopping, source, promoting);
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
        NextSquares::r#move(angle, Agility::Hopping, source, hopping);
        NextSquares::r#move(angle.rotate45ccw(), Agility::Hopping, source, hopping);
        NextSquares::r#move(angle.rotate90ccw(), Agility::Hopping, source, hopping);
        NextSquares::r#move(angle.rotate180(), Agility::Hopping, source, hopping);
        NextSquares::r#move(angle.rotate90cw(), Agility::Hopping, source, hopping);
        NextSquares::r#move(angle.rotate45cw(), Agility::Hopping, source, hopping);
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
        NextSquares::r#move(Angle::Ccw0, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw45, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw90, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw135, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw180, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw225, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw270, Agility::Hopping, source, hopping);
        NextSquares::r#move(Angle::Ccw315, Agility::Hopping, source, hopping);
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
        let promoting = &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback)
        };
        NextSquares::r#move(Angle::Ccw45, Agility::Sliding, source, promoting);
        NextSquares::r#move(Angle::Ccw135, Agility::Sliding, source, promoting);
        NextSquares::r#move(Angle::Ccw225, Agility::Sliding, source, promoting);
        NextSquares::r#move(Angle::Ccw315, Agility::Sliding, source, promoting);
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
        let promoting = &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback)
        };
        NextSquares::r#move(Angle::Ccw0, Agility::Sliding, source, promoting);
        NextSquares::r#move(Angle::Ccw90, Agility::Sliding, source, promoting);
        NextSquares::r#move(Angle::Ccw180, Agility::Sliding, source, promoting);
        NextSquares::r#move(Angle::Ccw270, Agility::Sliding, source, promoting);
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
            NextSquares::r#move(Angle::Ccw45, Agility::Sliding, source, sliding);
            NextSquares::r#move(Angle::Ccw135, Agility::Sliding, source, sliding);
            NextSquares::r#move(Angle::Ccw225, Agility::Sliding, source, sliding);
            NextSquares::r#move(Angle::Ccw315, Agility::Sliding, source, sliding);
        }
        {
            let hopping = &mut |destination| {
                callback(destination, Promotability::Deny, Agility::Hopping, None)
            };
            NextSquares::r#move(Angle::Ccw0, Agility::Hopping, source, hopping);
            NextSquares::r#move(Angle::Ccw90, Agility::Hopping, source, hopping);
            NextSquares::r#move(Angle::Ccw180, Agility::Hopping, source, hopping);
            NextSquares::r#move(Angle::Ccw270, Agility::Hopping, source, hopping);
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
            NextSquares::r#move(Angle::Ccw0, Agility::Sliding, source, sliding);
            NextSquares::r#move(Angle::Ccw90, Agility::Sliding, source, sliding);
            NextSquares::r#move(Angle::Ccw180, Agility::Sliding, source, sliding);
            NextSquares::r#move(Angle::Ccw270, Agility::Sliding, source, sliding);
        }
        {
            let hopping = &mut |destination| {
                callback(destination, Promotability::Deny, Agility::Hopping, None)
            };
            NextSquares::r#move(Angle::Ccw45, Agility::Hopping, source, hopping);
            NextSquares::r#move(Angle::Ccw135, Agility::Hopping, source, hopping);
            NextSquares::r#move(Angle::Ccw225, Agility::Hopping, source, hopping);
            NextSquares::r#move(Angle::Ccw315, Agility::Hopping, source, hopping);
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
        // TODO for文の方を変えた方が高速だろ……☆（＾～＾）
        for rank in RANK_2..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let mut ab_adr = Address::new(file, rank).abs();

                // 後手ならひっくり返すぜ☆（＾～＾）
                if phase == Phase::Second {
                    ab_adr = ab_adr.rotate_180();
                }
                assert_in_board_with_frame_as_absolute(
                    ab_adr.address(),
                    &format!(
                        "square::for_from_rank2_to_rank9. phase={} rank={}, file={}, ab_adr={}.",
                        phase,
                        rank,
                        file,
                        ab_adr.address()
                    ),
                );
                callback(ab_adr);
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
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn r#move<F1>(angle: Angle, agility: Agility, start: &AbsoluteAddress, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress) -> bool,
    {
        match agility {
            Agility::Sliding => {
                let mut next = start.clone();
                loop {
                    // 西隣から反時計回りだぜ☆（＾～＾）
                    next.add_mut(&Address::new(1, 0).rel().rotate(angle));
                    if next.has_jumped_out_of_the_board() {
                        break;
                    }

                    if callback(next) {
                        break;
                    }
                }
            }
            // 桂馬専用☆（＾～＾）行き先の無いところに置いてないはずだぜ☆（＾～＾）
            Agility::Knight => {
                let mut next = start.clone();
                // 西隣から反時計回りだぜ☆（＾～＾）
                next.add_mut(&Address::new(1, 0).rel().rotate(angle).double_rank());
                if !next.has_jumped_out_of_the_board() {
                    assert_in_board_as_absolute(&next, "桂馬☆（＾～＾）");
                    callback(next);
                }
            }
            Agility::Hopping => {
                let mut next = start.clone();
                // 西隣から反時計回りだぜ☆（＾～＾）
                next.add_mut(&Address::new(1, 0).rel().rotate(angle));
                if !next.has_jumped_out_of_the_board() {
                    assert_in_board_as_absolute(&next, "隣☆（＾～＾）");
                    callback(next);
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

pub enum Promotability {
    /// 成ることはできないぜ☆（＾～＾）
    Deny,
    /// 成る、成らない両方あるぜ☆（＾～＾）
    Any,
    /// 必ず成れだぜ☆（＾～＾）
    Forced,
}

/// 行き先があるかないかのチェックに使うぜ☆（＾～＾）
/// TODO 成れるときは要らないぜ☆（＾～＾）
pub struct MovePermission {
    min_rank: i8,
    max_rank: i8,
}
impl MovePermission {
    pub fn from_pawn_or_lance(friend: Phase) -> Self {
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
    pub fn from_knight(friend: Phase) -> Self {
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
    pub fn check(&self, destination: &AbsoluteAddress) -> bool {
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
    /// 成らずに一番奥の段に移動することはできません。
    fn case_of_pawn_lance<F1>(
        friend: Phase,
        destinaion: &AbsoluteAddress,
        callback_next: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(friend, &destinaion) {
            // 自陣から見て一番奥の段
            callback_next(
                *destinaion,
                Promotability::Forced,
                Agility::Hopping,
                move_permission,
            )
        } else if Promoting::is_second_third_farthest_rank_from_friend(friend, &destinaion) {
            // 自陣から見て二番、三番目の奥の段
            callback_next(
                *destinaion,
                Promotability::Any,
                Agility::Hopping,
                move_permission,
            )
        } else {
            callback_next(
                *destinaion,
                Promotability::Deny,
                Agility::Hopping,
                move_permission,
            )
        }
    }

    /// 成らずに一番奥の段、奥から２番目の段に移動することはできません。
    fn case_of_knight<F1>(
        friend: Phase,
        destination: &AbsoluteAddress,
        callback_next: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        // TODO 成らずに一番奥の段、奥から２番目の段に移動することはできません。
        if Promoting::is_first_second_farthest_rank_from_friend(friend, &destination) {
            callback_next(
                *destination,
                Promotability::Forced,
                Agility::Knight,
                move_permission,
            )
        } else if Promoting::is_third_farthest_rank_from_friend(friend, &destination) {
            callback_next(
                *destination,
                Promotability::Any,
                Agility::Knight,
                move_permission,
            )
        } else {
            callback_next(
                *destination,
                Promotability::Deny,
                Agility::Knight,
                move_permission,
            )
        }
    }

    /// TODO 自陣から見て奥から１～３段目に入るときに成れます。元位置が３段目のときは、動けば成るか選べます。
    fn case_of_silver<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        destination: &AbsoluteAddress,
        callback_next: &mut F1,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(friend, &source) {
            callback_next(*destination, Promotability::Any, Agility::Hopping, None)
        } else if Promoting::is_opponent_area_rank(friend, &destination) {
            callback_next(*destination, Promotability::Any, Agility::Hopping, None)
        } else {
            callback_next(*destination, Promotability::Deny, Agility::Hopping, None)
        }
    }

    /// TODO 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    fn case_of_bishop_rook<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        destination: &AbsoluteAddress,
        callback_next: &mut F1,
    ) -> bool
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_opponent_area_rank(friend, &source)
            || Promoting::is_opponent_area_rank(friend, &destination)
        {
            callback_next(*destination, Promotability::Any, Agility::Sliding, None)
        } else {
            callback_next(*destination, Promotability::Deny, Agility::Sliding, None)
        }
    }

    /// 自陣から見て、一番遠いの段
    fn is_farthest_rank_from_friend(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.rank() < RANK_2)
            || (friend == Phase::Second && RANK_8 < destination.rank())
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    fn is_first_second_farthest_rank_from_friend(
        friend: Phase,
        destination: &AbsoluteAddress,
    ) -> bool {
        (friend == Phase::First && destination.rank() < RANK_3)
            || (friend == Phase::Second && RANK_7 < destination.rank())
    }
    /// 自陣から見て、二番目、三番目に遠いの段
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
    fn is_third_farthest_rank_from_friend(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.rank() == RANK_3)
            || (friend == Phase::Second && RANK_7 == destination.rank())
    }
    /// 敵陣の段
    fn is_opponent_area_rank(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.rank() < RANK_4)
            || (friend == Phase::Second && RANK_6 < destination.rank())
    }
}
