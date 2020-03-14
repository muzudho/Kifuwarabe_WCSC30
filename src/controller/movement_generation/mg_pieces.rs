use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::controller::movement_generation::mg_square::*;
use crate::model::dto::search_part::sp_position_dto::SPPositionDto;
use crate::model::vo::game_part::gp_phase_vo::Phase;

/// 盤上の駒の動き。
/// https://doc.rust-lang.org/std/ops/trait.FnMut.html
pub fn make_movement_on_board<F1>(
    src_phase: &Phase,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    callback_movement: &mut F1,
) where
    F1: FnMut(u64),
{
    // 盤上の駒☆（＾～＾）
    MGSquares::for_all(&mut |src_square| {
        let callback_squares = &mut |dst_square2| {
            use super::super::super::model::dto::search_part::sp_position_dto::ThingsInTheSquare::*;
            match current_position.what_is_in_the_square(src_phase, &dst_square2, speed_of_light) {
                Space => {
                    callback_movement(
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
                    callback_movement(
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

        let piece = current_position.get_piece_by_square(&src_square);
        let ps = speed_of_light.get_piece_struct_vo(piece);
        if *src_phase == ps.phase() {
            use crate::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo::*;
            match ps.piece_type() {
                Pawn => {
                    MGPieceSquares::looking_for_square_from_pawn_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Lance => {
                    MGPieceSquares::looking_for_squares_from_lance_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Knight => {
                    MGPieceSquares::looking_for_squares_from_knight_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Silver => {
                    MGPieceSquares::looking_for_squares_from_silver_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Gold => {
                    MGPieceSquares::looking_for_squares_from_gold_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                King => {
                    MGPieceSquares::looking_for_squares_from_king_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Bishop => {
                    MGPieceSquares::looking_for_squares_from_bishop_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Rook => {
                    MGPieceSquares::looking_for_squares_from_rook_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Horse => {
                    MGPieceSquares::looking_for_squares_from_horse_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Dragon => {
                    MGPieceSquares::looking_for_squares_from_dragon_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                _ => {}
            }
        }
    });
}
