use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::controller::movement_generation::mg_square::*;
use crate::model::dto::search_part::sp_position_dto::SPPositionDto;
use crate::model::vo::game_part::gp_phase_vo::Phase;

/// 盤上の駒の動き。
/// https://doc.rust-lang.org/std/ops/trait.FnMut.html
pub fn make_movement_on_board<F1>(
    friend: &Phase,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    callback_movement: &mut F1,
) where
    F1: FnMut(u64),
{
    // 盤上の駒☆（＾～＾）
    MGSquares::for_all(&mut |source| {
        let callback_destination = &mut |destination, promotability| {
            use super::super::super::model::dto::search_part::sp_position_dto::ThingsInTheSquare::*;
            use crate::controller::movement_generation::mg_square::Promotability::*;
            let things_in_the_square =
                current_position.what_is_in_the_square(friend, &destination, speed_of_light);
            match things_in_the_square {
                Space | Opponent => {
                    // 成れるかどうかの判定☆（＾ｑ＾）
                    let promotion = match &promotability {
                        Forced => true,
                        _ => false,
                    };
                    match &promotability {
                        Any => {
                            callback_movement(
                                MLMovementDto {
                                    src: source.clone(),
                                    dst: destination.clone(),
                                    pro: false,
                                    drop: GPPieceTypeVo::KaraPieceType,
                                }
                                .to_hash(speed_of_light),
                            );
                            callback_movement(
                                MLMovementDto {
                                    src: source.clone(),
                                    dst: destination.clone(),
                                    pro: true,
                                    drop: GPPieceTypeVo::KaraPieceType,
                                }
                                .to_hash(speed_of_light),
                            );
                        }
                        _ => {
                            callback_movement(
                                MLMovementDto {
                                    src: source.clone(),
                                    dst: destination.clone(),
                                    pro: promotion,
                                    drop: GPPieceTypeVo::KaraPieceType,
                                }
                                .to_hash(speed_of_light),
                            );
                        }
                    };
                }
                Friend => {}
            };

            match things_in_the_square {
                Space => false,
                _ => true,
            }
        };

        let piece = current_position.get_piece_by_square(&source);
        let ps = speed_of_light.get_piece_struct_vo(piece);
        if *friend == ps.phase() {
            use crate::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
            match piece {
                Pawn1 => {
                    MGPieceSquares::looking_for_square_from_1player_pawn_on_board(
                        &Phase::First,
                        &source,
                        callback_destination,
                    );
                }
                Pawn2 => {
                    MGPieceSquares::looking_for_square_from_2player_pawn_on_board(
                        &Phase::Second,
                        &source,
                        callback_destination,
                    );
                }
                Lance1 => {
                    MGPieceSquares::looking_for_squares_from_1player_lance_on_board(
                        &Phase::First,
                        &source,
                        callback_destination,
                    );
                }
                Lance2 => {
                    MGPieceSquares::looking_for_squares_from_2player_lance_on_board(
                        &Phase::Second,
                        &source,
                        callback_destination,
                    );
                }
                Knight1 => {
                    MGPieceSquares::looking_for_squares_from_1player_knight_on_board(
                        &Phase::First,
                        &source,
                        callback_destination,
                    );
                }
                Knight2 => {
                    MGPieceSquares::looking_for_squares_from_2player_knight_on_board(
                        &Phase::Second,
                        &source,
                        callback_destination,
                    );
                }
                Silver1 => {
                    MGPieceSquares::looking_for_squares_from_1player_silver_on_board(
                        &Phase::First,
                        &source,
                        callback_destination,
                    );
                }
                Silver2 => {
                    MGPieceSquares::looking_for_squares_from_2player_silver_on_board(
                        &Phase::Second,
                        &source,
                        callback_destination,
                    );
                }
                Gold1 | PromotedPawn1 | PromotedLance1 | PromotedKnight1 | PromotedSilver1 => {
                    MGPieceSquares::looking_for_squares_from_1player_gold_on_board(
                        &source,
                        callback_destination,
                    );
                }
                Gold2 | PromotedPawn2 | PromotedLance2 | PromotedKnight2 | PromotedSilver2 => {
                    MGPieceSquares::looking_for_squares_from_2player_gold_on_board(
                        &source,
                        callback_destination,
                    );
                }
                King1 => {
                    MGPieceSquares::looking_for_squares_from_1player_king_on_board(
                        &source,
                        callback_destination,
                    );
                }
                King2 => {
                    MGPieceSquares::looking_for_squares_from_2player_king_on_board(
                        &source,
                        callback_destination,
                    );
                }
                Bishop1 => {
                    MGPieceSquares::looking_for_squares_from_1player_bishop_on_board(
                        &Phase::First,
                        &source,
                        callback_destination,
                    );
                }
                Bishop2 => {
                    MGPieceSquares::looking_for_squares_from_2player_bishop_on_board(
                        &Phase::Second,
                        &source,
                        callback_destination,
                    );
                }
                Rook1 => {
                    MGPieceSquares::looking_for_squares_from_1player_rook_on_board(
                        &Phase::First,
                        &source,
                        callback_destination,
                    );
                }
                Rook2 => {
                    MGPieceSquares::looking_for_squares_from_2player_rook_on_board(
                        &Phase::Second,
                        &source,
                        callback_destination,
                    );
                }
                Horse1 => {
                    MGPieceSquares::looking_for_squares_from_1player_horse_on_board(
                        &source,
                        callback_destination,
                    );
                }
                Horse2 => {
                    MGPieceSquares::looking_for_squares_from_2player_horse_on_board(
                        &source,
                        callback_destination,
                    );
                }
                Dragon1 => {
                    MGPieceSquares::looking_for_squares_from_1player_dragon_on_board(
                        &source,
                        callback_destination,
                    );
                }
                Dragon2 => {
                    MGPieceSquares::looking_for_squares_from_2player_dragon_on_board(
                        &source,
                        callback_destination,
                    );
                }
                _ => {}
            }
        }
    });
}
