use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::model::vo::game_part::gp_piece_vo::*;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use crate::controller::movement_generation::mg_square::*;
use crate::controller::movement_generation::square::*;
use crate::model::dto::search_part::position::Position;
use crate::model::dto::search_part::sp_earth_dto::SPEarthDto;
use crate::model::vo::game_part::gp_phase_vo::Phase;

pub struct MGMovements {}
impl MGMovements {
    /// 盤上の駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    pub fn make_movement_on_board<F1>(
        friend: &Phase,
        current_position: &Position,
        speed_of_light: &MLSpeedOfLightVo,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        // 盤上の駒☆（＾～＾）
        MGSquares::for_all(&mut |source| {
            let callback_next = &mut |destination, promotability| {
                use super::super::super::model::dto::search_part::position::ThingsInTheSquare::*;
                use crate::controller::movement_generation::square::Promotability::*;
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
                        NextSquares::looking_for_square_from_1player_pawn_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Pawn2 => {
                        NextSquares::looking_for_square_from_2player_pawn_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Lance1 => {
                        NextSquares::looking_for_squares_from_1player_lance_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Lance2 => {
                        NextSquares::looking_for_squares_from_2player_lance_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Knight1 => {
                        NextSquares::looking_for_squares_from_1player_knight_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Knight2 => {
                        NextSquares::looking_for_squares_from_2player_knight_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Silver1 => {
                        NextSquares::looking_for_squares_from_1player_silver_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Silver2 => {
                        NextSquares::looking_for_squares_from_2player_silver_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Gold1 | PromotedPawn1 | PromotedLance1 | PromotedKnight1 | PromotedSilver1 => {
                        NextSquares::looking_for_squares_from_1player_gold_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Gold2 | PromotedPawn2 | PromotedLance2 | PromotedKnight2 | PromotedSilver2 => {
                        NextSquares::looking_for_squares_from_2player_gold_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    King1 => {
                        NextSquares::looking_for_squares_from_1player_king_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    King2 => {
                        NextSquares::looking_for_squares_from_2player_king_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Bishop1 => {
                        NextSquares::looking_for_squares_from_1player_bishop_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Bishop2 => {
                        NextSquares::looking_for_squares_from_2player_bishop_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Rook1 => {
                        NextSquares::looking_for_squares_from_1player_rook_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Rook2 => {
                        NextSquares::looking_for_squares_from_2player_rook_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Horse1 => {
                        NextSquares::looking_for_squares_from_1player_horse_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Horse2 => {
                        NextSquares::looking_for_squares_from_2player_horse_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Dragon1 => {
                        NextSquares::looking_for_squares_from_1player_dragon_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Dragon2 => {
                        NextSquares::looking_for_squares_from_2player_dragon_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    _ => {}
                }
            }
        });
    }

    /// 持ち駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    pub fn make_movement_on_hand<F1>(
        sp_earth_dto: &SPEarthDto,
        speed_of_light: &MLSpeedOfLightVo,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        GPHandPieces::for_all(&mut |any_piece_type| {
            let hand_piece = speed_of_light
                .get_piece_struct_vo_by_phase_and_piece_type(
                    &sp_earth_dto.get_phase(&Person::Friend),
                    any_piece_type,
                )
                .piece();

            if 0 < sp_earth_dto
                .get_current_position()
                .get_hand(hand_piece, speed_of_light)
            {
                // 駒を持っていれば
                use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
                match *hand_piece {
                    // ▲歩、▲香 は１段目には進めない
                    Pawn1 | Lance1 => Squares::for_from_rank2_to_rank9(&mut |destination| {
                        MGMovements::make_hand(
                            &hand_piece,
                            sp_earth_dto,
                            speed_of_light,
                            &destination,
                            callback_movement,
                        );
                    }),
                    // ▲桂 は１、２段目には進めない
                    Knight1 => Squares::for_from_rank3_to_rank9(&mut |destination| {
                        MGMovements::make_hand(
                            &hand_piece,
                            sp_earth_dto,
                            speed_of_light,
                            &destination,
                            callback_movement,
                        );
                    }),
                    // ▽歩、▽香 は９段目には進めない
                    Pawn2 | Lance2 => Squares::for_from_rank1_to_rank8(&mut |destination| {
                        MGMovements::make_hand(
                            &hand_piece,
                            sp_earth_dto,
                            speed_of_light,
                            &destination,
                            callback_movement,
                        );
                    }),
                    // ▲桂 は８、９段目には進めない
                    Knight2 => Squares::for_from_rank1_to_rank7(&mut |destination| {
                        MGMovements::make_hand(
                            &hand_piece,
                            sp_earth_dto,
                            speed_of_light,
                            &destination,
                            callback_movement,
                        );
                    }),
                    _ => {
                        MGSquares::for_all(&mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                sp_earth_dto,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        });
                    }
                }
            }
        });
    }

    fn make_hand<F1>(
        hand_piece: &GPPieceVo,
        sp_earth_dto: &SPEarthDto,
        speed_of_light: &MLSpeedOfLightVo,
        destination: &Square,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        let exists_piece = sp_earth_dto
            .get_current_position()
            .get_piece_by_square(&destination);

        if let GPPieceVo::NonePiece = exists_piece {
            // 駒が無いところに打つ
            let current_position = sp_earth_dto.get_current_position();
            let ps_dst = speed_of_light.get_piece_struct_vo(hand_piece);
            let piece_type_dst = ps_dst.piece_type();
            // 行先の無いところに駒を進めることの禁止☆（＾～＾）
            use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
            match *hand_piece {
                Pawn1 | Pawn2 => {
                    // ひよこ　は２歩できない
                    if current_position.exists_fu_by_phase_suji(
                        &ps_dst.phase(),
                        destination.file,
                        speed_of_light,
                    ) {
                        return;
                    }
                }
                _ => {}
            }
            callback_movement(
                MLMovementDto {
                    src: Square::from_usquare(SQUARE_DROP), // 駒台
                    dst: destination.clone(),               // どの升へ行きたいか
                    pro: false,                             // 打に成りは無し
                    drop: num_to_piece_type(
                        speed_of_light
                            .get_piece_type_struct_vo_from_piece_type(&piece_type_dst)
                            .serial_piece_number,
                    ), // 打った駒種類
                }
                .to_hash(speed_of_light),
            );
        }
    }
}