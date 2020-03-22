//!
//! 陣
//!

use crate::model::univ::gam::misc::square::*;

/// 先手陣
pub struct SenteJin {}
impl SenteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_isquare(91),
            Square::from_isquare(81),
            Square::from_isquare(71),
            Square::from_isquare(61),
            Square::from_isquare(51),
            Square::from_isquare(41),
            Square::from_isquare(31),
            Square::from_isquare(21),
            Square::from_isquare(11),
            Square::from_isquare(92),
            Square::from_isquare(82),
            Square::from_isquare(72),
            Square::from_isquare(62),
            Square::from_isquare(52),
            Square::from_isquare(42),
            Square::from_isquare(32),
            Square::from_isquare(22),
            Square::from_isquare(12),
            Square::from_isquare(93),
            Square::from_isquare(83),
            Square::from_isquare(73),
            Square::from_isquare(63),
            Square::from_isquare(53),
            Square::from_isquare(43),
            Square::from_isquare(33),
            Square::from_isquare(23),
            Square::from_isquare(13),
        ]
    }
}

/// 後手陣
pub struct GoteJin {}
impl GoteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_isquare(91),
            Square::from_isquare(81),
            Square::from_isquare(71),
            Square::from_isquare(61),
            Square::from_isquare(51),
            Square::from_isquare(41),
            Square::from_isquare(31),
            Square::from_isquare(21),
            Square::from_isquare(11),
            Square::from_isquare(92),
            Square::from_isquare(82),
            Square::from_isquare(72),
            Square::from_isquare(62),
            Square::from_isquare(52),
            Square::from_isquare(42),
            Square::from_isquare(32),
            Square::from_isquare(22),
            Square::from_isquare(12),
            Square::from_isquare(93),
            Square::from_isquare(83),
            Square::from_isquare(73),
            Square::from_isquare(63),
            Square::from_isquare(53),
            Square::from_isquare(43),
            Square::from_isquare(33),
            Square::from_isquare(23),
            Square::from_isquare(13),
        ]
    }
}
