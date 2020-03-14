use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::controller::movement_generation::mg_square::*;
use crate::model::dto::search_part::sp_position_dto::SPPositionDto;
use crate::model::vo::game_part::gp_phase_vo::Phase;

/// 盤上の歩の動き。
/// https://doc.rust-lang.org/std/ops/trait.FnMut.html
pub fn make_movement_of_pawn_on_board<F1>(
    src_phase: &Phase,
    src_square: &Square,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut callback_movement: F1,
) where
    F1: FnMut(u64),
{
    MGPieceSquares::looking_for_square_from_pawn_on_board(src_square, &mut |dst_square| {
        use super::super::super::model::dto::search_part::sp_position_dto::ThingsInTheSquare::*;
        match current_position.what_is_in_the_square(src_phase, &dst_square, speed_of_light) {
            Space | Opponent => {
                callback_movement(
                    MLMovementDto {
                        src: src_square.clone(),
                        dst: dst_square.clone(),
                        pro: false, // 成らず
                        drop: GPPieceTypeVo::KaraPieceType,
                    }
                    .to_hash(speed_of_light),
                )
            }
            Friend => {}
        }
    });
}

/// 盤上の桂の動き。
/// https://doc.rust-lang.org/std/ops/trait.FnMut.html
pub fn make_movement_of_knight_on_board<F1>(
    src_phase: &Phase,
    src_square: &Square,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut callback_movement: F1,
) where
    F1: FnMut(u64),
{
    MGPieceSquares::looking_for_squares_from_knight_on_board(src_square, &mut |dst_square| {
        use super::super::super::model::dto::search_part::sp_position_dto::ThingsInTheSquare::*;
        match current_position.what_is_in_the_square(src_phase, &dst_square, speed_of_light) {
            Space | Opponent => {
                callback_movement(
                    MLMovementDto {
                        src: src_square.clone(),
                        dst: dst_square.clone(),
                        pro: false, // 成らず
                        drop: GPPieceTypeVo::KaraPieceType,
                    }
                    .to_hash(speed_of_light),
                )
            }
            Friend => {}
        }
    });
}
