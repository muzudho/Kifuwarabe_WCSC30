pub mod position;
pub mod square;

use crate::entities::cosmic::smart::features::HandPiece;
use crate::entities::cosmic::smart::features::HandType;
use crate::entities::law::cryptographic::num_to_lower_case;
use crate::record::RESIGN_MOVE;
use crate::take1base::Move;

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
