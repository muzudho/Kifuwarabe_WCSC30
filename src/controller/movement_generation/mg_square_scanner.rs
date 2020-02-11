use super::super::super::model::vo::game_part::gp_square_vo::Square;
use super::super::super::model::vo::game_part::gp_square_vo::*;

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

    /// 北東隣の升から南西へ☆（＾～＾）
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
}
