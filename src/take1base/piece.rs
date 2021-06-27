use num_derive::FromPrimitive;

pub const PIECE_MEANING_LEN: usize = 28;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は▲先手、 2 は▽後手。
///
// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum Piece {
    // ▲玉
    K1,
    // ▲飛
    R1,
    // ▲角
    B1,
    // ▲金
    G1,
    // ▲銀
    S1,
    // ▲桂
    N1,
    // ▲香
    L1,
    // ▲歩
    P1,
    // ▲竜
    PR1,
    // ▲馬
    PB1,
    // ▲全
    PS1,
    // ▲圭
    PN1,
    // ▲杏
    PL1,
    // ▲と
    PP1,
    // ▽ライオン
    King2,
    // ▽キリン
    Rook2,
    // ▽ゾウ
    Bishop2,
    // ▽イヌ
    Gold2,
    // ▽ネコ
    Silver2,
    // ▽ウサギ
    Knight2,
    // ▽イノシシ
    Lance2,
    // ▽ヒヨコ
    Pawn2,
    // ▽パワーアップキリン
    Dragon2,
    // ▽パワーアップゾウ
    Horse2,
    // ▽パワーアップネコ
    PromotedSilver2,
    // ▽パワーアップウサギ
    PromotedKnight2,
    // ▽パワーアップイノシシ
    PromotedLance2,
    // ▽パワーアップヒヨコ
    PromotedPawn2,
}
