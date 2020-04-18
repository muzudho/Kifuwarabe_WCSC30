//!
//! 現局面を使った指し手生成
//!

use crate::controller::movement_generation::movements::*;
use crate::speed_of_light::*;
use crate::universe::game::board::square::Square;
use crate::universe::game::game::Game;
use crate::universe::game::position::person::Person;
use std::collections::HashSet;

/// 現局面の指し手を返すぜ☆（＾～＾）
/// 利きがどのように変わるかも返して欲しいぜ☆（＾～＾）
pub fn generate_movement(
    game: &mut Game,
    speed_of_light: &SpeedOfLight,
    movement_set: &mut HashSet<u64>,
) {
    // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
    get_potential_movement(&game, &speed_of_light, &mut |movement| {
        &movement_set.insert(movement);
    });
}

///
/// 現局面の、任意の移動先升の、
/// - 盤上の駒の移動
/// - 打
/// の指し手を生成。
///
/// 王手回避漏れや、千日手などのチェックは行っていない
///
/// https://doc.rust-lang.org/std/ops/trait.FnMut.html
///
pub fn get_potential_movement<F1>(
    game: &Game,
    speed_of_light: &SpeedOfLight,
    callback_movement: &mut F1,
) where
    F1: FnMut(u64),
{
    // 盤上の駒の移動。
    MGMovements::make_movements_on_board(
        game.history.get_phase(&Person::Friend),
        &game.position.current_board,
        &speed_of_light,
        callback_movement,
    );
    // 持ち駒の打。
    MGMovements::make_movements_on_hand(game, &speed_of_light, callback_movement);
}

pub struct MGSquares {}
impl MGSquares {
    /// 全升☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank_src in 1..10 {
            for file_src in (1..10).rev() {
                callback(Square::from_file_rank(file_src, rank_src));
            }
        }
    }
}
