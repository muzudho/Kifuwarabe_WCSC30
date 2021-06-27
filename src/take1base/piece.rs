use num_derive::FromPrimitive;

pub const PIECE_MEANING_LEN: usize = 28;

///
/// 先後付きの駒と空白。
/// 接尾辞の 1 は▲先手、 2 は▽後手。
///
// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum Piece {
    // ▲玉 King
    K1,
    // ▲飛 Rook
    R1,
    // ▲角 Bishop
    B1,
    // ▲金 Gold
    G1,
    // ▲銀 Silver
    S1,
    // ▲桂 Knight
    N1,
    // ▲香 Lance
    L1,
    // ▲歩 Pawn
    P1,
    // ▲竜 Promoted Rook
    PR1,
    // ▲馬 Promoted Bishop
    PB1,
    // ▲全 Promoted Silver
    PS1,
    // ▲圭 Promoted Knight
    PN1,
    // ▲杏 Promoted Lance
    PL1,
    // ▲と Promoted Pawn
    PP1,
    // ▽玉
    K2,
    // ▽飛
    R2,
    // ▽角
    B2,
    // ▽金
    G2,
    // ▽銀
    S2,
    // ▽桂
    N2,
    // ▽香
    L2,
    // ▽歩
    P2,
    // ▽竜
    PR2,
    // ▽馬
    PB2,
    // ▽全
    PS2,
    // ▽圭
    PN2,
    // ▽杏
    PL2,
    // ▽と
    PP2,
}
