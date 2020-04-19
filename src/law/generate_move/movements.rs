use crate::cosmic::shogi::recording::Movement;
use crate::cosmic::shogi::state::Phase;
use crate::cosmic::smart::features::{num_to_piece_type, HandPieces};
use crate::cosmic::smart::square::{AbsoluteAddress, Address};
use crate::cosmic::toy_box::{Board, Piece};
use crate::law::generate_move::movement_generator::PublicNextSquares;
use crate::law::generate_move::squares::MovePermission;
use crate::law::generate_move::squares::NextSquares;
use crate::law::speed_of_light::SpeedOfLight;

pub struct MGMovements {}
impl MGMovements {
    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    pub fn all_pieces_on_board<F1>(
        friend: Phase,
        board: &Board,
        speed_of_light: &SpeedOfLight,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        // 盤上の駒☆（＾～＾）
        PublicNextSquares::for_all(&mut |source| {
            MGMovements::a_piece_on_board(friend, &source, board, speed_of_light, callback)
        });
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    fn a_piece_on_board<F1>(
        friend: Phase,
        source: &AbsoluteAddress,
        board: &Board,
        speed_of_light: &SpeedOfLight,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        let callback_next =
            &mut |destination, promotability, _agility, move_permission: Option<MovePermission>| {
                use crate::cosmic::toy_box::ThingsInTheSquare::*;
                use crate::law::generate_move::squares::Promotability::*;
                let things_in_the_square =
                    board.what_is_in_the_square(friend, &destination, speed_of_light);
                match things_in_the_square {
                    Space | Opponent => {
                        // 成れるかどうかの判定☆（＾ｑ＾）
                        let promotion = match &promotability {
                            Forced => true,
                            _ => false,
                        };

                        // 成りじゃない場合は、行き先のない動きを制限されるぜ☆（＾～＾）
                        let forbidden = if let Some(move_permission_val) = move_permission {
                            if move_permission_val.check(&destination) {
                                false
                            } else {
                                true
                            }
                        } else {
                            false
                        };

                        match &promotability {
                            Any => {
                                // 成ったり、成れなかったりできるとき。
                                if !forbidden {
                                    callback(
                                        Movement {
                                            source: source.clone(),
                                            destination: destination.clone(),
                                            promote: false,
                                            drop: None,
                                        }
                                        .to_hash(speed_of_light),
                                    );
                                }
                                callback(
                                    Movement {
                                        source: source.clone(),
                                        destination: destination.clone(),
                                        promote: true,
                                        drop: None,
                                    }
                                    .to_hash(speed_of_light),
                                );
                            }
                            _ => {
                                // 成れるか、成れないかのどちらかのとき。
                                if promotion || !forbidden {
                                    callback(
                                        Movement {
                                            source: source.clone(),
                                            destination: destination.clone(),
                                            promote: promotion,
                                            drop: None,
                                        }
                                        .to_hash(speed_of_light),
                                    );
                                }
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

        if let Some(piece) = board.piece_at(&source) {
            if friend == piece.phase(speed_of_light) {
                NextSquares::piece_of(piece.r#type(speed_of_light), friend, &source, callback_next);
            }
        }
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    pub fn all_pieces_on_hand<F1>(
        friend: Phase,
        board: &Board,
        speed_of_light: &SpeedOfLight,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        HandPieces::for_all(&mut |any_piece_type| {
            // 持ち駒
            let hand = any_piece_type.add_phase(friend);

            if 0 < board.get_hand(hand, speed_of_light) {
                // 駒を持っていれば
                use crate::cosmic::smart::features::PieceType::*;
                match hand.r#type(speed_of_light) {
                    // 歩、香
                    Pawn | Lance => NextSquares::drop_pawn_lance(
                        hand.phase(speed_of_light),
                        &mut |destination| {
                            MGMovements::a_piece_on_hand(
                                &hand,
                                &board,
                                speed_of_light,
                                &destination,
                                callback,
                            );
                        },
                    ),
                    // 桂
                    Knight => {
                        NextSquares::drop_knight(hand.phase(speed_of_light), &mut |destination| {
                            MGMovements::a_piece_on_hand(
                                &hand,
                                &board,
                                speed_of_light,
                                &destination,
                                callback,
                            );
                        })
                    }
                    // それ以外の駒が打てる範囲は盤面全体。
                    _ => {
                        PublicNextSquares::for_all(&mut |destination| {
                            MGMovements::a_piece_on_hand(
                                &hand,
                                &board,
                                speed_of_light,
                                &destination,
                                callback,
                            );
                        });
                    }
                }
            }
        });
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `hand_piece` - 持ち駒だぜ☆（＾～＾）
    /// * `board` - 現局面の盤上だぜ☆（＾～＾）
    /// * `speed_of_light` - 光速だぜ☆（＾～＾）
    /// * `destination` - 移動先升だぜ☆（＾～＾）
    /// * `callback` - 指し手のハッシュを受け取れだぜ☆（＾～＾）
    fn a_piece_on_hand<F1>(
        hand: &Piece,
        board: &Board,
        speed_of_light: &SpeedOfLight,
        destination: &AbsoluteAddress,
        callback: &mut F1,
    ) where
        F1: FnMut(u64),
    {
        if let None = board.piece_at(&destination) {
            // 駒が無いところに打つ
            use crate::cosmic::toy_box::Piece::*;
            match *hand {
                Pawn1 | Pawn2 => {
                    // ひよこ　は２歩できない☆（＾～＾）
                    if board.exists_pawn_on_file(
                        hand.phase(speed_of_light),
                        destination.file(),
                        speed_of_light,
                    ) {
                        return;
                    }
                }
                _ => {}
            }
            callback(
                Movement {
                    source: Address::default().abs(), // 駒台
                    destination: destination.clone(), // どの升へ行きたいか
                    promote: false,                   // 打に成りは無し
                    drop: num_to_piece_type(
                        hand.r#type(speed_of_light).serial_number(speed_of_light),
                    ), // 打った駒種類
                }
                .to_hash(speed_of_light),
            );
        }
    }
}
