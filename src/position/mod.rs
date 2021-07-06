pub mod position;

use crate::entities::cosmic::smart::features::HandAddress;
use crate::entities::cosmic::smart::square::RelAdr;
use crate::entities::cosmic::smart::square::FILE_0;
use crate::entities::cosmic::smart::square::FILE_10;
use crate::entities::cosmic::smart::square::RANK_0;
use crate::entities::cosmic::smart::square::RANK_10;
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
pub type Square = u8;

/// 升の検索等で、該当なしの場合
pub const SQUARE_NONE: Square = 0;

/// 盤上のマスなら真。（調べ方は、ざっくり）
pub fn is_board_square(sq: Square) -> bool {
    11 <= sq && sq < 100
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

pub fn rank(sq: Square) -> u8 {
    sq % 10
}
pub fn file(sq: Square) -> u8 {
    sq / 10
}

pub fn square_from(file: u8, rank: u8) -> Square {
    file * 10 + rank
}

/// 壁の中にいる☆（＾～＾）
pub fn square_wall(sq: Square) -> bool {
    file(sq) % 10 == 0 || rank(sq) % 10 == 0
}

pub fn square_offset(sq: Square, r: &RelAdr) -> Square {
    // TODO rankの符号はどうだったか……☆（＾～＾） 絶対番地の使い方をしてれば問題ないだろ☆（＾～＾）
    // TODO sum は負数になることもあり、そのときは明らかにイリーガルだぜ☆（＾～＾）
    let sum = (sq as isize + r.get_address()) as u8;

    // Initialize.
    let mut rank = sum % 10;
    let mut file = 0;
    // Carry.
    if 9 < rank {
        rank = rank % 10;
        file += 1;
    }
    file += sum / 10 % 10;
    // Carry over flow.
    if 9 < file {
        file = file % 10;
    }

    square_from(file, rank)
}

pub fn square_rotate_180(sq: Square) -> Square {
    let file = FILE_10 - file(sq);
    let rank = RANK_10 - rank(sq);
    debug_assert!(FILE_0 < file && file < FILE_10, "file={}", file);
    debug_assert!(RANK_0 < rank && rank < RANK_10, "rank={}", rank);
    square_from(file, rank)
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

/// sfen
pub fn to_move_code(move_: Move) -> String {
    if move_ == RESIGN_MOVE {
        return "resign".to_string();
    }
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
