//!
//! square は 将棋盤座標
//!
//! 91 81 71 ...
//! 92 82 72
//! 93 83 73
//!
use super::super::super::controller::common::conv;
use super::super::super::model::master::place::*;

pub struct Square {
    /// 行番号。いわゆる段。上から 1, 2, 3 ...
    rank: i8,
    /// 列番号。いわゆる筋。右から 1, 2, 3 ...
    file: i8,
}
impl Square {
    pub fn from_umasu(ms: umasu) -> Self {
        let (file1, rank1) = conv::ms_to_suji_dan(ms);
        Square {
            rank: rank1,
            file: file1,
        }
    }

    pub fn to_umasu(&self) -> umasu {
        conv::suji_dan_to_ms(self.file, self.rank)
    }
}
