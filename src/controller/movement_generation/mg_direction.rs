//!
//! 駒の動ける方向☆（＾～＾）
//!

// 駒の動ける方向数、終端子込み
pub const KM_UGOKI_LN: usize = 9;

/// 駒の動ける方向☆（＾～＾）
pub struct MGDirection {}
impl MGDirection {
    /// 全方向☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(usize) -> bool,
    {
        // (方向)
        for i_dir in 0..KM_UGOKI_LN {
            if callback(i_dir) {
                break;
            }
        }
    }
}
