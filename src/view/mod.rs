use crate::entities::spaceship::equipment::Beam;
use crate::entities::spaceship::equipment::DestinationDisplay;
use crate::entities::spaceship::equipment::PvString;
use crate::position::destructure_move;
use crate::position::position::Position;
use crate::position::to_move_code;
use crate::search::CentiPawn;
use crate::take1base::Move;

/// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
pub fn print_move_list(title: &str, position: &Position, move_list: &Vec<Move>) {
    Beam::shoot(&format!("+\n| {}", title));
    Beam::shoot(&format!("| Moves count={}", move_list.len()));
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
        Beam::shoot(&format!("| [{}] {}", i, move_name));
    }
    Beam::shoot("+");
}

// マスの一覧を表示するぜ☆（＾～＾）
// pub fn print_sq_list(title: &str, sq_list: &Vec<Square>) {
//     Beam::shoot(&format!("+\n| {}", title));
//     Beam::shoot(&format!("| Square count={}", sq_list.len()));
//     // ソート
//     let mut sq_list2 = sq_list.clone();
//     sq_list2.sort();

//     for (i, sq) in sq_list2.into_iter().enumerate() {
//         Beam::shoot(&format!("| [{}] {}", i, sq));
//     }
//     Beam::shoot("+");
// }

/// 情報表示
pub fn print_info(
    display: &mut DestinationDisplay,
    cur_depth: Option<usize>,
    state_nodes_nps: Option<(u64, u64)>,
    value: Option<CentiPawn>,
    move_: Option<Move>,
    pv_string: &Option<PvString>,
) {
    // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
    Beam::shoot(&format!(
        "info{}{}{}{} currmove {}{}",
        // 思考を開始してからのミリ秒☆（＾～＾）
        if let Some(pv_string_val) = pv_string {
            match pv_string_val {
                PvString::PV(msec, _pv) => format!(" time {}", msec),
                PvString::String(_x) => "".to_string(),
            }
        } else {
            "".to_string()
        },
        if let Some(num) = cur_depth {
            // 単に読み筋の長さ☆（＾～＾）
            format!(" depth {}", num)
        } else {
            "".to_string()
        },
        if let Some((state_node, nps)) = state_nodes_nps {
            format!(" nodes {} nps {}", state_node, nps)
        } else {
            "".to_string()
        },
        if let Some(value) = value {
            // // 自分が勝つ
            // " score mate +".to_string()
            // // 自分が負ける
            // " score mate -".to_string()
            format!(" score cp {}", value)
        } else {
            "".to_string()
        },
        if let Some(move_) = move_ {
            format!("{}", to_move_code(move_))
        } else {
            "".to_string()
        },
        if let Some(pv_string) = pv_string {
            match pv_string {
                PvString::PV(_sec, pv) => format!(" pv {}", pv),
                PvString::String(x) => format!(" string {}", x),
            }
        } else {
            "".to_string()
        }
    ));
    display.first = false;
    display.previous = display.stopwatch.elapsed();
}
