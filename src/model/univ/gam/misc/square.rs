//! Square is shogi coordinate. file*10+rank.
//!
//!           North
//!   91 81 71 61 51 41 31 21 11
//!   92 82 72 62 52 42 32 22 12
//! W 93 83 73 63 53 43 33 23 13 E
//! E 94 84 74 64 54 44 34 24 14 A
//! S 95 85 75 65 55 45 35 25 15 S
//! T 96 86 76 66 56 46 36 26 16 T
//!   97 87 77 67 57 47 37 27 17
//!   98 88 78 68 58 48 38 28 18
//!   99 89 79 69 59 49 39 29 19
//!           Source
//!
//!
//!              North
//!   00 01 02 03 04 05 06 07 08 09
//!   10 11 12 13 14 15 16 17 18 19
//!   20 21 22 23 24 25 26 27 28 29
//! E 30 31 32 33 34 35 36 37 38 39 W
//! A 40 41 42 43 44 45 46 47 48 49 E
//! S 50 51 51 53 54 55 56 57 58 59 S
//! T 60 61 62 63 64 65 66 67 68 69 T
//!   70 71 72 73 74 75 76 77 78 79
//!   80 81 82 83 84 85 86 87 88 89
//!   90 91 92 93 94 95 96 97 98 99
//!              Source
//!
//! None is 0.
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::common_use::cu_conv_controller::*;
use crate::controller::common_use::cu_geo_teigi_controller::*;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

//
// 盤、升、筋、段
//

#[allow(non_camel_case_types)]
pub type isquare = i8;

/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MIN: isquare = 11;

/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MAX: isquare = 99;

// 盤のヨコ幅、タテ幅。
// pub const BOARD_WIDTH: i8 = 9;
// pub const BOARD_HEIGHT: i8 = 9;
// 正方形という前提☆（＾～＾）
pub const BOARD_DIAGONAL_LENGTH: isquare = 9;
// 枠も使う☆（＾～＾）
pub const BOARD_MEMORY_AREA: isquare = 111;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;

/// 筋、段は 1 から始まる、という明示。
/// 増減はよく使うので u8 ではなく i8 にした。
pub const FILE_0: i8 = 0;
pub const FILE_1: i8 = 1;
pub const FILE_9: i8 = 9;
pub const FILE_10: i8 = 10;
pub const RANK_0: i8 = 0;
pub const RANK_1: i8 = 1;
pub const RANK_2: i8 = 2;
pub const RANK_3: i8 = 3;
pub const RANK_4: i8 = 4;
pub const RANK_5: i8 = 5;
pub const RANK_6: i8 = 6;
pub const RANK_7: i8 = 7;
pub const RANK_8: i8 = 8; //うさぎの打てる段の上限
pub const RANK_9: i8 = 9;
pub const RANK_10: i8 = 10;

/*
pub const NORTH1: isquare = -1;
pub const SOUTH1: isquare = 1;
pub const EAST1: isquare = -1;
pub const WEST1: isquare = 1;
*/

/// 升の検索等で、該当なしの場合
pub const SQUARE_NONE: isquare = 0;

/// 指し手。打の場合のsrc
pub const SQUARE_DROP: isquare = 0;

/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    /// Square is shogi coordinate. file*10+rank.
    ///
    ///           North
    ///   91 81 71 61 51 41 31 21 11
    ///   92 82 72 62 52 42 32 22 12
    /// W 93 83 73 63 53 43 33 23 13 E
    /// E 94 84 74 64 54 44 34 24 14 A
    /// S 95 85 75 65 55 45 35 25 15 S
    /// T 96 86 76 66 56 46 36 26 16 T
    ///   97 87 77 67 57 47 37 27 17
    ///   98 88 78 68 58 48 38 28 18
    ///   99 89 79 69 59 49 39 29 19
    ///           Source
    pub address: isquare,
}
impl Square {
    pub fn from_address(square: isquare) -> Self {
        Square { address: square }
    }
    pub fn from_file_rank(file1: i8, rank1: i8) -> Self {
        Square {
            address: file1 * 10 + rank1,
        }
    }
    pub fn from_point(p: &Point) -> Self {
        debug_assert!(p_in_ban(&p), "(204b)from_point x={},y={}", p.x, p.y);

        Square::from_address((p.x * 10 + p.y) as isquare)
    }

    /// 列番号。いわゆる筋。右から 1, 2, 3 ...
    pub fn get_file(&self) -> i8 {
        self.address / 10
    }

    /// 行番号。いわゆる段。上から 1, 2, 3 ...
    pub fn get_rank(&self) -> i8 {
        self.address % 10
    }

    pub fn to_file_rank(&self) -> (i8, i8) {
        (self.get_file(), self.get_rank())
    }

    /// x, y に名称変更したもの☆（＾～＾）
    pub fn to_point(&self) -> Point {
        assert_in_board(self.address, "(203b)sq_to_p");
        Point {
            x: self.get_file(),
            y: self.get_rank(),
        }
    }
}
