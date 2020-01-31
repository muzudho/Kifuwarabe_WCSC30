//!
//! 駒の動く方向
//!

use std::fmt;

/**
 * 後手から見た盤を想像すること。筋、段を第一象限と同じ向きに合わせる。
 * 駒が戻る方向10方向。東から反時計回り。boolは長い利きなら真
 */
#[derive(Clone)]
pub enum PieceDirection {
    // 東
    E(bool),
    // 北東
    NE(bool),
    // 北北東（桂馬が戻る動き）
    NNE,
    // 北
    N(bool),
    // 北北西（桂馬が戻る動き）
    NNW,
    // 北西
    NW(bool),
    // 西
    W(bool),
    // 南西
    SW(bool),
    // 南南西（桂馬の動き）
    SSW,
    // 南
    S(bool),
    // 南南東（桂馬の動き）
    SSE,
    // 南東
    SE(bool),
    // 要素数より1小さい数。エラー値用に使っても可
    Owari,
}
impl fmt::Display for PieceDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceDirection::*;
        match *self {
            E(b) => {
                if b {
                    write!(f, "長東")
                } else {
                    write!(f, "東")
                }
            }
            NE(b) => {
                if b {
                    write!(f, "長北東")
                } else {
                    write!(f, "北東")
                }
            }
            NNE => write!(f, "北北東"),
            N(b) => {
                if b {
                    write!(f, "長北")
                } else {
                    write!(f, "北")
                }
            }
            NNW => write!(f, "北北西"),
            NW(b) => {
                if b {
                    write!(f, "長北西")
                } else {
                    write!(f, "北西")
                }
            }
            W(b) => {
                if b {
                    write!(f, "長西")
                } else {
                    write!(f, "西")
                }
            }
            SW(b) => {
                if b {
                    write!(f, "長南西")
                } else {
                    write!(f, "南西")
                }
            }
            SSW => write!(f, "南南西"),
            S(b) => {
                if b {
                    write!(f, "長南")
                } else {
                    write!(f, "南")
                }
            }
            SSE => write!(f, "南南東"),
            SE(b) => {
                if b {
                    write!(f, "長南東")
                } else {
                    write!(f, "南東")
                }
            }
            Owari => write!(f, "×"),
        }
    }
}
