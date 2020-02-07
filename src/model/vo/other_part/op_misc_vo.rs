#![allow(dead_code)]
//!
//! いろんな値、定義☆（＾～＾）
//!

/********
 * 局面 *
 ********/
pub enum KyNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/**************
 * 予想の結果 *
 **************/
pub enum DoingResult {
    // 起こった
    Done,
    // 起こらなかった
    None,
    // それ以外
    Owari,
}
