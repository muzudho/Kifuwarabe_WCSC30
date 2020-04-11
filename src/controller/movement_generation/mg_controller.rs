//!
//! 現局面を使った指し手生成
//!

use super::squares::*;
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::common_use::cu_conv_controller::*;
use crate::controller::movement_generation::mg_choicing_controller::*;
use crate::controller::movement_generation::mg_direction::*;
use crate::controller::movement_generation::movements::*;
use crate::model::univ::gam::board::*;
use crate::model::univ::gam::misc::movement_builder::*;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::phase::Phase;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece_direction::*;
use crate::model::univ::gam::misc::piece_movement::*;
use crate::model::univ::gam::misc::piece_struct::PieceStruct;
use crate::model::univ::gam::misc::piece_type::*;
use crate::model::univ::gam::misc::square::*;
use crate::model::univ::gam::misc::square_and_piece::SquareAndPiece;
use crate::model::univ::gam::position::*;
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
    get_up_potential_movement(&game, &speed_of_light, &mut |movement| {
        &movement_set.insert(movement);
    });

    if false {
        // 王が取られる局面を除く手を選ぶぜ☆（＾～＾）
        select_movement_except_check(movement_set, &game, &speed_of_light);

        // 自殺手は省くぜ☆（＾～＾）
        select_movement_except_suiceid(movement_set, game, speed_of_light);
    }
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
pub fn get_up_potential_movement<F1>(
    game: &Game,
    speed_of_light: &MLSpeedOfLightVo,
    callback_movement: &mut F1,
) where
    F1: FnMut(u64),
{
    // 盤上の駒の移動。
    MGMovements::make_movement_on_board(
        &game.history.get_phase(&Person::Friend),
        &game.position.current_board,
        &speed_of_light,
        callback_movement,
    );
    // 持ち駒の打。
    MGMovements::make_movement_on_hand(game, &speed_of_light, callback_movement);
}

