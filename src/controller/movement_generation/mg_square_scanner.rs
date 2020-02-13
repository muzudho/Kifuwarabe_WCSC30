use super::super::super::model::vo::game_part::gp_square_vo::Square;
use super::super::super::model::vo::game_part::gp_square_vo::*;

pub struct GPSquares {}
impl GPSquares {
    /// 全升☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        // (段)
        for rank_src in 1..10 {
            // (筋)
            for file_src in 1..10 {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }
}

pub struct SquareScanner {}
impl SquareScanner {
    /// 東隣の升から東へ☆（＾～＾）
    pub fn for_each_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_east in 1..9 {
            if start_square.file + i_east < SUJI_10 {
                if callback(Square::from_file_rank(
                    start_square.file + i_east,
                    start_square.rank,
                )) {
                    break;
                }
            }
        }
    }

    /// 北隣の升から北へ☆（＾～＾）
    pub fn for_each_north<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_north in 1..9 {
            if start_square.rank + i_north < DAN_10 {
                if callback(Square::from_file_rank(
                    start_square.file,
                    start_square.rank + i_north,
                )) {
                    break;
                }
            }
        }
    }

    /// 北東隣の升から北東へ☆（＾～＾）
    pub fn for_each_north_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_ne in 1..9 {
            if start_square.file + i_ne < SUJI_10 && start_square.rank + i_ne < DAN_10 {
                if callback(Square::from_file_rank(
                    start_square.file + i_ne,
                    start_square.rank + i_ne,
                )) {
                    break;
                }
            }
        }
    }

    /// 北西隣の升から北西へ☆（＾～＾）
    pub fn for_each_north_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_nw in 1..9 {
            if SUJI_0 < start_square.file - i_nw && start_square.rank + i_nw < DAN_10 {
                if callback(Square::from_file_rank(
                    start_square.file - i_nw,
                    start_square.rank + i_nw,
                )) {
                    break;
                }
            }
        }
    }

    /// 南隣の升から南へ☆（＾～＾）
    pub fn for_each_south<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_north in 1..9 {
            if DAN_0 < start_square.rank - i_north {
                if callback(Square::from_file_rank(
                    start_square.file,
                    start_square.rank - i_north,
                )) {
                    break;
                }
            }
        }
    }

    /// 南東隣の升から南東へ☆（＾～＾）
    pub fn for_each_south_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_se in 1..9 {
            if start_square.file + i_se < SUJI_10 && DAN_0 < start_square.rank - i_se {
                if callback(Square::from_file_rank(
                    start_square.file + i_se,
                    start_square.rank - i_se,
                )) {
                    break;
                }
            }
        }
    }
    /// 南西隣の升から南西へ☆（＾～＾）
    pub fn for_each_south_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_sw in 1..9 {
            if SUJI_0 < start_square.file - i_sw && DAN_0 < start_square.rank - i_sw {
                if callback(Square::from_file_rank(
                    start_square.file - i_sw,
                    start_square.rank - i_sw,
                )) {
                    break;
                }
            }
        }
    }

    /// 西隣の升から西へ☆（＾～＾）
    pub fn for_each_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square) -> bool,
    {
        for i_west in 1..9 {
            if SUJI_0 < start_square.file - i_west {
                if callback(Square::from_file_rank(
                    start_square.file - i_west,
                    start_square.rank,
                )) {
                    break;
                }
            }
        }
    }

    /// 東隣☆（＾～＾）
    pub fn next_to_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.file + 1 < SUJI_10 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank,
            ));
        }
    }

    /// 北隣☆（＾～＾）
    pub fn next_to_north<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.rank + 1 < DAN_10 {
            callback(Square::from_file_rank(
                start_square.file,
                start_square.rank + 1,
            ));
        }
    }
    /// 北東隣☆（＾～＾）
    pub fn next_to_north_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.file + 1 < SUJI_10 && start_square.rank + 1 < DAN_10 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank + 1,
            ));
        }
    }

    /// 北北東隣☆（＾～＾）
    pub fn next_to_north_north_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.file + 1 < SUJI_10 && start_square.rank + 2 < DAN_10 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank + 2,
            ));
        }
    }

    /// 北北西隣☆（＾～＾）
    pub fn next_to_north_north_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if SUJI_0 < start_square.file - 1 && start_square.rank + 2 < DAN_10 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank + 2,
            ));
        }
    }

    /// 北西隣☆（＾～＾）
    pub fn next_to_north_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.file - 1 > SUJI_0 && DAN_10 > start_square.rank + 1 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank + 1,
            ));
        }
    }

    /// 南隣☆（＾～＾）
    pub fn next_to_south<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if DAN_0 < start_square.rank - 1 {
            callback(Square::from_file_rank(
                start_square.file,
                start_square.rank - 1,
            ));
        }
    }

    /// 南東隣☆（＾～＾）
    pub fn next_to_south_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.file + 1 < SUJI_10 && DAN_0 < start_square.rank - 1 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank - 1,
            ));
        }
    }

    /// 南南東隣☆（＾～＾）
    pub fn next_to_south_south_east<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if start_square.file + 1 < SUJI_10 && DAN_0 < start_square.rank - 2 {
            callback(Square::from_file_rank(
                start_square.file + 1,
                start_square.rank - 2,
            ));
        }
    }
    /// 南南西隣☆（＾～＾）
    pub fn next_to_south_south_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if SUJI_0 < start_square.file - 1 && DAN_0 < start_square.rank - 2 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank - 2,
            ));
        }
    }

    /// 南西隣☆（＾～＾）
    pub fn next_to_south_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if SUJI_0 < start_square.file - 1 && DAN_0 < start_square.rank - 1 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank - 1,
            ));
        }
    }

    /// 西☆（＾～＾）
    pub fn next_to_west<F1>(start_square: &Square, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        if SUJI_0 < start_square.file - 1 {
            callback(Square::from_file_rank(
                start_square.file - 1,
                start_square.rank,
            ));
        }
    }
}
