use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::controller::movement_generation::mg_square::*;
use crate::model::dto::search_part::sp_position_dto::SPPositionDto;
use crate::model::vo::game_part::gp_phase_vo::Phase;

/// 盤上の駒の動き。
/// https://doc.rust-lang.org/std/ops/trait.FnMut.html
pub fn make_movement_on_board<F1, F2>(
    src_phase: &Phase,
    src_square: &Square,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut callback_movement: F1,
    mut callback_movements: F2,
) where
    F1: FnMut(u64),
    F2: FnMut(u64),
{
    let callback_square = &mut |dst_square1| {
        use super::super::super::model::dto::search_part::sp_position_dto::ThingsInTheSquare::*;
        match current_position.what_is_in_the_square(src_phase, &dst_square1, speed_of_light) {
            Space | Opponent => {
                callback_movement(
                    MLMovementDto {
                        src: src_square.clone(),
                        dst: dst_square1.clone(),
                        pro: false, // 成らず
                        drop: GPPieceTypeVo::KaraPieceType,
                    }
                    .to_hash(speed_of_light),
                )
            }
            Friend => {}
        }
    };

    let callback_squares = &mut |dst_square2| {
        use super::super::super::model::dto::search_part::sp_position_dto::ThingsInTheSquare::*;
        match current_position.what_is_in_the_square(src_phase, &dst_square2, speed_of_light) {
            Space => {
                callback_movements(
                    MLMovementDto {
                        src: src_square.clone(),
                        dst: dst_square2.clone(),
                        pro: false, // 成らず
                        drop: GPPieceTypeVo::KaraPieceType,
                    }
                    .to_hash(speed_of_light),
                );
                false
            }
            Opponent => {
                callback_movements(
                    MLMovementDto {
                        src: src_square.clone(),
                        dst: dst_square2.clone(),
                        pro: false, // 成らず
                        drop: GPPieceTypeVo::KaraPieceType,
                    }
                    .to_hash(speed_of_light),
                );
                true
            }
            Friend => true,
        }
    };

    MGPieceSquares::looking_for_square_from_pawn_on_board(src_square, callback_square);
    MGPieceSquares::looking_for_squares_from_lance_on_board(src_square, callback_squares);
    MGPieceSquares::looking_for_squares_from_knight_on_board(src_square, callback_square);
    MGPieceSquares::looking_for_squares_from_silver_on_board(src_square, callback_square);
    MGPieceSquares::looking_for_squares_from_gold_on_board(src_square, callback_square);
    MGPieceSquares::looking_for_squares_from_king_on_board(src_square, callback_square);
    MGPieceSquares::looking_for_squares_from_bishop_on_board(src_square, callback_squares);
    MGPieceSquares::looking_for_squares_from_rook_on_board(src_square, callback_squares);
    MGPieceSquares::looking_for_squares_from_horse_on_board(
        src_square,
        callback_square,
        callback_squares,
    );
    MGPieceSquares::looking_for_squares_from_dragon_on_board(
        src_square,
        callback_square,
        callback_squares,
    );
}
