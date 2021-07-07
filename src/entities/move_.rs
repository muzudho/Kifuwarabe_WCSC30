use crate::entities::cosmic::smart::features::HandType;
use crate::position::destructure_move;
use crate::position::square_to_hand_type;
use crate::position::Square;
use crate::take1base::Move;

/// 初期値として 移動元マス、移動先マス、成りの有無 を指定してください
pub fn new_move(from: Square, to: Square, promote: bool) -> Move {
    let mut num: u16;

    // 移動元マス
    // .... .... .sss ssss
    // 11～99: 盤
    // 100～115: 持駒
    num = from.number() as u16;

    // 移動先マス
    // ..dd dddd d... ....
    num += (to.number() as u16) << 7;

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
pub fn to_move_object(num: Move) -> (Option<Square>, Square, bool, Option<HandType>) {
    let (from, to, promote) = destructure_move(num);

    if from.is_board() {
        // 盤上
        return (Some(from), to, promote, None);
    } else {
        // 打
        let hand = square_to_hand_type(from);

        return (None, to, promote, Some(hand));
    }
}
