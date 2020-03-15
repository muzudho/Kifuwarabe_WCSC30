use super::super::super::model::vo::game_part::gp_square_vo::Square;

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
}
