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
use crate::entities::law::speed_of_light::Nine299792458;
use crate::position::RelAdr;
use crate::position::Square;
use std::cmp::max;

/// 打はテストできない
pub fn _assert_in_board_as_absolute(sq: Square, hint: &str) {
    debug_assert!(
        (10 < sq.number() && sq.number() < 20)
            || (20 < sq.number() && sq.number() < 30)
            || (30 < sq.number() && sq.number() < 40)
            || (40 < sq.number() && sq.number() < 50)
            || (50 < sq.number() && sq.number() < 60)
            || (60 < sq.number() && sq.number() < 70)
            || (70 < sq.number() && sq.number() < 80)
            || (80 < sq.number() && sq.number() < 90)
            || (90 < sq.number() && sq.number() < 100),
        "abs-sq=|{}| hint={}",
        sq.number(),
        hint
    );
}

fn test_dort(test_name: &str, expected: &str, actual: &DictOrthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        "{}: expected={} | actual={:?}",
        test_name,
        expected,
        actual
    );
}
fn test_d45ort(test_name: &str, expected: &str, actual: &Degree45Orthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        "{}: expected={} | actual={:?}",
        test_name,
        expected,
        actual
    );
}
fn test_rsq(test_name: &str, expected: &str, actual: &RelAdr) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        "{}: expected={} | actual={:?}",
        test_name,
        expected,
        actual
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
        // TODO speed_of_light に West とか相対座標を入れておきたい。
        let mut ort = Degree45Orthant::new(&RelAdr::new(0, -1));
        test_d45ort("f1", "CoIIIOrCoIV", &ort);
        ort = Degree45Orthant::new(&RelAdr::new(1, -1));
        test_d45ort("f2", "IVOrI", &ort);
        ort = Degree45Orthant::new(&Nine299792458::west());
        test_d45ort("f3", "IVOrI", &ort);
        ort = Degree45Orthant::new(&RelAdr::new(1, 1));
        test_d45ort("f4", "IVOrI", &ort);
        ort = Degree45Orthant::new(&RelAdr::new(0, 1));
        test_d45ort("f5", "CoIOrCoII", &ort);
        ort = Degree45Orthant::new(&RelAdr::new(-1, 1));
        test_d45ort("f6", "IIOrIII", &ort);
        ort = Degree45Orthant::new(&RelAdr::new(-1, 0));
        test_d45ort("f7", "IIOrIII", &ort);
        ort = Degree45Orthant::new(&RelAdr::new(-1, -1));
        test_d45ort("f8", "IIOrIII", &ort);
    }
    // 相対番地のテスト
    {
        test_rsq("b1", "(0x -1y -1adr)", &RelAdr::new(0, -1));
        test_rsq("b2", "(1x -1y 9adr)", &RelAdr::new(1, -1));
        test_rsq("b3", "(1x 0y 10adr)", &Nine299792458::west());
        test_rsq("b4", "(1x 1y 11adr)", &RelAdr::new(1, 1));
        test_rsq("b5", "(0x 1y 1adr)", &RelAdr::new(0, 1));
        test_rsq("b6", "(-1x 1y -9adr)", &RelAdr::new(-1, 1));
        test_rsq("b7", "(-1x 0y -10adr)", &RelAdr::new(-1, 0));
        test_rsq("b8", "(-1x -1y -11adr)", &RelAdr::new(-1, -1));
    }
    // 45°回転のテスト
    {
        let mut r = RelAdr::new(0, -1);
        test_rsq("a1", "(0x -1y -1adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a2", "(1x -1y 9adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a3", "(1x 0y 10adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a4", "(1x 1y 11adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a5", "(0x 1y 1adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a6", "(-1x 1y -9adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a7", "(-1x 0y -10adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a8", "(-1x -1y -11adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a9", "(0x -1y -1adr)", &r);
    }
    // 90°回転のテスト＜その１＞
    {
        let mut r = RelAdr::new(0, -1);
        test_rsq("c1", "(0x -1y -1adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c2", "(1x 0y 10adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c3", "(0x 1y 1adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c4", "(-1x 0y -10adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c5", "(0x -1y -1adr)", &r);
    }
    // 90°回転のテスト＜その２＞
    {
        let mut r = RelAdr::new(1, -1);
        test_rsq("d1", "(1x -1y 9adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d2", "(1x 1y 11adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d3", "(-1x 1y -9adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d4", "(-1x -1y -11adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d5", "(1x -1y 9adr)", &r);
    }
    // 桂馬のテスト
    {
        let mut r = RelAdr::new(0, -1);
        test_rsq("g1", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw45);
        test_rsq("g2", "(1x -1y 9adr)", &r);
        r.double_rank();
        test_rsq("g3", "(1x -2y 8adr)", &r);

        let mut r = RelAdr::new(0, -1);
        test_rsq("g4", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw315);
        test_rsq("g5", "(-1x -1y -11adr)", &r);
        r.double_rank();
        test_rsq("g6", "(-1x -2y -12adr)", &r);

        let mut r = RelAdr::new(0, 1);
        test_rsq("g7", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw45);
        test_rsq("g8", "(-1x 1y -9adr)", &r);
        r.double_rank();
        test_rsq("g9", "(-1x 2y -8adr)", &r);

        let mut r = RelAdr::new(0, 1);
        test_rsq("g10", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw315);
        test_rsq("g11", "(1x 1y 11adr)", &r);
        r.double_rank();
        test_rsq("g12", "(1x 2y 12adr)", &r);
    }
    // 角度指定回転のテスト(北から)
    {
        // 0
        let mut r = RelAdr::new(0, -1);
        test_rsq("h1", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x -1y -1adr)", &r);

        // 45
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw45);
        test_rsq("h3", "(1x -1y 9adr)", &r);

        // 90
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw90);
        test_rsq("h4", "(1x 0y 10adr)", &r);

        // 135
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw135);
        test_rsq("h5", "(1x 1y 11adr)", &r);

        // 180
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x 1y 1adr)", &r);

        // 225
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw225);
        test_rsq("h7", "(-1x 1y -9adr)", &r);

        // 270
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw270);
        test_rsq("h8", "(-1x 0y -10adr)", &r);

        // 315
        r = RelAdr::new(0, -1);
        r.rotate(Angle::Ccw315);
        test_rsq("h9", "(-1x -1y -11adr)", &r);
    }
    // 角度指定回転のテスト(南から)
    {
        // 0
        let mut r = RelAdr::new(0, 1);
        test_rsq("h1", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x 1y 1adr)", &r);

        // 45
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw45);
        test_rsq("h3", "(-1x 1y -9adr)", &r);

        // 90
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw90);
        test_rsq("h4", "(-1x 0y -10adr)", &r);

        // 135
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw135);
        test_rsq("h5", "(-1x -1y -11adr)", &r);

        // 180
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x -1y -1adr)", &r);

        // 225
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw225);
        test_rsq("h7", "(1x -1y 9adr)", &r);

        // 270
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw270);
        test_rsq("h8", "(1x 0y 10adr)", &r);

        // 315
        r = RelAdr::new(0, 1);
        r.rotate(Angle::Ccw315);
        test_rsq("h9", "(1x 1y 11adr)", &r);
    }
}

