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
use std::fmt;
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

#[derive(Debug)]
pub enum Rotation {
    C0,
    // 45° counterclockwise rotation
    C45,
    C90,
    C135,
    C180,
    C225,
    C270,
    C315,
}
impl Rotation {
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate135cw(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C225,
            C45 => C270,
            C90 => C315,
            C135 => C0,
            C180 => C45,
            C225 => C90,
            C270 => C135,
            C315 => C180,
        }
    }
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate90cw(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C270,
            C45 => C315,
            C90 => C0,
            C135 => C45,
            C180 => C90,
            C225 => C135,
            C270 => C180,
            C315 => C225,
        }
    }
    /// 時計回り(Clockwise)☆（＾～＾）
    pub fn rotate45cw(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C315,
            C45 => C0,
            C90 => C45,
            C135 => C90,
            C180 => C135,
            C225 => C180,
            C270 => C225,
            C315 => C270,
        }
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate45ccw(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C45,
            C45 => C90,
            C90 => C135,
            C135 => C180,
            C180 => C225,
            C225 => C270,
            C270 => C315,
            C315 => C0,
        }
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate90ccw(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C90,
            C45 => C135,
            C90 => C180,
            C135 => C225,
            C180 => C270,
            C225 => C315,
            C270 => C0,
            C315 => C45,
        }
    }
    /// 反時計回り(Counterclockwise)☆（＾～＾）
    pub fn rotate135ccw(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C135,
            C45 => C180,
            C90 => C225,
            C135 => C270,
            C180 => C315,
            C225 => C0,
            C270 => C45,
            C315 => C90,
        }
    }
    /// 点対称☆（＾～＾）
    pub fn rotate180(&self) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match self {
            C0 => C180,
            C45 => C225,
            C90 => C270,
            C135 => C315,
            C180 => C0,
            C225 => C45,
            C270 => C90,
            C315 => C135,
        }
    }
}

/// 相対升。
pub struct RelativeSquare {
    /// xより y寄りなら真。
    pub co: bool,
    pub orthant: u8,
    pub address: isquare,
}
impl RelativeSquare {
    /// 符号は 象限からでしか復元できないぜ☆（*＾～＾*）
    pub fn from_relative_address(orthant: u8, adr: i8) -> Self {
        match orthant {
            1 => RelativeSquare::from_file_and_rank(
                RelativeSquare::abs_x(adr),
                RelativeSquare::abs_y(adr),
            ),
            2 => RelativeSquare::from_file_and_rank(
                -RelativeSquare::abs_x(adr) - 1,
                RelativeSquare::completion(RelativeSquare::abs_y(adr)),
            ),
            3 => RelativeSquare::from_file_and_rank(
                -RelativeSquare::abs_x(adr),
                -RelativeSquare::abs_y(adr),
            ),
            4 => RelativeSquare::from_file_and_rank(
                RelativeSquare::abs_x(adr) + 1,
                -RelativeSquare::completion(RelativeSquare::abs_y(adr)),
            ),
            _ => panic!("ない象限☆（＾～＾）orthant={}", orthant),
        }
    }

    pub fn from_file_and_rank(file: i8, rank: i8) -> Self {
        // Decision tree.
        let (orthant1, co1) = if file < 0 {
            // 相対番地盤の半分より東側☆（＾～＾） II, III 象限のどちらかは確定☆（＾～＾）
            if 0 < rank {
                // 相対番地盤の半分より南東側☆（＾～＾） II 象限に確定☆（＾～＾）
                // 対角線は co の方☆（＾～＾）
                (2, file.abs() <= rank.abs())
            } else {
                // x=0, y=0の境界線上も含むぜ☆（＾～＾）III 象限に確定☆（＾～＾）
                // 対角線は co じゃない方☆（＾～＾）
                (3, file.abs() < rank.abs())
            }
        } else if 0 < file {
            // 相対番地盤の半分より西側☆（＾～＾） I, IV 象限のどちらかは確定☆（＾～＾）
            if rank < 0 {
                // 相対番地盤の半分より北西側☆（＾～＾） IV 象限に確定☆（＾～＾）
                // 対角線は co の方☆（＾～＾）
                (4, file.abs() <= rank.abs())
            } else {
                // x=0, y=0の境界線上も含むぜ☆（＾～＾） I 象限に確定☆（＾～＾）
                // 対角線は co じゃない方☆（＾～＾）
                (1, file.abs() < rank.abs())
            }
        } else {
            // x=0の垂直の境界線上☆（＾～＾） I, III 象限のどちらかの co は確定☆（＾～＾）
            if rank < 0 {
                (3, true)
            } else if 0 < rank {
                (1, true)
            } else {
                panic!("真ん中は取り扱い不可☆（＾～＾）");
            }
        };
        RelativeSquare {
            co: co1,
            orthant: orthant1,
            address: 10 * file + rank,
        }
    }

