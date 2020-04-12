//!
//! 駒の動き
//!

use crate::controller::movement_generation::mg_controller::KM_UGOKI_LN;
use crate::model::univ::gam::misc::piece_type::*;
use crate::model::univ::gam::misc::square::Angle;
use std::fmt;

#[derive(Clone)]
pub struct PieceMove {
    pub angle: Angle,
    pub slider: bool,
    pub keima: bool,
}
impl PieceMove {
    pub fn new(angle1: Angle, slider1: bool, keima1: bool) -> Self {
        PieceMove {
            angle: angle1,
            slider: slider1,
            keima: keima1,
        }
    }
}
impl fmt::Debug for PieceMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?}{}{})",
            self.angle,
            if self.slider { " slider" } else { "" },
            if self.keima { " keima" } else { "" }
        )
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
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                slider: false,
                keima: false,
            }),
            None,
        ],
        // R
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: true,
                keima: false,
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
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                slider: true,
                keima: false,
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
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
            }),
            None,
            None,
            None,
        ],
        // S
        [
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                slider: false,
                keima: false,
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
                slider: false,
                keima: true,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: true,
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
                slider: true,
                keima: false,
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
                slider: false,
                keima: false,
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
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                slider: false,
                keima: false,
            }),
            None,
        ],
        // PB
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw45,
                slider: true,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw135,
                slider: true,
                keima: false,
            }),
            None,
        ],
        // PS
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
            }),
            None,
            None,
            None,
        ],
        // PN
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
            }),
            None,
            None,
            None,
        ],
        // PL
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
            }),
            None,
            None,
            None,
        ],
        // PP
        [
            Some(PieceMove {
                angle: Angle::Ccw180,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw225,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw270,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw315,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw0,
                slider: false,
                keima: false,
            }),
            Some(PieceMove {
                angle: Angle::Ccw90,
                slider: false,
                keima: false,
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
