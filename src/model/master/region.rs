//!
//! 陣
//!

use super::super::super::model::master::square::*;

/// 先手陣
pub struct SenteJin {}
impl SenteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_umasu(91),
            Square::from_umasu(81),
            Square::from_umasu(71),
            Square::from_umasu(61),
            Square::from_umasu(51),
            Square::from_umasu(41),
            Square::from_umasu(31),
            Square::from_umasu(21),
            Square::from_umasu(11),
            Square::from_umasu(92),
            Square::from_umasu(82),
            Square::from_umasu(72),
            Square::from_umasu(62),
            Square::from_umasu(52),
            Square::from_umasu(42),
            Square::from_umasu(32),
            Square::from_umasu(22),
            Square::from_umasu(12),
            Square::from_umasu(93),
            Square::from_umasu(83),
            Square::from_umasu(73),
            Square::from_umasu(63),
            Square::from_umasu(53),
            Square::from_umasu(43),
            Square::from_umasu(33),
            Square::from_umasu(23),
            Square::from_umasu(13),
        ]
    }
}

/// 後手陣
pub struct GoteJin {}
impl GoteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            Square::from_umasu(91),
            Square::from_umasu(81),
            Square::from_umasu(71),
            Square::from_umasu(61),
            Square::from_umasu(51),
            Square::from_umasu(41),
            Square::from_umasu(31),
            Square::from_umasu(21),
            Square::from_umasu(11),
            Square::from_umasu(92),
            Square::from_umasu(82),
            Square::from_umasu(72),
            Square::from_umasu(62),
            Square::from_umasu(52),
            Square::from_umasu(42),
            Square::from_umasu(32),
            Square::from_umasu(22),
            Square::from_umasu(12),
            Square::from_umasu(93),
            Square::from_umasu(83),
            Square::from_umasu(73),
            Square::from_umasu(63),
            Square::from_umasu(53),
            Square::from_umasu(43),
            Square::from_umasu(33),
            Square::from_umasu(23),
            Square::from_umasu(13),
        ]
    }
}