    pub fn rotate_countercrockwise(&self, rot: &Rotation) -> Self {
        use crate::model::univ::gam::misc::square::Rotation::*;
        match rot {
            C0 => RelativeSquare {
                co: self.co,
                orthant: self.orthant,
                address: self.address,
            },
            C45 => self.rotation_45_countercrockwise(),
            C90 => self.rotation_90_countercrockwise(),
            C135 => {
                let r90 = self.rotation_90_countercrockwise();
                println!("> r90={:?}", r90);
                let r90_45 = r90.rotation_45_countercrockwise();
                println!("> r90_45={:?}", r90_45);
                r90_45
            }
            C180 => self.rotation_180(),
            C225 => {
                /*
                let r180 = self.rotation_180();
                println!("> r180={:?}", r180);
                let r180_45 = r180.rotation_45_countercrockwise();
                println!("> r180+45={:?}", r180_45);
                r180_45
                */
                self.rotation_180().rotation_45_countercrockwise()
            }
            C270 => self.rotation_180().rotation_90_countercrockwise(),
            C315 => {
                //*
                let r180 = self.rotation_180();
                println!("> r180={:?}", r180);
                let r180_90 = r180.rotation_90_countercrockwise();
                println!("> r180+90={:?}", r180_90);
                let r180_90_45 = r180_90.rotation_45_countercrockwise();
                println!("> r180+90+45={:?}", r180_90_45);
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

    pub fn rotation_180(&self) -> Self {
        RelativeSquare {
            co: self.co,
            orthant: (self.orthant + 1) % 4 + 1,
            address: -self.address,
        }
    }

    pub fn rotation_90_countercrockwise(&self) -> Self {
        let new_adr = if !self.co {
            if self.orthant % 2 == 1 {
                // 1ort, 3ort
                // println!("1ort, 3ort");
                -1 * self.get_sign()
                    * (10 * RelativeSquare::completion11(self.get_abs_y())
                        + RelativeSquare::completion(self.get_abs_x()))
            } else {
                // 2ort, 4ort
                // println!("2ort, 4ort");
                self.get_sign()
                    * (10 * self.get_abs_x() + RelativeSquare::completion11(self.get_abs_y()))
            }
        } else {
            if self.orthant % 2 == 1 {
                // co1ort, co3ort
                // println!("co1ort, co3ort");
                -1 * self.get_sign()
                    * (10 * self.get_abs_x() + RelativeSquare::completion11(self.get_abs_y()))
            } else {
                // co2ort, co4ort
                // println!("co2ort, co4ort");
                self.get_sign()
                    * (10 * RelativeSquare::completion(self.get_abs_y())
                        + RelativeSquare::completion11(self.get_abs_x()))
            }
        };

        // 90°回転後に 0 rank なら、 II 象限ではなく III 象限だし、 IV 象限ではなく I 象限だぜ☆（＾～＾）
        // 1 象限足せばいい☆（＾～＾）
        let new_orthant = if RelativeSquare::abs_y(new_adr) == 0 {
            (self.orthant + 1) % 4 + 1
        } else {
            (self.orthant) % 4 + 1
        };

        RelativeSquare::from_relative_address(new_orthant, new_adr)
        /*
        RelativeSquare {
            co: !self.co,
            orthant: (self.orthant) % 4 + 1,
            address: new_adr,
        }
        */
    }

    pub fn rotation_45_countercrockwise(&self) -> Self {
        let new_adr = if !self.co {
            if self.orthant % 2 == 1 {
                // 1ort, 3ort
                // println!("1ort, 3ort");
                self.get_sign() * (10 * (self.get_abs_x() - self.get_abs_y()) + self.get_abs_x())
            } else {
                // 2ort, 4ort
                // println!("2ort, 4ort");
                self.get_sign()
                    * (10 * (self.get_abs_x() + 1) + self.get_abs_x()
                        - RelativeSquare::completion9(self.get_abs_y()))
            }
        } else {
            if self.orthant % 2 == 1 {
                // co1ort, co3ort
                // println!("co1ort, co3ort");
                -1 * self.get_sign()
                    * (10 * (self.get_abs_y() - self.get_abs_x() - 1)
                        + RelativeSquare::completion(self.get_abs_y()))
            } else {
                // co2ort, co4ort
                // println!("co2ort, co4ort");
                self.get_sign()
                    * (10 * RelativeSquare::completion9(self.get_abs_y())
                        + self.get_abs_x()
                        + self.get_abs_y()
                        + 1)
            }
        };
        let mut new_orthant = match (self.co, self.orthant) {
            (false, 1) => 1,
            (true, 1) => 2,
            (true, 2) => 2,
            (false, 2) => 3,
            (false, 3) => 3,
            (true, 3) => 4,
            (true, 4) => 4,
            (false, 4) => 1,
            _ => panic!("orthant fail 345."),
            //_ => panic!("co={},orthant={}", self.co, self.orthant),
        };

        // 45°回転後に 0 rank なら、 II 象限ではなく III 象限だし、 IV 象限ではなく I 象限だぜ☆（＾～＾）
        // 1 象限足せばいい☆（＾～＾）
        if RelativeSquare::abs_y(new_adr) == 0 {
            new_orthant = new_orthant % 4 + 1;
        };

        RelativeSquare::from_relative_address(new_orthant, new_adr)
        /*
        let (new_co, new_orthant) = match (self.co, self.orthant) {
            (false, 1) => (true, 1),
            (true, 1) => (true, 2),
            (true, 2) => (false, 2),
            (false, 2) => (false, 3),
            (false, 3) => (true, 3),
            (true, 3) => (true, 4),
            (true, 4) => (false, 4),
            (false, 4) => (false, 1),
            _ => panic!("orthant fail 345."),
            //_ => panic!("co={},orthant={}", self.co, self.orthant),
        };
        RelativeSquare {
            co: new_co,
            orthant: new_orthant,
            address: new_adr,
        }
        */
    }

    fn completion9(abn: i8) -> i8 {
        9 - abn
    }
    fn completion(abn: i8) -> i8 {
        10 - abn
    }
    fn completion11(abn: i8) -> i8 {
        // (11 - abn).abs() % 10
        11 - abn
    }
    fn get_sign(&self) -> i8 {
        self.address / self.address.abs()
    }
    fn get_abs_x(&self) -> i8 {
        RelativeSquare::abs_x(self.address)
    }
    fn get_abs_y(&self) -> i8 {
        RelativeSquare::abs_y(self.address)
    }
    fn abs_x(adr: i8) -> i8 {
        (adr / 10).abs() % 10
    }
    fn abs_y(adr: i8) -> i8 {
        adr.abs() % 10
    }
    /*
    fn rel_x(adr: i8) -> i8 {
        let sign = adr / adr.abs();
        sign * RelativeSquare::abs_x(adr)
    }
    fn rel_y(adr: i8) -> i8 {
        let sign = adr / adr.abs();
        sign * RelativeSquare::abs_y(adr)
    }
    */
}

impl fmt::Debug for RelativeSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}{}ort,{})",
            if self.co { "co" } else { "" },
            self.orthant,
            self.address
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
        assert_in_board_as_absolute(self.address, "(203b)sq_to_p");
        Point {
            x: self.get_file(),
            y: self.get_rank(),
        }
    }
}
