//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

//use crate::entities::cosmic::smart::square::BOARD_MEMORY_AREA;

use num_derive::FromPrimitive;
use std::fmt;

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const HAND_MAX: usize = 18;
// Note: 持ち駒には玉も含むぜ☆（＾～＾）
pub const HAND_ADDRESS_LEN: usize = 16;
pub static PIECE_WHITE_SPACE: &str = "    ";

pub const PIECE_TYPE_LEN: usize = 14;

/// USIでCopyするので、Copyが要る。
#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    // 玉 King
    K,
    // 飛 Rook
    R,
    // 角 Bishop
    B,
    // 金 Gold
    G,
    // 銀 Silver
    S,
    // 桂 Knight
    N,
    // 香 Lance
    L,
    // 歩 Pawn
    P,
    // 竜 Promoted Rook (Dragon)
    PR,
    // 馬 Promoted Bishop (Horse)
    PB,
    // 全 Promoted Silver
    PS,
    // 圭 Promoted Knight
    PN,
    // 杏 Promoted Lance
    PL,
    // と Promoted Pawn
    PP,
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceType::*;
        match *self {
            K => write!(f, "ら"),
            R => write!(f, "き"),
            B => write!(f, "ぞ"),
            G => write!(f, "い"),
            S => write!(f, "ね"),
            N => write!(f, "う"),
            L => write!(f, "い"),
            P => write!(f, "ひ"),
            PR => write!(f, "PK"),
            PB => write!(f, "PZ"),
            PS => write!(f, "PN"),
            PN => write!(f, "PU"),
            PL => write!(f, "PS"),
            PP => write!(f, "PH"),
        }
    }
}

pub const HAND_ADDRESS_TYPE_LEN: usize = 8;
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum HandType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub enum HandPiece {
    King1,
    Rook1,
    Bishop1,
    Gold1,
    Silver1,
    Knight1,
    Lance1,
    Pawn1,
    King2,
    Rook2,
    Bishop2,
    Gold2,
    Silver2,
    Knight2,
    Lance2,
    Pawn2,
}

/*
// 利きボード☆（＾～＾）
#[derive(Clone, Copy)]
pub struct ControlBoard {
    position: [isize; BOARD_MEMORY_AREA as usize],
}
impl Default for ControlBoard {
    fn default() -> Self {
        ControlBoard {
            position: [0; BOARD_MEMORY_AREA as usize],
        }
    }
}
impl ControlBoard {
    pub fn get(&self, index: usize) -> isize {
        self.position[index]
    }
    pub fn add(&mut self, index: usize, offset: isize) {
        self.position[index] += offset
    }
}
*/
