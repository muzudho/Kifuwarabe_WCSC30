pub mod position;
pub mod rel_square;
pub mod rotation;
pub mod square;

use crate::entities::cosmic::smart::features::HandPiece;
use crate::entities::cosmic::smart::features::HandType;
use crate::entities::law::cryptographic::num_to_lower_case;
use crate::record::RESIGN_MOVE;
use crate::take1base::Move;

//
// 盤、升、筋、段
//

// #[allow(non_camel_case_types)]
// pub type isquare = isize;

// 配列サイズなので 1 大きめだぜ☆（＾～＾）
pub const BOARD_MEMORY_AREA: u8 = 100;

/// 筋、段は 1 から始まる、という明示。
/// usize が速い☆（＾～＾）
pub const FILE_0: u8 = 0;
pub const FILE_1: u8 = 1;
pub const FILE_9: u8 = 9;
pub const FILE_10: u8 = 10;
// pub const FILE_11: u8 = 11;
pub const RANK_0: u8 = 0;
pub const RANK_1: u8 = 1;
pub const RANK_2: u8 = 2;
pub const RANK_3: u8 = 3;
pub const RANK_4: u8 = 4;
// pub const RANK_5: u8 = 5;
pub const RANK_6: u8 = 6;
pub const RANK_7: u8 = 7;
pub const RANK_8: u8 = 8; //うさぎの打てる段の上限
pub const RANK_9: u8 = 9;
pub const RANK_10: u8 = 10;
// pub const RANK_11: u8 = 11;

/// 引き算もするところでは unsigned ではダメなところもある☆（＾～＾）
// pub const I_FILE_0: i8 = 0;
// pub const I_FILE_1: i8 = 1;
// pub const I_FILE_9: i8 = 9;
// pub const I_FILE_10: i8 = 10;
// pub const I_RANK_0: i8 = 0;
// pub const I_RANK_1: i8 = 1;
// pub const I_RANK_2: i8 = 2;
// pub const I_RANK_3: i8 = 3;
// pub const I_RANK_4: i8 = 4;
// pub const I_RANK_6: i8 = 6;
// pub const I_RANK_7: i8 = 7;
// pub const I_RANK_8: i8 = 8; //うさぎの打てる段の上限
// pub const I_RANK_9: i8 = 9;
// pub const I_RANK_10: i8 = 10;

/// マス番号。
/// 100以上は持駒。 K1=100, R1=101 .. P2=115
/// Square is shogi coordinate. file*10+rank.
///
///           North
///   91 81 71 61 51 41 31 21 11
///   92 82 72 62 52 42 32 22 12
/// W 93 83 73 63 53 43 33 23 13 E
/// E 94 84 74 64 54 44 34 24 14 A
/// S 95 85 75 65 55 45 35 25 15 S
/// T 96 86 76 66 56 46 36 26 16 T
///   97 87 77 67 57 47 37 27 17
///   98 88 78 68 58 48 38 28 18
///   99 89 79 69 59 49 39 29 19
///           Source
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square(u8);

/// 升の検索等で、該当なしの場合
pub const SQUARE_NONE: Square = Square(0);

pub fn hand_type_to_square(ha: HandPiece) -> Square {
    match ha {
        HandPiece::King1 => Square(100),
        HandPiece::Rook1 => Square(101),
        HandPiece::Bishop1 => Square(102),
        HandPiece::Gold1 => Square(103),
        HandPiece::Silver1 => Square(104),
        HandPiece::Knight1 => Square(105),
        HandPiece::Lance1 => Square(106),
        HandPiece::Pawn1 => Square(107),
        HandPiece::King2 => Square(108),
        HandPiece::Rook2 => Square(109),
        HandPiece::Bishop2 => Square(110),
        HandPiece::Gold2 => Square(111),
        HandPiece::Silver2 => Square(112),
        HandPiece::Knight2 => Square(113),
        HandPiece::Lance2 => Square(114),
        HandPiece::Pawn2 => Square(115),
        // _ => panic!("(Err.44) Hand address fail"),
    }
}
pub fn square_to_hand_type(sq: Square) -> HandType {
    match sq.number() {
        100 | 108 => HandType::King,
        101 | 109 => HandType::Rook,
        102 | 110 => HandType::Bishop,
        103 | 111 => HandType::Gold,
        104 | 112 => HandType::Silver,
        105 | 113 => HandType::Knight,
        106 | 114 => HandType::Lance,
        107 | 115 => HandType::Pawn,
        _ => panic!("square_to_hand_type sq={}", sq.number()),
    }
}

pub fn square_to_hand_piece(sq: Square) -> HandPiece {
    match sq.number() {
        100 => HandPiece::King1,
        101 => HandPiece::Rook1,
        102 => HandPiece::Bishop1,
        103 => HandPiece::Gold1,
        104 => HandPiece::Silver1,
        105 => HandPiece::Knight1,
        106 => HandPiece::Lance1,
        107 => HandPiece::Pawn1,
        108 => HandPiece::King2,
        109 => HandPiece::Rook2,
        110 => HandPiece::Bishop2,
        111 => HandPiece::Gold2,
        112 => HandPiece::Silver2,
        113 => HandPiece::Knight2,
        114 => HandPiece::Lance2,
        115 => HandPiece::Pawn2,
        _ => panic!("(Err.44) Hand address fail"),
    }
}

/// 相対番地。絶対番地と同じだが、回転の中心を原点に固定した操作が行われるぜ☆（＾～＾）
///
/// 18  8  -2 -12 -22
/// 19  9  -1 -11 -21
/// 20 10   0 -10 -20
/// 21 11   1 - 9 -19
/// 22 12   2 - 8 -18
///
/// file, rank から 相対番地は作れますが、相対番地から file, rank を作ることはできません(不定)。
/// そこから、 file, rank で持ちます。
///
/// メモリを使わないようにしようぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub struct RelAdr {
    file: i8,
    rank: i8,
}

pub fn destructure_move(m: Move) -> (Square, Square, bool) {
    // 移動元マス
    // .pdd dddd dsss ssss - m
    // 0000 0000 0111 1111 - Mask 0x007f
    let from = Square((m & 0x007f) as u8);

    // 移動先マス
    // .pdd dddd dsss ssss - m
    // 0011 1111 1000 0000 - Mask 0x3f80
    // 演算子の優先順位は `&` より `>>` の方が高いことに注意（＾～＾）
    let to = Square(((m & 0x3f80) >> 7) as u8);

    // 成
    // .pdd dddd dsss ssss - m
    // 0100 0000 0000 0000 - Mask 0x4000
    let promote = ((m & 0x4000) >> 14) == 1;

    return (from, to, promote);
}

/// sfen
pub fn to_move_code(move_: Move) -> String {
    if move_ == RESIGN_MOVE {
        return "resign".to_string();
    }
    let (from, to, promote) = destructure_move(move_);

    if from.is_hand() {
        // 打
        let drop = from.to_drop_code();
        format!(
            "{}{}{}{}",
            drop,
            to.file(),
            num_to_lower_case(to.rank().into()),
            if promote { "+" } else { "" }
        )
    } else {
        // 盤上
        format!(
            "{}{}{}{}{}",
            from.file(),
            num_to_lower_case(from.rank().into()),
            to.file(),
            num_to_lower_case(to.rank().into()),
            if promote { "+" } else { "" }
        )
    }
}
