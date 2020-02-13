//!
//! 指し手の要素☆（＾～＾）
//!

use super::super::super::controller::common_use::cu_asserts_controller::*;
use super::super::super::controller::common_use::cu_conv_controller::*;
use super::super::super::model::dto::search_part::sp_position_dto::*;
use super::super::super::model::vo::game_part::gp_phase_vo::Phase;
use super::super::super::model::vo::game_part::gp_piece_struct_vo::GPPieceStructVo;
use super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::model::vo::game_part::gp_square_and_piece_struct_vo::GPSquareAndPieceStructVo;
use super::super::super::model::vo::game_part::gp_square_and_piece_vo::*;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_piece_direction_vo::*;
use super::super::super::model::vo::other_part::op_piece_movement_vo::*;
use super::mg_square_scanner::SquareScanner;
use std::collections::HashSet;
use std::hash::BuildHasher;

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
pub fn make_no_promotion_source_by_square_and_piece<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&square_dst, "make_no_promotion_source_by_square_and_piece");

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    if !this_piece_has_a_destination(square_dst, ps_dst) {
        return;
    }

    let piece_type_num = speed_of_light
        .get_piece_type_struct_vo_from_piece_type(&ps_dst.piece_type())
        .serial_piece_number;

    for i_dir in 0..KM_UGOKI_LN {
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
        use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    SquareScanner::for_each_east(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 東
                    SquareScanner::next_to_east(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    SquareScanner::for_each_north_east(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 北東
                    SquareScanner::next_to_north_east(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            NNE => {
                // 北北東
                SquareScanner::next_to_north_north_east(square_dst, &mut |next_square| {
                    make_no_promotion_source_by_piece_next(
                        ps_dst.piece(),
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                        next_square,
                    )
                });
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    SquareScanner::for_each_north(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 北
                    SquareScanner::next_to_north(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            NNW => {
                // 北北西
                SquareScanner::next_to_north_north_west(square_dst, &mut |next_square| {
                    make_no_promotion_source_by_piece_next(
                        ps_dst.piece(),
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                        next_square,
                    )
                });
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    SquareScanner::for_each_north_west(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 北西
                    SquareScanner::next_to_north_west(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    SquareScanner::for_each_west(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 西
                    SquareScanner::next_to_west(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    SquareScanner::for_each_south_west(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 南西
                    SquareScanner::next_to_south_west(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            SSW => {
                // 南南西
                SquareScanner::next_to_south_south_west(square_dst, &mut |next_square| {
                    make_no_promotion_source_by_piece_next(
                        ps_dst.piece(),
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                        next_square,
                    )
                });
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    SquareScanner::for_each_south(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 南
                    SquareScanner::next_to_south(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            SSE => {
                // 南南東
                SquareScanner::next_to_south_south_east(square_dst, &mut |next_square| {
                    make_no_promotion_source_by_piece_next(
                        ps_dst.piece(),
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                        next_square,
                    )
                });
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    SquareScanner::for_each_south_east(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_sliding(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                } else {
                    // 南東
                    SquareScanner::next_to_south_east(square_dst, &mut |next_square| {
                        make_no_promotion_source_by_piece_next(
                            ps_dst.piece(),
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            Owari => break,
        }
    }
}

/// この駒には行き先があります。
fn this_piece_has_a_destination(square_dst: &Square, ps_dst: &GPPieceStructVo) -> bool {
    let (_dx, dy) = square_dst.to_file_rank();

    use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
    match ps_dst.piece() {
        Knight1 => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {
                return false;
            }
        }
        Lance1 | Pawn1 => {
            // ▼しし、▼ひよこ　は１段目には進めない
            if dy < DAN_2 {
                return false;
            }
        }
        Knight2 => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {
                return false;
            }
        }
        Lance2 | Pawn2 => {
            // △しし、△ひよこ　は９段目には進めない
            if DAN_8 < dy {
                return false;
            }
        }
        _ => {}
    }

    true
}

// 成る前を含めない、長い利き
fn make_no_promotion_source_by_piece_sliding<F1>(
    dst_piece: &GPPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    if current_position.has_sq_km(&next_square, dst_piece, speed_of_light) {
        // TODO ポインター渡しできないもんか……☆（＾～＾）あるいはハッシュ☆（＾～＾）
        gets_square(next_square);
    } else if current_position.exists_km(&next_square, speed_of_light) {
        // ループを抜けるぜ☆（＾～＾）
        return true;
    }
    false
}

/// 成る前を含めない、隣への利き
fn make_no_promotion_source_by_piece_next<F1>(
    dst_piece: &GPPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    if current_position.has_sq_km(&next_square, dst_piece, speed_of_light) {
        gets_square(next_square);
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
pub fn make_before_promotion_source_by_square_piece<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&square_dst, "make_before_promotion_source_by_square_piece");

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
        .get_piece_struct_vo(ps_dst.demote())
        .piece_type();
    let piece_src = speed_of_light
        .get_piece_struct_vo_by_phase_and_piece_type(&ps_dst.phase(), piece_type_src)
        .piece();
    let square_dst_piece_src = GPSquareAndPieceVo::new(square_dst, piece_src);

    let piece_type_narumae_num = speed_of_light
        .get_piece_type_struct_vo_from_piece(ps_dst.demote())
        .serial_piece_number;

    for i_dir in 0..KM_UGOKI_LN {
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
        use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    SquareScanner::for_each_east(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 東
                    SquareScanner::next_to_east(&square_dst_piece_src.square, &mut |next_square| {
                        make_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    SquareScanner::for_each_north_east(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 北東
                    SquareScanner::next_to_north_east(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
            }
            NNE => {
                // 北北東
                SquareScanner::next_to_north_north_east(
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        make_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    },
                );
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    SquareScanner::for_each_north(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 北
                    SquareScanner::next_to_north(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
            }
            NNW => {
                // 北北西
                SquareScanner::next_to_north_north_west(
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        make_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    },
                );
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    SquareScanner::for_each_north_west(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 北西
                    SquareScanner::next_to_north_west(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    SquareScanner::for_each_west(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 西
                    SquareScanner::next_to_west(&square_dst_piece_src.square, &mut |next_square| {
                        make_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    });
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    SquareScanner::for_each_south_west(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 南西
                    SquareScanner::next_to_south_west(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
            }
            SSW => {
                // 南南西
                SquareScanner::next_to_south_south_west(
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        make_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    },
                );
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    SquareScanner::for_each_south(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 南
                    SquareScanner::next_to_south(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
            }
            SSE => {
                // 南南東
                SquareScanner::next_to_south_south_east(
                    &square_dst_piece_src.square,
                    &mut |next_square| {
                        make_before_promotion_source_next(
                            &square_dst_piece_src.piece,
                            current_position,
                            speed_of_light,
                            &mut gets_square,
                            next_square,
                        )
                    },
                );
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    SquareScanner::for_each_south_east(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_sliding(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                } else {
                    // 南東
                    SquareScanner::next_to_south_east(
                        &square_dst_piece_src.square,
                        &mut |next_square| {
                            make_before_promotion_source_next(
                                &square_dst_piece_src.piece,
                                current_position,
                                speed_of_light,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
            }
            Owari => break,
        }
    }
}

/// 成る前の移動元、長い利き
fn make_before_promotion_source_sliding<F1>(
    source_piece: &GPPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    if current_position.has_sq_km(&next_square, source_piece, speed_of_light) {
        // 指定の駒があれば、その升は移動元になる☆ 続行☆（＾～＾）
        gets_square(next_square);
    } else if current_position.exists_km(&next_square, speed_of_light) {
        // なんか他の駒があれば終わり☆ ループを抜けるぜ☆（＾～＾）
        return true;
    }
    false
}

/// 成る前の移動元、 隣升への利き
fn make_before_promotion_source_next<F1>(
    source_piece: &GPPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    if current_position.has_sq_km(&next_square, source_piece, speed_of_light) {
        gets_square(next_square);
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
pub fn make_drop_by_square_piece<F1>(
    destination_sqp: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_piece_type_hash: F1,
) where
    F1: FnMut(usize),
{
    assert_banjo_sq(&destination_sqp.square, "make_drop_by_square_piece");

    let ps_dst = speed_of_light.get_piece_struct_vo(&destination_sqp.piece);
    let piece_type_dst = ps_dst.piece_type();
    if !speed_of_light
        .get_piece_type_struct_vo_from_piece_type(&piece_type_dst)
        .can_drop
    {
        return; // 打って出てくることがない駒なら終了
    }

    // +------------------------+
    // | 打ちたいところは空升か |
    // +------------------------+
    let km_banjo = current_position.get_piece_by_square(&destination_sqp.square);
    match km_banjo {
        GPPieceVo::NonePiece => {}
        _ => {
            return;
        } // 駒があるところに打つ手は終了
    }
    // 駒が無いところに打つ

    // +------------------+
    // | 持っている駒か？ |
    // +------------------+
    if current_position.get_hand(&destination_sqp.piece, speed_of_light) < 1 {
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

    assert_banjo_sq(&sq, "Ｉnsert_da_piece_type_by_ms_km＜その２＞");
    //let (_x,y) = ms_to_suji_dan(ms);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
    match destination_sqp.piece {
        Knight1 => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {
                return;
            }
        }
        // ▼しし、▼ひよこ　は１段目には進めない
        Lance1 => {
            if dy < DAN_2 {
                return;
            }
        }
        Pawn1 => {
            // ▼ひよこ　は２歩できない
            if dy < DAN_2
                || current_position.exists_fu_by_phase_suji(&ps_dst.phase(), suji, speed_of_light)
            {
                return;
            }
        }
        Knight2 => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {
                return;
            }
        }
        // △しし、△ひよこ　は９段目には進めない
        Lance2 => {
            if DAN_8 < dy {
                return;
            }
        }
        Pawn2 => {
            // △ひよこ　は２歩できない
            if DAN_8 < dy
                || current_position.exists_fu_by_phase_suji(&ps_dst.phase(), suji, speed_of_light)
            {
                return;
            }
        }
        _ => {}
    }

    gets_piece_type_hash(
        speed_of_light
            .get_piece_type_struct_vo_from_piece_type(&piece_type_dst)
            .serial_piece_number,
    );
}

/// 移動先升生成
///
/// これは普通☆（＾～＾）
/// 動かしたい駒と、その駒がある升　を指定することで　指し手を生成するぜ☆（＾～＾）
///
/// 1. 移動元升
/// 2. 移動したい駒
///
/// 駒の移動先を取得。合法手生成の動き☆（＾～＾）
///
/// km_src   : 移動元の駒
/// ms_src   : 移動元の升
/// to_nari  : 成りの手を生成するなら真
pub fn make_destination_by_square_piece<S: BuildHasher>(
    source_sqp: &GPSquareAndPieceVo,
    to_nari: bool,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    // result, result2 で入れ直しがあるのでむずかしい☆（＾～＾）
    // 成れない動きをあとで除外する☆（＾～＾）
    result: &mut HashSet<Square, S>,
) {
    assert_banjo_sq(&source_sqp.square, "make_destination_by_square_piece");

    // 移動先の升、駒構造体、駒種類インデックス
    let ps_src = speed_of_light.get_piece_struct_vo(&source_sqp.piece);
    let source_sqps = GPSquareAndPieceStructVo::new(&source_sqp.square, &ps_src);
    let piece_type_src = ps_src.piece_type();

    // +--------------+
    // | 成れる駒か？ |
    // +--------------+
    if to_nari
        && !speed_of_light
            .get_piece_type_struct_vo_from_piece_type(&piece_type_src)
            .can_promote
    {
        return; // 成れる駒でないなら、成りの動きはしない
    }

    let piece_type_num = speed_of_light
        .get_piece_type_struct_vo_from_piece_type(&piece_type_src)
        .serial_piece_number;

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &PieceDirection;
        if &Phase::First == &ps_src.phase() {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[piece_type_num][i_dir]);
            p_kmdir = &_kmdir;
        } else {
            p_kmdir = &KM_UGOKI.back[piece_type_num][i_dir]
        };

        // 駒の位置を開始地点に、離れていくように調べていく
        use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    SquareScanner::for_each_east(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 東
                    SquareScanner::next_to_east(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    SquareScanner::for_each_north_east(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 北東
                    SquareScanner::next_to_north_east(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            NNE => {
                // 北北東
                SquareScanner::next_to_north_north_east(&source_sqps.square, &mut |next_square| {
                    make_destination_next(
                        &source_sqps.piece_struct.phase(),
                        current_position,
                        speed_of_light,
                        result,
                        next_square,
                    )
                });
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    SquareScanner::for_each_north(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 北
                    SquareScanner::next_to_north(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            NNW => {
                // 北北西
                SquareScanner::next_to_north_north_west(&source_sqps.square, &mut |next_square| {
                    make_destination_next(
                        &source_sqps.piece_struct.phase(),
                        current_position,
                        speed_of_light,
                        result,
                        next_square,
                    )
                });
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    SquareScanner::for_each_north_west(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 北西
                    SquareScanner::next_to_north_west(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    SquareScanner::for_each_west(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 西
                    SquareScanner::next_to_west(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    SquareScanner::for_each_south_west(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 南西
                    SquareScanner::next_to_south_west(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            SSW => {
                // 南南西
                SquareScanner::next_to_south_south_west(&source_sqps.square, &mut |next_square| {
                    make_destination_next(
                        &source_sqps.piece_struct.phase(),
                        current_position,
                        speed_of_light,
                        result,
                        next_square,
                    )
                });
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    SquareScanner::for_each_south(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 南
                    SquareScanner::next_to_south(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            SSE => {
                // 南南東
                SquareScanner::next_to_south_south_east(&source_sqps.square, &mut |next_square| {
                    make_destination_next(
                        &source_sqps.piece_struct.phase(),
                        current_position,
                        speed_of_light,
                        result,
                        next_square,
                    )
                });
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    SquareScanner::for_each_south_east(&source_sqps.square, &mut |next_square| {
                        make_destination_sliding(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                } else {
                    // 南東
                    SquareScanner::next_to_south_east(&source_sqps.square, &mut |next_square| {
                        make_destination_next(
                            &source_sqps.piece_struct.phase(),
                            current_position,
                            speed_of_light,
                            result,
                            next_square,
                        )
                    });
                }
            }
            Owari => break,
        }
    }

    if to_nari {
        // +------------------------------+
        // | 成れる動き以外での成りの禁止 |
        // +------------------------------+
        use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
        match source_sqp.piece {
            Rook1 | Bishop1 | Silver1 => {
                // ▼きりん、▼ぞう、▼ねこ　は
                // 移動元または移動先が　１～３段目なら成れる
                remake_promotion_destination_rook_bishop_silver1(&source_sqp.square, result);
            }
            Knight1 | Lance1 | Pawn1 => {
                // ▼うさぎ、▼しし、▼ひよこ　は
                // 移動先が　１～３段目なら成れる
                remake_promotion_destination_knight_lance_pawn1(result);
            }
            Rook2 | Bishop2 | Silver2 => {
                // △きりん、△ぞう、△ねこ　は
                // 移動元または移動先が　７～９段目なら成れる
                remake_promotion_destination_rook_bishop_silver2(&source_sqp.square, result);
            }
            Knight2 | Lance2 | Pawn2 => {
                // △うさぎ、△しし、△ひよこ　は
                // 移動先が　７～９段目なら成れる
                remake_promotion_destination_knight_lance_pawn2(result);
            }
            _ => {}
        }
    } else {
        // +----------------------------------------+
        // | 行先の無いところに駒を進めることの禁止 |
        // +----------------------------------------+
        use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
        match source_sqp.piece {
            Knight1 => {
                // ▼うさぎ　は１、２段目には進めない
                remake_forbidden_destination_knight1(result);
            }
            Lance1 | Pawn1 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                remake_forbidden_destination_lance_pawn1(result);
            }
            Knight2 => {
                // △うさぎ　は８、９段目には進めない
                remake_forbidden_destination_knight2(result);
            }
            Lance2 | Pawn2 => {
                // △しし、△ひよこ　は９段目には進めない
                remake_forbidden_destination_lance_pawn2(result);
            }
            _ => {}
        }
    }
}

/// 移動先升、長い利き
///
/// # Arguments
///
/// * `speed_of_light` - 盤上の駒の Phase を調べるために使う☆（＾～＾）
fn make_destination_sliding<S: BuildHasher>(
    source_piece_phase: &Phase,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
    next_square: Square,
) -> bool {
    // 自駒でなければ進める。
    let dst_phase = current_position.get_phase_by_sq(&next_square, speed_of_light);
    if dst_phase != *source_piece_phase {
        result.insert(next_square);
    }

    // 駒があったのなら、ループ終わり。
    dst_phase != Phase::None
}

/// 移動先升、 隣☆（＾～＾）
fn make_destination_next<S: BuildHasher>(
    source_piece_phase: &Phase,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
    next_square: Square,
) {
    let dst_phase = current_position.get_phase_by_sq(&next_square, speed_of_light);
    if dst_phase != *source_piece_phase {
        result.insert(next_square);
    }
}

/// 移動先升、成り：▼きりん、▼ぞう、▼ねこ
fn remake_promotion_destination_rook_bishop_silver1<S: BuildHasher>(
    sq_src: &Square,
    result: &mut HashSet<Square, S>,
) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if sq_src.rank < DAN_4 && square_dst.rank < DAN_4 {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for ms_dst in result2.iter() {
        result.insert(ms_dst.clone());
    }
}

/// 移動先升、成り： ▼うさぎ、▼しし、▼ひよこ
fn remake_promotion_destination_knight_lance_pawn1<S: BuildHasher>(
    result: &mut HashSet<Square, S>,
) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if square_dst.rank < DAN_4 {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
    }
}

/// 移動先升、成り：△きりん、△ぞう、△ねこ
fn remake_promotion_destination_rook_bishop_silver2<S: BuildHasher>(
    sq_src: &Square,
    result: &mut HashSet<Square, S>,
) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if DAN_6 < sq_src.rank && DAN_6 < square_dst.rank {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
    }
}

/// 移動先升、成り：△うさぎ、△しし、△ひよこ
fn remake_promotion_destination_knight_lance_pawn2<S: BuildHasher>(
    result: &mut HashSet<Square, S>,
) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if DAN_6 < square_dst.rank {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
    }
}

/// 移動先升、行き先の無い駒：▼うさぎ
fn remake_forbidden_destination_knight1<S: BuildHasher>(result: &mut HashSet<Square, S>) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if square_dst.rank < DAN_3 {
        } else {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
    }
}

/// 移動先升、行き先の無い駒：▼しし、▼ひよこ
fn remake_forbidden_destination_lance_pawn1<S: BuildHasher>(result: &mut HashSet<Square, S>) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if square_dst.rank < DAN_2 {
        } else {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
    }
}

/// 移動先升、行き先の無い駒：△うさぎ
fn remake_forbidden_destination_knight2<S: BuildHasher>(result: &mut HashSet<Square, S>) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if DAN_7 < square_dst.rank {
        } else {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
    }
}

/// 移動先升、行き先の無い駒：△しし、△ひよこ
fn remake_forbidden_destination_lance_pawn2<S: BuildHasher>(result: &mut HashSet<Square, S>) {
    let mut result2: HashSet<Square> = HashSet::<Square>::new();
    for square_dst in result.iter() {
        if DAN_8 < square_dst.rank {
        } else {
            result2.insert(square_dst.clone());
        }
    }
    // 入れ直し
    result.clear();
    for square_dst in result2.iter() {
        result.insert(square_dst.clone());
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
pub fn make_no_promotion_source_by_phase_square<F1>(
    phase: &Phase,
    square_dst: &Square,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&square_dst, "make_no_promotion_source_by_phase_square");

    // 移動先の筋、段
    let (_dx, dy) = square_dst.to_file_rank();

    // 駒種類
    for piece_type in PIECE_TYPE_ARRAY.iter() {
        // 行先の無いところに駒を進めることの禁止☆（＾～＾）
        let km = speed_of_light
            .get_piece_struct_vo_by_phase_and_piece_type(&phase, *piece_type)
            .piece()
            .clone();
        use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo::*;
        match km {
            Knight1 => {
                // ▼うさぎ　は１、２段目には進めない
                if dy < DAN_3 {
                    continue;
                }
            }
            Lance1 | Pawn1 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                if dy < DAN_2 {
                    continue;
                }
            }
            Knight2 => {
                // △うさぎ　は８、９段目には進めない
                if DAN_7 < dy {
                    continue;
                }
            }
            Lance2 | Pawn2 => {
                // △しし、△ひよこ　は９段目には進めない
                if DAN_8 < dy {
                    continue;
                }
            }
            _ => {}
        }

        let dst_sq_piece = GPSquareAndPieceVo::new(
            square_dst,
            &GPPieceVo::from_phase_and_piece_type(phase, *piece_type),
        );

        let piece_type_num = speed_of_light
            .get_piece_type_struct_vo_from_piece_type(piece_type)
            .serial_piece_number;
        for i_dir in 0..KM_UGOKI_LN {
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
            use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        SquareScanner::for_each_east(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_sliding(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    } else {
                        // 東
                        SquareScanner::next_to_east(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        SquareScanner::for_each_north_east(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北東
                        SquareScanner::next_to_north_east(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                NNE => {
                    // 北北東
                    SquareScanner::next_to_north_north_east(
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        SquareScanner::for_each_north(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_sliding(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    } else {
                        // 北
                        SquareScanner::next_to_north(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    }
                }
                NNW => {
                    // 北北西
                    SquareScanner::next_to_north_north_west(
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        SquareScanner::for_each_north_west(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北西
                        SquareScanner::next_to_north_west(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        SquareScanner::for_each_west(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_sliding(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    } else {
                        // 西
                        SquareScanner::next_to_west(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        SquareScanner::for_each_south_west(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南西
                        SquareScanner::next_to_south_west(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                SSW => {
                    // 南南西
                    SquareScanner::next_to_south_south_west(
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        SquareScanner::for_each_south(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_sliding(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    } else {
                        // 南
                        SquareScanner::next_to_south(&dst_sq_piece.square, &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        });
                    }
                }
                SSE => {
                    // 南南東
                    SquareScanner::next_to_south_south_east(
                        &dst_sq_piece.square,
                        &mut |next_square| {
                            make_no_promotion_source_by_phase_next(
                                &dst_sq_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        SquareScanner::for_each_south_east(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_sliding(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南東
                        SquareScanner::next_to_south_east(
                            &dst_sq_piece.square,
                            &mut |next_square| {
                                make_no_promotion_source_by_phase_next(
                                    &dst_sq_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                Owari => break,
            }
        }
    }
}

// 移動元升、長い利き☆（＾～＾）
fn make_no_promotion_source_by_phase_sliding<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    let exists_piece = current_position.get_piece_by_square(&next_square);
    if *exists_piece == dst_sq_piece.piece {
        gets_square(next_square);
    }
    // End of sliding.
    if *exists_piece != GPPieceVo::NonePiece {
        return true;
    }
    false
}
// 移動元升、隣☆（＾～＾）
fn make_no_promotion_source_by_phase_next<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    let exists_piece = current_position.get_piece_by_square(&next_square);
    if *exists_piece == dst_sq_piece.piece {
        gets_square(next_square);
    }
}

/// 移動元升生成（成る前）
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 手番の先後と、移動先升　を指定することで　指し手を生成するぜ☆（＾～＾）
pub fn make_before_promotion_source_by_phase_square<F1>(
    phase: &Phase,
    square_dst: &Square,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&square_dst, "make_before_promotion_source_by_phase_square");

    // 駒種類
    for piece_type in PIECE_TYPE_ARRAY.iter() {
        let km_src = speed_of_light
            .get_piece_struct_vo_by_phase_and_piece_type(&phase, *piece_type)
            .piece();

        // +--------------------+
        // | 移動前は非成駒か？ |
        // +--------------------+
        let ps_src = speed_of_light.get_piece_struct_vo(km_src);
        if ps_src.is_promoted() {
            continue; // 成る前に成駒なら、成りの動きをしていない
        }

        let prokm_src = ps_src.promote();
        if let GPPieceVo::NonePiece = prokm_src {
            // 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
            continue;
        }

        let dst_sq_and_demoted_piece = GPSquareAndPieceVo::new(
            square_dst,
            &GPPieceVo::from_phase_and_piece_type(phase, *piece_type),
        );

        // 成れる駒は、成る前の駒の動きも調べる
        // 成り駒に、行先の無いところは無いぜ☆

        let piece_type_num = speed_of_light
            .get_piece_type_struct_vo_from_piece_type(piece_type)
            .serial_piece_number;
        for i_dir in 0..KM_UGOKI_LN {
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
            use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        SquareScanner::for_each_east(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 東
                        SquareScanner::next_to_east(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        SquareScanner::for_each_north_east(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北東
                        SquareScanner::next_to_north_east(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                NNE => {
                    // 北北東
                    SquareScanner::next_to_north_north_east(
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            make_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        SquareScanner::for_each_north(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北
                        SquareScanner::next_to_north(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                NNW => {
                    // 北北西
                    SquareScanner::next_to_north_north_west(
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            make_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        SquareScanner::for_each_north_west(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 北西
                        SquareScanner::next_to_north_west(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        SquareScanner::for_each_west(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 西
                        SquareScanner::next_to_west(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        SquareScanner::for_each_south_west(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南西
                        SquareScanner::next_to_south_west(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                SSW => {
                    // 南南西
                    SquareScanner::next_to_south_south_west(
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            make_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        SquareScanner::for_each_south(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南
                        SquareScanner::next_to_south(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                SSE => {
                    // 南南東
                    SquareScanner::next_to_south_south_east(
                        &dst_sq_and_demoted_piece.square,
                        &mut |next_square| {
                            make_before_promotion_source_by_phase_next(
                                &dst_sq_and_demoted_piece,
                                current_position,
                                &mut gets_square,
                                next_square,
                            )
                        },
                    );
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        SquareScanner::for_each_south_east(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_sliding(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    } else {
                        // 南東
                        SquareScanner::next_to_south_east(
                            &dst_sq_and_demoted_piece.square,
                            &mut |next_square| {
                                make_before_promotion_source_by_phase_next(
                                    &dst_sq_and_demoted_piece,
                                    current_position,
                                    &mut gets_square,
                                    next_square,
                                )
                            },
                        );
                    }
                }
                Owari => break,
            }
        }
    }
}

/// 成る前移動元升、長い利き☆（＾～＾）
fn make_before_promotion_source_by_phase_sliding<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
    next_square: Square,
) -> bool
where
    F1: FnMut(Square),
{
    let exists_piece = current_position.get_piece_by_square(&next_square);
    // 指定した駒に一致すれば。
    if *exists_piece == dst_sq_and_demoted_piece.piece {
        gets_square(next_square);
    }
    // End of sliding.
    if *exists_piece != GPPieceVo::NonePiece {
        return true;
    }
    false
}
/// 成る前移動元升、 隣☆（＾～＾）
fn make_before_promotion_source_by_phase_next<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
    next_square: Square,
) where
    F1: FnMut(Square),
{
    let exists_piece = current_position.get_piece_by_square(&next_square);
    if *exists_piece == dst_sq_and_demoted_piece.piece {
        gets_square(next_square);
    }
}

/*
 * 合い駒スペースを算出
 *
 * phase_atk  : 攻めている方の先後
 * ms_atk  : 攻め駒の居る升
 * ms_tgt  : 狙われている駒の居る升
 * piece_type_atk : 攻め駒の駒種類
 */
/*
#[allow(dead_code)]
pub fn get_ms_vec_as_aigoma(
    phase_atk:&Phase,
    ms_atk:&Square,
    ms_tgt:&Square,
    piece_type_attacker:GPPieceTypeVo
    )->Vec<Square> {
    let vec = Vec::new();

    use teigi::shogi_syugo::GPPieceTypeVo::*;
    match piece_type_attacker {
        Rook => {
            // 北方向
            // 西方向
            // 南方向
            // 東方向
        },
        Z => {
            // 北東方向
            // 北西方向
            // 南西方向
            // 南東方向
        },
        S => {
            if match_phase(&Phase::First, &phase_atk) {
                // 北方向

            } else {
                // 南方向

            }
        },
        PK => {
            // 北方向
            // 西方向
            // 南方向
            // 東方向
        },
        PZ => {
            // 北東方向
            // 北西方向
            // 南西方向
            // 南東方向
        },
        _ => {}
    }
    vec
}
*/
