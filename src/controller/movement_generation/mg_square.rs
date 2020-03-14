use super::super::super::model::vo::game_part::gp_square_vo::Square;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use crate::model::vo::game_part::gp_phase_vo::Phase;

/// 駒が動ける升☆（＾～＾）
pub struct MGPieceSquares {}
impl MGPieceSquares {
    /// 盤上の歩から動けるマスを見ます。
    pub fn looking_for_square_from_1player_pawn_on_board<F1>(
        src_phase: &Phase,
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::north_of(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段に移動することはできません。
            callback_squares(dst_square)
        });
    }
    pub fn looking_for_square_from_2player_pawn_on_board<F1>(
        src_phase: &Phase,
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::south_of(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段に移動することはできません。
            callback_squares(dst_square)
        });
    }

    /// 盤上の香から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_lance_on_board<F1>(
        src_phase: &Phase,
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_north_from(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段に移動することはできません。
            callback_squares(dst_square)
        });
    }
    pub fn looking_for_squares_from_2player_lance_on_board<F1>(
        src_phase: &Phase,
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_south_from(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段に移動することはできません。
            callback_squares(dst_square)
        });
    }

    /// 盤上の桂から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_knight_on_board<F1>(
        src_phase: &Phase,
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::north_west_keima_of(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段、奥から２番目の段に移動することはできません。
            callback_squares(dst_square)
        });
        MGSquares::north_east_keima_of(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段、奥から２番目の段に移動することはできません。
            callback_squares(dst_square)
        });
    }
    pub fn looking_for_squares_from_2player_knight_on_board<F1>(
        src_phase: &Phase,
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::south_east_keima_of(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段、奥から２番目の段に移動することはできません。
            callback_squares(dst_square)
        });
        MGSquares::south_west_keima_of(src_square, &mut |dst_square| {
            // TODO 成らずに一番奥の段、奥から２番目の段に移動することはできません。
            callback_squares(dst_square)
        });
    }

    /// 盤上の銀から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_silver_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }
    pub fn looking_for_squares_from_2player_silver_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }

    /// 盤上の金、と、杏、圭、全から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_gold_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }
    pub fn looking_for_squares_from_2player_gold_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }

    /// 盤上の玉から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_king_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }
    pub fn looking_for_squares_from_2player_king_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }

    /// 盤上の角から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_bishop_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_north_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::looking_north_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::looking_south_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::looking_south_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
    }
    pub fn looking_for_squares_from_2player_bishop_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_south_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::looking_south_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::looking_north_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::looking_north_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
    }

    /// 盤上の飛から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_rook_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_north_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_west_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_east_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_south_from(src_square, &mut |dst_square| callback_squares(dst_square));
    }
    pub fn looking_for_squares_from_2player_rook_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_south_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_east_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_west_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_north_from(src_square, &mut |dst_square| callback_squares(dst_square));
    }

    /// 盤上の馬から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_horse_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_north_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_north_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_south_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_south_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
    }
    pub fn looking_for_squares_from_2player_horse_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::looking_south_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::south_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_south_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_north_east_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
        MGSquares::north_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_north_west_from(src_square, &mut |dst_square| {
            callback_squares(dst_square)
        });
    }

    /// 盤上の竜から動けるマスを見ます。
    pub fn looking_for_squares_from_1player_dragon_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_north_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_west_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_east_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_south_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }
    pub fn looking_for_squares_from_2player_dragon_on_board<F1>(
        src_square: &Square,
        callback_squares: &mut F1,
    ) where
        F1: FnMut(Square) -> bool,
    {
        MGSquares::south_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_south_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::south_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_east_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_west_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_east_of(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::looking_north_from(src_square, &mut |dst_square| callback_squares(dst_square));
        MGSquares::north_west_of(src_square, &mut |dst_square| callback_squares(dst_square));
    }
}

