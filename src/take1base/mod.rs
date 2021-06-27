use num_derive::FromPrimitive;
use std::fmt;

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
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲、▽ が半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::take1base::Piece::*;
        match *self {
            K1 => write!(f, " ▲K "),
            R1 => write!(f, " ▲R "),
            B1 => write!(f, " ▲B "),
            G1 => write!(f, " ▲G "),
            S1 => write!(f, " ▲S "),
            N1 => write!(f, " ▲N "),
            L1 => write!(f, " ▲L "),
            P1 => write!(f, " ▲P "),
            PR1 => write!(f, " ▲PR"),
            PB1 => write!(f, " ▲PB"),
            PS1 => write!(f, " ▲PS"),
            PN1 => write!(f, " ▲PN"),
            PL1 => write!(f, " ▲PL"),
            PP1 => write!(f, " ▲PP"),
            K2 => write!(f, " ▽k "),
            R2 => write!(f, " ▽r "),
            B2 => write!(f, " ▽b "),
            G2 => write!(f, " ▽g "),
            S2 => write!(f, " ▽s "),
            N2 => write!(f, " ▽n "),
            L2 => write!(f, " ▽l "),
            P2 => write!(f, " ▽p "),
            PR2 => write!(f, " ▽pr"),
            PB2 => write!(f, " ▽pb"),
            PS2 => write!(f, " ▽ps"),
            PN2 => write!(f, " ▽pn"),
            PL2 => write!(f, " ▽pl"),
            PP2 => write!(f, " ▽pp"),
        }
    }
}

/// 指し手
pub type Move = u16;
