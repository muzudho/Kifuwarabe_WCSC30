//!
//! 指し手の要素☆（＾～＾）
//!

use super::super::super::controller::common::conv::*;
use super::super::super::controller::consoles::asserts::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece_movement::*;
use super::super::super::model::master::piece_type::*;
use super::super::super::model::master::square::*;
use super::super::super::model::search::search_part::*;
use super::super::super::model::vo::phase::Phase;
use super::super::super::model::vo::phase::*;
use super::super::super::model::vo::piece_direction::*;
use super::super::super::model::vo::piece_vo::PieceVo;
use super::super::super::model::vo::speed_of_light::*;
use std::collections::HashSet;

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
    sq_dst: &Square,
    ps_dst: &PieceVo,
    search_part: &SearchPart,
    speed_of_light: &SpeedOfLight,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&sq_dst, "make_no_promotion_source_by_square_and_piece");

    /*
     * Square は 将棋盤座標
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     *
     * x,y を使うと混乱するので、s,d を使う
     */
    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx, dy) = sq_dst.to_file_rank();

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use super::super::super::model::master::piece::Piece::*;
    match ps_dst.piece() {
        Knight1 => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {
                return;
            }
        }
        Lance1 | Pawn1 => {
            // ▼しし、▼ひよこ　は１段目には進めない
            if dy < DAN_2 {
                return;
            }
        }
        Knight2 => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {
                return;
            }
        }
        Lance2 | Pawn2 => {
            // △しし、△ひよこ　は９段目には進めない
            if DAN_8 < dy {
                return;
            }
        }
        _ => {}
    }

    let kms_num = kms_to_num(&ps_dst.piece_type());

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &PieceDirection;
        if match_sn(&Phase::Sen, &ps_dst.phase()) {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use super::super::super::model::vo::piece_direction::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    for i_east in 1..9 {
                        if dx + i_east < SUJI_10 {
                            let sq_src = Square::from_file_rank(dx + i_east, dy);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                // TODO ポインター渡しできないもんか……☆（＾～＾）あるいはハッシュ☆（＾～＾）
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 西東
                    if dx + 1 < SUJI_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    for i_ne in 1..9 {
                        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 北東
                    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 北北東
            NNE => {
                if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                    let sq_src = Square::from_file_rank(dx + 1, dy + 2);
                    if search_part.get_current_position().has_sq_km(
                        &sq_src,
                        ps_dst.piece(),
                        speed_of_light,
                    ) {
                        gets_square(sq_src);
                    }
                }
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    for i_south in 1..9 {
                        if dy + i_south < DAN_10 {
                            let sq_src = Square::from_file_rank(dx, dy + i_south);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 北
                    if dy + 1 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx, dy + 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 北北西
            NNW => {
                if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                    let sq_src = Square::from_file_rank(dx - 1, dy + 2);
                    if search_part.get_current_position().has_sq_km(
                        &sq_src,
                        ps_dst.piece(),
                        speed_of_light,
                    ) {
                        gets_square(sq_src);
                    }
                }
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    for i_se in 1..9 {
                        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 北西
                    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    for i_east in 1..9 {
                        if SUJI_0 < dx - i_east {
                            // 進みたいマスから戻ったマス
                            let sq_src = Square::from_file_rank(dx - i_east, dy);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                // 指定の駒があれば、その升は移動元。続行
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                // なんか他の駒があれば終わり
                                break;
                            }
                        }
                    }
                } else {
                    // 西
                    if SUJI_0 < dx - 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    for i_ne in 1..9 {
                        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 南西
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 南南西
            SSW => {
                if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                    let sq_src = Square::from_file_rank(dx - 1, dy - 2);
                    if search_part.get_current_position().has_sq_km(
                        &sq_src,
                        ps_dst.piece(),
                        speed_of_light,
                    ) {
                        gets_square(sq_src);
                    }
                }
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    for i_north in 1..9 {
                        if DAN_0 < dy - i_north {
                            let sq_src = Square::from_file_rank(dx, dy - i_north);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 南
                    if DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx, dy - 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 南南東
            SSE => {
                if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                    let sq_src = Square::from_file_rank(dx + 1, dy - 2);
                    if search_part.get_current_position().has_sq_km(
                        &sq_src,
                        ps_dst.piece(),
                        speed_of_light,
                    ) {
                        gets_square(sq_src);
                    }
                }
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    for i_nw in 1..9 {
                        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                ps_dst.piece(),
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 南東
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            ps_dst.piece(),
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            Owari => break,
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
    sq_dst: &Square,
    ps_dst: &PieceVo,
    search_part: &SearchPart,
    speed_of_light: &SpeedOfLight,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&sq_dst, "make_before_promotion_source_by_square_piece");

    // +--------------------+
    // | 移動後は成り駒か？ |
    // +--------------------+
    if !ps_dst.is_promoted() {
        return; // 成り駒でないなら、成りの動きをしていない
    }

    // +--------------------+
    // | 移動前は成る前の駒 |
    // +--------------------+
    // 前提として、成った駒であることは分かっているとするぜ☆（＾～＾）
    let kms_src = speed_of_light
        .piece_vo_master
        .get_piece_vo(ps_dst.demote())
        .piece_type();
    let km_src = speed_of_light
        .piece_vo_master
        .get_piece_vo_by_phase_and_piece_type(&ps_dst.phase(), &kms_src)
        .piece();

    /*
     * Square は 将棋盤座標
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     *
     * x,y を使うと混乱するので、s,d を使う
     */
    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx, dy) = sq_dst.to_file_rank();

    // 例えば移動先の駒種類が「ぱひ」なら、「ぱひ」が動いた可能性の他に、
    // 「ひ」が動いたのかもしれない。
    // 「ぱひ」は、敵陣の１～３段目にいて、動きが北だった場合、元が「ひ」の可能性がある。
    //
    // 成る前に戻れない駒は、成ったかどうかを考えなくていいぜ☆（＾～＾）
    if !ps_dst.can_demote() {
        return;
    }

    let kms_narumae_num = kms_to_num(
        &speed_of_light
            .piece_vo_master
            .get_piece_vo(ps_dst.demote())
            .piece_type(),
    );

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &PieceDirection;
        if match_sn(&Phase::Sen, &ps_dst.phase()) {
            p_kmdir = &KM_UGOKI.back[kms_narumae_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_narumae_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use super::super::super::model::vo::piece_direction::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    for i_east in 1..9 {
                        if dx + i_east < SUJI_10 {
                            let sq_src = Square::from_file_rank(dx + i_east, dy);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 西東
                    if dx + 1 < SUJI_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    for i_ne in 1..9 {
                        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 北東
                    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 北北東
            NNE => {
                if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                    let sq_src = Square::from_file_rank(dx + 1, dy + 2);
                    if search_part
                        .get_current_position()
                        .has_sq_km(&sq_src, km_src, speed_of_light)
                    {
                        gets_square(sq_src);
                    }
                }
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    for i_south in 1..9 {
                        if dy + i_south < DAN_10 {
                            let sq_src = Square::from_file_rank(dx, dy + i_south);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 北
                    if dy + 1 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx, dy + 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 北北西
            NNW => {
                if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                    let sq_src = Square::from_file_rank(dx - 1, dy + 2);
                    if search_part
                        .get_current_position()
                        .has_sq_km(&sq_src, km_src, speed_of_light)
                    {
                        gets_square(sq_src);
                    }
                }
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    for i_se in 1..9 {
                        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 北西
                    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    for i_east in 1..9 {
                        if SUJI_0 < dx - i_east {
                            // 進みたいマスから戻ったマス
                            let sq_src = Square::from_file_rank(dx - i_east, dy);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                // 指定の駒があれば、その升は移動元。続行
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                // なんか他の駒があれば終わり
                                break;
                            }
                        }
                    }
                } else {
                    // 西
                    if SUJI_0 < dx - 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    for i_ne in 1..9 {
                        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 南西
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 南南西
            SSW => {
                if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                    let sq_src = Square::from_file_rank(dx - 1, dy - 2);
                    if search_part
                        .get_current_position()
                        .has_sq_km(&sq_src, km_src, speed_of_light)
                    {
                        gets_square(sq_src);
                    }
                }
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    for i_north in 1..9 {
                        if DAN_0 < dy - i_north {
                            let sq_src = Square::from_file_rank(dx, dy - i_north);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 南
                    if DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx, dy - 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            // 南南東
            SSE => {
                if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                    let sq_src = Square::from_file_rank(dx + 1, dy - 2);
                    if search_part
                        .get_current_position()
                        .has_sq_km(&sq_src, km_src, speed_of_light)
                    {
                        gets_square(sq_src);
                    }
                }
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    for i_nw in 1..9 {
                        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
                            if search_part.get_current_position().has_sq_km(
                                &sq_src,
                                km_src,
                                speed_of_light,
                            ) {
                                gets_square(sq_src);
                            } else if search_part
                                .get_current_position()
                                .exists_km(&sq_src, speed_of_light)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    // 南東
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
                        if search_part.get_current_position().has_sq_km(
                            &sq_src,
                            km_src,
                            speed_of_light,
                        ) {
                            gets_square(sq_src);
                        }
                    }
                }
            }
            Owari => break,
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
/// 2. 移動先の駒    piece_dst  ※先後が要るので、kmsではなくkm。
///
/// そこに打てる駒種類を返す。
pub fn make_drop_piece_type_by_square_piece<F1>(
    sq_dst: &Square,
    piece_dst: &Piece,
    search_part: &SearchPart,
    speed_of_light: &SpeedOfLight,
    mut gets_piece_type_hash: F1,
) where
    F1: FnMut(usize),
{
    assert_banjo_sq(&sq_dst, "make_drop_piece_type_by_square_piece");

    let ps_dst = speed_of_light.piece_vo_master.get_piece_vo(piece_dst);
    let kms_dst = ps_dst.piece_type();
    if !kms_can_da(&kms_dst) {
        return; // 打って出てくることがない駒なら終了
    }

    // +------------------------+
    // | 打ちたいところは空升か |
    // +------------------------+
    let km_banjo = search_part
        .get_current_position()
        .get_piece_by_square(sq_dst);
    match km_banjo {
        Piece::Kara => {}
        _ => {
            return;
        } // 駒があるところに打つ手は終了
    }
    // 駒が無いところに打つ

    // +------------------+
    // | 持っている駒か？ |
    // +------------------+
    if search_part
        .get_current_position()
        .get_hand(piece_dst, speed_of_light)
        < 1
    {
        return; // 持っていない駒は打てない
    }

    // 回転していない将棋盤から見た筋番号
    let (suji, dy) = sq_dst.to_file_rank();
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
    let sq = kaiten180_sq_by_sq_sn(&sq_dst, &ps_dst.phase());

    assert_banjo_sq(&sq, "Ｉnsert_da_kms_by_ms_km＜その２＞");
    //let (_x,y) = ms_to_suji_dan(ms);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use super::super::super::model::master::piece::Piece::*;
    match piece_dst.clone() {
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
                || search_part.get_current_position().exists_fu_by_sn_suji(
                    &ps_dst.phase(),
                    suji,
                    speed_of_light,
                )
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
                || search_part.get_current_position().exists_fu_by_sn_suji(
                    &ps_dst.phase(),
                    suji,
                    speed_of_light,
                )
            {
                return;
            }
        }
        _ => {}
    }

    gets_piece_type_hash(kms_to_num(&kms_dst));
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
/// search_part       : 探索部
pub fn make_destination_by_square_piece(
    sq_src: &Square,
    km_src: &Piece,
    to_nari: bool,
    search_part: &SearchPart,
    speed_of_light: &SpeedOfLight,
    // result, result2 で入れ直しがあるのでむずかしい☆（＾～＾）
    // 成れない動きをあとで除外する☆（＾～＾）
    result: &mut HashSet<Square>,
) {
    assert_banjo_sq(&sq_src, "make_destination_by_square_piece");

    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx, dy) = sq_src.to_file_rank();
    let ps_src = speed_of_light.piece_vo_master.get_piece_vo(km_src);
    let kms_src = ps_src.piece_type();

    // +--------------+
    // | 成れる駒か？ |
    // +--------------+
    if to_nari && !kms_can_pro(&kms_src) {
        return; // 成れる駒でないなら、成りの動きはしない
    }
    let kms_num = kms_to_num(&kms_src);

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &PieceDirection;
        let ps_src = &ps_src;
        if match_sn(&Phase::Sen, &ps_src.phase()) {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
            p_kmdir = &_kmdir;
        } else {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        };

        // 駒の位置を開始地点に、離れていくように調べていく
        use super::super::super::model::vo::piece_direction::PieceDirection::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    for i_east in 1..9 {
                        if dx + i_east < SUJI_10 {
                            let sq_src = Square::from_file_rank(dx + i_east, dy);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 西東
                    if dx + 1 < SUJI_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    for i_ne in 1..9 {
                        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                            let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北東
                    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy + 1);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 北北東
            NNE => {
                if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                    let sq_src = Square::from_file_rank(dx + 1, dy + 2);
                    let sn_ms = search_part
                        .get_current_position()
                        .get_sn_by_sq(&sq_src, speed_of_light);
                    if !match_sn(&sn_ms, &ps_src.phase()) {
                        result.insert(sq_src);
                    }
                }
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    for i_south in 1..9 {
                        if dy + i_south < DAN_10 {
                            let sq_src = Square::from_file_rank(dx, dy + i_south);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北
                    if dy + 1 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx, dy + 1);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 北北西
            NNW => {
                if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                    let sq_src = Square::from_file_rank(dx - 1, dy + 2);
                    let sn_ms = search_part
                        .get_current_position()
                        .get_sn_by_sq(&sq_src, speed_of_light);
                    if !match_sn(&sn_ms, &ps_src.phase()) {
                        result.insert(sq_src);
                    }
                }
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    for i_se in 1..9 {
                        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                            let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北西
                    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy + 1);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    for i_east in 1..9 {
                        if SUJI_0 < dx - i_east {
                            let sq_src = Square::from_file_rank(dx - i_east, dy);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 西
                    if SUJI_0 < dx - 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    for i_ne in 1..9 {
                        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                            let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南西
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx - 1, dy - 1);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 南南西
            SSW => {
                if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                    let sq_src = Square::from_file_rank(dx - 1, dy - 2);
                    let sn_ms = search_part
                        .get_current_position()
                        .get_sn_by_sq(&sq_src, speed_of_light);
                    if !match_sn(&sn_ms, &ps_src.phase()) {
                        result.insert(sq_src);
                    }
                }
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    for i_north in 1..9 {
                        if DAN_0 < dy - i_north {
                            let sq_src = Square::from_file_rank(dx, dy - i_north);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南
                    if DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx, dy - 1);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            // 南南東
            SSE => {
                if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                    let sq_src = Square::from_file_rank(dx + 1, dy - 2);
                    let sn_ms = search_part
                        .get_current_position()
                        .get_sn_by_sq(&sq_src, speed_of_light);
                    if !match_sn(&sn_ms, &ps_src.phase()) {
                        result.insert(sq_src);
                    }
                }
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    for i_nw in 1..9 {
                        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                            let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            if !match_sn(&sn_ms, &ps_src.phase()) {
                                result.insert(sq_src);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南東
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                        let sq_src = Square::from_file_rank(dx + 1, dy - 1);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        if !match_sn(&sn_ms, &ps_src.phase()) {
                            result.insert(sq_src);
                        }
                    }
                }
            }
            Owari => break,
        }
    }

    if to_nari {
        // +------------------------------+
        // | 成れる動き以外での成りの禁止 |
        // +------------------------------+
        use super::super::super::model::master::piece::Piece::*;
        match km_src.clone() {
            Rook1 | Bishop1 | Silver1 => {
                // ▼きりん、▼ぞう、▼ねこ　は
                // 移動元または移動先が　１～３段目なら成れる
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if sq_src.get_rank() < DAN_4 && sq_dst.get_rank() < DAN_4 {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for ms_dst in result2.iter() {
                    result.insert(ms_dst.clone());
                }
            }
            Knight1 | Lance1 | Pawn1 => {
                // ▼うさぎ、▼しし、▼ひよこ　は
                // 移動先が　１～３段目なら成れる
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if sq_dst.get_rank() < DAN_4 {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            Rook2 | Bishop2 | Silver2 => {
                // △きりん、△ぞう、△ねこ　は
                // 移動元または移動先が　７～９段目なら成れる
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if DAN_6 < sq_src.get_rank() && DAN_6 < sq_dst.get_rank() {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            Knight2 | Lance2 | Pawn2 => {
                // △うさぎ、△しし、△ひよこ　は
                // 移動先が　７～９段目なら成れる
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if DAN_6 < sq_dst.get_rank() {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            _ => {}
        }
    } else {
        // +----------------------------------------+
        // | 行先の無いところに駒を進めることの禁止 |
        // +----------------------------------------+
        use super::super::super::model::master::piece::Piece::*;
        match km_src {
            Knight1 => {
                // ▼うさぎ　は１、２段目には進めない
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if sq_dst.get_rank() < DAN_3 {
                    } else {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            Lance1 | Pawn1 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if sq_dst.get_rank() < DAN_2 {
                    } else {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            Knight2 => {
                // △うさぎ　は８、９段目には進めない
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if DAN_7 < sq_dst.get_rank() {
                    } else {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            Lance2 | Pawn2 => {
                // △しし、△ひよこ　は９段目には進めない
                let mut result2: HashSet<Square> = HashSet::<Square>::new();
                for sq_dst in result.iter() {
                    if DAN_8 < sq_dst.get_rank() {
                    } else {
                        result2.insert(sq_dst.clone());
                    }
                }
                // 入れ直し
                result.clear();
                for sq_dst in result2.iter() {
                    result.insert(sq_dst.clone());
                }
            }
            _ => {}
        }
    }
}

/// 移動元升生成
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 手番の先後と、移動先升　を指定することで　指し手を生成するぜ☆（＾～＾）
///
/// 1. 手番の先後    sn
/// 2. 移動先升      ms_dst
///
/// その升に到達できる駒が居る升を取得☆（＾～＾）
/// TODO 成りの動きも考えたい。升だけではなく、成りの有無☆（＾～＾）
pub fn make_no_promotion_source_by_phase_square<F1>(
    sn: &Phase,
    sq_dst: &Square,
    search_part: &SearchPart,
    speed_of_light: &SpeedOfLight,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&sq_dst, "make_no_promotion_source_by_phase_square");

    // 移動先の筋、段
    let (dx, dy) = sq_dst.to_file_rank();

    // 駒種類
    for kms in KMS_ARRAY.iter() {
        // 行先の無いところに駒を進めることの禁止☆（＾～＾）
        let km = speed_of_light
            .piece_vo_master
            .get_piece_vo_by_phase_and_piece_type(&sn, &kms)
            .piece()
            .clone();
        use super::super::super::model::master::piece::Piece::*;
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

        let kms_num = kms_to_num(&kms);
        for i_dir in 0..KM_UGOKI_LN {
            // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir: &PieceDirection;
            if match_sn(&Phase::Sen, &sn) {
                p_kmdir = &KM_UGOKI.back[kms_num][i_dir];
            // g_writeln(&format!("get_src_by_sn_ms 先手なら kms={} kms_num={} p_kmdir={}",
            //     kms, kms_num, p_kmdir
            // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
                p_kmdir = &_kmdir;
                // g_writeln(&format!("get_src_by_sn_ms 後手なら kms={} kms_num={} p_kmdir={}",
                //     kms, kms_num, p_kmdir
                // ));
            }

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use super::super::super::model::vo::piece_direction::PieceDirection::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        for i_east in 1..9 {
                            if dx + i_east < SUJI_10 {
                                let sq_src = Square::from_file_rank(dx + i_east, dy);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 東
                        if dx + 1 < SUJI_10 {
                            let sq_src = Square::from_file_rank(dx + 1, dy);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        for i_ne in 1..9 {
                            if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                                let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北東
                        if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                            let sq_src = Square::from_file_rank(dx + 1, dy + 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 北北東
                NNE => {
                    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy + 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        for i_south in 1..9 {
                            if dy + i_south < DAN_10 {
                                let sq_src = Square::from_file_rank(dx, dy + i_south);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北
                        if dy + 1 < DAN_10 {
                            let sq_src = Square::from_file_rank(dx, dy + 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            // g_writeln(&format!("get_src_by_sn_ms 北 ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                            //     ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                            // ));
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 北北西
                NNW => {
                    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx - 1, dy + 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        for i_se in 1..9 {
                            if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                                let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北西
                        if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                            let sq_src = Square::from_file_rank(dx - 1, dy + 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        for i_east in 1..9 {
                            if SUJI_0 < dx - i_east {
                                let sq_src = Square::from_file_rank(dx - i_east, dy);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 西
                        if SUJI_0 < dx - 1 {
                            let sq_src = Square::from_file_rank(dx - 1, dy);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        for i_ne in 1..9 {
                            if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                                let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南西
                        if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                            let sq_src = Square::from_file_rank(dx - 1, dy - 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 南南西
                SSW => {
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                        let sq_src = Square::from_file_rank(dx - 1, dy - 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        for i_north in 1..9 {
                            if DAN_0 < dy - i_north {
                                let sq_src = Square::from_file_rank(dx, dy - i_north);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南
                        if DAN_0 < dy - 1 {
                            let sq_src = Square::from_file_rank(dx, dy - 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            // g_writeln(&format!("get_src_by_sn_ms 南 kms={} kms_num={} ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                            //     kms, kms_num, ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                            // ));
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 南南東
                SSE => {
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                        let sq_src = Square::from_file_rank(dx + 1, dy - 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        for i_nw in 1..9 {
                            if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                                let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南東
                        if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                            let sq_src = Square::from_file_rank(dx + 1, dy - 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                Owari => break,
            }
        }
    }
}

/// 移動元升生成（成る前）
///
/// 他のコンピューター将棋ソフトでは、現局面から指せる指し手を生成する現実指向だが、
/// きふわらべ　は理想指向☆（＾～＾）
/// 手番の先後と、移動先升　を指定することで　指し手を生成するぜ☆（＾～＾）
pub fn make_before_promotion_source_by_phase_square<F1>(
    sn: &Phase,
    sq_dst: &Square,
    search_part: &SearchPart,
    speed_of_light: &SpeedOfLight,
    mut gets_square: F1,
) where
    F1: FnMut(Square),
{
    assert_banjo_sq(&sq_dst, "make_before_promotion_source_by_phase_square");

    // 移動先の筋、段
    let (dx, dy) = sq_dst.to_file_rank();

    // 駒種類
    for kms in KMS_ARRAY.iter() {
        let km_src = speed_of_light
            .piece_vo_master
            .get_piece_vo_by_phase_and_piece_type(&sn, &kms)
            .piece();

        // +--------------------+
        // | 移動前は非成駒か？ |
        // +--------------------+
        let ps_src = speed_of_light.piece_vo_master.get_piece_vo(km_src);
        if ps_src.is_promoted() {
            continue; // 成る前に成駒なら、成りの動きをしていない
        }

        let prokm_src = ps_src.promote();
        match prokm_src {
            Piece::Kara => {
                continue;
            } // 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
            _ => {} // 成れる駒は、成る前の駒の動きも調べる
        }

        // 成り駒に、行先の無いところは無いぜ☆

        let kms_num = kms_to_num(&kms);
        for i_dir in 0..KM_UGOKI_LN {
            // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir: &PieceDirection;
            if match_sn(&Phase::Sen, &sn) {
                p_kmdir = &KM_UGOKI.back[kms_num][i_dir];
            // g_writeln(&format!("get_src_by_sn_ms 先手なら kms={} kms_num={} p_kmdir={}",
            //     kms, kms_num, p_kmdir
            // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
                p_kmdir = &_kmdir;
                // g_writeln(&format!("get_src_by_sn_ms 後手なら kms={} kms_num={} p_kmdir={}",
                //     kms, kms_num, p_kmdir
                // ));
            }

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use super::super::super::model::vo::piece_direction::PieceDirection::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        for i_east in 1..9 {
                            if dx + i_east < SUJI_10 {
                                let sq_src = Square::from_file_rank(dx + i_east, dy);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 東
                        if dx + 1 < SUJI_10 {
                            let sq_src = Square::from_file_rank(dx + 1, dy);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        for i_ne in 1..9 {
                            if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                                let sq_src = Square::from_file_rank(dx + i_ne, dy + i_ne);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北東
                        if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                            let sq_src = Square::from_file_rank(dx + 1, dy + 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 北北東
                NNE => {
                    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx + 1, dy + 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        for i_south in 1..9 {
                            if dy + i_south < DAN_10 {
                                let sq_src = Square::from_file_rank(dx, dy + i_south);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北
                        if dy + 1 < DAN_10 {
                            let sq_src = Square::from_file_rank(dx, dy + 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            // g_writeln(&format!("get_src_by_sn_ms 北 ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                            //     ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                            // ));
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 北北西
                NNW => {
                    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                        let sq_src = Square::from_file_rank(dx - 1, dy + 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        for i_se in 1..9 {
                            if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                                let sq_src = Square::from_file_rank(dx - i_se, dy + i_se);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北西
                        if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                            let sq_src = Square::from_file_rank(dx - 1, dy + 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        for i_east in 1..9 {
                            if SUJI_0 < dx - i_east {
                                let sq_src = Square::from_file_rank(dx - i_east, dy);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 西
                        if SUJI_0 < dx - 1 {
                            let sq_src = Square::from_file_rank(dx - 1, dy);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        for i_ne in 1..9 {
                            if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                                let sq_src = Square::from_file_rank(dx - i_ne, dy - i_ne);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南西
                        if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                            let sq_src = Square::from_file_rank(dx - 1, dy - 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 南南西
                SSW => {
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                        let sq_src = Square::from_file_rank(dx - 1, dy - 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        for i_north in 1..9 {
                            if DAN_0 < dy - i_north {
                                let sq_src = Square::from_file_rank(dx, dy - i_north);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南
                        if DAN_0 < dy - 1 {
                            let sq_src = Square::from_file_rank(dx, dy - 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            // g_writeln(&format!("get_src_by_sn_ms 南 kms={} kms_num={} ms_src={} sn_ms=>{} kms_ms={} match_sn={} match_kms={}",
                            //     kms, kms_num, ms_src, sn_ms, kms_ms, match_sn( &sn_ms, &sn ), match_kms( &kms_ms, &kms )
                            // ));
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                // 南南東
                SSE => {
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                        let sq_src = Square::from_file_rank(dx + 1, dy - 2);
                        let sn_ms = search_part
                            .get_current_position()
                            .get_sn_by_sq(&sq_src, speed_of_light);
                        let kms_ms = speed_of_light
                            .piece_vo_master
                            .get_piece_vo(
                                search_part
                                    .get_current_position()
                                    .get_piece_by_square(&sq_src),
                            )
                            .piece_type();
                        if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                            gets_square(sq_src);
                        }
                    }
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        for i_nw in 1..9 {
                            if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                                let sq_src = Square::from_file_rank(dx + i_nw, dy - i_nw);
                                let sn_ms = search_part
                                    .get_current_position()
                                    .get_sn_by_sq(&sq_src, speed_of_light);
                                let kms_ms = speed_of_light
                                    .piece_vo_master
                                    .get_piece_vo(
                                        search_part
                                            .get_current_position()
                                            .get_piece_by_square(&sq_src),
                                    )
                                    .piece_type();
                                if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                    gets_square(sq_src);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南東
                        if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                            let sq_src = Square::from_file_rank(dx + 1, dy - 1);
                            let sn_ms = search_part
                                .get_current_position()
                                .get_sn_by_sq(&sq_src, speed_of_light);
                            let kms_ms = speed_of_light
                                .piece_vo_master
                                .get_piece_vo(
                                    search_part
                                        .get_current_position()
                                        .get_piece_by_square(&sq_src),
                                )
                                .piece_type();
                            if match_sn(&sn_ms, &sn) && match_kms(&kms_ms, &kms) {
                                gets_square(sq_src);
                            }
                        }
                    }
                }
                Owari => break,
            }
        }
    }
}

/*
 * 合い駒スペースを算出
 *
 * sn_atk  : 攻めている方の先後
 * ms_atk  : 攻め駒の居る升
 * ms_tgt  : 狙われている駒の居る升
 * kms_atk : 攻め駒の駒種類
 */
/*
#[allow(dead_code)]
pub fn get_ms_vec_as_aigoma(
    sn_atk:&Phase,
    ms_atk:&Square,
    ms_tgt:&Square,
    kms_atk:&PieceType
    )->Vec<Square> {
    let vec = Vec::new();

    use teigi::shogi_syugo::PieceType::*;
    match *kms_atk {
        K => {
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
            if match_sn(&Phase::Sen, &sn_atk) {
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
