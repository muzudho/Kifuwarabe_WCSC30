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
                    // TODO 成れるかどうかの判定は☆（＾ｑ＾）？
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
                    // TODO 成れるかどうかの判定は☆（＾ｑ＾）？
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
            use crate::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
            match piece {
                Pawn1 => {
                    MGPieceSquares::looking_for_square_from_1player_pawn_on_board(
                        &src_phase,
                        &src_square,
                        callback_squares,
                    );
                }
                Pawn2 => {
                    MGPieceSquares::looking_for_square_from_2player_pawn_on_board(
                        &src_phase,
                        &src_square,
                        callback_squares,
                    );
                }
                Lance1 => {
                    MGPieceSquares::looking_for_squares_from_1player_lance_on_board(
                        &src_phase,
                        &src_square,
                        callback_squares,
                    );
                }
                Lance2 => {
                    MGPieceSquares::looking_for_squares_from_2player_lance_on_board(
                        &src_phase,
                        &src_square,
                        callback_squares,
                    );
                }
                Knight1 => {
                    MGPieceSquares::looking_for_squares_from_1player_knight_on_board(
                        &src_phase,
                        &src_square,
                        callback_squares,
                    );
                }
                Knight2 => {
                    MGPieceSquares::looking_for_squares_from_2player_knight_on_board(
                        &src_phase,
                        &src_square,
                        callback_squares,
                    );
                }
                Silver1 => {
                    MGPieceSquares::looking_for_squares_from_1player_silver_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Silver2 => {
                    MGPieceSquares::looking_for_squares_from_2player_silver_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Gold1 | PromotedPawn1 | PromotedLance1 | PromotedKnight1 | PromotedSilver1 => {
                    MGPieceSquares::looking_for_squares_from_1player_gold_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Gold2 | PromotedPawn2 | PromotedLance2 | PromotedKnight2 | PromotedSilver2 => {
                    MGPieceSquares::looking_for_squares_from_2player_gold_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                King1 => {
                    MGPieceSquares::looking_for_squares_from_1player_king_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                King2 => {
                    MGPieceSquares::looking_for_squares_from_2player_king_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Bishop1 => {
                    MGPieceSquares::looking_for_squares_from_1player_bishop_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Bishop2 => {
                    MGPieceSquares::looking_for_squares_from_2player_bishop_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Rook1 => {
                    MGPieceSquares::looking_for_squares_from_1player_rook_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Rook2 => {
                    MGPieceSquares::looking_for_squares_from_2player_rook_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Horse1 => {
                    MGPieceSquares::looking_for_squares_from_1player_horse_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Horse2 => {
                    MGPieceSquares::looking_for_squares_from_2player_horse_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Dragon1 => {
                    MGPieceSquares::looking_for_squares_from_1player_dragon_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                Dragon2 => {
                    MGPieceSquares::looking_for_squares_from_2player_dragon_on_board(
                        &src_square,
                        callback_squares,
                    );
                }
                _ => {}
            }
        }
    });
}
