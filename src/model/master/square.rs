//!
//! square は 将棋盤座標
//!
//! 91 81 71 ...
//! 92 82 72
//! 93 83 73
//!
use super::super::super::model::master::place::*;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Hash)]
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
    pub fn from_file_rank(file1: i8, rank1: i8) -> Self {
        Square {
            rank: rank1,
            file: file1,
        }
    }
    pub fn to_umasu(&self) -> umasu {
        (self.file * 10 + self.rank) as umasu
    }

    pub fn to_file_rank(&self) -> (i8, i8) {
        (self.file, self.rank)
    }
}
