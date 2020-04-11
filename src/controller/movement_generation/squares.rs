use crate::controller::common_use::cu_asserts_controller::assert_in_board_as_absolute;
use crate::controller::common_use::cu_asserts_controller::assert_in_board_as_relative;
use crate::controller::common_use::cu_asserts_controller::assert_in_board_with_frame_as_absolute;
use crate::controller::io::*;
use crate::model::univ::gam::misc::phase::Phase;
use crate::model::univ::gam::misc::square::Square;
use crate::model::univ::gam::misc::square::*;

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
    pub fn looking_for_square_from_pawn_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 =
            &mut |destination| Promoting::case_of_pawn_lance(friend, &destination, callback_next);

        let rotation = if *friend == Phase::First {
            Rotation::C0
        } else {
            Rotation::C180
        };

        // 回転しなければ北隣だぜ☆（＾～＾）
        Squares::next_of(&rotation, source, func1);
    }

    /// 盤上の香から動けるマスを見ます。
    pub fn looking_for_squares_from_lance_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 =
            &mut |destination| Promoting::case_of_pawn_lance(friend, &destination, callback_next);
        Squares::looking_east_from(Counterclockwise::Rotate90, friend, source, func1);
    }

    /// 盤上の桂から動けるマスを見ます。
    pub fn looking_for_squares_from_knight_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 =
            &mut |destination| Promoting::case_of_knight(friend, &destination, callback_next);
        Squares::north_east_keima_of(UpsideDown::Origin, friend, source, func1);
        Squares::north_east_keima_of(UpsideDown::Flip, &friend.turn(), source, func1);
    }

    /// 盤上の銀から動けるマスを見ます。
    pub fn looking_for_squares_from_silver_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        };

        let rotation = if *friend == Phase::First {
            Rotation::C0
        } else {
            Rotation::C180
        };
        // 回転しなければ北隣だぜ☆（＾～＾）
        println!("銀1={:?}", rotation);
        Squares::next_of(&rotation, source, func1);
        println!("銀2={:?}", rotation.rotate45ccw());
        Squares::next_of(&rotation.rotate45ccw(), source, func1);
        println!("銀3={:?}", rotation.rotate90ccw().rotate45ccw());
        Squares::next_of(&rotation.rotate90ccw().rotate45ccw(), source, func1);
        println!("銀4={:?}", rotation.rotate90cw().rotate45cw());
        Squares::next_of(&rotation.rotate90cw().rotate45cw(), source, func1);
        println!("銀5={:?}", rotation.rotate45cw());
        Squares::next_of(&rotation.rotate45cw(), source, func1);
    }

    /// 盤上の金、と、杏、圭、全から動けるマスを見ます。
    pub fn looking_for_squares_from_gold_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| callback_next(destination, Promotability::Deny);
        let rotation = if *friend == Phase::First {
            Rotation::C0
        } else {
            Rotation::C180
        };
        // 回転しなければ北隣だぜ☆（＾～＾）
        Squares::next_of(&rotation, source, func1);
        Squares::next_of(&rotation.rotate45ccw(), source, func1);
        Squares::next_of(&rotation.rotate90ccw(), source, func1);
        Squares::next_of(&rotation.rotate180(), source, func1);
        Squares::next_of(&rotation.rotate90cw(), source, func1);
        Squares::next_of(&rotation.rotate45cw(), source, func1);
    }

    /// 盤上の玉から動けるマスを見ます。
    pub fn looking_for_squares_from_king_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| callback_next(destination, Promotability::Deny);

        let rotation = if *friend == Phase::First {
            Rotation::C0
        } else {
            Rotation::C180
        };
        // 回転しなければ北隣だぜ☆（＾～＾）
        Squares::next_of(&rotation, source, func1);
        Squares::next_of(&rotation.rotate45ccw(), source, func1);
        Squares::next_of(&rotation.rotate90ccw(), source, func1);
        Squares::next_of(&rotation.rotate90ccw().rotate45ccw(), source, func1);
        Squares::next_of(&rotation.rotate180(), source, func1);
        Squares::next_of(&rotation.rotate90cw().rotate45cw(), source, func1);
        Squares::next_of(&rotation.rotate90cw(), source, func1);
        Squares::next_of(&rotation.rotate45cw(), source, func1);
    }

    /// 盤上の角から動けるマスを見ます。
    pub fn looking_for_squares_from_bishop_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        };
        Squares::looking_north_east_from(UpsideDown::Flip, &friend.turn(), source, func1);
        Squares::looking_north_east_from(UpsideDown::Origin, friend, source, func1);
        Squares::looking_north_east_from(UpsideDown::Origin, &friend.turn(), source, func1);
        Squares::looking_north_east_from(UpsideDown::Flip, friend, source, func1);
    }

    /// 盤上の飛から動けるマスを見ます。
    pub fn looking_for_squares_from_rook_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        };
        Squares::looking_east_from(Counterclockwise::Origin, &friend.turn(), source, func1);
        Squares::looking_east_from(Counterclockwise::Origin, friend, source, func1);
        Squares::looking_east_from(Counterclockwise::Rotate90, friend, source, func1);
        Squares::looking_east_from(Counterclockwise::Rotate90, &friend.turn(), source, func1);
    }

    /// 盤上の馬から動けるマスを見ます。
    pub fn looking_for_squares_from_horse_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| callback_next(destination, Promotability::Deny);
        let rotation = if *friend == Phase::First {
            Rotation::C0
        } else {
            Rotation::C180
        };
        // 回転しなければ北隣だぜ☆（＾～＾）
        Squares::next_of(&rotation, source, func1);
        Squares::next_of(&rotation.rotate90ccw(), source, func1);
        Squares::next_of(&rotation.rotate180(), source, func1);
        Squares::next_of(&rotation.rotate90cw(), source, func1);
        Squares::looking_north_east_from(UpsideDown::Flip, &friend.turn(), source, func1);
        Squares::looking_north_east_from(UpsideDown::Origin, friend, source, func1);
        Squares::looking_north_east_from(UpsideDown::Origin, &friend.turn(), source, func1);
        Squares::looking_north_east_from(UpsideDown::Flip, friend, source, func1);
    }

    /// 盤上の竜から動けるマスを見ます。
    pub fn looking_for_squares_from_dragon_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        let func1 = &mut |destination| callback_next(destination, Promotability::Deny);
        Squares::north_east_of(UpsideDown::Flip, &friend.turn(), source, func1);
        Squares::north_east_of(UpsideDown::Origin, friend, source, func1);
        Squares::north_east_of(UpsideDown::Origin, &friend.turn(), source, func1);
        Squares::north_east_of(UpsideDown::Flip, friend, source, func1);
        Squares::looking_east_from(Counterclockwise::Origin, &friend.turn(), source, func1);
        Squares::looking_east_from(Counterclockwise::Origin, friend, source, func1);
        Squares::looking_east_from(Counterclockwise::Rotate90, friend, source, func1);
        Squares::looking_east_from(Counterclockwise::Rotate90, &friend.turn(), source, func1);
    }
}

