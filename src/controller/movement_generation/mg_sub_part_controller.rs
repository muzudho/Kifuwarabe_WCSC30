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
                    make_no_promotion_source_by_piece_sliding_to_east(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 西東
                    make_no_promotion_source_by_piece_to_west_east(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    make_no_promotion_source_by_piece_sliding_to_north_east(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 北東
                    make_no_promotion_source_by_piece_to_north_east(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            NNE => {
                // 北北東
                make_no_promotion_source_by_piece_to_north_north_east(
                    square_dst,
                    ps_dst,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    make_no_promotion_source_by_piece_sliding_to_north(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 北
                    make_no_promotion_source_by_piece_to_north(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            NNW => {
                // 北北西
                make_no_promotion_source_by_piece_to_north_north_west(
                    square_dst,
                    ps_dst,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    make_no_promotion_source_by_piece_sliding_to_north_west(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 北西
                    make_no_promotion_source_by_piece_to_north_west(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    make_no_promotion_source_by_piece_sliding_to_west(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 西
                    make_no_promotion_source_by_piece_to_west(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    make_no_promotion_source_by_piece_sliding_to_south_west(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 南西
                    make_no_promotion_source_by_piece_to_south_west(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            SSW => {
                // 南南西
                make_no_promotion_source_by_piece_to_south_south_west(
                    square_dst,
                    ps_dst,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    make_no_promotion_source_by_piece_sliding_to_south(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 南
                    make_no_promotion_source_by_piece_to_south(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            SSE => {
                // 南南東
                make_no_promotion_source_by_piece_to_south_south_east(
                    square_dst,
                    ps_dst,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    make_no_promotion_source_by_piece_sliding_to_south_east(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 南東
                    make_no_promotion_source_by_piece_to_south_east(
                        square_dst,
                        ps_dst,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
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

// 成る前を含めない、長い西
fn make_no_promotion_source_by_piece_sliding_to_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_east(*square_dst, &mut |next_square| {
        if current_position.has_sq_km(&next_square, ps_dst.piece(), speed_of_light) {
            // TODO ポインター渡しできないもんか……☆（＾～＾）あるいはハッシュ☆（＾～＾）
            gets_square(next_square);
        } else if current_position.exists_km(&next_square, speed_of_light) {
            // ループを抜けるぜ☆（＾～＾）
            return true;
        }
        false
    });
}

/// 成る前を含めない、西東
fn make_no_promotion_source_by_piece_to_west_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dx + 1 < SUJI_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長い北東
fn make_no_promotion_source_by_piece_sliding_to_north_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    for i_ne in 1..9 {
        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
            if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前を含めない、北東
fn make_no_promotion_source_by_piece_to_north_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、北北東
fn make_no_promotion_source_by_piece_to_north_north_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 2);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長い北
fn make_no_promotion_source_by_piece_sliding_to_north<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    for i_south in 1..9 {
        if dy + i_south < DAN_10 {
            let sq_src = Square::from_file_rank(dx, dy + i_south);
            if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前を含めない、北
fn make_no_promotion_source_by_piece_to_north<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx, dy + 1);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、北北西
fn make_no_promotion_source_by_piece_to_north_north_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 2);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長い北西
fn make_no_promotion_source_by_piece_sliding_to_north_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    for i_se in 1..9 {
        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
            if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前を含めない、北西
fn make_no_promotion_source_by_piece_to_north_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長い西
fn make_no_promotion_source_by_piece_sliding_to_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_west(*square_dst, &mut |next_square| {
        if current_position.has_sq_km(&next_square, ps_dst.piece(), speed_of_light) {
            // 指定の駒があれば、その升は移動元。続行
            gets_square(next_square);
        } else if current_position.exists_km(&next_square, speed_of_light) {
            // なんか他の駒があれば終わり
            return true;
        }
        false
    });
}

/// 成る前を含めない、西
fn make_no_promotion_source_by_piece_to_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if SUJI_0 < dx - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長い南西
fn make_no_promotion_source_by_piece_sliding_to_south_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    for i_ne in 1..9 {
        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
            if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前を含めない、南西
fn make_no_promotion_source_by_piece_to_south_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、南南西
fn make_no_promotion_source_by_piece_to_south_south_west<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 2);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長い南
fn make_no_promotion_source_by_piece_sliding_to_south<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    for i_north in 1..9 {
        if DAN_0 < dy - i_north {
            let sq_src = Square::from_file_rank(dx, dy - i_north);
            if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前を含めない、南
fn make_no_promotion_source_by_piece_to_south<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx, dy - 1);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、南南東
fn make_no_promotion_source_by_piece_to_south_south_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 2);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前を含めない、長南東
fn make_no_promotion_source_by_piece_sliding_to_south_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    for i_nw in 1..9 {
        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
            if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前を含めない、南東
fn make_no_promotion_source_by_piece_to_south_east<F1>(
    square_dst: &Square,
    ps_dst: &GPPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
        if current_position.has_sq_km(&sq_src, ps_dst.piece(), speed_of_light) {
            gets_square(sq_src);
        }
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
        use super::super::super::model::vo::other_part::op_piece_direction_vo::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    make_before_promotion_source_sliding_to_east(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 西東
                    make_before_promotion_source_to_west_east(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    make_before_promotion_source_sliding_to_north_east(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 北東
                    make_before_promotion_source_to_north_east(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            NNE => {
                // 北北東
                make_before_promotion_source_to_north_north_east(
                    &square_dst_piece_src,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    make_before_promotion_source_sliding_to_north(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 北
                    make_before_promotion_source_to_north(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            NNW => {
                // 北北西
                make_before_promotion_source_to_north_north_west(
                    &square_dst_piece_src,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    make_before_promotion_source_sliding_to_north_west(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 北西
                    make_before_promotion_source_to_north_west(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    make_before_promotion_source_sliding_to_west(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 西
                    make_before_promotion_source_to_west(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    make_before_promotion_source_sliding_to_south_west(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 南西
                    make_before_promotion_source_to_south_west(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            SSW => {
                // 南南西
                make_before_promotion_source_to_south_south_west(
                    &square_dst_piece_src,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    make_before_promotion_source_sliding_to_south(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 南
                    make_before_promotion_source_to_south(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            SSE => {
                // 南南東
                make_before_promotion_source_to_south_south_east(
                    &square_dst_piece_src,
                    current_position,
                    speed_of_light,
                    &mut gets_square,
                );
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    make_before_promotion_source_sliding_to_south_east(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                } else {
                    // 南東
                    make_before_promotion_source_to_south_east(
                        &square_dst_piece_src,
                        current_position,
                        speed_of_light,
                        &mut gets_square,
                    );
                }
            }
            Owari => break,
        }
    }
}

/// 成る前の移動元、長い東
fn make_before_promotion_source_sliding_to_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_east(square_dst_piece_src.square, &mut |next_square| {
        if current_position.has_sq_km(&next_square, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(next_square);
        } else if current_position.exists_km(&next_square, speed_of_light) {
            // ループを抜けるぜ☆（＾～＾）
            return true;
        }
        false
    });
}

/// 成る前の移動元、 西東
fn make_before_promotion_source_to_west_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dx + 1 < SUJI_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い北東
fn make_before_promotion_source_sliding_to_north_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    for i_ne in 1..9 {
        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
            if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前の移動元、 北東
fn make_before_promotion_source_to_north_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 北北東
fn make_before_promotion_source_to_north_north_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 2);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い北
fn make_before_promotion_source_sliding_to_north<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    for i_south in 1..9 {
        if dy + i_south < DAN_10 {
            let sq_src = Square::from_file_rank(dx, dy + i_south);
            if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前の移動元、 北
fn make_before_promotion_source_to_north<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx, dy + 1);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 北北西
fn make_before_promotion_source_to_north_north_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 2);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い北西
fn make_before_promotion_source_sliding_to_north_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    for i_se in 1..9 {
        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
            if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前の移動元、 北西
fn make_before_promotion_source_to_north_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い西
fn make_before_promotion_source_sliding_to_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    // 進みたいマスから戻ったマス
    SquareScanner::for_each_west(square_dst_piece_src.square, &mut |next_square| {
        if current_position.has_sq_km(&next_square, &square_dst_piece_src.piece, speed_of_light) {
            // 指定の駒があれば、その升は移動元。続行
            gets_square(next_square);
        } else if current_position.exists_km(&next_square, speed_of_light) {
            // なんか他の駒があれば終わり
            return true;
        }
        false
    });
}

/// 成る前の移動元、 西
fn make_before_promotion_source_to_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if SUJI_0 < dx - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い南西
fn make_before_promotion_source_sliding_to_south_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    for i_ne in 1..9 {
        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
            if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前の移動元、 南西
fn make_before_promotion_source_to_south_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 南南西
fn make_before_promotion_source_to_south_south_west<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 2);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い南
fn make_before_promotion_source_sliding_to_south<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    for i_north in 1..9 {
        if DAN_0 < dy - i_north {
            let sq_src = Square::from_file_rank(dx, dy - i_north);
            if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前の移動元、 南
fn make_before_promotion_source_to_south<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx, dy - 1);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 南南東
fn make_before_promotion_source_to_south_south_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 2);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
        }
    }
}

/// 成る前の移動元、 長い南東
fn make_before_promotion_source_sliding_to_south_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    for i_nw in 1..9 {
        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
            if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
                gets_square(sq_src);
            } else if current_position.exists_km(&sq_src, speed_of_light) {
                break;
            }
        }
    }
}

/// 成る前の移動元、 南東
fn make_before_promotion_source_to_south_east<F1>(
    square_dst_piece_src: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = square_dst_piece_src.square.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
        if current_position.has_sq_km(&sq_src, &square_dst_piece_src.piece, speed_of_light) {
            gets_square(sq_src);
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
pub fn make_drop_piece_type_by_square_piece<F1>(
    destination_sqp: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_piece_type_hash: F1,
) where
    F1: FnMut(usize),
{
    assert_banjo_sq(
        &destination_sqp.square,
        "make_drop_piece_type_by_square_piece",
    );

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
                    make_destination_sliding_to_east(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 西東
                    make_destination_to_west_east(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    make_destination_sliding_to_north_east(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 北東
                    make_destination_to_north_east(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            NNE => {
                // 北北東
                make_destination_sliding_to_north_north_east(
                    &source_sqps,
                    current_position,
                    speed_of_light,
                    result,
                );
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    make_destination_sliding_to_north(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 北
                    make_destination_to_north(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            NNW => {
                // 北北西
                make_destination_to_north_north_west(
                    &source_sqps,
                    current_position,
                    speed_of_light,
                    result,
                );
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    make_destination_sliding_to_north_west(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 北西
                    make_destination_to_north_west(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    make_destination_sliding_to_west(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 西
                    make_destination_to_west(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    make_destination_sliding_to_south_west(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 南西
                    make_destination_to_south_west(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            SSW => {
                // 南南西
                make_destination_to_south_south_west(
                    &source_sqps,
                    current_position,
                    speed_of_light,
                    result,
                );
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    make_destination_sliding_to_south(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 南
                    make_destination_to_south(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                }
            }
            SSE => {
                // 南南東
                make_destination_to_south_south_east(
                    &source_sqps,
                    current_position,
                    speed_of_light,
                    result,
                );
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    make_destination_sliding_to_south_east(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
                } else {
                    // 南東
                    make_destination_to_south_east(
                        &source_sqps,
                        current_position,
                        speed_of_light,
                        result,
                    );
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

/// 移動先升、長い東
///
/// # Arguments
///
/// * `speed_of_light` - 盤上の駒の Phase を調べるために使う☆（＾～＾）
fn make_destination_sliding_to_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    SquareScanner::for_each_east(sq_dst_ps_src.square, &mut |next_square| {
        // 自駒でなければ進める。
        let dst_phase = current_position.get_phase_by_sq(&next_square, speed_of_light);
        if dst_phase != sq_dst_ps_src.piece_struct.phase() {
            result.insert(next_square);
        }

        // 駒があったのなら、ループ終わり。
        dst_phase == Phase::None
    });
}

/// 移動先升、 西東
fn make_destination_to_west_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.file + 1 < SUJI_10 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file + 1, sq_dst_ps_src.square.rank);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い北東
fn make_destination_sliding_to_north_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    for i_ne in 1..9 {
        if sq_dst_ps_src.square.file + i_ne < SUJI_10 && sq_dst_ps_src.square.rank + i_ne < DAN_10 {
            let dst_sq = Square::from_file_rank(
                sq_dst_ps_src.square.file + i_ne,
                sq_dst_ps_src.square.rank + i_ne,
            );
            let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
            if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
                result.insert(dst_sq);
            }
            if &dst_phase != &Phase::None {
                break;
            }
        }
    }
}

/// 移動先升、 北東
fn make_destination_to_north_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.file + 1 < SUJI_10 && sq_dst_ps_src.square.rank + 1 < DAN_10 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file + 1, sq_dst_ps_src.square.rank + 1);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 北北東
fn make_destination_sliding_to_north_north_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.file + 1 < SUJI_10 && sq_dst_ps_src.square.rank + 2 < DAN_10 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file + 1, sq_dst_ps_src.square.rank + 2);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い北
fn make_destination_sliding_to_north<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    for i_south in 1..9 {
        if sq_dst_ps_src.square.rank + i_south < DAN_10 {
            let dst_sq = Square::from_file_rank(
                sq_dst_ps_src.square.file,
                sq_dst_ps_src.square.rank + i_south,
            );
            let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
            if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
                result.insert(dst_sq);
            }
            if &dst_phase != &Phase::None {
                break;
            }
        }
    }
}

/// 移動先升、 北
fn make_destination_to_north<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.rank + 1 < DAN_10 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file, sq_dst_ps_src.square.rank + 1);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 北北西
fn make_destination_to_north_north_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if SUJI_0 < sq_dst_ps_src.square.file - 1 && sq_dst_ps_src.square.rank + 2 < DAN_10 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file - 1, sq_dst_ps_src.square.rank + 2);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い北西
fn make_destination_sliding_to_north_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    for i_se in 1..9 {
        if SUJI_0 < sq_dst_ps_src.square.file - i_se && sq_dst_ps_src.square.rank + i_se < DAN_10 {
            let dst_sq = Square::from_file_rank(
                sq_dst_ps_src.square.file - i_se,
                sq_dst_ps_src.square.rank + i_se,
            );
            let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
            if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
                result.insert(dst_sq);
            }
            if &dst_phase != &Phase::None {
                break;
            }
        }
    }
}

/// 移動先升、 北西
fn make_destination_to_north_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.file - 1 > SUJI_0 && DAN_10 > sq_dst_ps_src.square.rank + 1 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file - 1, sq_dst_ps_src.square.rank + 1);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い西
fn make_destination_sliding_to_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    SquareScanner::for_each_west(sq_dst_ps_src.square, &mut |next_square| {
        let dst_phase = current_position.get_phase_by_sq(&next_square, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(next_square);
        }
        if &dst_phase != &Phase::None {
            return true;
        }
        false
    });
}

/// 移動先升、 西
fn make_destination_to_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if SUJI_0 < sq_dst_ps_src.square.file - 1 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file - 1, sq_dst_ps_src.square.rank);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い南西
fn make_destination_sliding_to_south_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    for i_ne in 1..9 {
        if SUJI_0 < sq_dst_ps_src.square.file - i_ne && DAN_0 < sq_dst_ps_src.square.rank - i_ne {
            let dst_sq = Square::from_file_rank(
                sq_dst_ps_src.square.file - i_ne,
                sq_dst_ps_src.square.rank - i_ne,
            );
            let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
            if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
                result.insert(dst_sq);
            }
            if &dst_phase != &Phase::None {
                break;
            }
        }
    }
}

/// 移動先升、 南西
fn make_destination_to_south_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if SUJI_0 < sq_dst_ps_src.square.file - 1 && DAN_0 < sq_dst_ps_src.square.rank - 1 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file - 1, sq_dst_ps_src.square.rank - 1);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 南南西
fn make_destination_to_south_south_west<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if SUJI_0 < sq_dst_ps_src.square.file - 1 && DAN_0 < sq_dst_ps_src.square.rank - 2 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file - 1, sq_dst_ps_src.square.rank - 2);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い南
fn make_destination_sliding_to_south<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    for i_north in 1..9 {
        if DAN_0 < sq_dst_ps_src.square.file - i_north {
            let dst_sq = Square::from_file_rank(
                sq_dst_ps_src.square.file,
                sq_dst_ps_src.square.rank - i_north,
            );
            let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
            if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
                result.insert(dst_sq);
            }
            if &dst_phase != &Phase::None {
                break;
            }
        }
    }
}

/// 移動先升、 南
fn make_destination_to_south<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if DAN_0 < sq_dst_ps_src.square.rank - 1 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file, sq_dst_ps_src.square.rank - 1);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 南南東
fn make_destination_to_south_south_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.file + 1 < SUJI_10 && DAN_0 < sq_dst_ps_src.square.rank - 2 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file + 1, sq_dst_ps_src.square.rank - 2);
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
    }
}

/// 移動先升、 長い南東
fn make_destination_sliding_to_south_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    for i_nw in 1..9 {
        if sq_dst_ps_src.square.file + i_nw < SUJI_10 && DAN_0 < sq_dst_ps_src.square.rank - i_nw {
            let dst_sq = Square::from_file_rank(
                sq_dst_ps_src.square.file + i_nw,
                sq_dst_ps_src.square.rank - i_nw,
            );
            let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
            if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
                result.insert(dst_sq);
            }
            if &dst_phase != &Phase::None {
                break;
            }
        }
    }
}

/// 移動先升、 南東
fn make_destination_to_south_east<S: BuildHasher>(
    sq_dst_ps_src: &GPSquareAndPieceStructVo,
    current_position: &SPPositionDto,
    speed_of_light: &MLSpeedOfLightVo,
    result: &mut HashSet<Square, S>,
) {
    if sq_dst_ps_src.square.file + 1 < SUJI_10 && DAN_0 < sq_dst_ps_src.square.rank - 1 {
        let dst_sq =
            Square::from_file_rank(sq_dst_ps_src.square.file + 1, sq_dst_ps_src.square.rank - 1);
        // 自駒でなければ進める。
        let dst_phase = current_position.get_phase_by_sq(&dst_sq, speed_of_light);
        if &dst_phase != &sq_dst_ps_src.piece_struct.phase() {
            result.insert(dst_sq);
        }
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
                        make_no_promotion_source_by_phase_sliding_to_east(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 東
                        make_no_promotion_source_by_phase_to_east(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        make_no_promotion_source_by_phase_sliding_to_north_east(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 北東
                        make_no_promotion_source_by_phase_to_north_east(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                NNE => {
                    // 北北東
                    make_no_promotion_source_by_phase_to_north_north_east(
                        &dst_sq_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        make_no_promotion_source_by_phase_sliding_to_north(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 北
                        make_no_promotion_source_by_phase_to_north(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                NNW => {
                    // 北北西
                    make_no_promotion_source_by_phase_to_north_north_west(
                        &dst_sq_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        make_no_promotion_source_by_phase_sliding_to_north_west(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 北西
                        make_no_promotion_source_by_phase_to_north_west(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        make_no_promotion_source_by_phase_sliding_to_west(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 西
                        make_no_promotion_source_by_phase_to_west(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        make_no_promotion_source_by_phase_sliding_to_south_west(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 南西
                        make_no_promotion_source_by_phase_to_south_west(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                SSW => {
                    // 南南西
                    make_no_promotion_source_by_phase_to_south_south_west(
                        &dst_sq_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        make_no_promotion_source_by_phase_sliding_to_south(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 南
                        make_no_promotion_source_by_phase_to_south(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                SSE => {
                    // 南南東
                    make_no_promotion_source_by_phase_to_south_south_east(
                        &dst_sq_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        make_no_promotion_source_by_phase_sliding_to_south_east(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 南東
                        make_no_promotion_source_by_phase_to_south_east(
                            &dst_sq_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                Owari => break,
            }
        }
    }
}

// 移動元升、長い東
fn make_no_promotion_source_by_phase_sliding_to_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_east(dst_sq_piece.square, &mut |next_square| {
        let exists_piece = current_position.get_piece_by_square(&next_square);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(next_square);
        }
        // End of sliding.
        if *exists_piece != GPPieceVo::NonePiece {
            return true;
        }
        false
    });
}
// 移動元升、東
fn make_no_promotion_source_by_phase_to_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い北東
fn make_no_promotion_source_by_phase_sliding_to_north_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    for i_ne in 1..9 {
        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
// 移動元升、北東
fn make_no_promotion_source_by_phase_to_north_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、北北東
fn make_no_promotion_source_by_phase_to_north_north_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い北
fn make_no_promotion_source_by_phase_sliding_to_north<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    for i_south in 1..9 {
        if dy + i_south < DAN_10 {
            let sq_src = Square::from_file_rank(dx, dy + i_south);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
// 移動元升、北
fn make_no_promotion_source_by_phase_to_north<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx, dy + 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、北北西
fn make_no_promotion_source_by_phase_to_north_north_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い北西
fn make_no_promotion_source_by_phase_sliding_to_north_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    for i_se in 1..9 {
        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
// 移動元升、北西
fn make_no_promotion_source_by_phase_to_north_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い西
fn make_no_promotion_source_by_phase_sliding_to_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_west(dst_sq_piece.square, &mut |next_square| {
        let exists_piece = current_position.get_piece_by_square(&next_square);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(next_square);
        }
        // End of sliding.
        if *exists_piece != GPPieceVo::NonePiece {
            return true;
        }
        false
    });
}
// 移動元升、西
fn make_no_promotion_source_by_phase_to_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い南西
fn make_no_promotion_source_by_phase_sliding_to_south_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    for i_ne in 1..9 {
        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
// 移動元升、南西
fn make_no_promotion_source_by_phase_to_south_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、南南西
fn make_no_promotion_source_by_phase_to_south_south_west<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い南
fn make_no_promotion_source_by_phase_sliding_to_south<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    for i_north in 1..9 {
        if DAN_0 < dy - i_north {
            let sq_src = Square::from_file_rank(dx, dy - i_north);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
// 移動元升、南
fn make_no_promotion_source_by_phase_to_south<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx, dy - 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、南南東
fn make_no_promotion_source_by_phase_to_south_south_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
    }
}
// 移動元升、長い南東
fn make_no_promotion_source_by_phase_sliding_to_south_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    for i_nw in 1..9 {
        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
// 移動元升、南東
fn make_no_promotion_source_by_phase_to_south_east<F1>(
    dst_sq_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_piece.piece {
            gets_square(sq_src);
        }
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
                        make_before_promotion_source_by_phase_sliding_to_east(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 東
                        make_before_promotion_source_by_phase_to_east(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        make_before_promotion_source_by_phase_sliding_to_north_east(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 北東
                        make_before_promotion_source_by_phase_to_north_east(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                NNE => {
                    // 北北東
                    make_before_promotion_source_by_phase_to_north_north_east(
                        &dst_sq_and_demoted_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        make_before_promotion_source_by_phase_sliding_to_north(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 北
                        make_before_promotion_source_by_phase_to_north(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                NNW => {
                    // 北北西
                    make_before_promotion_source_by_phase_to_north_north_west(
                        &dst_sq_and_demoted_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        make_before_promotion_source_by_phase_sliding_to_north_west(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 北西
                        make_before_promotion_source_by_phase_to_north_west(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        make_before_promotion_source_by_phase_sliding_to_west(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 西
                        make_before_promotion_source_by_phase_to_west(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        make_before_promotion_source_by_phase_sliding_to_south_west(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 南西
                        make_before_promotion_source_by_phase_to_south_west(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                SSW => {
                    // 南南西
                    make_before_promotion_source_by_phase_to_south_south_west(
                        &dst_sq_and_demoted_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        make_before_promotion_source_by_phase_sliding_to_south(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 南
                        make_before_promotion_source_by_phase_to_south(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                SSE => {
                    // 南南東
                    make_before_promotion_source_by_phase_to_south_south_east(
                        &dst_sq_and_demoted_piece,
                        current_position,
                        &mut gets_square,
                    );
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        make_before_promotion_source_by_phase_sliding_to_south_east(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    } else {
                        // 南東
                        make_before_promotion_source_by_phase_to_south_east(
                            &dst_sq_and_demoted_piece,
                            current_position,
                            &mut gets_square,
                        );
                    }
                }
                Owari => break,
            }
        }
    }
}

/// 成る前移動元升、長い東
fn make_before_promotion_source_by_phase_sliding_to_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_east(dst_sq_and_demoted_piece.square, &mut |next_square| {
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
    });
}
/// 成る前移動元升、 東
fn make_before_promotion_source_by_phase_to_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い北東
fn make_before_promotion_source_by_phase_sliding_to_north_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    for i_ne in 1..9 {
        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_and_demoted_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
/// 成る前移動元升、 北東
fn make_before_promotion_source_by_phase_to_north_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 北北東
fn make_before_promotion_source_by_phase_to_north_north_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx + 1, dy + 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い北
fn make_before_promotion_source_by_phase_sliding_to_north<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    for i_south in 1..9 {
        if dy + i_south < DAN_10 {
            let sq_src = Square::from_file_rank(dx, dy + i_south);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_and_demoted_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
/// 成る前移動元升、 北
fn make_before_promotion_source_by_phase_to_north<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dy + 1 < DAN_10 {
        let sq_src = Square::from_file_rank(dx, dy + 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        // g_writeln(&format!("get_src_by_phase_ms 北 ms_src={} phase_ms=>{} piece_type_ms={} match_phase={} match_piece_type={}",
        //     ms_src, phase_ms, piece_typece_type_ms, match_phase( &phase_ms, &phase ), match_piece_type( piece_type_ms, *piece_type )
        // ));
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 北北西
fn make_before_promotion_source_by_phase_to_north_north_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い北西
fn make_before_promotion_source_by_phase_sliding_to_north_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    for i_se in 1..9 {
        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_and_demoted_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
/// 成る前移動元升、 北西
fn make_before_promotion_source_by_phase_to_north_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い西
fn make_before_promotion_source_by_phase_sliding_to_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    SquareScanner::for_each_west(dst_sq_and_demoted_piece.square, &mut |next_square| {
        let exists_piece = current_position.get_piece_by_square(&next_square);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(next_square);
        }
        // End of sliding.
        if *exists_piece != GPPieceVo::NonePiece {
            return true;
        }
        false
    });
}
/// 成る前移動元升、 西
fn make_before_promotion_source_by_phase_to_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い南西
fn make_before_promotion_source_by_phase_sliding_to_south_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    for i_ne in 1..9 {
        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_and_demoted_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
/// 成る前移動元升、 南西
fn make_before_promotion_source_by_phase_to_south_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 南南西
fn make_before_promotion_source_by_phase_to_south_south_west<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx - 1, dy - 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い南
fn make_before_promotion_source_by_phase_sliding_to_south<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    for i_north in 1..9 {
        if DAN_0 < dy - i_north {
            let sq_src = Square::from_file_rank(dx, dy - i_north);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_and_demoted_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
/// 成る前移動元升、 南
fn make_before_promotion_source_by_phase_to_south<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx, dy - 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        // g_writeln(&format!("get_src_by_phase_ms 南 piece_type={} piece_type_num={} ms_src={} phase_ms=>{} piece_type_ms={} match_phase={} match_piece_type={}",
        //     piece_type, piece_type_num, ms_src, phase_ms, piece_type_ms, match_phase( &phase_ms, &phase ), match_piece_type( piece_type_ms, *piece_type )
        // ));
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 南南東
fn make_before_promotion_source_by_phase_to_south_south_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 2);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
    }
}
/// 成る前移動元升、 長い南東
fn make_before_promotion_source_by_phase_sliding_to_south_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    for i_nw in 1..9 {
        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
            let exists_piece = current_position.get_piece_by_square(&sq_src);
            if *exists_piece == dst_sq_and_demoted_piece.piece {
                gets_square(sq_src);
            }
            // End of sliding.
            if *exists_piece != GPPieceVo::NonePiece {
                break;
            }
        }
    }
}
/// 成る前移動元升、 南東
fn make_before_promotion_source_by_phase_to_south_east<F1>(
    dst_sq_and_demoted_piece: &GPSquareAndPieceVo,
    current_position: &SPPositionDto,
    gets_square: &mut F1,
) where
    F1: FnMut(Square),
{
    let (dx, dy) = dst_sq_and_demoted_piece.square.to_file_rank();
    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
        let exists_piece = current_position.get_piece_by_square(&sq_src);
        if *exists_piece == dst_sq_and_demoted_piece.piece {
            gets_square(sq_src);
        }
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
