//!
//! square は 将棋盤座標
//!
//! 91 81 71 ...
//! 92 82 72
//! 93 83 73
//!
use super::super::super::controller::common::conv;
use super::super::super::model::master::place::*;

#[derive(Clone)]
pub struct Square {
    /// 行番号。いわゆる段。上から 1, 2, 3 ...
    rank: i8,
    /// 列番号。いわゆる筋。右から 1, 2, 3 ...
    file: i8,
}
impl Square {
    pub fn from_umasu(ms: umasu) -> Self {
        Square {
            rank: (ms % 10) as i8,
            file: (ms / 10) as i8,
        }
    }
    pub fn from_file_rank(file: i8, rank: i8) -> Self {
        Square::from_umasu(conv::suji_dan_to_ms(file, rank))
    }

    pub fn to_umasu(&self) -> umasu {
        conv::suji_dan_to_ms(self.file, self.rank)
    }

    pub fn to_file_rank(&self) -> (i8, i8) {
        (self.file, self.rank)
    }
}
