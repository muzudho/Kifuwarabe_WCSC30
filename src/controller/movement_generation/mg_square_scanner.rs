use super::super::super::model::vo::game_part::gp_square_vo::Square;
use super::super::super::model::vo::game_part::gp_square_vo::*;

pub struct SquareScanner {}
impl SquareScanner {
    pub fn for_each_east<F1>(start_square: Square, callback: &mut F1)
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
}