/// 成れるか、成れないか☆（＾～＾）
struct Promoting {}
impl Promoting {
    /// 成らずに一番奥の段に移動することはできません。
    fn case_of_pawn_lance<F1>(friend: &Phase, destinaion: &Square, callback_next: &mut F1) -> bool
    where
        F1: FnMut(Square, Promotability) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(friend, &destinaion) {
            // 自陣から見て一番奥の段
            callback_next(*destinaion, Promotability::Forced)
        } else if Promoting::is_second_third_farthest_rank_from_friend(friend, &destinaion) {
            // 自陣から見て二番、三番目の奥の段
            callback_next(*destinaion, Promotability::Any)
        } else {
            callback_next(*destinaion, Promotability::Deny)
        }
    }

    /// 成らずに一番奥の段、奥から２番目の段に移動することはできません。
    fn case_of_knight<F1>(friend: &Phase, destination: &Square, callback_next: &mut F1) -> bool
    where
        F1: FnMut(Square, Promotability) -> bool,
    {
        // TODO 成らずに一番奥の段、奥から２番目の段に移動することはできません。
        if Promoting::is_first_second_farthest_rank_from_friend(friend, &destination) {
            callback_next(*destination, Promotability::Forced)
        } else if Promoting::is_third_farthest_rank_from_friend(friend, &destination) {
            callback_next(*destination, Promotability::Any)
        } else {
            callback_next(*destination, Promotability::Deny)
        }
    }

    /// TODO 自陣から見て奥から１～３段目に入るときに成れます。元位置が３段目のときは、動けば成るか選べます。
    fn case_of_silver<F1>(
        friend: &Phase,
        source: &Square,
        destination: &Square,
        callback_next: &mut F1,
    ) -> bool
    where
        F1: FnMut(Square, Promotability) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(friend, &source) {
            callback_next(*destination, Promotability::Any)
        } else if Promoting::is_opponent_area_rank(friend, &destination) {
            callback_next(*destination, Promotability::Any)
        } else {
            callback_next(*destination, Promotability::Deny)
        }
    }

    /// TODO 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    fn case_of_bishop_rook<F1>(
        friend: &Phase,
        source: &Square,
        destination: &Square,
        callback_next: &mut F1,
    ) -> bool
    where
        F1: FnMut(Square, Promotability) -> bool,
    {
        if Promoting::is_opponent_area_rank(friend, &source)
            || Promoting::is_opponent_area_rank(friend, &destination)
        {
            callback_next(*destination, Promotability::Any)
        } else {
            callback_next(*destination, Promotability::Deny)
        }
    }

    /// 自陣から見て、一番遠いの段
    fn is_farthest_rank_from_friend(friend: &Phase, destination: &Square) -> bool {
        (*friend == Phase::First && destination.get_rank() < RANK_2)
            || (*friend == Phase::Second && RANK_8 < destination.get_rank())
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    fn is_first_second_farthest_rank_from_friend(friend: &Phase, destination: &Square) -> bool {
        (*friend == Phase::First && destination.get_rank() < RANK_3)
            || (*friend == Phase::Second && RANK_7 < destination.get_rank())
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    fn is_second_third_farthest_rank_from_friend(friend: &Phase, destination: &Square) -> bool {
        (*friend == Phase::First
            && RANK_1 < destination.get_rank()
            && destination.get_rank() < RANK_4)
            || (*friend == Phase::Second
                && RANK_6 < destination.get_rank()
                && destination.get_rank() < RANK_9)
    }
    /// 自陣から見て、三番目に遠いの段
    fn is_third_farthest_rank_from_friend(friend: &Phase, destination: &Square) -> bool {
        (*friend == Phase::First && destination.get_rank() == RANK_3)
            || (*friend == Phase::Second && RANK_7 == destination.get_rank())
    }
    /// 敵陣の段
    fn is_opponent_area_rank(friend: &Phase, destination: &Square) -> bool {
        (*friend == Phase::First && destination.get_rank() < RANK_4)
            || (*friend == Phase::Second && RANK_6 < destination.get_rank())
    }
}

pub enum Mirror {
    Origin,
    /// East から West へ反転。
    Mirror,
    Both,
}

pub enum UpsideDown {
    Origin,
    Flip,
}

pub enum Counterclockwise {
    Origin,
    Rotate90,
}

pub struct Squares {}
impl Squares {
    fn rotate180_as_relative(phase: &Phase, square: isquare) -> isquare {
        if *phase == Phase::Second {
            -square
        } else {
            square
        }
    }
    fn rotate180_as_absolute(phase: &Phase, square: isquare) -> isquare {
        if *phase == Phase::Second {
            110 - square
        } else {
            square
        }
    }

    /// 相対番地が、反時計回りに９０°回転するぜ☆（＾～＾）
    fn rotate90_counterclockwise_as_relative(
        counterclockwise: &Counterclockwise,
        upside_down: &UpsideDown,
        adr: isquare,
    ) -> isquare {
        let rel = match counterclockwise {
            Counterclockwise::Rotate90 => {
                let result = (adr.abs() % 10 - 1) * 10 + (10 - ((adr / 10).abs() % 10));
                if 0 < adr {
                    -result
                } else {
                    result
                }
            }
            Counterclockwise::Origin => adr,
        };

        match upside_down {
            UpsideDown::Flip => -rel,
            UpsideDown::Origin => rel,
        }
    }

    fn upside_down(upside_down: &UpsideDown, address: isquare) -> isquare {
        match upside_down {
            UpsideDown::Flip => address / 10 * 10 - (10 - (address.abs() % 10)) + 10,
            UpsideDown::Origin => address,
        }
    }

    fn has_jumped_out_horizontally(counterclockwise: &Counterclockwise, address: i8) -> bool {
        match counterclockwise {
            // Horizontally.
            Counterclockwise::Origin => address / 10 % 10 == 0,
            // Vertically.
            Counterclockwise::Rotate90 => address % 10 == 0,
        }
    }

    fn has_jumped_out_of_the_board(address: i8) -> bool {
        address / 10 % 10 == 0 || address % 10 == 0
    }

    /// 2段目～9段目 全升☆（＾～＾）
    /// 1段目～8段目 全升☆ がほしければ phase.turn() しろだぜ☆（＾～＾）
    pub fn for_from_rank2_to_rank9<F1>(phase: &Phase, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank in RANK_2..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let adr1 = Square::from_file_rank(file, rank).address;
                let adr2 = Squares::rotate180_as_absolute(phase, adr1);
                assert_in_board_with_frame_as_absolute(
                    adr2,
                    &format!(
                        "square::for_from_rank2_to_rank9(). rank={}, file={}, adr1={}, adr2={}.",
                        rank, file, adr1, adr2
                    ),
                );
                callback(Square::from_address(adr2));
            }
        }
    }

    /// 3段目～9段目 全升☆（＾～＾）
    /// 1段目～7段目 全升☆ がほしければ phase.turn() しろだぜ☆（＾～＾）
    pub fn for_from_rank3_to_rank9<F1>(phase: &Phase, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank in RANK_3..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                callback(Square::from_address(Squares::rotate180_as_absolute(
                    phase,
                    Square::from_file_rank(file, rank).address,
                )));
            }
        }
    }

    /// 東隣の升から東へ☆（＾～＾）
    /// 西隣の升から西へ にしたかったら phase.turn() しろだぜ☆（＾～＾）
    /// 北隣の升から北へ にしたかったら 時計回りに９０°回転しろだぜ☆（＾～＾）
    /// 南隣の升から南へ にしたかったら 時計回りに９０°回転して、 phase.turn() しろだぜ☆（＾～＾）
    pub fn looking_east_from<F1>(
        counterclockwise: Counterclockwise,
        phase: &Phase,
        start: &Square,
        callback: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate90_counterclockwise_as_relative(
                &counterclockwise,
                &UpsideDown::Origin,
                Squares::rotate180_as_relative(phase, -10),
            );
            if Squares::has_jumped_out_horizontally(&counterclockwise, next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }

    /// 北東隣の升から北東へ☆（＾～＾）
    /// 南東隣の升から南東へ☆ にしたかったら phase.turn() しろだぜ☆（＾～＾）
    /// 南東隣の升から南東へ☆ にしたかったら upside_down しろだぜ☆（＾～＾）
    /// 北西隣の升から北西へ☆ にしたかったら にしたかったら upside_down して、 phase.turn() しろだぜ☆（＾～＾）
    pub fn looking_north_east_from<F1>(
        upside_down: UpsideDown,
        phase: &Phase,
        start: &Square,
        callback: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate180_as_relative(phase, Squares::upside_down(&upside_down, -11));
            if Squares::has_jumped_out_of_the_board(next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }

    /// 北隣☆（＾～＾） 回転もできるぜ☆（＾～＾）
    pub fn next_of<F1>(rotation: &Rotation, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        // 北隣＋回転☆（＾～＾）
        // println!("start={}", start.address);
        // println!("north={:?}", RelativeSquare::from_file_and_rank(0, -1));
        let rel = RelativeSquare::from_file_and_rank(0, -1)
            .rotate_countercrockwise(rotation)
            .get_address();
        // println!("rot={:?} {}", rotation, rel);
        let next = start.address + rel;
        // println!("next={}", next);
        if !Squares::has_jumped_out_of_the_board(next) {
            assert_in_board_as_absolute(
                next,
                "北隣＋回転☆（＾～＾）",
                /*
                &format!(
                    "北隣＋回転☆（＾～＾） start.address={} rotation={:?} next={}",
                    start.address, rotation, next
                ),
                */
            );
            callback(Square::from_address(next));
        }
    }
    /// 東隣☆（＾～＾） 西隣もついでに作れだぜ☆（＾～＾）
    pub fn east_of<F1>(
        counterclockwise: Counterclockwise,
        mirror: Mirror,
        phase: &Phase,
        start: &Square,
        callback: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        match mirror {
            Mirror::Origin | Mirror::Both => {
                // 東隣☆（＾～＾）
                let rel_east = -10;
                let rel_rot180 = Squares::rotate180_as_relative(phase, rel_east);
                let rot90rel = Squares::rotate90_counterclockwise_as_relative(
                    &counterclockwise,
                    &UpsideDown::Origin,
                    rel_rot180,
                );
                let next = rot90rel + start.address;
                /*
                IO::logln(&format!(
                    "東隣 {} rel_east={} {}{}next={} + S{}",
                    phase,
                    rel_east,
                    match *phase {
                        Phase::Second => format!("rot180={} ", rel_rot180),
                        _ => "".to_string(),
                    },
                    match counterclockwise {
                        Counterclockwise::Rotate90 => format!("rot90={} ", rot90rel),
                        _ => "".to_string(),
                    },
                    next,
                    start.address,
                ));
                */
                if !Squares::has_jumped_out_horizontally(&counterclockwise, next) {
                    assert_in_board_as_absolute(
                        next,
                        &format!(
                            "東隣e☆（＾～＾） start.address={} rot180rel={} rot90rel={}",
                            start.address, rel_rot180, rot90rel
                        ),
                    );
                    callback(Square::from_address(next));
                }
            }
            _ => {}
        };

        match mirror {
            Mirror::Mirror | Mirror::Both => {
                // 東隣を、上下反転させて先後反転させれば、西隣だぜ☆（＾～＾）
                let rot180rel = Squares::rotate180_as_relative(phase, 10);
                let rot90rel = Squares::rotate90_counterclockwise_as_relative(
                    &counterclockwise,
                    &UpsideDown::Flip,
                    rot180rel,
                );
                let next = start.address + rot90rel;
                IO::logln(&format!(
                    "西隣 rot180rel={} rot90rel={} next={}",
                    rot180rel, rot90rel, next
                ));
                if !Squares::has_jumped_out_horizontally(&counterclockwise, next) {
                    assert_in_board_as_absolute(next, "西隣w☆（＾～＾）");
                    callback(Square::from_address(next));
                }
            }
            _ => {}
        }
    }

    /// 北東隣☆（＾～＾）
    /// 南西隣 にしたかったら phase.turn() しろだぜ☆（＾～＾）
    pub fn north_east_of<F1>(
        upside_down: UpsideDown,
        phase: &Phase,
        start: &Square,
        callback: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address
            + Squares::rotate180_as_relative(phase, Squares::upside_down(&upside_down, -11));
        if !Squares::has_jumped_out_of_the_board(next) {
            assert_in_board_as_absolute(next, "北東隣☆（＾～＾）");
            callback(Square::from_address(next));
        }
    }

    /// 北北東隣☆（＾～＾）
    /// スタート地点は、行き先の有る駒　である前提だぜ☆（＾～＾）
    /// 南南西隣 にしたかったら phase.turn() しろだぜ☆（＾～＾）
    /// 南南東隣 にしたかったら upside_down しろだぜ☆（＾～＾）
    /// 北北西隣 にしたかったら upside_down して、 phase.turn() しろだぜ☆（＾～＾）
    pub fn north_east_keima_of<F1>(
        upside_down: UpsideDown,
        phase: &Phase,
        start: &Square,
        callback: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        let rel = Squares::rotate180_as_relative(phase, Squares::upside_down(&upside_down, -10));
        assert_in_board_as_relative(rel, &format!("東隣nek☆（＾～＾） rel={}", rel));
        let mut next = start.address + rel;
        if !Squares::has_jumped_out_horizontally(&Counterclockwise::Origin, next) {
            assert_in_board_as_absolute(next, "東隣nek☆（＾～＾）");
            next += Squares::rotate180_as_relative(phase, Squares::upside_down(&upside_down, -1));
            if !Squares::has_jumped_out_horizontally(&Counterclockwise::Rotate90, next) {
                assert_in_board_as_absolute(next, "北nek東隣☆（＾～＾）");
                next +=
                    Squares::rotate180_as_relative(phase, Squares::upside_down(&upside_down, -1));
                if !Squares::has_jumped_out_horizontally(&Counterclockwise::Rotate90, next) {
                    assert_in_board_as_absolute(
                        next,
                        &format!("start=|{}| 北北東隣☆（＾～＾）", start.address),
                    );
                    callback(Square::from_address(next));
                }
            }
        }
    }
}
