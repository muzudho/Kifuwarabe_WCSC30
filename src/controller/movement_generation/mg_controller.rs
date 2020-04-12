//!
//! 現局面を使った指し手生成
//!

use super::squares::*;
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::movement_generation::movements::*;
use crate::model::univ::gam::board::*;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece_struct::PieceStruct;
use crate::model::univ::gam::misc::square::*;
use crate::model::univ::game::Game;
use crate::model::univ::speed_of_light::*;
use std::collections::HashSet;

/// 現局面の指し手を返すぜ☆（＾～＾）
/// 利きがどのように変わるかも返して欲しいぜ☆（＾～＾）
pub fn generate_movement(
    game: &mut Game,
    speed_of_light: &MLSpeedOfLightVo,
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
    speed_of_light: &MLSpeedOfLightVo,
    callback_movement: &mut F1,
) where
    F1: FnMut(u64),
{
    // 盤上の駒の移動。
    MGMovements::make_all_movements_on_board(
        game.history.get_phase(&Person::Friend),
        &game.position.current_board,
        &speed_of_light,
        callback_movement,
    );
    // 持ち駒の打。
    MGMovements::make_movement_on_hand(game, &speed_of_light, callback_movement);
}

/// 成る前を含めない、移動元升生成
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 移動先升とそこにある駒　を指定することで　指し手を生成するぜ☆（＾～＾）
///
/// 1. 移動先を指定          ms_dst
/// 2. 移動先にある駒を指定  piece_dst
///
/// その願いが叶う移動元の一覧を返す。
/// 最大２０升。合法手生成の逆の動き☆（＾～＾）
///
/// 「成る前」を調べるのは別関数でやるぜ☆（＾～＾）
///
/// TODO 先手１段目の香車とか、必ず成らないといけないぜ☆（＾～＾）
pub fn lookup_no_promotion_source_by_square_and_piece<F1>(
    destination: &Square,
    ps_dst: &PieceStruct,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
) where
    F1: FnMut(Square),
{
    assert_in_board_as_absolute(
        destination.address,
        "make_no_promotion_source_by_square_and_piece",
    );

    /*
    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    if !this_piece_has_a_destination(ps_dst.phase(), destination, ps_dst) {
        return;
    }
    */

    NextSquares::looking_for_squares_from_on_board(
        ps_dst.piece_type(),
        ps_dst.phase(),
        destination,
        &mut |next_square, _promotability, agility| {
            lookup_no_promotion_source(
                agility,
                Some(ps_dst.piece),
                current_board,
                speed_of_light,
                &mut lookups_the_square,
                next_square,
            )
        },
    );
}

/*
/// この駒には行き先があります。
fn this_piece_has_a_destination(
    _friend: Phase,
    destination: &Square,
    ps_dst: &PieceStruct,
) -> bool {
    let (_dx, dy) = destination.to_file_rank();

    use crate::model::univ::gam::misc::piece::Piece::*;
    match &ps_dst.piece {
        Knight1 => {
            // ▲うさぎ　は１、２段目には進めない
            if dy < RANK_3 {
                return false;
            }
        }
        Lance1 | Pawn1 => {
            // ▲しし、▲ひよこ　は１段目には進めない
            if dy < RANK_2 {
                return false;
            }
        }
        Knight2 => {
            // ▽うさぎ　は８、９段目には進めない
            if RANK_7 < dy {
                return false;
            }
        }
        Lance2 | Pawn2 => {
            // ▽しし、▽ひよこ　は９段目には進めない
            if RANK_8 < dy {
                return false;
            }
        }
        _ => {}
    }

    true
}
*/

// 成る前を含めない、利き
fn lookup_no_promotion_source<F1>(
    agility: Agility,
    opt_dst_piece: Option<Piece>,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    lookups_the_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    match agility {
        Agility::Sliding => {
            if let Some(dst_piece) = opt_dst_piece {
                if current_board.has_sq_km(&next_square, &dst_piece, speed_of_light) {
                    // 指定の駒があれば止まるぜ☆（＾～＾）
                    lookups_the_square(next_square);
                    return true;
                } else if current_board.exists_km(&next_square) {
                    // 何か駒があれば止まるぜ☆（＾～＾）
                    return true;
                }
            } else {
                lookups_the_square(next_square);
                if current_board.exists_km(&next_square) {
                    // 何か駒があれば止まるぜ☆（＾～＾）
                    return true;
                }
            }
        }
        _ => {
            if let Some(dst_piece) = opt_dst_piece {
                if current_board.has_sq_km(&next_square, &dst_piece, speed_of_light) {
                    // 指定の駒があれば止まるぜ☆（＾～＾）
                    lookups_the_square(next_square);
                }
            } else {
                lookups_the_square(next_square);
            }
            // 1マスで終わりだぜ☆（＾～＾）
            return true;
        }
    }
    false
}

/// 成る前の移動元升生成
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 移動先升とそこにある駒　を指定することで　指し手を生成するぜ☆（＾～＾）
///
/// 1. 移動先の升        ms_dst
/// 2. 移動先にある駒    piece_dst
///
/// 成り　の動きでその結果になるような、元の升を返す☆（＾～＾）
pub fn lookup_before_promotion_source_by_square_piece<F1>(
    square_dst: &Square,
    ps_dst: &PieceStruct,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
) where
    F1: FnMut(Square),
{
    assert_in_board_as_absolute(
        square_dst.address,
        "make_before_promotion_source_by_square_piece",
    );

    // +--------------------+
    // | 移動後は成り駒か？ |
    // +--------------------+
    if !ps_dst.is_promoted() {
        return; // 成り駒でないなら、成りの動きをしていない
    }

    // 例えば移動先の駒種類が「ぱひ」なら、「ぱひ」が動いた可能性の他に、
    // 「ひ」が動いたのかもしれない。
    // 「ぱひ」は、敵陣の１～３段目にいて、動きが北だった場合、元が「ひ」の可能性がある。
    //
    // 成る前に戻れない駒は、成ったかどうかを考えなくていいぜ☆（＾～＾）
    if !ps_dst.can_demote() {
        return;
    }

    // +--------------------+
    // | 移動前は成る前の駒 |
    // +--------------------+
    // 前提として、成った駒であることは分かっているとするぜ☆（＾～＾）
    NextSquares::looking_for_squares_from_on_board(
        speed_of_light
            .get_piece_struct(&ps_dst.demoted)
            .piece_type(),
        ps_dst.phase(),
        square_dst,
        &mut |next_square, _promotability, agility| {
            lookup_before_promotion(
                agility,
                &ps_dst.piece,
                current_board,
                speed_of_light,
                &mut lookups_the_square,
                next_square,
            )
        },
    );
}

/// 成る前の移動元、利き
fn lookup_before_promotion<F1>(
    agility: Agility,
    source_piece: &Piece,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    match agility {
        Agility::Sliding => {
            if current_board.has_sq_km(&next_square, source_piece, speed_of_light) {
                // 指定の駒があれば、その升は移動元になる☆ 続行☆（＾～＾）
                lookups_the_square(next_square);
            } else if current_board.exists_km(&next_square) {
                // なんか他の駒があれば終わり☆ ループを抜けるぜ☆（＾～＾）
                return true;
            }
        }
        _ => {
            if current_board.has_sq_km(&next_square, source_piece, speed_of_light) {
                lookups_the_square(next_square);
            }
            return true;
        }
    }
    false
}