pub struct MGSquares {}
impl MGSquares {
    /// 全升☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank_src in 1..10 {
            for file_src in (1..10).rev() {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }
    /// 東隣の升から東へ☆（＾～＾）
    pub fn looking_east_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start_square.file - 1;
        loop {
            if FILE_0 < i_file {
                if callback(Square::from_file_rank(i_file, start_square.rank)) {
                    break;
                }
            } else {
                break;
            }
            i_file -= 1;
        }
    }

    /// 北隣の升から北へ☆（＾～＾）
    pub fn looking_north_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_rank = start_square.rank - 1;
        loop {
            if RANK_0 < i_rank {
                if callback(Square::from_file_rank(start_square.file, i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_rank -= 1;
        }
    }

    /// 北東隣の升から北東へ☆（＾～＾）
    pub fn looking_north_east_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start_square.file - 1;
        let mut i_rank = start_square.rank - 1;
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
    pub fn looking_north_west_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start_square.file + 1;
        let mut i_rank = start_square.rank - 1;
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
    pub fn looking_south_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_rank = start_square.rank + 1;
        loop {
            if i_rank < RANK_10 {
                if callback(Square::from_file_rank(start_square.file, i_rank)) {
                    break;
                }
            } else {
                break;
            }
            i_rank += 1;
        }
    }

    /// 南東隣の升から南東へ☆（＾～＾）
    pub fn looking_south_east_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start_square.file - 1;
        let mut i_rank = start_square.rank + 1;
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
    pub fn looking_south_west_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start_square.file + 1;
        let mut i_rank = start_square.rank + 1;
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
    pub fn looking_west_from<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        let mut i_file = start_square.file + 1;
        loop {
            if i_file < FILE_10 {
                if callback(Square::from_file_rank(i_file, start_square.rank)) {
                    break;
                }
            } else {
                break;
            }
            i_file += 1;
        }
    }

    /// 東隣☆（＾～＾）
    pub fn east_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if FILE_0 < start_square.file - 1 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank,
            ));
        }
    }

    /// 北隣☆（＾～＾）
    pub fn north_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if RANK_0 < start_square.rank - 1 {
            callback(Square::from_file_rank(
                start_square.file,
                start_square.rank - 1,
            ));
        }
    }
    /// 北東隣☆（＾～＾）
    pub fn north_east_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if FILE_0 < start_square.file - 1 && RANK_0 < start_square.rank - 1 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank - 1,
            ));
        }
    }

    /// 北北東隣☆（＾～＾）
    pub fn north_east_keima_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if FILE_0 < start_square.file - 1 && RANK_0 < start_square.rank - 2 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank - 2,
            ));
        }
    }

    /// 北北西隣☆（＾～＾）
    pub fn north_west_keima_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if start_square.file + 1 < FILE_10 && RANK_0 < start_square.rank - 2 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank - 2,
            ));
        }
    }

    /// 北西隣☆（＾～＾）
    pub fn north_west_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if start_square.file + 1 < FILE_10 && RANK_0 < start_square.rank - 1 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank - 1,
            ));
        }
    }

    /// 南隣☆（＾～＾）
    pub fn south_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if start_square.rank + 1 < RANK_10 {
            callback(Square::from_file_rank(
                start_square.file,
                start_square.rank + 1,
            ));
        }
    }

    /// 南東隣☆（＾～＾）
    pub fn south_east_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if FILE_0 < start_square.file - 1 && start_square.rank + 1 < RANK_10 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank + 1,
            ));
        }
    }

    /// 南南東隣☆（＾～＾）
    pub fn south_east_keima_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if FILE_0 < start_square.file - 1 && start_square.rank + 2 < RANK_10 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank + 2,
            ));
        }
    }
    /// 南南西隣☆（＾～＾）
    pub fn south_west_keima_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if start_square.file + 1 < FILE_10 && start_square.rank + 2 < RANK_10 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank + 2,
            ));
        }
    }

    /// 南西隣☆（＾～＾）
    pub fn south_west_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if start_square.file + 1 < FILE_10 && start_square.rank + 1 < RANK_10 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank + 1,
            ));
        }
    }

    /// 西☆（＾～＾）
    pub fn west_of<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        if start_square.file + 1 < RANK_10 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank,
            ));
        }
    }
}
