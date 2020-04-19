use crate::cosmic::shogi::state::Phase;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::{
    AbsoluteAddress, Angle, RelativeAddress, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3,
    RANK_4, RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::law::diagnostic::{assert_in_board_as_absolute, assert_in_board_with_frame_as_absolute};
use std::fmt;

/// 機敏性。
#[derive(Clone, Copy, Debug)]
pub enum Agility {
    /// 隣へ１つ進む駒。
    Hopping,
    /// 長い利き。
    Sliding,
    /// 桂馬。
    Keima,
}

pub enum Promotability {
    /// 成ることはできないぜ☆（＾～＾）
    Deny,
    /// 成る、成らない両方あるぜ☆（＾～＾）
    Any,
    /// 必ず成れだぜ☆（＾～＾）
    Forced,
}

/// 駒が動ける升☆（＾～＾）
pub struct NextSquares {}
impl NextSquares {
    /// 盤上の歩から動けるマスを見ます。
    pub fn looking_for_squares_from_on_board<F1>(
        piece_type: PieceType,
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => {
                NextSquares::looking_for_square_from_pawn_on_board(friend, source, callback_next)
            }
            PieceType::Lance => {
                NextSquares::looking_for_squares_from_lance_on_board(friend, source, callback_next)
            }
            PieceType::Knight => {
                NextSquares::looking_for_squares_from_knight_on_board(friend, source, callback_next)
            }
            PieceType::Silver => {
                NextSquares::looking_for_squares_from_silver_on_board(friend, source, callback_next)
            }
            PieceType::Gold => {
                NextSquares::looking_for_squares_from_gold_on_board(friend, source, callback_next)
            }
            PieceType::King => {
                NextSquares::looking_for_squares_from_king_on_board(source, callback_next)
            }
            PieceType::Bishop => {
                NextSquares::looking_for_squares_from_bishop_on_board(friend, source, callback_next)
            }
            PieceType::Rook => {
                NextSquares::looking_for_squares_from_rook_on_board(friend, source, callback_next)
            }
            PieceType::PromotedPawn => {
                NextSquares::looking_for_squares_from_gold_on_board(friend, source, callback_next)
            }
            PieceType::PromotedLance => {
                NextSquares::looking_for_squares_from_gold_on_board(friend, source, callback_next)
            }
            PieceType::PromotedKnight => {
                NextSquares::looking_for_squares_from_gold_on_board(friend, source, callback_next)
            }
            PieceType::PromotedSilver => {
                NextSquares::looking_for_squares_from_gold_on_board(friend, source, callback_next)
            }
            PieceType::Horse => {
                NextSquares::looking_for_squares_from_horse_on_board(source, callback_next)
            }
            PieceType::Dragon => {
                NextSquares::looking_for_squares_from_dragon_on_board(source, callback_next)
            }
        }
    }

    /// 盤上の歩から動けるマスを見ます。
    fn looking_for_square_from_pawn_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::case_of_pawn_lance(
                friend,
                &destination,
                callback_next,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };

        Squares::looking_next_from(angle, Agility::Hopping, source, promoting);
        /*
        IO::debugln(&format!(
            "歩の動き source={:?} angle={:?} forbidden={:?}",
            source,
            angle,
            MovePermission::from_pawn_or_lance(friend),
        ));
        */
    }

    /// 盤上の香から動けるマスを見ます。
    fn looking_for_squares_from_lance_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::case_of_pawn_lance(
                friend,
                &destination,
                callback_next,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };

        Squares::looking_next_from(angle, Agility::Sliding, source, promoting);
    }

    /// 盤上の桂から動けるマスを見ます。
    fn looking_for_squares_from_knight_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::case_of_knight(
                friend,
                &destination,
                callback_next,
                Some(MovePermission::from_knight(friend)),
            )
        };

        let angle = if friend == Phase::First {
            Angle::Ccw225
        } else {
            Angle::Ccw45
        };

        Squares::looking_next_from(angle, Agility::Keima, source, promoting);

        let angle = angle.rotate90ccw();
        Squares::looking_next_from(angle, Agility::Keima, source, promoting);
    }

    /// 盤上の銀から動けるマスを見ます。
    fn looking_for_squares_from_silver_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        };

        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };
        // println!("銀1={:?}", angle);
        Squares::looking_next_from(angle, Agility::Hopping, source, promoting);
        // println!("銀2={:?}", angle.rotate45ccw());
        Squares::looking_next_from(angle.rotate45ccw(), Agility::Hopping, source, promoting);
        // println!("銀3={:?}", angle.rotate90ccw().rotate45ccw());
        Squares::looking_next_from(
            angle.rotate90ccw().rotate45ccw(),
            Agility::Hopping,
            source,
            promoting,
        );
        // println!("銀4={:?}", angle.rotate90cw().rotate45cw());
        Squares::looking_next_from(
            angle.rotate90cw().rotate45cw(),
            Agility::Hopping,
            source,
            promoting,
        );
        // println!("銀5={:?}", angle.rotate45cw());
        Squares::looking_next_from(angle.rotate45cw(), Agility::Hopping, source, promoting);
    }

    /// 盤上の金、と、杏、圭、全から動けるマスを見ます。
    fn looking_for_squares_from_gold_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let hopping = &mut |destination| {
            callback_next(destination, Promotability::Deny, Agility::Hopping, None)
        };
        let angle = if friend == Phase::First {
            Angle::Ccw270
        } else {
            Angle::Ccw90
        };
        Squares::looking_next_from(angle, Agility::Hopping, source, hopping);
        Squares::looking_next_from(angle.rotate45ccw(), Agility::Hopping, source, hopping);
        Squares::looking_next_from(angle.rotate90ccw(), Agility::Hopping, source, hopping);
        Squares::looking_next_from(angle.rotate180(), Agility::Hopping, source, hopping);
        Squares::looking_next_from(angle.rotate90cw(), Agility::Hopping, source, hopping);
        Squares::looking_next_from(angle.rotate45cw(), Agility::Hopping, source, hopping);
    }

    /// 盤上の玉から動けるマスを見ます。
    fn looking_for_squares_from_king_on_board<F1>(source: &AbsoluteAddress, callback_next: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let hopping = &mut |destination| {
            callback_next(destination, Promotability::Deny, Agility::Hopping, None)
        };
        Squares::looking_next_from(Angle::Ccw0, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw45, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw90, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw135, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw180, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw225, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw270, Agility::Hopping, source, hopping);
        Squares::looking_next_from(Angle::Ccw315, Agility::Hopping, source, hopping);
    }

    /// 盤上の角から動けるマスを見ます。
    fn looking_for_squares_from_bishop_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        };
        Squares::looking_next_from(Angle::Ccw45, Agility::Sliding, source, promoting);
        Squares::looking_next_from(Angle::Ccw135, Agility::Sliding, source, promoting);
        Squares::looking_next_from(Angle::Ccw225, Agility::Sliding, source, promoting);
        Squares::looking_next_from(Angle::Ccw315, Agility::Sliding, source, promoting);
    }

    /// 盤上の飛から動けるマスを見ます。
    fn looking_for_squares_from_rook_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let promoting = &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        };
        Squares::looking_next_from(Angle::Ccw0, Agility::Sliding, source, promoting);
        Squares::looking_next_from(Angle::Ccw90, Agility::Sliding, source, promoting);
        Squares::looking_next_from(Angle::Ccw180, Agility::Sliding, source, promoting);
        Squares::looking_next_from(Angle::Ccw270, Agility::Sliding, source, promoting);
    }

    /// 盤上の馬から動けるマスを見ます。
    fn looking_for_squares_from_horse_on_board<F1>(source: &AbsoluteAddress, callback_next: &mut F1)
    where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let sliding = &mut |destination| {
                callback_next(destination, Promotability::Deny, Agility::Sliding, None)
            };
            Squares::looking_next_from(Angle::Ccw45, Agility::Sliding, source, sliding);
            Squares::looking_next_from(Angle::Ccw135, Agility::Sliding, source, sliding);
            Squares::looking_next_from(Angle::Ccw225, Agility::Sliding, source, sliding);
            Squares::looking_next_from(Angle::Ccw315, Agility::Sliding, source, sliding);
        }
        {
            let hopping = &mut |destination| {
                callback_next(destination, Promotability::Deny, Agility::Hopping, None)
            };
            Squares::looking_next_from(Angle::Ccw0, Agility::Hopping, source, hopping);
            Squares::looking_next_from(Angle::Ccw90, Agility::Hopping, source, hopping);
            Squares::looking_next_from(Angle::Ccw180, Agility::Hopping, source, hopping);
            Squares::looking_next_from(Angle::Ccw270, Agility::Hopping, source, hopping);
        }
    }

    /// 盤上の竜から動けるマスを見ます。
    fn looking_for_squares_from_dragon_on_board<F1>(
        source: &AbsoluteAddress,
        callback_next: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let sliding = &mut |destination| {
                callback_next(destination, Promotability::Deny, Agility::Sliding, None)
            };
            Squares::looking_next_from(Angle::Ccw0, Agility::Sliding, source, sliding);
            Squares::looking_next_from(Angle::Ccw90, Agility::Sliding, source, sliding);
            Squares::looking_next_from(Angle::Ccw180, Agility::Sliding, source, sliding);
            Squares::looking_next_from(Angle::Ccw270, Agility::Sliding, source, sliding);
        }
        {
            let hopping = &mut |destination| {
                callback_next(destination, Promotability::Deny, Agility::Hopping, None)
            };
            Squares::looking_next_from(Angle::Ccw45, Agility::Hopping, source, hopping);
            Squares::looking_next_from(Angle::Ccw135, Agility::Hopping, source, hopping);
            Squares::looking_next_from(Angle::Ccw225, Agility::Hopping, source, hopping);
            Squares::looking_next_from(Angle::Ccw315, Agility::Hopping, source, hopping);
        }
    }
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
        if destination.get_rank() < self.min_rank || self.max_rank < destination.get_rank() {
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
                Agility::Keima,
                move_permission,
            )
        } else if Promoting::is_third_farthest_rank_from_friend(friend, &destination) {
            callback_next(
                *destination,
                Promotability::Any,
                Agility::Keima,
                move_permission,
            )
        } else {
            callback_next(
                *destination,
                Promotability::Deny,
                Agility::Keima,
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
        (friend == Phase::First && destination.get_rank() < RANK_2)
            || (friend == Phase::Second && RANK_8 < destination.get_rank())
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    fn is_first_second_farthest_rank_from_friend(
        friend: Phase,
        destination: &AbsoluteAddress,
    ) -> bool {
        (friend == Phase::First && destination.get_rank() < RANK_3)
            || (friend == Phase::Second && RANK_7 < destination.get_rank())
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    fn is_second_third_farthest_rank_from_friend(
        friend: Phase,
        destination: &AbsoluteAddress,
    ) -> bool {
        (friend == Phase::First
            && RANK_1 < destination.get_rank()
            && destination.get_rank() < RANK_4)
            || (friend == Phase::Second
                && RANK_6 < destination.get_rank()
                && destination.get_rank() < RANK_9)
    }
    /// 自陣から見て、三番目に遠いの段
    fn is_third_farthest_rank_from_friend(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.get_rank() == RANK_3)
            || (friend == Phase::Second && RANK_7 == destination.get_rank())
    }
    /// 敵陣の段
    fn is_opponent_area_rank(friend: Phase, destination: &AbsoluteAddress) -> bool {
        (friend == Phase::First && destination.get_rank() < RANK_4)
            || (friend == Phase::Second && RANK_6 < destination.get_rank())
    }
}

pub struct Squares {}
impl Squares {
    fn has_jumped_out_of_the_board(address: i8) -> bool {
        address / 10 % 10 == 0 || address % 10 == 0
    }

    /// 2段目～9段目 全升☆（＾～＾）
    /// 1段目～8段目 全升☆ がほしければ phase.turn() しろだぜ☆（＾～＾）
    pub fn for_from_rank2_to_rank9<F1>(phase: Phase, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        // TODO for文の方を変えた方が高速だろ……☆（＾～＾）
        for rank in RANK_2..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let mut ab_adr = AbsoluteAddress::from_file_rank(file, rank);

                // 後手ならひっくり返すぜ☆（＾～＾）
                if phase == Phase::Second {
                    ab_adr = ab_adr.rotate_180();
                }
                assert_in_board_with_frame_as_absolute(
                    ab_adr.address,
                    &format!(
                        "square::for_from_rank2_to_rank9. phase={} rank={}, file={}, ab_adr={}.",
                        phase, rank, file, ab_adr.address
                    ),
                );
                callback(ab_adr);
            }
        }
    }

    /// 3段目～9段目 全升☆（＾～＾）
    /// 1段目～7段目 全升☆ がほしければ phase.turn() しろだぜ☆（＾～＾）
    pub fn for_from_rank3_to_rank9<F1>(phase: Phase, callback: &mut F1)
    where
        F1: FnMut(AbsoluteAddress),
    {
        for rank in RANK_3..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let mut ab_adr = AbsoluteAddress::from_file_rank(file, rank);
                if phase == Phase::Second {
                    ab_adr = ab_adr.rotate_180();
                }

                callback(ab_adr);
            }
        }
    }

    /// 隣☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 行き先のない駒の判定に使うぜ☆（＾～＾）
    pub fn looking_next_from<F1>(
        angle: Angle,
        agility: Agility,
        start: &AbsoluteAddress,
        callback: &mut F1,
    ) where
        F1: FnMut(AbsoluteAddress) -> bool,
    {
        match agility {
            Agility::Sliding => {
                let mut next = start.address;
                loop {
                    // 回転の起角は西隣だぜ☆（＾～＾）
                    next += RelativeAddress::from_file_and_rank(1, 0)
                        .rotate(angle)
                        .get_address();
                    if Squares::has_jumped_out_of_the_board(next) {
                        break;
                    }

                    if callback(AbsoluteAddress::from_address(next)) {
                        break;
                    }
                }
            }
            Agility::Keima => {
                // 隣☆（＾～＾）桂馬用☆（＾～＾）
                // 回転の起角は西隣だぜ☆（＾～＾）
                let rel = RelativeAddress::from_file_and_rank(1, 0).rotate(angle);
                if !Squares::has_jumped_out_of_the_board(start.address + rel.get_address()) {
                    let rel = rel.double_rank();
                    if !Squares::has_jumped_out_of_the_board(start.address + rel.get_address()) {
                        let next = start.address + rel.get_address();

                        if !Squares::has_jumped_out_of_the_board(next) {
                            assert_in_board_as_absolute(next, "隣＋桂馬☆（＾～＾）");
                            callback(AbsoluteAddress::from_address(next));
                        }
                    }
                }
            }
            Agility::Hopping => {
                // 回転の起角は西隣だぜ☆（＾～＾）
                let rel = RelativeAddress::from_file_and_rank(1, 0)
                    .rotate(angle)
                    .get_address();
                // println!("angle={:?} {}", angle, rel);
                let next = start.address + rel;
                // println!("next={}", next);
                if !Squares::has_jumped_out_of_the_board(next) {
                    assert_in_board_as_absolute(
                        next,
                        "隣☆（＾～＾）",
                        /*
                        &format!(
                            "隣☆（＾～＾） start.address={} angle={:?} next={}",
                            start.address, angle, next
                        ),
                        */
                    );

                    callback(AbsoluteAddress::from_address(next));
                }
            }
        }
    }
}
