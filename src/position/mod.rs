pub mod position;

use crate::entities::law::cryptographic::num_to_lower_case;
use crate::take1base::Move;

/// マス番号。
/// 100以上は持駒。 K1=100, R1=101 .. P2=115
pub type Square = u8;

/// 盤上にも、駒台にも無いとき
const SQUARE_NONE: Square = 0;

/// 盤上のマスなら真
fn IsBoardSquare(sq: Square) -> bool {
    sq < 100
}
/// 持駒なら真
fn IsHandSquare(sq: Square) -> bool {
    100 <= sq
}
/// マスでないなら真
fn IsNoneSquare(sq: Square) -> bool {
    sq == SQUARE_NONE
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
