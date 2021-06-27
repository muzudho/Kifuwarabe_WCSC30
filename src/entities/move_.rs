use crate::entities::cosmic::recording::Phase;
use crate::entities::movement::Movement;
use crate::take1base::Move;

/// 初期値として 移動元マス、移動先マス、成りの有無 を指定してください
pub fn newMove(phase: Phase, movement: &Movement) -> Move {
    let mut num: u16 = 0;

    if let Some(src) = movement.source {
        // 移動元マス
        // .... .... .sss ssss
        num = src.address() as u16;
    } else if let Some(drp) = movement.drop {
        // 打
        match phase {
            First => match drp {
                King => {}
                Rook => {}
                Bishop => {}
                Gold => {}
                Silver => {}
                Knight => {}
                Lance => {}
                Pawn => {}
            },
            Second => match drp {
                King => {}
                Rook => {}
                Bishop => {}
                Gold => {}
                Silver => {}
                Knight => {}
                Lance => {}
                Pawn => {}
            },
        }
    } else {
        // 投了
        return 0;
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
