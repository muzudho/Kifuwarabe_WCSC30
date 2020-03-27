use crate::controller::common_use::cu_asserts_controller::assert_in_board;
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
        Squares::north_of(friend, source, &mut |dst_square| {
            Promoting::case_of_pawn_lance(friend, &dst_square, callback_next)
        });
    }

    /// 盤上の香から動けるマスを見ます。
    pub fn looking_for_squares_from_lance_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_north_from(friend, source, &mut |dst_square| {
            Promoting::case_of_pawn_lance(friend, &dst_square, callback_next)
        });
    }

    /// 盤上の桂から動けるマスを見ます。
    pub fn looking_for_squares_from_knight_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_east_keima_of(UpsideDown::Origin, friend, source, &mut |dst_square| {
            Promoting::case_of_knight(friend, &dst_square, callback_next)
        });
        Squares::north_east_keima_of(
            UpsideDown::Flip,
            &friend.turn(),
            source,
            &mut |dst_square| Promoting::case_of_knight(friend, &dst_square, callback_next),
        );
    }

    /// 盤上の銀から動けるマスを見ます。
    pub fn looking_for_squares_from_silver_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_of(friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
        Squares::north_east_of(UpsideDown::Origin, friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
        Squares::north_east_of(
            UpsideDown::Flip,
            &friend.turn(),
            source,
            &mut |dst_square| {
                Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
            },
        );
        Squares::north_east_of(
            UpsideDown::Origin,
            &friend.turn(),
            source,
            &mut |dst_square| {
                Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
            },
        );
        Squares::north_east_of(UpsideDown::Flip, friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
    }

    /// 盤上の金、と、杏、圭、全から動けるマスを見ます。
    pub fn looking_for_squares_from_gold_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(UpsideDown::Origin, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(
            UpsideDown::Flip,
            &friend.turn(),
            source,
            &mut |destination| callback_next(destination, Promotability::Deny),
        );
        Squares::east_of(Mirror::Both, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(&friend.turn(), source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }

    /// 盤上の玉から動けるマスを見ます。
    pub fn looking_for_squares_from_king_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_east_of(
            UpsideDown::Flip,
            &friend.turn(),
            source,
            &mut |destination| callback_next(destination, Promotability::Deny),
        );
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(UpsideDown::Origin, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(Mirror::Both, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(
            UpsideDown::Origin,
            &friend.turn(),
            source,
            &mut |destination| callback_next(destination, Promotability::Deny),
        );
        Squares::north_of(&friend.turn(), source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(UpsideDown::Flip, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }

    /// 盤上の角から動けるマスを見ます。
    pub fn looking_for_squares_from_bishop_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_south_east_from(&friend.turn(), source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_north_east_from(friend, source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_north_east_from(&friend.turn(), source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_south_east_from(friend, source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
    }

    /// 盤上の飛から動けるマスを見ます。
    pub fn looking_for_squares_from_rook_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_north_from(friend, source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_east_from(&friend.turn(), source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_east_from(friend, source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_south_from(friend, source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
    }

    /// 盤上の馬から動けるマスを見ます。
    pub fn looking_for_squares_from_horse_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_south_east_from(&friend.turn(), source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |dst_square| {
            callback_next(dst_square, Promotability::Deny)
        });
        Squares::looking_north_east_from(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(Mirror::Both, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_north_east_from(&friend.turn(), source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(&friend.turn(), source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_south_east_from(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }

    /// 盤上の竜から動けるマスを見ます。
    pub fn looking_for_squares_from_dragon_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_east_of(
            UpsideDown::Flip,
            &friend.turn(),
            source,
            &mut |destination| callback_next(destination, Promotability::Deny),
        );
        Squares::looking_north_from(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(UpsideDown::Origin, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_east_from(&friend.turn(), source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_east_from(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(
            UpsideDown::Origin,
            &friend.turn(),
            source,
            &mut |destination| callback_next(destination, Promotability::Deny),
        );
        Squares::looking_south_from(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(UpsideDown::Flip, friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
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

pub struct Squares {}
impl Squares {
    fn rotate(phase: &Phase, square: isquare) -> isquare {
        if *phase == Phase::Second {
            -square
        } else {
            square
        }
    }

    fn upside_down(upside_down: &UpsideDown, address: isquare) -> isquare {
        match upside_down {
            UpsideDown::Flip => address / 10 * 10 - (10 - (address % 10)) + 10,
            UpsideDown::Origin => address,
        }
    }

    /*
    fn downside_up(upside_down: &UpsideDown, address: isquare) -> isquare {
        match upside_down {
            UpsideDown::Flip => address / 10 * 10 - (10 - (address % 10)) - 10,
            UpsideDown::Origin => address,
        }
    }
    */

    fn has_jumped_out_horizontally(address: i8) -> bool {
        address / 10 % 10 == 0
    }

    fn has_jumped_out_vertically(address: i8) -> bool {
        address % 10 == 0
    }

    fn has_jumped_out_of_the_board(address: i8) -> bool {
        address / 10 % 10 == 0 || address % 10 == 0
    }

    /// 2段目～9段目 全升☆（＾～＾）
    pub fn for_from_rank2_to_rank9<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank_src in RANK_2..RANK_10 {
            for file_src in (FILE_1..FILE_10).rev() {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }

    /// 3段目～9段目 全升☆（＾～＾）
    pub fn for_from_rank3_to_rank9<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank_src in RANK_3..RANK_10 {
            for file_src in (FILE_1..FILE_10).rev() {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }

    /// 1段目～8段目 全升☆（＾～＾）
    pub fn for_from_rank1_to_rank8<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank_src in RANK_1..RANK_9 {
            for file_src in (FILE_1..FILE_10).rev() {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }

    /// 1段目～7段目 全升☆（＾～＾）
    pub fn for_from_rank1_to_rank7<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank_src in RANK_1..RANK_8 {
            for file_src in (FILE_1..FILE_10).rev() {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }
    /// 東隣の升から東へ☆（＾～＾）
    /// 西隣の升から西へ にしたかったら phase.turn() しろだぜ☆（＾～＾）
    pub fn looking_east_from<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate(phase, -10);
            if Squares::has_jumped_out_horizontally(next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }

    /// 北隣の升から北へ☆（＾～＾）
    pub fn looking_north_from<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate(phase, -1);
            if Squares::has_jumped_out_vertically(next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }
    /// 南隣の升から南へ☆（＾～＾）
    pub fn looking_south_from<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate(phase, 1);
            if Squares::has_jumped_out_vertically(next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }

    /// 北東隣の升から北東へ☆（＾～＾）
    /// 南東隣 にしたかったら phase.turn() しろだぜ☆（＾～＾）
    pub fn looking_north_east_from<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate(phase, -11);
            if Squares::has_jumped_out_of_the_board(next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }

    /// 南東隣の升から南東へ☆（＾～＾）
    /// 北西隣 にしたかったら phase.turn() しろだぜ☆（＾～＾）
    pub fn looking_south_east_from<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address;
        loop {
            next += Squares::rotate(phase, -9);
            if Squares::has_jumped_out_of_the_board(next) {
                break;
            } else if callback(Square::from_address(next)) {
                break;
            }
        }
    }

    /// 北隣☆（＾～＾）
    /// 南隣 にしたかったら phase.turn() しろだぜ☆（＾～＾）
    pub fn north_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, -1);
        if !Squares::has_jumped_out_vertically(next) {
            assert_in_board(next, "北隣☆（＾～＾）");
            callback(Square::from_address(next));
        }
    }
    /*
    /// 南隣☆（＾～＾）
    pub fn south_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, 1);
        if !Squares::has_jumped_out_vertically(next) {
            assert_in_board(next, "南隣☆（＾～＾）");
            callback(Square::from_address(next));
        }
    }
    */

    /// 東隣☆（＾～＾） 西隣もついでに作れだぜ☆（＾～＾）
    pub fn east_of<F1>(mirror: Mirror, phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        match mirror {
            Mirror::Origin | Mirror::Both => {
                // 東隣☆（＾～＾）
                let next = start.address + Squares::rotate(phase, -10);
                if !Squares::has_jumped_out_horizontally(next) {
                    assert_in_board(next, "東隣☆（＾～＾）");
                    callback(Square::from_address(next));
                }
            }
            _ => {}
        };

        match mirror {
            Mirror::Mirror | Mirror::Both => {
                // 西隣☆（＾～＾）
                let next = start.address + Squares::rotate(phase, 10);
                if !Squares::has_jumped_out_horizontally(next) {
                    assert_in_board(next, "西隣w☆（＾～＾）");
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
        let next = start.address + Squares::rotate(phase, Squares::upside_down(&upside_down, -11));
        if !Squares::has_jumped_out_of_the_board(next) {
            assert_in_board(next, "北東隣☆（＾～＾）");
            callback(Square::from_address(next));
        }
    }
    /*
    /// 南東隣☆（＾～＾）
    /// 北西隣 にしたかったら phase.turn() しろだぜ☆（＾～＾）
    pub fn south_east_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, -9);
        if !Squares::has_jumped_out_of_the_board(next) {
            assert_in_board(next, "南東隣☆（＾～＾）");
            callback(Square::from_address(next));
        }
    }
    */

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
        let mut next =
            start.address + Squares::rotate(phase, Squares::upside_down(&upside_down, -10));
        if !Squares::has_jumped_out_horizontally(next) {
            assert_in_board(next, "東隣☆（＾～＾）");
            next += Squares::rotate(phase, Squares::upside_down(&upside_down, -1));
            if !Squares::has_jumped_out_vertically(next) {
                assert_in_board(next, "北東隣☆（＾～＾）");
                next += Squares::rotate(phase, Squares::upside_down(&upside_down, -1));
                if !Squares::has_jumped_out_vertically(next) {
                    assert_in_board(
                        next,
                        &format!("start=|{}| 北北東隣☆（＾～＾）", start.address),
                    );
                    callback(Square::from_address(next));
                }
            }
        }
    }
}
