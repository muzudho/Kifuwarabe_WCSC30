use std::fmt;

pub const KMS_LN: usize = 16;
// 先後なしの駒と空白
#[derive(Copy, Clone)]
pub enum PieceType {
    // らいおん
    R,
    // きりん
    K,
    // ぞう
    Z,
    // いぬ
    I,
    // ねこ
    N,
    // うさぎ
    U,
    // いのしし
    S,
    // ひよこ
    H,
    // ぱわーあっぷきりん
    PK,
    // ぱわーあっぷぞう
    PZ,
    // ぱわーあっぷねこ
    PN,
    // ぱわーあっぷうさぎ
    PU,
    // ぱわーあっぷいのしし
    PS,
    // ぱわーあっぷひよこ
    PH,
    // 空マス
    Kara,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari,
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceType::*;
        match *self {
            R => write!(f, "ら"),
            K => write!(f, "き"),
            Z => write!(f, "ぞ"),
            I => write!(f, "い"),
            N => write!(f, "ね"),
            U => write!(f, "う"),
            S => write!(f, "い"),
            H => write!(f, "ひ"),
            PK => write!(f, "PK"),
            PZ => write!(f, "PZ"),
            PN => write!(f, "PN"),
            PU => write!(f, "PU"),
            PS => write!(f, "PS"),
            PH => write!(f, "PH"),
            Kara => write!(f, "　"),
            Owari => write!(f, "×"),
        }
    }
}
