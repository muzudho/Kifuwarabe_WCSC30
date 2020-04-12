//!
//! 駒の動き
//!

use crate::controller::movement_generation::mg_controller::KM_UGOKI_LN;
use crate::model::univ::gam::misc::piece_type::*;
use crate::model::univ::gam::misc::square::Angle;
use std::fmt;

/// 機敏性。
#[derive(Clone, Copy, Debug)]
pub enum Agility {
    /// 隣へ１つ進む駒。
    Hopping,
    /// 長い利き。
    Sliding,
    /// 桂馬。
    Keima,
}

#[derive(Clone)]
pub struct PieceMove {
    pub angle: Angle,
    pub agility: Agility,
}
impl PieceMove {
    pub fn new(angle1: Angle, agility1: Agility) -> Self {
        PieceMove {
            angle: angle1,
            agility: agility1,
        }
    }
}
impl fmt::Debug for PieceMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?} {:?})", self.angle, self.agility)
    }
}

// 駒が戻る動き
pub struct PieceMovement {
    // 駒種類ごとに、駒の動きを保持。動ける方向は、駒ごとに可変長配列
    // 角度、スライダー、桂馬。
    // const にしたいので、固定長配列にしているぜ☆（＾～＾）
    pub back: [[Option<PieceMove>; KM_UGOKI_LN]; KMS_LN],
}
/// 駒の動き。
pub const KM_UGOKI: PieceMovement = PieceMovement {
    back: [
        // K
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                agility: Agility::Hopping,
            }),
            None,
        ],
        // R
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Sliding,
            }),
            None,
            None,
            None,
            None,
            None,
        ],
        // B
        [
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                agility: Agility::Sliding,
            }),
            None,
            None,
            None,
            None,
            None,
        ],
        // G
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
        ],
        // S
        [
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
            None,
        ],
        // N
        [
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Keima,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Keima,
            }),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        // L
        [
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Sliding,
            }),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        // P
        [
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        // PR
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                agility: Agility::Hopping,
            }),
            None,
        ],
        // PB
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                agility: Agility::Sliding,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                agility: Agility::Sliding,
            }),
            None,
        ],
        // PS
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
        ],
        // PN
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
        ],
        // PL
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
        ],
        // PP
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                agility: Agility::Hopping,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                agility: Agility::Hopping,
            }),
            None,
            None,
            None,
        ],
        /*空升*/
        [None, None, None, None, None, None, None, None, None],
        /*終り*/
        [None, None, None, None, None, None, None, None, None],
    ],
};
