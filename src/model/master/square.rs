//! square は 将棋盤座標
//!
//! 91 81 71 61 51 41 31 21 11
//! 92 82 72 62 52 42 32 22 12
//! 93 83 73 63 53 43 33 23 13
//! 94 84 74 64 54 44 34 24 14
//! 95 85 75 65 55 45 35 25 15
//! 96 86 76 66 56 46 36 26 16
//! 97 87 77 67 57 47 37 27 17
//! 98 88 78 68 58 48 38 28 18
//! 99 89 79 69 59 49 39 29 19
//!
//! こう並べてみると、縦に進んでいるな☆（＾～＾）
//! なお、後手番から見ると
//!
//! 19  29  39  49  59  69  79  89  99
//! 18  28  38  48  58  68  78  88  98
//! 17  27  37  47  57  67  77  87  97
//! 16  26  36  46  56  66  76  86  96
//! 15  25  35  45  55  65  75  85  95
//! 14  24  34  44  54  64  74  84  94
//! 13  23  33  43  53  63  73  83  93
//! 12  22  32  42  52  62  72  82  92
//! 11  21  31  41  51  61  71  81  91
//!
//! 第一象限と同じになるからオススメ☆（＾～＾）
use super::super::super::controller::common::conv::*;
use super::super::super::controller::consoles::asserts::*;
use super::super::super::controller::geometries::geo_teigi::*;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

//
// 盤、升、筋、段
//

/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MIN: usize = 11;

/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MAX: usize = 99;

// 盤のヨコ幅、タテ幅。
// 筋と段は x,y とは逆方向なので、幅も左端、下端を指す。
//pub const BAN_W :i8 = 9;
//pub const BAN_H :i8 = 9;
pub const BAN_SIZE: usize = 100;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;

/// 筋、段は 1 から始まる、という明示。
/// 増減はよく使うので u8 ではなく i8 にした。
pub const SUJI_0: i8 = 0;
pub const SUJI_1: i8 = 1;
pub const SUJI_9: i8 = 9;
pub const SUJI_10: i8 = 10;
pub const DAN_0: i8 = 0;
pub const DAN_1: i8 = 1;
pub const DAN_2: i8 = 2;
pub const DAN_3: i8 = 3;
pub const DAN_4: i8 = 4;
pub const DAN_5: i8 = 5;
pub const DAN_6: i8 = 6;
pub const DAN_7: i8 = 7;
pub const DAN_8: i8 = 8; //うさぎの打てる段の上限
pub const DAN_9: i8 = 9;
pub const DAN_10: i8 = 10;

/// 升番号 0～99。
/// 10の位を筋、1の位を段とする。0筋、0段は未使用（番兵として使用）
/// 該当なしの場合 0 を使う
#[allow(non_camel_case_types)]
pub type umasu = usize;

/// 升の検索等で、該当なしの場合
pub const MASU_0: umasu = 0;

/// 指し手。打の場合のsrc
pub const SS_SRC_DA: umasu = 0;

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

    pub fn from_point(p: &Point) -> Self {
        debug_assert!(p_in_ban(&p), "(204b)from_point x={},y={}", p.x, p.y);

        Square::from_umasu((p.x * 10 + p.y) as umasu)
    }

    /// x, y に名称変更したもの☆（＾～＾）
    pub fn to_point(&self) -> Point {
        assert_banjo_sq(&self, "(203b)sq_to_p");
        Point {
            x: self.file,
            y: self.rank,
        }
    }
}
