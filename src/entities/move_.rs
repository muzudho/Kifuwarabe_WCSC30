use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::features::HandAddressType;
use crate::entities::cosmic::smart::square::AbsoluteAddress;
use crate::entities::movement::Movement;
use crate::take1base::Move;

/*
/// 初期値として 移動元マス、移動先マス、成りの有無 を指定してください
pub fn new_move(phase: Phase, movement: &Movement) -> Move {
    let mut num: u16 = 0;

    if let Some(src) = movement.source {
        // 移動元マス
        // .... .... .sss ssss
        num = src.address() as u16;
    } else if let Some(drp) = movement.drop {
        // 打
        num = match phase {
            Phase::First => match drp {
                HandAddressType::King => 100,
                HandAddressType::Rook => 101,
                HandAddressType::Bishop => 102,
                HandAddressType::Gold => 103,
                HandAddressType::Silver => 104,
                HandAddressType::Knight => 105,
                HandAddressType::Lance => 106,
                HandAddressType::Pawn => 107,
            },
            Phase::Second => match drp {
                HandAddressType::King => 108,
                HandAddressType::Rook => 109,
                HandAddressType::Bishop => 110,
                HandAddressType::Gold => 111,
                HandAddressType::Silver => 112,
                HandAddressType::Knight => 113,
                HandAddressType::Lance => 114,
                HandAddressType::Pawn => 115,
            },
        };
    } else {
        panic!("move_::new_move srouce error")
    }

    // 移動先マス
    // ..dd dddd d... ....
    num += (movement.destination.address() as u16) << 7;

    if movement.promote {
        // 成
        // .p.. .... .... ....
        num += 0x4000;
    }

    return num;
}
*/

/// 初期値として 移動元マス、移動先マス、成りの有無 を指定してください
pub fn new_move2(
    phase: Phase,
    source: Option<u16>,
    destination: u16,
    promote: bool,
    drop: Option<HandAddressType>,
) -> Move {
    let mut num: u16;

    if let Some(src) = source {
        // 移動元マス
        // .... .... .sss ssss
        num = src;
    } else if let Some(drp) = drop {
        // 打
        num = match phase {
            Phase::First => match drp {
                HandAddressType::King => 100,
                HandAddressType::Rook => 101,
                HandAddressType::Bishop => 102,
                HandAddressType::Gold => 103,
                HandAddressType::Silver => 104,
                HandAddressType::Knight => 105,
                HandAddressType::Lance => 106,
                HandAddressType::Pawn => 107,
            },
            Phase::Second => match drp {
                HandAddressType::King => 108,
                HandAddressType::Rook => 109,
                HandAddressType::Bishop => 110,
                HandAddressType::Gold => 111,
                HandAddressType::Silver => 112,
                HandAddressType::Knight => 113,
                HandAddressType::Lance => 114,
                HandAddressType::Pawn => 115,
            },
        };
    } else {
        panic!("move_::new_move srouce error")
    }

    // 移動先マス
    // ..dd dddd d... ....
    num += destination << 7;

    if promote {
        // 成
        // .p.. .... .... ....
        num += 0x4000;
    }

    return num;
}

/// to_movement - 移動元マス、移動先マス、成りの有無
pub fn to_movement(phase: Phase, num: Move) -> Movement {
    // 移動元マス
    // .pdd dddd dsss ssss - num
    // 0000 0000 0111 1111 - Mask 0x007f
    let from = num & 0x007f;

    // 移動先マス
    // .pdd dddd dsss ssss - num
    // 0011 1111 1000 0000 - Mask 0x3f80
    // 演算子の優先順位は `&` より `>>` の方が高いことに注意（＾～＾）
    let to = (num & 0x3f80) >> 7;

    // 成
    // .pdd dddd dsss ssss - num
    // 0100 0000 0000 0000 - Mask 0x4000
    let promote = (num & 0x4000) >> 14;

    if from < 100 {
        // 盤上
        if let Some(dst) = AbsoluteAddress::from_absolute_address(to as usize) {
            return Movement::new(
                AbsoluteAddress::from_absolute_address(from as usize),
                dst,
                promote == 1,
                None,
            );
        } else {
            panic!("dst {}", to)
        }
    } else {
        // 打
        let hand = match phase {
            Phase::First => match num {
                100 => HandAddressType::King,
                101 => HandAddressType::Rook,
                102 => HandAddressType::Bishop,
                103 => HandAddressType::Gold,
                104 => HandAddressType::Silver,
                105 => HandAddressType::Knight,
                106 => HandAddressType::Lance,
                107 => HandAddressType::Pawn,
                _ => panic!("move_::to_movement num={}", num),
            },
            Phase::Second => match num {
                108 => HandAddressType::King,
                109 => HandAddressType::Rook,
                110 => HandAddressType::Bishop,
                111 => HandAddressType::Gold,
                112 => HandAddressType::Silver,
                113 => HandAddressType::Knight,
                114 => HandAddressType::Lance,
                115 => HandAddressType::Pawn,
                _ => panic!("move_::to_movement num={}", num),
            },
        };

        if let Some(dst) = AbsoluteAddress::from_absolute_address(to as usize) {
            return Movement::new(None, dst, promote == 1, Some(hand));
        } else {
            panic!("to={}", to)
        }
    }
}
