//!
//! 陣
//!

use crate::model::univ::gam::misc::square::*;

/// 先手陣
pub struct SenteJin {}
impl SenteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_address(91),
            Square::from_address(81),
            Square::from_address(71),
            Square::from_address(61),
            Square::from_address(51),
            Square::from_address(41),
            Square::from_address(31),
            Square::from_address(21),
            Square::from_address(11),
            Square::from_address(92),
            Square::from_address(82),
            Square::from_address(72),
            Square::from_address(62),
            Square::from_address(52),
            Square::from_address(42),
            Square::from_address(32),
            Square::from_address(22),
            Square::from_address(12),
            Square::from_address(93),
            Square::from_address(83),
            Square::from_address(73),
            Square::from_address(63),
            Square::from_address(53),
            Square::from_address(43),
            Square::from_address(33),
            Square::from_address(23),
            Square::from_address(13),
        ]
    }
}

/// 後手陣
pub struct GoteJin {}
impl GoteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_address(91),
            Square::from_address(81),
            Square::from_address(71),
            Square::from_address(61),
            Square::from_address(51),
            Square::from_address(41),
            Square::from_address(31),
            Square::from_address(21),
            Square::from_address(11),
            Square::from_address(92),
            Square::from_address(82),
            Square::from_address(72),
            Square::from_address(62),
            Square::from_address(52),
            Square::from_address(42),
            Square::from_address(32),
            Square::from_address(22),
            Square::from_address(12),
            Square::from_address(93),
            Square::from_address(83),
            Square::from_address(73),
            Square::from_address(63),
            Square::from_address(53),
            Square::from_address(43),
            Square::from_address(33),
            Square::from_address(23),
            Square::from_address(13),
        ]
    }
}
