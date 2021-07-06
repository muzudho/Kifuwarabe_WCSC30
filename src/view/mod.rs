use crate::entities::spaceship::equipment::Beam;
use crate::position::destructure_move;
use crate::position::position::Position;
use crate::position::to_move_code;
use crate::position::Square;
use crate::take1base::Move;

/// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
pub fn print_move_list(position: &Position, move_list: &Vec<Move>) {
    Beam::shoot(&format!("Moves count={}", move_list.len()));
    // 辞書順ソート
    let mut move_names = Vec::new();
    for move_ in move_list {
        let (_, to, _) = destructure_move(*move_);
        let ss_str = format!(
            "{}{}",
            format!("{}", to_move_code(*move_)),
            if let Some(captured) = position.piece_at(to) {
                format!(" ({})", captured.piece)
            } else {
                "".to_string()
            }
        );
        move_names.push(ss_str);
    }
    // move_names.sort();
    move_names.sort_by(|y_str, x_str| {
        let y_arr: Vec<_> = y_str.chars().collect();
        let x_arr: Vec<_> = x_str.chars().collect();
        use std::cmp::min;
        let len = min(y_arr.len(), x_arr.len());

        use std::cmp::Ordering;
        for i in 0..len {
            match x_arr[i].cmp(&y_arr[i]) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }

        // Returns Ordering::Greater, Ordering::Less, Ordering::Equal.
        x_arr.len().cmp(&y_arr.len())
    });
    move_names.reverse();

    for (i, move_name) in move_names.into_iter().enumerate() {
        Beam::shoot(&format!("[{}] {}", i, move_name));
    }
}

/// マスの一覧を表示するぜ☆（＾～＾）
pub fn print_sq_list(sq_list: &Vec<Square>) {
    Beam::shoot(&format!("+\n| Square count={}", sq_list.len()));
    // ソート
    let mut sq_list2 = sq_list.clone();
    sq_list2.sort();

    for (i, sq) in sq_list2.into_iter().enumerate() {
        Beam::shoot(&format!("| [{}] {}", i, sq));
    }
    Beam::shoot("+");
}
