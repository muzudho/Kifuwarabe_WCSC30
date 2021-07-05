pub mod position;

use crate::entities::cosmic::smart::features::HandAddress;
use crate::entities::law::cryptographic::num_to_lower_case;
use crate::take1base::Move;

/// マス番号。
/// 100以上は持駒。 K1=100, R1=101 .. P2=115
pub type Square = u8;

/// 升の検索等で、該当なしの場合
pub const SQUARE_NONE: Square = 0;

/// 盤上のマスなら真
pub fn is_board_square(sq: Square) -> bool {
    sq < 100
}
/// 持駒なら真
pub fn is_hand_square(sq: Square) -> bool {
    100 <= sq
}
//  /// マスでないなら真
// pub fn is_none_square(sq: Square) -> bool {
//     sq == SQUARE_NONE
// }
/// マス、または持駒なら真
pub fn is_square(sq: Square) -> bool {
    (11 <= sq && sq < 20)
        || (21 <= sq && sq < 30)
        || (31 <= sq && sq < 40)
        || (41 <= sq && sq < 50)
        || (51 <= sq && sq < 60)
        || (61 <= sq && sq < 70)
        || (71 <= sq && sq < 80)
        || (81 <= sq && sq < 90)
        || (91 <= sq && sq < 100)
        || (100 <= sq && sq < 116)
}

pub fn square_to_hand_address(sq: Square) -> HandAddress {
    match sq {
        100 => HandAddress::King1,
        101 => HandAddress::Rook1,
        102 => HandAddress::Bishop1,
        103 => HandAddress::Gold1,
        104 => HandAddress::Silver1,
        105 => HandAddress::Knight1,
        106 => HandAddress::Lance1,
        107 => HandAddress::Pawn1,
        108 => HandAddress::King2,
        109 => HandAddress::Rook2,
        110 => HandAddress::Bishop2,
        111 => HandAddress::Gold2,
        112 => HandAddress::Silver2,
        113 => HandAddress::Knight2,
        114 => HandAddress::Lance2,
        115 => HandAddress::Pawn2,
        _ => panic!("(Err.44) Hand address fail"),
    }
}
pub fn hand_address_to_square(ha: HandAddress) -> Square {
    match ha {
        HandAddress::King1 => 100,
        HandAddress::Rook1 => 101,
        HandAddress::Bishop1 => 102,
        HandAddress::Gold1 => 103,
        HandAddress::Silver1 => 104,
        HandAddress::Knight1 => 105,
        HandAddress::Lance1 => 106,
        HandAddress::Pawn1 => 107,
        HandAddress::King2 => 108,
        HandAddress::Rook2 => 109,
        HandAddress::Bishop2 => 110,
        HandAddress::Gold2 => 111,
        HandAddress::Silver2 => 112,
        HandAddress::Knight2 => 113,
        HandAddress::Lance2 => 114,
        HandAddress::Pawn2 => 115,
        // _ => panic!("(Err.44) Hand address fail"),
    }
}

pub fn destructure_move(num: Move) -> (Square, Square, bool) {
    // 移動元マス
    // .pdd dddd dsss ssss - num
    // 0000 0000 0111 1111 - Mask 0x007f
    let from = (num & 0x007f) as Square;

    // 移動先マス
    // .pdd dddd dsss ssss - num
    // 0011 1111 1000 0000 - Mask 0x3f80
    // 演算子の優先順位は `&` より `>>` の方が高いことに注意（＾～＾）
    let to = ((num & 0x3f80) >> 7) as Square;

    // 成
    // .pdd dddd dsss ssss - num
    // 0100 0000 0000 0000 - Mask 0x4000
    let promote = ((num & 0x4000) >> 14) == 1;

    return (from, to, promote);
}

pub fn to_move_code(move_: Move) -> String {
    let (from, to, promote) = destructure_move(move_);
    let from_file = from / 10;
    let from_rank = from % 10;
    let to_file = to / 10;
    let to_rank = to % 10;

    if 99 < from {
        // 打
        let drop = match from {
            101 | 109 => "R*",
            102 | 110 => "B*",
            103 | 111 => "G*",
            104 | 112 => "S*",
            105 | 113 => "N*",
            106 | 114 => "L*",
            107 | 115 => "P*",
            _ => panic!("(Err.46) drop fail"),
        };
        format!(
            "{}{}{}{}",
            drop,
            to_file,
            num_to_lower_case(to_rank.into()),
            if promote { "+" } else { "" }
        )
    } else {
        // 盤上
        format!(
            "{}{}{}{}{}",
            from_file,
            num_to_lower_case(from_rank.into()),
            to_file,
            num_to_lower_case(to_rank.into()),
            if promote { "+" } else { "" }
        )
    }
}
