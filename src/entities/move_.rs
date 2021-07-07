use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::features::HandType;
use crate::position::destructure_move;
use crate::position::is_board_square;
use crate::position::Square;
use crate::take1base::Move;

/// 初期値として 移動元マス、移動先マス、成りの有無 を指定してください
pub fn new_move(from: Square, to: Square, promote: bool) -> Move {
    let mut num: u16;

    // 移動元マス
    // .... .... .sss ssss
    // 11～99: 盤
    // 100～115: 持駒
    num = from as u16;

    // 移動先マス
    // ..dd dddd d... ....
    num += (to as u16) << 7;

    if promote {
        // 成
        // .p.. .... .... ....
        num += 0x4000;
    }

    return num;
}

/// to_move_object - 移動元マス、移動先マス、成りの有無
///
/// # Returns
///
/// `Option<Square>` - from. 移動元升。Dropのときは None だぜ☆（＾～＾）
/// `Square` - to. 移動先升
/// `bool` - promote. 移動後に成るなら真
/// `Option<HandType>` - drop. 打の場合、打った駒種類
pub fn to_move_object(phase: Phase, num: Move) -> (Option<Square>, Square, bool, Option<HandType>) {
    let (from, to, promote) = destructure_move(num);

    if is_board_square(from) {
        // 盤上
        return (Some(from), to, promote, None);
    } else {
        // 打
        let hand = match phase {
            Phase::First => match from {
                100 => HandType::King,
                101 => HandType::Rook,
                102 => HandType::Bishop,
                103 => HandType::Gold,
                104 => HandType::Silver,
                105 => HandType::Knight,
                106 => HandType::Lance,
                107 => HandType::Pawn,
                _ => panic!("move_::to_move_object phase={} from={}", phase, from),
            },
            Phase::Second => match from {
                108 => HandType::King,
                109 => HandType::Rook,
                110 => HandType::Bishop,
                111 => HandType::Gold,
                112 => HandType::Silver,
                113 => HandType::Knight,
                114 => HandType::Lance,
                115 => HandType::Pawn,
                _ => panic!("move_::to_move_object phase={} from={}", phase, from),
            },
        };

        return (None, to, promote, Some(hand));
    }
}
