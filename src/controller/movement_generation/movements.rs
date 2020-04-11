use crate::controller::movement_generation::mg_square::*;
use crate::controller::movement_generation::squares::*;
use crate::model::univ::gam::board::Board;
use crate::model::univ::gam::misc::movement_builder::*;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::phase::Phase;
use crate::model::univ::gam::misc::piece::*;
use crate::model::univ::gam::misc::piece_type::*;
use crate::model::univ::gam::misc::square::*;
use crate::model::univ::gam::position::Position;
use crate::model::univ::game::Game;
use crate::model::univ::speed_of_light::*;

pub struct MGMovements {}
impl MGMovements {
    /// 盤上の駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    pub fn make_movement_on_board<F1>(
        friend: &Phase,
        current_board: &Board,
        speed_of_light: &MLSpeedOfLightVo,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        // 盤上の駒☆（＾～＾）
        MGSquares::for_all(&mut |source| {
            let callback_next = &mut |destination, promotability| {
                use crate::controller::movement_generation::squares::Promotability::*;
                use crate::model::univ::gam::board::ThingsInTheSquare::*;
                let things_in_the_square =
                    current_board.what_is_in_the_square(friend, &destination, speed_of_light);
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
                                    MovementBuilder {
                                        src: source.clone(),
                                        dst: destination.clone(),
                                        pro: false,
                                        drop: None,
                                    }
                                    .to_hash(speed_of_light),
                                );
                                callback_movement(
                                    MovementBuilder {
                                        src: source.clone(),
                                        dst: destination.clone(),
                                        pro: true,
                                        drop: None,
                                    }
                                    .to_hash(speed_of_light),
                                );
                            }
                            _ => {
                                callback_movement(
                                    MovementBuilder {
                                        src: source.clone(),
                                        dst: destination.clone(),
                                        pro: promotion,
                                        drop: None,
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

            if let Some(piece) = current_board.get_piece_by_square(&source) {
                let ps = speed_of_light.get_piece_struct(&piece);

                if *friend == ps.phase() {
                    use crate::model::univ::gam::misc::piece_type::PieceType::*;
                    match ps.piece_type() {
                        Pawn => {
                            NextSquares::looking_for_square_from_pawn_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        Lance => {
                            NextSquares::looking_for_squares_from_lance_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        Knight => {
                            NextSquares::looking_for_squares_from_knight_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        Silver => {
                            NextSquares::looking_for_squares_from_silver_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        Gold | PromotedPawn | PromotedLance | PromotedKnight | PromotedSilver => {
                            NextSquares::looking_for_squares_from_gold_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        King => {
                            NextSquares::looking_for_squares_from_king_on_board(
                                &source,
                                callback_next,
                            );
                        }
                        Bishop => {
                            NextSquares::looking_for_squares_from_bishop_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        Rook => {
                            NextSquares::looking_for_squares_from_rook_on_board(
                                friend,
                                &source,
                                callback_next,
                            );
                        }
                        Horse => {
                            NextSquares::looking_for_squares_from_horse_on_board(
                                &source,
                                callback_next,
                            );
                        }
                        Dragon => {
                            NextSquares::looking_for_squares_from_dragon_on_board(
                                &source,
                                callback_next,
                            );
                        }
                    }
                }
            }
        });
    }

    /// 持ち駒の動き。
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    pub fn make_movement_on_hand<F1>(
        game: &Game,
        speed_of_light: &MLSpeedOfLightVo,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        GPHandPieces::for_all(&mut |any_piece_type| {
            let hand_piece = &speed_of_light
                .get_piece_struct_by_phase_and_piece_type(
                    &game.history.get_phase(&Person::Friend),
                    any_piece_type,
                )
                .piece;

            if 0 < game
                .position
                .current_board
                .get_hand(hand_piece, speed_of_light)
            {
                // 駒を持っていれば
                use crate::model::univ::gam::misc::piece::Piece::*;
                match *hand_piece {
                    // ▲歩、▲香 は１段目には進めない
                    Pawn1 | Lance1 => {
                        Squares::for_from_rank2_to_rank9(&Phase::First, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    // ▲桂 は１、２段目には進めない
                    Knight1 => {
                        Squares::for_from_rank3_to_rank9(&Phase::First, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    // ▽歩、▽香 は９段目には進めない
                    Pawn2 | Lance2 => {
                        Squares::for_from_rank2_to_rank9(&Phase::Second, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    // ▲桂 は８、９段目には進めない
                    Knight2 => {
                        Squares::for_from_rank3_to_rank9(&Phase::Second, &mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
                                speed_of_light,
                                &destination,
                                callback_movement,
                            );
                        })
                    }
                    _ => {
                        MGSquares::for_all(&mut |destination| {
                            MGMovements::make_hand(
                                &hand_piece,
                                &game.position,
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
        hand_piece: &Piece,
        position: &Position,
        speed_of_light: &MLSpeedOfLightVo,
        destination: &Square,
        callback_movement: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        if let None = position.current_board.get_piece_by_square(&destination) {
            // 駒が無いところに打つ
            let current_board = &position.current_board;
            let ps_dst = speed_of_light.get_piece_struct(hand_piece);
            let piece_type_dst = ps_dst.piece_type();
            // 行先の無いところに駒を進めることの禁止☆（＾～＾）
            use crate::model::univ::gam::misc::piece::Piece::*;
            match *hand_piece {
                Pawn1 | Pawn2 => {
                    // ひよこ　は２歩できない
                    if current_board.exists_fu_by_phase_suji(
                        &ps_dst.phase(),
                        destination.get_file(),
                        speed_of_light,
                    ) {
                        return;
                    }
                }
                _ => {}
            }
            callback_movement(
                MovementBuilder {
                    src: Square::from_address(SQUARE_DROP), // 駒台
                    dst: destination.clone(),               // どの升へ行きたいか
                    pro: false,                             // 打に成りは無し
                    drop: num_to_piece_type(
                        speed_of_light
                            .get_piece_type_struct_from_piece_type(&piece_type_dst)
                            .serial_piece_number,
                    ), // 打った駒種類
                }
                .to_hash(speed_of_light),
            );
        }
    }
}