/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  piece_dst
///
/// 盤上の駒の移動の最初の１つ。打を除く
pub fn get_movement_by_square_and_piece_on_board<F1>(
    sq_dst: &Square,
    piece_dst: Piece,
    position: &Position,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_in_board_as_absolute(sq_dst.address, "Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let ps_dst = speed_of_light.get_piece_struct(&piece_dst);
    let (phase1, _piece_type_dst) = &ps_dst.phase_piece_type;

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if let Some(phase2) = position
        .current_board
        .get_phase_by_sq(&sq_dst, speed_of_light)
    {
        if phase2 == *phase1 {
            return;
        }
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = MovementBuilder::default();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    let mut mv_src_hashset: HashSet<Square> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    lookup_no_promotion_source_by_square_and_piece(
        &sq_dst,
        &ps_dst,
        &position.current_board,
        &speed_of_light,
        |square| {
            mv_src_hashset.insert(square);
        },
    );
    for sq_src in &mv_src_hashset {
        assert_in_board_as_absolute(
            sq_src.address,
            "make_no_promotion_source_by_square_and_piece(成らず)",
        );

        ss_hash_builder.src = sq_src.clone();
        // 成らず
        ss_hash_builder.pro = false;
        ss_hash_builder.drop = None;
        gets_movement(ss_hash_builder.to_hash(speed_of_light));
    }

    // +--------------+
    // | 盤上（成り） |
    // +--------------+
    mv_src_hashset.clear();
    lookup_before_promotion_source_by_square_piece(
        sq_dst,
        &ps_dst,
        &position.current_board,
        &speed_of_light,
        |square| {
            mv_src_hashset.insert(square);
        },
    );
    for sq_src in &mv_src_hashset {
        assert_in_board_as_absolute(sq_src.address, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成り)");

        ss_hash_builder.src = sq_src.clone();
        // 成り
        ss_hash_builder.pro = true;
        ss_hash_builder.drop = None;
        gets_movement(ss_hash_builder.to_hash(speed_of_light));
    }
}

/// 打
///
/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  piece_dst
pub fn get_movement_by_square_and_piece_on_drop<F1>(
    sq_dst: &Square,
    piece_dst: &Piece,
    position: &Position,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_in_board_as_absolute(sq_dst.address, "get_movement_by_square_and_piece_on_drop");

    // 手番の先後、駒種類
    let ps_dst = speed_of_light.get_piece_struct(piece_dst);
    let (phase1, _piece_type_dst) = &ps_dst.phase_piece_type;

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if let Some(phase2) = position
        .current_board
        .get_phase_by_sq(&sq_dst, speed_of_light)
    {
        if phase2 == *phase1 {
            return;
        }
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = MovementBuilder::default();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    //let mut mv_src_hashset : HashSet<Square> = HashSet::<Square>::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
    lookup_drop_by_square_piece(
        &SquareAndPiece::new(&sq_dst, piece_dst),
        &position.current_board,
        &speed_of_light,
        |piece_type_hash| {
            da_piece_type_hashset.insert(piece_type_hash);
        },
    );
    // 打
    for num_piece_type_da in da_piece_type_hashset.iter() {
        let drop_piece_type_o = num_to_piece_type(*num_piece_type_da);

        let movement_hash = MovementBuilder {
            src: Square::from_address(SQUARE_DROP),
            dst: (*sq_dst).clone(),
            pro: false,
            drop: drop_piece_type_o,
        }
        .to_hash(speed_of_light);

        gets_movement(movement_hash);
    }
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
        "make_no_promotion_source_by_square_and_piece",
    );

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    if !this_piece_has_a_destination(square_dst, ps_dst) {
        return;
    }

    let piece_type_num = speed_of_light
        .get_piece_type_struct_from_piece_type(&ps_dst.piece_type())
        .serial_piece_number;

    MGDirection::for_all(&mut |i_dir| {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &PieceDirection;
        if &Phase::First == &ps_dst.phase() {
            p_kmdir = &KM_UGOKI.back[piece_type_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[piece_type_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use crate::model::univ::gam::misc::piece_direction::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    Squares::looking_next_from(&Rotation::Ccw180, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 東
                    Squares::next_of(&Rotation::Ccw180, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    Squares::looking_next_from(&Rotation::Ccw225, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 北東
                    Squares::next_of(&Rotation::Ccw225, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            NNE => {
                // 北北東
                Squares::north_east_keima_of(
                    UpsideDown::Origin,
                    &Phase::First,
                    square_dst,
                    &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    Squares::looking_next_from(&Rotation::Ccw270, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 北
                    Squares::next_of(&Rotation::Ccw270, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            NNW => {
                // 北北西
                Squares::north_east_keima_of(
                    UpsideDown::Flip,
                    &Phase::First.turn(),
                    square_dst,
                    &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    Squares::looking_next_from(&Rotation::Ccw315, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 北西
                    Squares::next_of(&Rotation::Ccw315, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    Squares::looking_next_from(&Rotation::Ccw0, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 西
                    Squares::next_of(&Rotation::Ccw0, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    Squares::looking_next_from(&Rotation::Ccw45, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 南西
                    Squares::next_of(&Rotation::Ccw45, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            SSW => {
                // 南南西
                Squares::north_east_keima_of(
                    UpsideDown::Origin,
                    &Phase::First.turn(),
                    square_dst,
                    &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    Squares::looking_next_from(&Rotation::Ccw90, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 南
                    Squares::next_of(&Rotation::Ccw90, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            SSE => {
                // 南南東
                Squares::north_east_keima_of(
                    UpsideDown::Flip,
                    &Phase::First,
                    square_dst,
                    &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    Squares::looking_next_from(&Rotation::Ccw135, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_sliding(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        )
                    });
                } else {
                    // 南東
                    Squares::next_of(&Rotation::Ccw135, square_dst, &mut |next_square| {
                        lookup_no_promotion_source_by_piece_next(
                            &ps_dst.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    });
                }
            }
            Owari => return true,
        }
        false
    });
}

/// この駒には行き先があります。
fn this_piece_has_a_destination(square_dst: &Square, ps_dst: &PieceStruct) -> bool {
    let (_dx, dy) = square_dst.to_file_rank();

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

// 成る前を含めない、長い利き
fn lookup_no_promotion_source_by_piece_sliding<F1>(
    dst_piece: &Piece,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    lookups_the_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    if current_board.has_sq_km(&next_square, dst_piece, speed_of_light) {
        // TODO ポインター渡しできないもんか……☆（＾～＾）あるいはハッシュ☆（＾～＾）
        lookups_the_square(next_square);
    } else if current_board.exists_km(&next_square) {
        // ループを抜けるぜ☆（＾～＾）
        return true;
    }
    false
}

/// 成る前を含めない、隣への利き
fn lookup_no_promotion_source_by_piece_next<F1>(
    dst_piece: &Piece,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    lookups_the_square: &mut F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    if current_board.has_sq_km(&next_square, dst_piece, speed_of_light) {
        lookups_the_square(next_square);
    }
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
    let piece_type_src = speed_of_light
        .get_piece_struct(&ps_dst.demoted)
        .piece_type();
    let piece_src = &speed_of_light
        .get_piece_struct_by_phase_and_piece_type(&ps_dst.phase(), piece_type_src)
        .piece;
    let square_dst_piece_src = SquareAndPiece::new(square_dst, piece_src);

    let piece_type_narumae_num = speed_of_light
        .get_piece_type_struct_from_piece(&ps_dst.demoted)
        .serial_piece_number;

    MGDirection::for_all(&mut |i_dir| {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &PieceDirection;
        if &Phase::First == &ps_dst.phase() {
            p_kmdir = &KM_UGOKI.back[piece_type_narumae_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[piece_type_narumae_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        // 進みたいマスから戻ったマス
        use crate::model::univ::gam::misc::piece_direction::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    Squares::looking_next_from(
                        &Rotation::Ccw180,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 東
                    Squares::next_of(
                        &Rotation::Ccw180,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    Squares::looking_next_from(
                        &Rotation::Ccw225,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 北東
                    Squares::next_of(
                        &Rotation::Ccw225,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            NNE => {
                // 北北東
                Squares::north_east_keima_of(
                    UpsideDown::Origin,
                    &Phase::First,
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        lookup_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    Squares::looking_next_from(
                        &Rotation::Ccw270,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 北
                    Squares::next_of(
                        &Rotation::Ccw270,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            NNW => {
                // 北北西
                Squares::north_east_keima_of(
                    UpsideDown::Flip,
                    &Phase::First.turn(),
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        lookup_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    Squares::looking_next_from(
                        &Rotation::Ccw315,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 北西
                    Squares::next_of(
                        &Rotation::Ccw315,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    Squares::looking_next_from(
                        &Rotation::Ccw0,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 西
                    Squares::next_of(
                        &Rotation::Ccw0,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    Squares::looking_next_from(
                        &Rotation::Ccw45,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 南西
                    Squares::next_of(
                        &Rotation::Ccw45,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            SSW => {
                // 南南西
                Squares::north_east_keima_of(
                    UpsideDown::Origin,
                    &Phase::First.turn(),
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        lookup_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    Squares::looking_next_from(
                        &Rotation::Ccw90,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 南
                    Squares::next_of(
                        &Rotation::Ccw90,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            SSE => {
                // 南南東
                Squares::north_east_keima_of(
                    UpsideDown::Flip,
                    &Phase::First,
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        lookup_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_board,
                            speed_of_light,
                            &mut lookups_the_square,
                            next_square,
                        );
                        true
                    },
                );
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    Squares::looking_next_from(
                        &Rotation::Ccw135,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 南東
                    Squares::next_of(
                        &Rotation::Ccw135,
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_board,
                                speed_of_light,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
            }
            Owari => return true,
        }
        false
    });
}

/// 成る前の移動元、長い利き
fn lookup_before_promotion_source_sliding<F1>(
    source_piece: &Piece,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    if current_board.has_sq_km(&next_square, source_piece, speed_of_light) {
        // 指定の駒があれば、その升は移動元になる☆ 続行☆（＾～＾）
        lookups_the_square(next_square);
    } else if current_board.exists_km(&next_square) {
        // なんか他の駒があれば終わり☆ ループを抜けるぜ☆（＾～＾）
        return true;
    }
    false
}

/// 成る前の移動元、 隣升への利き
fn lookup_before_promotion_source_next<F1>(
    source_piece: &Piece,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    if current_board.has_sq_km(&next_square, source_piece, speed_of_light) {
        lookups_the_square(next_square);
    }
}

/// 移動元升生成
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 手番の先後と、移動先升　を指定することで　指し手を生成するぜ☆（＾～＾）
///
/// 1. 手番の先後    phase
/// 2. 移動先升      ms_dst
///
/// その升に到達できる駒が居る升を取得☆（＾～＾）
/// TODO 成りの動きも考えたい。升だけではなく、成りの有無☆（＾～＾）
pub fn lookup_no_promotion_source_by_phase_square<F1>(
    phase: &Phase,
    square_dst: &Square,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
) where
    F1: FnMut(Square),
{
    assert_in_board_as_absolute(
        square_dst.address,
        "make_no_promotion_source_by_phase_square",
    );

    // 移動先の筋、段
    let (_dx, dy) = square_dst.to_file_rank();

    // 駒種類
    for piece_type in PIECE_TYPE_ARRAY.iter() {
        // 行先の無いところに駒を進めることの禁止☆（＾～＾）
        let km = &speed_of_light
            .get_piece_struct_by_phase_and_piece_type(&phase, *piece_type)
            .piece;
        use crate::model::univ::gam::misc::piece::Piece::*;
        match km {
            Knight1 => {
                // ▲うさぎ　は１、２段目には進めない
                if dy < RANK_3 {
                    continue;
                }
            }
            Lance1 | Pawn1 => {
                // ▲しし、▲ひよこ　は１段目には進めない
                if dy < RANK_2 {
                    continue;
                }
            }
            Knight2 => {
                // ▽うさぎ　は８、９段目には進めない
                if RANK_7 < dy {
                    continue;
                }
            }
            Lance2 | Pawn2 => {
                // ▽しし、▽ひよこ　は９段目には進めない
                if RANK_8 < dy {
                    continue;
                }
            }
            _ => {}
        }

        let dst_sq_piece = SquareAndPiece::new(
            square_dst,
            &Piece::from_phase_and_piece_type(phase, *piece_type),
        );

        let piece_type_num = speed_of_light
            .get_piece_type_struct_from_piece_type(piece_type)
            .serial_piece_number;
        MGDirection::for_all(&mut |i_dir| {
            // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir = if &Phase::First == phase {
                &KM_UGOKI.back[piece_type_num][i_dir]
            // g_writeln(&format!("get_src_by_phase_ms 先手なら piece_type={} piece_type_num={} p_kmdir={}",
            //     piece_type, piece_type_num, p_kmdir
            // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[piece_type_num][i_dir]);
                &_kmdir
                // g_writeln(&format!("get_src_by_phase_ms 後手なら piece_type={} piece_type_num={} p_kmdir={}",
                //     piece_type, piece_type_num, p_kmdir
                // ));
            };

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use crate::model::univ::gam::misc::piece_direction::PieceDirection::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        Squares::looking_next_from(
                            &Rotation::Ccw180,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 東
                        Squares::next_of(
                            &Rotation::Ccw180,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        Squares::looking_next_from(
                            &Rotation::Ccw225,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北東
                        Squares::next_of(
                            &Rotation::Ccw225,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                NNE => {
                    // 北北東
                    Squares::north_east_keima_of(
                        UpsideDown::Origin,
                        &Phase::First,
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            lookup_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        Squares::looking_next_from(
                            &Rotation::Ccw270,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北
                        Squares::next_of(
                            &Rotation::Ccw270,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                NNW => {
                    // 北北西
                    Squares::north_east_keima_of(
                        UpsideDown::Flip,
                        &Phase::First.turn(),
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            lookup_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        Squares::looking_next_from(
                            &Rotation::Ccw315,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北西
                        Squares::next_of(
                            &Rotation::Ccw315,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        Squares::looking_next_from(
                            &Rotation::Ccw0,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 西
                        Squares::next_of(
                            &Rotation::Ccw0,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        Squares::looking_next_from(
                            &Rotation::Ccw45,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南西
                        Squares::next_of(
                            &Rotation::Ccw45,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                SSW => {
                    // 南南西
                    Squares::north_east_keima_of(
                        UpsideDown::Origin,
                        &Phase::First.turn(),
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            lookup_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        Squares::looking_next_from(
                            &Rotation::Ccw90,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南
                        Squares::next_of(
                            &Rotation::Ccw90,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                SSE => {
                    // 南南東
                    Squares::north_east_keima_of(
                        UpsideDown::Flip,
                        &Phase::First,
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            lookup_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        Squares::looking_next_from(
                            &Rotation::Ccw135,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南東
                        Squares::next_of(
                            &Rotation::Ccw135,
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                lookup_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                Owari => return true,
            }
            false
        });
    }
}

// 移動元升、長い利き☆（＾～＾）
fn lookup_no_promotion_source_by_phase_sliding<F1>(
    dst_sq_piece: &SquareAndPiece,
    current_board: &Board,
    lookups_the_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    if let Some(piece) = current_board.get_piece_by_square(&next_square) {
        if piece == dst_sq_piece.piece {
            lookups_the_square(next_square);
        }
        false
    } else {
        // End of sliding.
        true
    }
}
// 移動元升、隣☆（＾～＾）
fn lookup_no_promotion_source_by_phase_next<F1>(
    dst_sq_piece: &SquareAndPiece,
    current_board: &Board,
    lookup_the_square: &mut F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    if let Some(piece) = current_board.get_piece_by_square(&next_square) {
        if piece == dst_sq_piece.piece {
            lookup_the_square(next_square);
        }
    }
}

/// 移動元升生成（成る前）
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 手番の先後と、移動先升　を指定することで　指し手を生成するぜ☆（＾～＾）
pub fn lookup_before_promotion_source_by_phase_square<F1>(
    phase: &Phase,
    square_dst: &Square,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_square: F1,
) where
    F1: FnMut(Square),
{
    assert_in_board_as_absolute(
        square_dst.address,
        "make_before_promotion_source_by_phase_square",
    );

    // 駒種類
    for piece_type in PIECE_TYPE_ARRAY.iter() {
        let km_src = &speed_of_light
            .get_piece_struct_by_phase_and_piece_type(&phase, *piece_type)
            .piece;

        // +--------------------+
        // | 移動前は非成駒か？ |
        // +--------------------+
        let ps_src = speed_of_light.get_piece_struct(km_src);
        if ps_src.is_promoted() {
            continue; // 成る前に成駒なら、成りの動きをしていない
        }

        if !ps_src.is_promotable() {
            // 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
            continue;
        }

        let dst_sq_and_demoted_piece = SquareAndPiece::new(
            square_dst,
            &Piece::from_phase_and_piece_type(phase, *piece_type),
        );

        // 成れる駒は、成る前の駒の動きも調べる
        // 成り駒に、行先の無いところは無いぜ☆

        let piece_type_num = speed_of_light
            .get_piece_type_struct_from_piece_type(piece_type)
            .serial_piece_number;
        MGDirection::for_all(&mut |i_dir| {
            // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            // let p_kmdir: &PieceDirection;
            let p_kmdir = if Phase::First == *phase {
                &KM_UGOKI.back[piece_type_num][i_dir]
            // g_writeln(&format!("get_src_by_phase_ms 先手なら piece_type={} piece_typece_type_num={} p_kmdir={}",
            //     piece_type, piece_type_num, p_kmdir
            // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[piece_type_num][i_dir]);
                &_kmdir
                // g_writeln(&format!("get_src_by_phase_ms 後手なら piece_type={} piece_type_num={} p_kmdir={}",
                //     piece_type, piece_type_num, p_kmdir
                // ));
            };

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use crate::model::univ::gam::misc::piece_direction::PieceDirection::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        Squares::looking_next_from(
                            &Rotation::Ccw180,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 東
                        Squares::next_of(
                            &Rotation::Ccw180,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        Squares::looking_next_from(
                            &Rotation::Ccw225,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北東
                        Squares::next_of(
                            &Rotation::Ccw225,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                NNE => {
                    // 北北東
                    Squares::north_east_keima_of(
                        UpsideDown::Origin,
                        &Phase::First,
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        Squares::looking_next_from(
                            &Rotation::Ccw270,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北
                        Squares::next_of(
                            &Rotation::Ccw270,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                NNW => {
                    // 北北西
                    Squares::north_east_keima_of(
                        UpsideDown::Flip,
                        &Phase::First.turn(),
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        Squares::looking_next_from(
                            &Rotation::Ccw315,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北西
                        Squares::next_of(
                            &Rotation::Ccw315,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        Squares::looking_next_from(
                            &Rotation::Ccw0,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 西
                        Squares::next_of(
                            &Rotation::Ccw0,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        Squares::looking_next_from(
                            &Rotation::Ccw45,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南西
                        Squares::next_of(
                            &Rotation::Ccw45,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                SSW => {
                    // 南南西
                    Squares::north_east_keima_of(
                        UpsideDown::Origin,
                        &Phase::First.turn(),
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        Squares::looking_next_from(
                            &Rotation::Ccw90,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南
                        Squares::next_of(
                            &Rotation::Ccw90,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                SSE => {
                    // 南南東
                    Squares::north_east_keima_of(
                        UpsideDown::Flip,
                        &Phase::First,
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            lookup_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_board,
                                &mut lookups_the_square,
                                next_square,
                            );
                            true
                        },
                    );
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        Squares::looking_next_from(
                            &Rotation::Ccw135,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南東
                        Squares::next_of(
                            &Rotation::Ccw135,
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                lookup_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_board,
                                    &mut lookups_the_square,
                                    next_square,
                                );
                                true
                            },
                        );
                    }
                }
                Owari => return true,
            }
            false
        });
    }
}

/// 成る前移動元升、長い利き☆（＾～＾）
fn lookup_before_promotion_source_by_phase_sliding<F1>(
    dst_sq_and_demoted_piece: &SquareAndPiece,
    current_board: &Board,
    lookups_the_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    if let Some(piece) = current_board.get_piece_by_square(&next_square) {
        // 指定した駒に一致すれば。
        if piece == dst_sq_and_demoted_piece.piece {
            lookups_the_square(next_square);
        }
        false
    } else {
        // End of sliding.
        return true;
    }
}
/// 成る前移動元升、 隣☆（＾～＾）
fn lookup_before_promotion_source_by_phase_next<F1>(
    dst_sq_and_demoted_piece: &SquareAndPiece,
    current_board: &Board,
    lookups_the_square: &mut F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    if let Some(piece) = current_board.get_piece_by_square(&next_square) {
        if piece == dst_sq_and_demoted_piece.piece {
            lookups_the_square(next_square);
        }
    }
}

/// 打の駒種類生成
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 打ちたい升　を指定することで　指し手を生成するぜ☆（＾～＾）
///
/// 1. 移動先の升    ms_dst
/// 2. 移動先の駒    piece_dst  ※先後が要るので、piece_typeではなくkm。
///
/// そこに打てる駒種類を返す。
///
/// # Arguments
///
/// * `lookups_the_drops` - Piece type hash.
pub fn lookup_drop_by_square_piece<F1>(
    destination_sqp: &SquareAndPiece,
    current_board: &Board,
    speed_of_light: &MLSpeedOfLightVo,
    mut lookups_the_drops: F1,
) where
    F1: FnMut(usize),
{
    assert_in_board_as_absolute(destination_sqp.square.address, "make_drop_by_square_piece");

    let ps_dst = speed_of_light.get_piece_struct(&destination_sqp.piece);
    let piece_type_dst = ps_dst.piece_type();
    if !speed_of_light
        .get_piece_type_struct_from_piece_type(&piece_type_dst)
        .can_drop
    {
        return; // 打って出てくることがない駒なら終了
    }

    // +------------------------+
    // | 打ちたいところは空升か |
    // +------------------------+
    if let Some(_piece_on_board) = current_board.get_piece_by_square(&destination_sqp.square) {
        // 駒があるところに打つ手は終了
        return;
    }
    // 駒が無いところに打つ

    // +------------------+
    // | 持っている駒か？ |
    // +------------------+
    if current_board.get_hand(&destination_sqp.piece, speed_of_light) < 1 {
        return; // 持っていない駒は打てない
    }

    // 回転していない将棋盤から見た筋番号
    let (suji, dy) = destination_sqp.square.to_file_rank();
    /*
     * Square は 将棋盤座標
     *
     * 考えることを打に限れば、先手も、後手も、後手から見た座標を使えば十分だぜ☆（＾～＾）
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     */
    let sq = kaiten180_sq_by_sq_phase(&destination_sqp.square, &ps_dst.phase());

    assert_in_board_as_absolute(sq.address, "Ｉnsert_da_piece_type_by_ms_km＜その２＞");
    //let (_x,y) = ms_to_suji_dan(ms);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use crate::model::univ::gam::misc::piece::Piece::*;
    match destination_sqp.piece {
        Knight1 => {
            // ▲うさぎ　は１、２段目には進めない
            if dy < RANK_3 {
                return;
            }
        }
        // ▲しし、▲ひよこ　は１段目には進めない
        Lance1 => {
            if dy < RANK_2 {
                return;
            }
        }
        Pawn1 => {
            // ▲ひよこ　は２歩できない
            if dy < RANK_2
                || current_board.exists_fu_by_phase_suji(&ps_dst.phase(), suji, speed_of_light)
            {
                return;
            }
        }
        Knight2 => {
            // ▽うさぎ　は８、９段目には進めない
            if RANK_7 < dy {
                return;
            }
        }
        // ▽しし、▽ひよこ　は９段目には進めない
        Lance2 => {
            if RANK_8 < dy {
                return;
            }
        }
        Pawn2 => {
            // ▽ひよこ　は２歩できない
            if RANK_8 < dy
                || current_board.exists_fu_by_phase_suji(&ps_dst.phase(), suji, speed_of_light)
            {
                return;
            }
        }
        _ => {}
    }

    lookups_the_drops(
        speed_of_light
            .get_piece_type_struct_from_piece_type(&piece_type_dst)
            .serial_piece_number,
    );
}
