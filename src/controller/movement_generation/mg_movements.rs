use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::model::vo::game_part::gp_piece_vo::*;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use crate::controller::movement_generation::mg_square::*;
use crate::model::dto::search_part::sp_earth_dto::SPEarthDto;
use crate::model::dto::search_part::sp_position_dto::SPPositionDto;
use crate::model::vo::game_part::gp_phase_vo::Phase;

pub struct MGMovements {}
impl MGMovements {
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
        MGSquaresLv2::for_all(&mut |source| {
            let callback_next = &mut |destination, promotability| {
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
                        MGNextSquares::looking_for_square_from_1player_pawn_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Pawn2 => {
                        MGNextSquares::looking_for_square_from_2player_pawn_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Lance1 => {
                        MGNextSquares::looking_for_squares_from_1player_lance_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Lance2 => {
                        MGNextSquares::looking_for_squares_from_2player_lance_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Knight1 => {
                        MGNextSquares::looking_for_squares_from_1player_knight_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Knight2 => {
                        MGNextSquares::looking_for_squares_from_2player_knight_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Silver1 => {
                        MGNextSquares::looking_for_squares_from_1player_silver_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Silver2 => {
                        MGNextSquares::looking_for_squares_from_2player_silver_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Gold1 | PromotedPawn1 | PromotedLance1 | PromotedKnight1 | PromotedSilver1 => {
                        MGNextSquares::looking_for_squares_from_1player_gold_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Gold2 | PromotedPawn2 | PromotedLance2 | PromotedKnight2 | PromotedSilver2 => {
                        MGNextSquares::looking_for_squares_from_2player_gold_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    King1 => {
                        MGNextSquares::looking_for_squares_from_1player_king_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    King2 => {
                        MGNextSquares::looking_for_squares_from_2player_king_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Bishop1 => {
                        MGNextSquares::looking_for_squares_from_1player_bishop_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Bishop2 => {
                        MGNextSquares::looking_for_squares_from_2player_bishop_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Rook1 => {
                        MGNextSquares::looking_for_squares_from_1player_rook_on_board(
                            &Phase::First,
                            &source,
                            callback_next,
                        );
                    }
                    Rook2 => {
                        MGNextSquares::looking_for_squares_from_2player_rook_on_board(
                            &Phase::Second,
                            &source,
                            callback_next,
                        );
                    }
                    Horse1 => {
                        MGNextSquares::looking_for_squares_from_1player_horse_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Horse2 => {
                        MGNextSquares::looking_for_squares_from_2player_horse_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Dragon1 => {
                        MGNextSquares::looking_for_squares_from_1player_dragon_on_board(
                            &source,
                            callback_next,
                        );
                    }
                    Dragon2 => {
                        MGNextSquares::looking_for_squares_from_2player_dragon_on_board(
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
                MGSquaresLv2::for_all(&mut |any_square| {
                    let exists_piece = sp_earth_dto
                        .get_current_position()
                        .get_piece_by_square(&any_square);

                    if let GPPieceVo::NonePiece = exists_piece {
                        // 駒が無いところに打つ
                        let current_position = sp_earth_dto.get_current_position();
                        let ps_dst = speed_of_light.get_piece_struct_vo(&hand_piece);
                        let piece_type_dst = ps_dst.piece_type();
                        let (file, rank) = any_square.to_file_rank();
                        // 行先の無いところに駒を進めることの禁止☆（＾～＾）
                        use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
                        match hand_piece {
                            Knight1 => {
                                // ▼うさぎ　は１、２段目には進めない
                                if rank < RANK_3 {
                                    return;
                                }
                            }
                            // ▼しし、▼ひよこ　は１段目には進めない
                            Lance1 => {
                                if rank < RANK_2 {
                                    return;
                                }
                            }
                            Pawn1 => {
                                // ▼ひよこ　は２歩できない
                                if rank < RANK_2
                                    || current_position.exists_fu_by_phase_suji(
                                        &ps_dst.phase(),
                                        file,
                                        speed_of_light,
                                    )
                                {
                                    return;
                                }
                            }
                            Knight2 => {
                                // △うさぎ　は８、９段目には進めない
                                if RANK_7 < rank {
                                    return;
                                }
                            }
                            // △しし、△ひよこ　は９段目には進めない
                            Lance2 => {
                                if RANK_8 < rank {
                                    return;
                                }
                            }
                            Pawn2 => {
                                // △ひよこ　は２歩できない
                                if RANK_8 < rank
                                    || current_position.exists_fu_by_phase_suji(
                                        &ps_dst.phase(),
                                        file,
                                        speed_of_light,
                                    )
                                {
                                    return;
                                }
                            }
                            _ => {}
                        }
                        callback_movement(
                            MLMovementDto {
                                src: Square::from_usquare(SS_SRC_DA), // 駒台
                                dst: any_square.clone(),              // どの升へ行きたいか
                                pro: false,                           // 打に成りは無し
                                drop: num_to_piece_type(
                                    speed_of_light
                                        .get_piece_type_struct_vo_from_piece_type(&piece_type_dst)
                                        .serial_piece_number,
                                ), // 打った駒種類
                            }
                            .to_hash(speed_of_light),
                        );
                    }
                });
            }
        });
    }
}