//
// 盤、升、筋、段
//

// #[allow(non_camel_case_types)]
// pub type isquare = isize;

// 配列サイズなので 1 大きめだぜ☆（＾～＾）
pub const BOARD_MEMORY_AREA: u8 = 100;

/// 筋、段は 1 から始まる、という明示。
/// usize が速い☆（＾～＾）
pub const FILE_0: u8 = 0;
pub const FILE_1: u8 = 1;
pub const FILE_9: u8 = 9;
pub const FILE_10: u8 = 10;
// pub const FILE_11: u8 = 11;
pub const RANK_0: u8 = 0;
pub const RANK_1: u8 = 1;
pub const RANK_2: u8 = 2;
pub const RANK_3: u8 = 3;
pub const RANK_4: u8 = 4;
// pub const RANK_5: u8 = 5;
pub const RANK_6: u8 = 6;
pub const RANK_7: u8 = 7;
pub const RANK_8: u8 = 8; //うさぎの打てる段の上限
pub const RANK_9: u8 = 9;
pub const RANK_10: u8 = 10;
// pub const RANK_11: u8 = 11;

/// 引き算もするところでは unsigned ではダメなところもある☆（＾～＾）
// pub const I_FILE_0: i8 = 0;
// pub const I_FILE_1: i8 = 1;
// pub const I_FILE_9: i8 = 9;
// pub const I_FILE_10: i8 = 10;
// pub const I_RANK_0: i8 = 0;
// pub const I_RANK_1: i8 = 1;
// pub const I_RANK_2: i8 = 2;
// pub const I_RANK_3: i8 = 3;
// pub const I_RANK_4: i8 = 4;
// pub const I_RANK_6: i8 = 6;
// pub const I_RANK_7: i8 = 7;
// pub const I_RANK_8: i8 = 8; //うさぎの打てる段の上限
// pub const I_RANK_9: i8 = 9;
// pub const I_RANK_10: i8 = 10;

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
    pub fn from_file_and_rank(file: isize, rank: isize) -> Self {
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
    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn new(r: &RelAdr) -> Self {
        let range = max(r.file().abs(), r.rank().abs());
        if r.file() == range {
            Degree45Orthant::IVOrI
        } else if r.file() == -range {
            Degree45Orthant::IIOrIII
        } else if r.rank() == range {
            Degree45Orthant::CoIOrCoII
        } else {
            Degree45Orthant::CoIIIOrCoIV
        }
    }
}

pub const ANGLE_LEN: usize = 8;
/// Counterclockwise(反時計回り)での回転方向。 45°ずつ☆（＾～＾）
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
