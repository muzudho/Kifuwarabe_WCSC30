//!
//! 駒
//!
//! 先後付き駒
//!

use super::super::super::controller::common::conv::*;
use super::phase::*;
use std::fmt;

/**
 * 先後付きの駒と空白
 */
#[derive(Copy, Clone)]
pub enum Piece {
    // ▼らいおん
    R0,
    // ▼きりん
    K0,
    // ▼ぞう
    Z0,
    // ▼いぬ
    I0,
    // ▼ねこ
    N0,
    // ▼うさぎ
    U0,
    // ▼いのしし
    S0,
    // ▼ひよこ
    H0,
    // ▼ぱわーあっぷきりん
    PK0,
    // ▼ぱわーあっぷぞう
    PZ0,
    // ▼ぱわーあっぷねこ
    PN0,
    // ▼ぱわーあっぷうさぎ
    PU0,
    // ▼ぱわーあっぷいのしし
    PS0,
    // ▼ぱわーあっぷひよこ
    PH0,
    // △ライオン
    R1,
    // △キリン
    K1,
    // △ゾウ
    Z1,
    // △イヌ
    I1,
    // △ネコ
    N1,
    // △ウサギ
    U1,
    // △イノシシ
    S1,
    // △ヒヨコ
    H1,
    // △パワーアップキリン
    PK1,
    // △パワーアップゾウ
    PZ1,
    // △パワーアップネコ
    PN1,
    // △パワーアップウサギ
    PU1,
    // △パワーアップイノシシ
    PS1,
    // △パワーアップヒヨコ
    PH1,
    // 空マス
    Kara,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Owari,
}

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX: usize = 18;
pub const KM_LN: usize = 30;
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::super::super::model::master::piece::Piece::*;
        match *self {
            R0 => write!(f, "▼ら"),
            K0 => write!(f, "▼き"),
            Z0 => write!(f, "▼ぞ"),
            I0 => write!(f, "▼い"),
            N0 => write!(f, "▼ね"),
            U0 => write!(f, "▼う"),
            S0 => write!(f, "▼し"),
            H0 => write!(f, "▼ひ"),
            PK0 => write!(f, "▼PK"),
            PZ0 => write!(f, "▼PZ"),
            PN0 => write!(f, "▼PN"),
            PU0 => write!(f, "▼PU"),
            PS0 => write!(f, "▼PS"),
            PH0 => write!(f, "▼PH"),
            R1 => write!(f, "△ラ"),
            K1 => write!(f, "△キ"),
            Z1 => write!(f, "△ゾ"),
            I1 => write!(f, "△イ"),
            N1 => write!(f, "△ネ"),
            U1 => write!(f, "△ウ"),
            S1 => write!(f, "△シ"),
            H1 => write!(f, "△ヒ"),
            PK1 => write!(f, "△pk"),
            PZ1 => write!(f, "△pz"),
            PN1 => write!(f, "△pn"),
            PU1 => write!(f, "△pu"),
            PS1 => write!(f, "△ps"),
            PH1 => write!(f, "△ph"),
            Kara => write!(f, "　　"),
            Owari => write!(f, "××"),
        }
    }
}

/**
 * 駒の一致比較
 */
pub fn match_km(a: &Piece, b: &Piece) -> bool {
    km_to_num(a) == km_to_num(b)
}

pub const KM_ARRAY_HALF_LN: usize = 14;
pub const KM_ARRAY_LN: usize = 28;
pub const KM_ARRAY: [Piece; KM_ARRAY_LN] = [
    Piece::R0,  // らいおん
    Piece::K0,  // きりん
    Piece::Z0,  // ぞう
    Piece::I0,  // いぬ
    Piece::N0,  // ねこ
    Piece::U0,  // うさぎ
    Piece::S0,  // いのしし
    Piece::H0,  // ひよこ
    Piece::PK0, // ぱわーあっぷきりん
    Piece::PZ0, // ぱわーあっぷぞう
    Piece::PN0, // ぱわーあっぷねこ
    Piece::PU0, // ぱわーあっぷうさぎ
    Piece::PS0, // ぱわーあっぷいのしし
    Piece::PH0, // ぱわーあっぷひよこ
    Piece::R1,  // らいおん
    Piece::K1,  // きりん
    Piece::Z1,  // ぞう
    Piece::I1,  // いぬ
    Piece::N1,  // ねこ
    Piece::U1,  // うさぎ
    Piece::S1,  // いのしし
    Piece::H1,  // ひよこ
    Piece::PK1, // ぱわーあっぷきりん
    Piece::PZ1, // ぱわーあっぷぞう
    Piece::PN1, // ぱわーあっぷねこ
    Piece::PU1, // ぱわーあっぷうさぎ
    Piece::PS1, // ぱわーあっぷいのしし
    Piece::PH1, // ぱわーあっぷひよこ
];
pub const SN_KM_ARRAY: [[Piece; KM_ARRAY_HALF_LN]; SN_LN] = [
    [
        Piece::R0,  // らいおん
        Piece::K0,  // きりん
        Piece::Z0,  // ぞう
        Piece::I0,  // いぬ
        Piece::N0,  // ねこ
        Piece::U0,  // うさぎ
        Piece::S0,  // いのしし
        Piece::H0,  // ひよこ
        Piece::PK0, // ぱわーあっぷきりん
        Piece::PZ0, // ぱわーあっぷぞう
        Piece::PN0, // ぱわーあっぷねこ
        Piece::PU0, // ぱわーあっぷうさぎ
        Piece::PS0, // ぱわーあっぷいのしし
        Piece::PH0, // ぱわーあっぷひよこ
    ],
    [
        Piece::R1,  // らいおん
        Piece::K1,  // きりん
        Piece::Z1,  // ぞう
        Piece::I1,  // いぬ
        Piece::N1,  // ねこ
        Piece::U1,  // うさぎ
        Piece::S1,  // いのしし
        Piece::H1,  // ひよこ
        Piece::PK1, // ぱわーあっぷきりん
        Piece::PZ1, // ぱわーあっぷぞう
        Piece::PN1, // ぱわーあっぷねこ
        Piece::PU1, // ぱわーあっぷうさぎ
        Piece::PS1, // ぱわーあっぷいのしし
        Piece::PH1, // ぱわーあっぷひよこ
    ],
    [
        Piece::Owari, // らいおん
        Piece::Owari, // きりん
        Piece::Owari, // ぞう
        Piece::Owari, // いぬ
        Piece::Owari, // ねこ
        Piece::Owari, // うさぎ
        Piece::Owari, // いのしし
        Piece::Owari, // ひよこ
        Piece::Owari, // ぱわーあっぷきりん
        Piece::Owari, // ぱわーあっぷぞう
        Piece::Owari, // ぱわーあっぷねこ
        Piece::Owari, // ぱわーあっぷうさぎ
        Piece::Owari, // ぱわーあっぷいのしし
        Piece::Owari, // ぱわーあっぷひよこ
    ],
];
