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
use crate::law::diagnostic::cu_asserts_controller::*;
use std::cmp::max;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt;
use std::hash::Hash;

fn test_dort(test_name: &str, expected: &str, actual: &DictOrthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_d45ort(test_name: &str, expected: &str, actual: &Degree45Orthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_rsq(test_name: &str, expected: &str, actual: &RelativeSquare) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}

pub fn test_rotation() {
    // 辞書象限のテスト
    {
        let mut ort = DictOrthant::from_file_and_rank(0, -1);
        test_dort("e1", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, -1);
        test_dort("e2", "IV", &ort);
        ort = DictOrthant::from_file_and_rank(1, 0);
        test_dort("e3", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, 1);
        test_dort("e4", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(0, 1);
        test_dort("e5", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 1);
        test_dort("e6", "II", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 0);
        test_dort("e7", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, -1);
        test_dort("e8", "IOrIII", &ort);
    }
    // 45°回転象限のテスト
    {
        let mut ort = Degree45Orthant::from_file_and_rank(0, -1);
        test_d45ort("f1", "CoIIIOrCoIV", &ort);
        ort = Degree45Orthant::from_file_and_rank(1, -1);
        test_d45ort("f2", "IVOrI", &ort);
        ort = Degree45Orthant::from_file_and_rank(1, 0);
        test_d45ort("f3", "IVOrI", &ort);
        ort = Degree45Orthant::from_file_and_rank(1, 1);
        test_d45ort("f4", "IVOrI", &ort);
        ort = Degree45Orthant::from_file_and_rank(0, 1);
        test_d45ort("f5", "CoIOrCoII", &ort);
        ort = Degree45Orthant::from_file_and_rank(-1, 1);
        test_d45ort("f6", "IIOrIII", &ort);
        ort = Degree45Orthant::from_file_and_rank(-1, 0);
        test_d45ort("f7", "IIOrIII", &ort);
        ort = Degree45Orthant::from_file_and_rank(-1, -1);
        test_d45ort("f8", "IIOrIII", &ort);
    }
    // 相対番地のテスト
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("b1", "(0x -1y -1adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(1, -1);
        test_rsq("b2", "(1x -1y 9adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(1, 0);
        test_rsq("b3", "(1x 0y 10adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(1, 1);
        test_rsq("b4", "(1x 1y 11adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("b5", "(0x 1y 1adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(-1, 1);
        test_rsq("b6", "(-1x 1y -9adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(-1, 0);
        test_rsq("b7", "(-1x 0y -10adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(-1, -1);
        test_rsq("b8", "(-1x -1y -11adr)", &rsq);
    }
    // 45°回転のテスト
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("a1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a2", "(1x -1y 9adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a3", "(1x 0y 10adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a4", "(1x 1y 11adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a5", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a6", "(-1x 1y -9adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a7", "(-1x 0y -10adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a8", "(-1x -1y -11adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a9", "(0x -1y -1adr)", &rsq);
    }
    // 90°回転のテスト＜その１＞
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("c1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c2", "(1x 0y 10adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c3", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c4", "(-1x 0y -10adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c5", "(0x -1y -1adr)", &rsq);
    }
    // 90°回転のテスト＜その２＞
    {
        let mut rsq = RelativeSquare::from_file_and_rank(1, -1);
        test_rsq("d1", "(1x -1y 9adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d2", "(1x 1y 11adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d3", "(-1x 1y -9adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d4", "(-1x -1y -11adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d5", "(1x -1y 9adr)", &rsq);
    }
    // 桂馬のテスト
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("g1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate(Angle::Ccw45);
        test_rsq("g2", "(1x -1y 9adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g3", "(1x -2y 8adr)", &rsq);

        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("g4", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate(Angle::Ccw315);
        test_rsq("g5", "(-1x -1y -11adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g6", "(-1x -2y -12adr)", &rsq);

        let mut rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("g7", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate(Angle::Ccw45);
        test_rsq("g8", "(-1x 1y -9adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g9", "(-1x 2y -8adr)", &rsq);

        let mut rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("g10", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate(Angle::Ccw315);
        test_rsq("g11", "(1x 1y 11adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g12", "(1x 2y 12adr)", &rsq);
    }
    // 角度指定回転のテスト(北から)
    {
        // 0
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("h1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x -1y -1adr)", &rsq);

        // 45
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw45);
        test_rsq("h3", "(1x -1y 9adr)", &rsq);

        // 90
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw90);
        test_rsq("h4", "(1x 0y 10adr)", &rsq);

        // 135
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw135);
        test_rsq("h5", "(1x 1y 11adr)", &rsq);

        // 180
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x 1y 1adr)", &rsq);

        // 225
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw225);
        test_rsq("h7", "(-1x 1y -9adr)", &rsq);

        // 270
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw270);
        test_rsq("h8", "(-1x 0y -10adr)", &rsq);

        // 315
        rsq = RelativeSquare::from_file_and_rank(0, -1);
        rsq = rsq.rotate(Angle::Ccw315);
        test_rsq("h9", "(-1x -1y -11adr)", &rsq);
    }
    // 角度指定回転のテスト(南から)
    {
        // 0
        let mut rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("h1", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x 1y 1adr)", &rsq);

        // 45
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw45);
        test_rsq("h3", "(-1x 1y -9adr)", &rsq);

        // 90
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw90);
        test_rsq("h4", "(-1x 0y -10adr)", &rsq);

        // 135
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw135);
        test_rsq("h5", "(-1x -1y -11adr)", &rsq);

        // 180
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x -1y -1adr)", &rsq);

        // 225
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw225);
        test_rsq("h7", "(1x -1y 9adr)", &rsq);

        // 270
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw270);
        test_rsq("h8", "(1x 0y 10adr)", &rsq);

        // 315
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        rsq = rsq.rotate(Angle::Ccw315);
        test_rsq("h9", "(1x 1y 11adr)", &rsq);
    }
}

//
// 盤、升、筋、段
//

#[allow(non_camel_case_types)]
pub type isquare = i8;

/*
/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MIN: isquare = 11;

/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MAX: isquare = 99;
*/

/*
// 盤のヨコ幅、タテ幅。
// pub const BOARD_WIDTH: i8 = 9;
// pub const BOARD_HEIGHT: i8 = 9;
// 正方形という前提☆（＾～＾）
pub const BOARD_DIAGONAL_LENGTH: isquare = 9;
*/

// 枠も使う☆（＾～＾）
pub const BOARD_MEMORY_AREA: isquare = 111;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;

/// 筋、段は 1 から始まる、という明示。
/// 増減はよく使うので u8 ではなく i8 にした。
// pub const FILE_0: i8 = 0;
pub const FILE_1: i8 = 1;
pub const FILE_9: i8 = 9;
pub const FILE_10: i8 = 10;
// pub const RANK_0: i8 = 0;
pub const RANK_1: i8 = 1;
pub const RANK_2: i8 = 2;
pub const RANK_3: i8 = 3;
pub const RANK_4: i8 = 4;
// pub const RANK_5: i8 = 5;
pub const RANK_6: i8 = 6;
pub const RANK_7: i8 = 7;
pub const RANK_8: i8 = 8; //うさぎの打てる段の上限
pub const RANK_9: i8 = 9;
pub const RANK_10: i8 = 10;

/// 升の検索等で、該当なしの場合
pub const SQUARE_NONE: isquare = 0;

/// 指し手。打の場合のsrc
pub const SQUARE_DROP: isquare = 0;

#[derive(Debug)]
pub enum DictOrthant {
    /// 第２象限。x=0, y=0 ともに含みません。
    II,
    /// 第４象限。x=0, y=0 ともに含みません。
    IV,
    /// 第１象限と第三象限。区別しません。x=0, y=0 ともに含みます。
    IOrIII,
}
impl DictOrthant {
    pub fn from_file_and_rank(file: i8, rank: i8) -> Self {
        if 0 <= file * rank {
            DictOrthant::IOrIII
        } else if file < 0 {
            DictOrthant::II
        } else {
            DictOrthant::IV
        }
    }
}

#[derive(Debug)]
pub enum Degree45Orthant {
    /// 正第４象限と、正第１象限☆（＾～＾）
    IVOrI,
    /// コ第１象限と、コ第２象限☆（＾～＾）
    CoIOrCoII,
    /// 正第２象限と、正第３象限☆（＾～＾）
    IIOrIII,
    /// コ第３象限と、コ第４象限☆（＾～＾）
    CoIIIOrCoIV,
}
impl Degree45Orthant {
    pub fn from_file_and_rank(file: i8, rank: i8) -> Self {
        let range = max(file.abs(), rank.abs());
        if file == range {
            Degree45Orthant::IVOrI
        } else if file == -range {
            Degree45Orthant::IIOrIII
        } else if rank == range {
            Degree45Orthant::CoIOrCoII
        } else {
            Degree45Orthant::CoIIIOrCoIV
        }
    }
}

/// Counterclockwise(反時計回り)での回転方向。
#[derive(Clone, Copy, Debug)]
pub enum Angle {
    /// 西。
    Ccw0,
    /// 南西。
    Ccw45,
    /// 南。
    Ccw90,
    /// 南東。
    Ccw135,
    /// 東。
    Ccw180,
    /// 北東。
    Ccw225,
    /// 北。
    Ccw270,
    /// 北西。
    Ccw315,
}
impl Angle {
    /*
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate135cw(&self) -> Self {
        use crate::cosmic::game::board::square::Angle::*;
        match self {
            Ccw0 => Ccw225,
            Ccw45 => Ccw270,
            Ccw90 => Ccw315,
            Ccw135 => Ccw0,
            Ccw180 => Ccw45,
            Ccw225 => Ccw90,
            Ccw270 => Ccw135,
            Ccw315 => Ccw180,
        }
    }
    */
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate90cw(&self) -> Self {
        use crate::cosmic::smart::square::Angle::*;
        match self {
            Ccw0 => Ccw270,
            Ccw45 => Ccw315,
            Ccw90 => Ccw0,
            Ccw135 => Ccw45,
            Ccw180 => Ccw90,
            Ccw225 => Ccw135,
            Ccw270 => Ccw180,
            Ccw315 => Ccw225,
        }
    }
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate45cw(&self) -> Self {
        use crate::cosmic::smart::square::Angle::*;
        match self {
            Ccw0 => Ccw315,
            Ccw45 => Ccw0,
            Ccw90 => Ccw45,
            Ccw135 => Ccw90,
            Ccw180 => Ccw135,
            Ccw225 => Ccw180,
            Ccw270 => Ccw225,
            Ccw315 => Ccw270,
        }
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate45ccw(&self) -> Self {
        use crate::cosmic::smart::square::Angle::*;
        match self {
            Ccw0 => Ccw45,
            Ccw45 => Ccw90,
            Ccw90 => Ccw135,
            Ccw135 => Ccw180,
            Ccw180 => Ccw225,
            Ccw225 => Ccw270,
            Ccw270 => Ccw315,
            Ccw315 => Ccw0,
        }
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate90ccw(&self) -> Self {
        use crate::cosmic::smart::square::Angle::*;
        match self {
            Ccw0 => Ccw90,
            Ccw45 => Ccw135,
            Ccw90 => Ccw180,
            Ccw135 => Ccw225,
            Ccw180 => Ccw270,
            Ccw225 => Ccw315,
            Ccw270 => Ccw0,
            Ccw315 => Ccw45,
        }
    }
    /*
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate135ccw(&self) -> Self {
        use crate::cosmic::game::board::square::Angle::*;
        match self {
            Ccw0 => Ccw135,
            Ccw45 => Ccw180,
            Ccw90 => Ccw225,
            Ccw135 => Ccw270,
            Ccw180 => Ccw315,
            Ccw225 => Ccw0,
            Ccw270 => Ccw45,
            Ccw315 => Ccw90,
        }
    }
    */
    /// 点対称☆（＾～＾）
    pub fn rotate180(&self) -> Self {
        use crate::cosmic::smart::square::Angle::*;
        match self {
            Ccw0 => Ccw180,
            Ccw45 => Ccw225,
            Ccw90 => Ccw270,
            Ccw135 => Ccw315,
            Ccw180 => Ccw0,
            Ccw225 => Ccw45,
            Ccw270 => Ccw90,
            Ccw315 => Ccw135,
        }
    }
}

/// 相対升。
/// file, rank から 相対番地は作れますが、相対番地から file, rank を作ることはできません(不定)。
/// そこから、 file, rank で持ちます。
#[derive(Clone)]
pub struct RelativeSquare {
    pub file: i8,
    pub rank: i8,
}
impl RelativeSquare {
    pub fn from_file_and_rank(file1: i8, rank1: i8) -> Self {
        RelativeSquare {
            file: file1,
            rank: rank1,
        }
    }

    pub fn get_address(&self) -> isquare {
        10 * self.file + self.rank
    }

    /*
    pub fn get_dict_orthant(&self) -> DictOrthant {
        DictOrthant::from_file_and_rank(self.file, self.rank)
    }
    */

    pub fn get_degree45_orthant(&self) -> Degree45Orthant {
        Degree45Orthant::from_file_and_rank(self.file, self.rank)
    }

    pub fn rotate(&self, angle: Angle) -> Self {
        use crate::cosmic::smart::square::Angle::*;
        match angle {
            Ccw0 => RelativeSquare {
                file: self.file,
                rank: self.rank,
            },
            Ccw45 => self.rotate_45_countercrockwise(),
            Ccw90 => self.rotate_90_countercrockwise(),
            Ccw135 => {
                let r90 = self.rotate_90_countercrockwise();
                // println!("> r90={:?}", r90);
                let r90_45 = r90.rotate_45_countercrockwise();
                // println!("> r90_45={:?}", r90_45);
                r90_45
            }
            Ccw180 => self.rotate_180(),
            Ccw225 => {
                /*
                let r180 = self.rotation_180();
                println!("> r180={:?}", r180);
                let r180_45 = r180.rotation_45_countercrockwise();
                println!("> r180+45={:?}", r180_45);
                r180_45
                */
                self.rotate_180().rotate_45_countercrockwise()
            }
            Ccw270 => self.rotate_180().rotate_90_countercrockwise(),
            Ccw315 => {
                //*
                let r180 = self.rotate_180();
                // println!("> r180={:?}", r180);
                let r180_90 = r180.rotate_90_countercrockwise();
                // println!("> r180+90={:?}", r180_90);
                let r180_90_45 = r180_90.rotate_45_countercrockwise();
                // println!("> r180+90+45={:?}", r180_90_45);
                r180_90_45
                // */
                /*
                self.rotation_180()
                    .rotation_90_countercrockwise()
                    .rotation_45_countercrockwise()
                */
            }
        }
    }

    /// 段を２倍にします。桂馬に使います。
    pub fn double_rank(&self) -> Self {
        let new_rank = 2 * self.rank;
        let carry = new_rank / 10;
        let new_file = if carry != 0 {
            self.file + carry
        } else {
            self.file
        };
        RelativeSquare {
            file: new_file,
            rank: new_rank,
        }
    }

    pub fn rotate_180(&self) -> Self {
        RelativeSquare {
            file: -self.file,
            rank: -self.rank,
        }
    }

    pub fn rotate_90_countercrockwise(&self) -> Self {
        // 象限は、何度回転するかによって境界線の位置が変わってくるので、回転の直前で調べるしかないぜ☆（＾～＾）
        // でも、 90°回転のときは 象限は１つしかないけどな☆（＾～＾）全象限同じ式だぜ☆（*＾～＾*）
        let new_file = -self.rank;
        let new_rank = self.file;
        RelativeSquare::from_file_and_rank(new_file, new_rank)
    }

    pub fn rotate_45_countercrockwise(&self) -> Self {
        // 象限は、何度回転するかによって境界線の位置が変わってくるので、回転の直前で調べるしかないぜ☆（＾～＾）
        let orthant = self.get_degree45_orthant();
        let (new_file, new_rank) = match orthant {
            Degree45Orthant::IVOrI => {
                let distance = self.file;
                let mut new_file = self.file;
                let mut new_rank = self.rank + distance;
                let over = new_rank.abs() - distance.abs();
                if 0 < over {
                    new_rank = distance;
                    new_file -= over;
                }
                (new_file, new_rank)
            }
            Degree45Orthant::IIOrIII => {
                let distance = self.file;
                let mut new_file = self.file;
                let mut new_rank = self.rank + distance;
                let over = new_rank.abs() - distance.abs();
                if 0 < over {
                    new_rank = distance;
                    new_file += over;
                }
                (new_file, new_rank)
            }
            Degree45Orthant::CoIOrCoII => {
                let distance = self.rank;
                let mut new_file = self.file - distance;
                let mut new_rank = self.rank;
                let over = new_rank.abs() - distance.abs();
                if 0 < over {
                    new_file = distance;
                    new_rank -= over;
                }
                (new_file, new_rank)
            }
            Degree45Orthant::CoIIIOrCoIV => {
                let distance = self.rank;
                let mut new_file = self.file - distance;
                let mut new_rank = self.rank;
                let over = new_rank.abs() - distance.abs();
                if 0 < over {
                    new_file = distance;
                    new_rank -= over;
                }
                (new_file, new_rank)
            }
        };

        RelativeSquare::from_file_and_rank(new_file, new_rank)
    }
}

/// 回転してみるまで象限は分からないので、出せるのは筋、段、相対番地だけだぜ☆（＾～＾）
impl fmt::Debug for RelativeSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}x {}y {}adr)",
            self.file,
            self.rank,
            self.get_address()
        )
    }
}

/// Square(升).
///
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
    pub fn from_address(address1: isquare) -> Self {
        assert_in_board_with_frame_as_absolute(
            address1,
            &format!("square::from_address({})", address1),
        );
        Square { address: address1 }
    }
    pub fn from_file_rank(file: i8, rank: i8) -> Self {
        let adr = file * 10 + rank;
        assert_in_board_with_frame_as_absolute(
            adr,
            &format!("{} = square::from_file_rank({}, {})", adr, file, rank),
        );
        Square { address: adr }
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

    /*
    pub fn rotate_180_on_1_orthant(&self) -> Self {
        Square::from_address(110 - self.address)
    }
    */
}
impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}x {}y {}adr)",
            self.get_file(),
            self.get_rank(),
            self.address
        )
    }
}
