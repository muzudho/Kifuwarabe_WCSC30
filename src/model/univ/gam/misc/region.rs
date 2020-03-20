//!
//! 陣
//!

use crate::model::univ::gam::misc::square::*;

/// 先手陣
pub struct SenteJin {}
impl SenteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_usquare(91),
            Square::from_usquare(81),
            Square::from_usquare(71),
            Square::from_usquare(61),
            Square::from_usquare(51),
            Square::from_usquare(41),
            Square::from_usquare(31),
            Square::from_usquare(21),
            Square::from_usquare(11),
            Square::from_usquare(92),
            Square::from_usquare(82),
            Square::from_usquare(72),
            Square::from_usquare(62),
            Square::from_usquare(52),
            Square::from_usquare(42),
            Square::from_usquare(32),
            Square::from_usquare(22),
            Square::from_usquare(12),
            Square::from_usquare(93),
            Square::from_usquare(83),
            Square::from_usquare(73),
            Square::from_usquare(63),
            Square::from_usquare(53),
            Square::from_usquare(43),
            Square::from_usquare(33),
            Square::from_usquare(23),
            Square::from_usquare(13),
        ]
    }
}

/// 後手陣
pub struct GoteJin {}
impl GoteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_usquare(91),
            Square::from_usquare(81),
            Square::from_usquare(71),
            Square::from_usquare(61),
            Square::from_usquare(51),
            Square::from_usquare(41),
            Square::from_usquare(31),
            Square::from_usquare(21),
            Square::from_usquare(11),
            Square::from_usquare(92),
            Square::from_usquare(82),
            Square::from_usquare(72),
            Square::from_usquare(62),
            Square::from_usquare(52),
            Square::from_usquare(42),
            Square::from_usquare(32),
            Square::from_usquare(22),
            Square::from_usquare(12),
            Square::from_usquare(93),
            Square::from_usquare(83),
            Square::from_usquare(73),
            Square::from_usquare(63),
            Square::from_usquare(53),
            Square::from_usquare(43),
            Square::from_usquare(33),
            Square::from_usquare(23),
            Square::from_usquare(13),
        ]
    }
}
