//!
//! 陣
//!

use super::super::super::model::master::place::*;

/**
 * 先手陣
 */
pub struct SenteJin {}
impl SenteJin {
    pub fn to_elm() -> Vec<umasu> {
        vec![
            91, 81, 71, 61, 51, 41, 31, 21, 11, 92, 82, 72, 62, 52, 42, 32, 22, 12, 93, 83, 73, 63,
            53, 43, 33, 23, 13,
        ]
    }
}

/**
 * 後手陣
 */
pub struct GoteJin {}
impl GoteJin {
    pub fn to_elm() -> Vec<umasu> {
        vec![
            91, 81, 71, 61, 51, 41, 31, 21, 11, 92, 82, 72, 62, 52, 42, 32, 22, 12, 93, 83, 73, 63,
            53, 43, 33, 23, 13,
        ]
    }
}
