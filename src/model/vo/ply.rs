//!
//! 手目
//!

/**
 * 手目数。何手目まで指せるか。
 * 棋譜を残す配列のサイズでもある。
 * 大会ルールが 256手として、終端子として投了を１個入れておけるようにする。
 */
pub const TEME_LN: usize = 257;

/**
 * 同一局面何回で千日手
 */
pub const SENNTITE_NUM: i8 = 4;