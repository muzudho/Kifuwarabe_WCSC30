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
    pub fn looking_for_squares_from_1player_lance_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_north_from(source, &mut |dst_square| {
            Promoting::case_of_pawn_lance(friend, &dst_square, callback_next)
        });
    }
    pub fn looking_for_squares_from_2player_lance_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_south_from(source, &mut |dst_square| {
            Promoting::case_of_pawn_lance(friend, &dst_square, callback_next)
        });
    }

    /// 盤上の桂から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_knight_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_keima_of(source, &mut |dst_square| {
            Promoting::case_of_knight(friend, &dst_square, callback_next)
        });
        Squares::north_east_keima_of(source, &mut |dst_square| {
            Promoting::case_of_knight(friend, &dst_square, callback_next)
        });
    }
    pub fn looking_for_squares_from_2player_knight_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::south_east_keima_of(source, &mut |dst_square| {
            Promoting::case_of_knight(friend, &dst_square, callback_next)
        });
        Squares::south_west_keima_of(source, &mut |dst_square| {
            Promoting::case_of_knight(friend, &dst_square, callback_next)
        });
    }

    /// 盤上の銀から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_silver_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
        Squares::north_of(friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
        Squares::north_east_of(friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
        Squares::south_west_of(friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
        Squares::south_east_of(friend, source, &mut |dst_square| {
            Promoting::case_of_silver(friend, &source, &dst_square, callback_next)
        });
    }
    pub fn looking_for_squares_from_2player_silver_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        });
        Squares::north_of(friend, source, &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        });
        Squares::south_west_of(friend, source, &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        });
        Squares::south_east_of(friend, source, &mut |destination| {
            Promoting::case_of_silver(friend, &source, &destination, callback_next)
        });
    }

    /// 盤上の金、と、杏、圭、全から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_gold_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }
    pub fn looking_for_squares_from_2player_gold_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }

    /// 盤上の玉から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_king_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }
    pub fn looking_for_squares_from_2player_king_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }

    /// 盤上の角から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_bishop_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_north_west_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_north_east_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_south_west_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_south_east_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
    }
    pub fn looking_for_squares_from_2player_bishop_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_south_east_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_south_west_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_north_east_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_north_west_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
    }

    /// 盤上の飛から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_rook_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_north_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_west_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_east_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_south_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
    }
    pub fn looking_for_squares_from_2player_rook_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_south_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_east_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_west_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
        Squares::looking_north_from(source, &mut |destination| {
            Promoting::case_of_bishop_rook(friend, &source, &destination, callback_next)
        });
    }

    /// 盤上の馬から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_horse_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_north_west_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |dst_square| {
            callback_next(dst_square, Promotability::Deny)
        });
        Squares::looking_north_east_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_south_west_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_south_east_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }
    pub fn looking_for_squares_from_2player_horse_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::looking_south_east_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_south_west_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_north_east_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_north_west_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }

    /// 盤上の竜から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_dragon_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_north_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_west_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_east_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_south_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
    }
    pub fn looking_for_squares_from_2player_dragon_on_board<F1>(
        friend: &Phase,
        source: &Square,
        callback_next: &mut F1,
    ) where
        F1: FnMut(Square, Promotability) -> bool,
    {
        Squares::north_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_south_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::north_east_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_east_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_west_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_west_of(friend, source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::looking_north_from(source, &mut |destination| {
            callback_next(destination, Promotability::Deny)
        });
        Squares::south_east_of(friend, source, &mut |destination| {
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

pub struct Squares {}
impl Squares {
    fn rotate(phase: &Phase, square: isquare) -> isquare {
        if *phase == Phase::Second {
            -square
        } else {
            square
        }
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
    pub fn looking_east_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start.get_file() - 1;
        loop {
            if FILE_0 < i_file {
                if callback(Square::from_file_rank(i_file, start.get_rank())) {
                    break;
                }
            } else {
                break;
            }
            i_file -= 1;
        }
    }

    /// 北隣の升から北へ☆（＾～＾）
    pub fn looking_north_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_rank = start.get_rank() - 1;
        loop {
            if RANK_0 < i_rank {
                if callback(Square::from_file_rank(start.get_file(), i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_rank -= 1;
        }
    }

    /// 北東隣の升から北東へ☆（＾～＾）
    pub fn looking_north_east_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start.get_file() - 1;
        let mut i_rank = start.get_rank() - 1;
        loop {
            if FILE_0 < i_file && RANK_0 < i_rank {
                if callback(Square::from_file_rank(i_file, i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_file -= 1;
            i_rank -= 1;
        }
    }

    /// 北西隣の升から北西へ☆（＾～＾）
    pub fn looking_north_west_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start.get_file() + 1;
        let mut i_rank = start.get_rank() - 1;
        loop {
            if i_file < FILE_10 && RANK_0 < i_rank {
                if callback(Square::from_file_rank(i_file, i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_file += 1;
            i_rank -= 1;
        }
    }

    /// 南隣の升から南へ☆（＾～＾）
    pub fn looking_south_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_rank = start.get_rank() + 1;
        loop {
            if i_rank < RANK_10 {
                if callback(Square::from_file_rank(start.get_file(), i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_rank += 1;
        }
    }

    /// 南東隣の升から南東へ☆（＾～＾）
    pub fn looking_south_east_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start.get_file() - 1;
        let mut i_rank = start.get_rank() + 1;
        loop {
            if FILE_0 < i_file && i_rank < RANK_10 {
                if callback(Square::from_file_rank(i_file, i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_file -= 1;
            i_rank += 1;
        }
    }
    /// 南西隣の升から南西へ☆（＾～＾）
    pub fn looking_south_west_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start.get_file() + 1;
        let mut i_rank = start.get_rank() + 1;
        loop {
            if i_file < RANK_10 && i_rank < RANK_10 {
                if callback(Square::from_file_rank(i_file, i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_file += 1;
            i_rank += 1;
        }
    }

    /// 西隣の升から西へ☆（＾～＾）
    pub fn looking_west_from<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start.get_file() + 1;
        loop {
            if i_file < FILE_10 {
                if callback(Square::from_file_rank(i_file, start.get_rank())) {
                    break;
                }
            } else {
                break;
            }
            i_file += 1;
        }
    }

    /// 北隣☆（＾～＾）
    pub fn north_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, -1);
        if next % 10 != 0 {
            assert_in_board(next, "北隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }
    /// 南隣☆（＾～＾）
    pub fn south_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, 1);
        if next % 10 != 0 {
            assert_in_board(next, "南隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }

    /// 東隣☆（＾～＾）
    pub fn east_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, -10);
        if next / 10 % 10 != 0 {
            assert_in_board(next, "東隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }

    /// 西隣☆（＾～＾）
    pub fn west_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, 10);
        if next / 10 % 10 != 0 {
            assert_in_board(next, "西隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }

    /// 北東隣☆（＾～＾）
    pub fn north_east_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, -11);
        if next % 10 != 0 && next / 10 % 10 != 0 {
            assert_in_board(next, "北東隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }
    /// 南西隣☆（＾～＾）
    pub fn south_west_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, 11);
        if next % 10 != 0 && next / 10 % 10 != 0 {
            assert_in_board(next, "南西隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }

    /// 南東隣☆（＾～＾）
    pub fn south_east_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, -9);
        if next % 10 != 0 && next / 10 % 10 != 0 {
            assert_in_board(next, "南東隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }
    /// 北西隣☆（＾～＾）
    pub fn north_west_of<F1>(phase: &Phase, start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let next = start.address + Squares::rotate(phase, 9);
        if next % 10 != 0 && next / 10 % 10 != 0 {
            assert_in_board(next, "北西隣☆（＾～＾）");
            callback(Square::from_isquare(next));
        }
    }

    /// 北北東隣☆（＾～＾）
    /// スタート地点は、行き先の有る駒　である前提だぜ☆（＾～＾）
    pub fn north_east_keima_of<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address - 10;
        if next / 10 % 10 != 0 {
            assert_in_board(next, "東隣☆（＾～＾）");
            next -= 1;
            if next % 10 != 0 {
                assert_in_board(next, "北東隣☆（＾～＾）");
                next -= 1;
                if next % 10 != 0 {
                    assert_in_board(
                        next,
                        &format!("start=|{}| 北北東隣☆（＾～＾）", start.address),
                    );
                    callback(Square::from_isquare(next));
                }
            }
        }
    }

    /// 北北西隣☆（＾～＾）
    /// スタート地点は、行き先の有る駒　である前提だぜ☆（＾～＾）
    pub fn north_west_keima_of<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address + 10;
        if next / 10 % 10 != 0 {
            assert_in_board(next, "西隣☆（＾～＾）");
            next -= 1;
            if next % 10 != 0 {
                assert_in_board(next, "北西隣☆（＾～＾）");
                next -= 1;
                if next % 10 != 0 {
                    assert_in_board(next, "北北西隣☆（＾～＾）");
                    callback(Square::from_isquare(next));
                }
            }
        }
    }

    /// 南南東隣☆（＾～＾）
    /// スタート地点は、行き先の有る駒　である前提だぜ☆（＾～＾）
    pub fn south_east_keima_of<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address - 10;
        if next / 10 % 10 != 0 {
            assert_in_board(next, "東隣☆（＾～＾）");
            next += 1;
            if next % 10 != 0 {
                assert_in_board(next, "南東隣☆（＾～＾）");
                next += 1;
                if next % 10 != 0 {
                    assert_in_board(next, "南南東隣☆（＾～＾）");
                    callback(Square::from_isquare(next));
                }
            }
        }
    }
    /// 南南西隣☆（＾～＾）
    /// スタート地点は、行き先の有る駒　である前提だぜ☆（＾～＾）
    pub fn south_west_keima_of<F1>(start: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut next = start.address + 10;
        if next / 10 % 10 != 0 {
            assert_in_board(next, "西隣☆（＾～＾）");
            next += 1;
            if next % 10 != 0 {
                assert_in_board(next, "南西隣☆（＾～＾）");
                next += 1;
                if next % 10 != 0 {
                    assert_in_board(next, "南南西隣☆（＾～＾）");
                    callback(Square::from_isquare(next));
                }
            }
        }
    }
}
