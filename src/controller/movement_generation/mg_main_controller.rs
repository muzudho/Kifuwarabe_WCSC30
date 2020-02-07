//!
//! 現局面を使った指し手生成
//!

use super::super::super::controller::common_part::cp_asserts_controller::*;
use super::super::super::controller::common_part::cp_conv_controller::*;
use super::super::super::controller::movement_generation::mg_sub_part_controller::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::dto::search_part::sp_main_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::model::vo::other_part::op_piece_type_vo::PieceType;
use super::super::super::model::vo::other_part::op_piece_type_vo::*;
use super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::model::vo::other_part::op_square_vo::*;
use std::collections::HashSet;

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
pub fn get_potential_movement<F1>(
    search_part: &SPMainDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement_callback: F1,
) where
    F1: FnMut(u64),
{
    // +----------------+
    // | 盤上の駒の移動 |
    // +----------------+
    for dan_src in 1..10 {
        for suji_src in 1..10 {
            let sq_src = Square::from_file_rank(suji_src, dan_src);
            let piece_src = search_part
                .get_current_position()
                .get_piece_by_square(&sq_src);

            if match_sn(
                &speed_of_light
                    .ml_piece_struct_master_vo
                    .get_piece_vo(&piece_src)
                    .phase(),
                &search_part.get_phase(&Person::Ji),
            ) {
                // 手番の駒

                let mut dst_hashset: HashSet<Square> = HashSet::<Square>::new();
                make_destination_by_square_piece(
                    &sq_src,
                    piece_src,
                    false, // 成らず
                    &search_part,
                    &speed_of_light,
                    &mut dst_hashset,
                );

                // g_writeln("テスト ポテンシャルムーブ insert_dst_by_sq_km(成らず).");
                // use consoles::visuals::dumps::*;
                // hyoji_sq_hashset( &dst_hashset );

                for sq_dst in &dst_hashset {
                    gets_movement_callback(
                        MLMovementDto {
                            src: sq_src.clone(),
                            dst: sq_dst.clone(),
                            pro: false, // 成らず
                            drop: PieceType::Kara,
                        }
                        .to_hash(),
                    );
                }

                dst_hashset.clear();
                make_destination_by_square_piece(
                    &sq_src,
                    piece_src,
                    true, // 成り
                    &search_part,
                    &speed_of_light,
                    &mut dst_hashset,
                );
                for sq_dst in &dst_hashset {
                    gets_movement_callback(
                        MLMovementDto {
                            src: sq_src.clone(),
                            dst: sq_dst.clone(),
                            pro: true, // 成り
                            drop: PieceType::Kara,
                        }
                        .to_hash(),
                    );
                }
            }
        }
    }

    // +----+
    // | 打 |
    // +----+
    for dan_dst in 1..10 {
        for suji_dst in 1..10 {
            let sq_dst = Square::from_file_rank(suji_dst, dan_dst);
            let piece_dst = search_part
                .get_current_position()
                .get_piece_by_square(&sq_dst);
            match piece_dst {
                OPPieceVo::Kara => {
                    // 駒が無いところに打つ

                    let mut da_kms_hashset = HashSet::new();
                    for kms_motigoma in MGS_ARRAY.iter() {
                        let ps_motigoma = speed_of_light
                            .ml_piece_struct_master_vo
                            .get_piece_vo_by_phase_and_piece_type(
                                &search_part.get_phase(&Person::Ji),
                                kms_motigoma,
                            );
                        let pc_motigoma = ps_motigoma.piece();
                        if 0 < search_part
                            .get_current_position()
                            .get_hand(pc_motigoma, speed_of_light)
                        {
                            // 駒を持っていれば
                            make_drop_piece_type_by_square_piece(
                                &sq_dst,
                                pc_motigoma,
                                &search_part,
                                &speed_of_light,
                                |piece_type_hash| {
                                    da_kms_hashset.insert(piece_type_hash);
                                },
                            );
                        }
                    }
                    for num_kms_da in da_kms_hashset {
                        let kms = num_to_kms(num_kms_da);
                        gets_movement_callback(
                            MLMovementDto {
                                src: Square::from_umasu(SS_SRC_DA), // 駒大
                                dst: sq_dst.clone(),                // どの升へ行きたいか
                                pro: false,                         // 打に成りは無し
                                drop: kms,                          // 打った駒種類
                            }
                            .to_hash(),
                        );
                    }
                }
                _ => {}
            }
        } //suji
    } //dan
}

/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  piece_dst
///
/// 盤上の駒の移動の最初の１つ。打を除く
pub fn get_movement_by_square_and_piece_on_board<F1>(
    sq_dst: &Square,
    piece_dst: OPPieceVo,
    search_part: &SPMainDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_banjo_sq(&sq_dst, "Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let ps_dst = speed_of_light
        .ml_piece_struct_master_vo
        .get_piece_vo(&piece_dst);
    let (sn, _kms_dst) = ps_dst.phase_piece_type();

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(
        &search_part
            .get_current_position()
            .get_sn_by_sq(&sq_dst, speed_of_light),
        &sn,
    ) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = MLMovementDto::new();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    let mut mv_src_hashset: HashSet<Square> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    make_no_promotion_source_by_square_and_piece(
        &sq_dst,
        &ps_dst,
        &search_part,
        &speed_of_light,
        |square| {
            mv_src_hashset.insert(square);
        },
    );
    for sq_src in &mv_src_hashset {
        assert_banjo_sq(
            &sq_src,
            "make_no_promotion_source_by_square_and_piece(成らず)",
        );

        ss_hash_builder.src = sq_src.clone();
        // 成らず
        ss_hash_builder.pro = false;
        ss_hash_builder.drop = PieceType::Kara;
        gets_movement(ss_hash_builder.to_hash());
    }

    // +--------------+
    // | 盤上（成り） |
    // +--------------+
    mv_src_hashset.clear();
    make_before_promotion_source_by_square_piece(
        sq_dst,
        &ps_dst,
        &search_part,
        &speed_of_light,
        |square| {
            mv_src_hashset.insert(square);
        },
    );
    for sq_src in &mv_src_hashset {
        assert_banjo_sq(&sq_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成り)");

        ss_hash_builder.src = sq_src.clone();
        // 成り
        ss_hash_builder.pro = true;
        ss_hash_builder.drop = PieceType::Kara;
        gets_movement(ss_hash_builder.to_hash());
    }
}

/// 打
///
/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  piece_dst
pub fn get_movement_by_square_and_piece_on_drop<F1>(
    sq_dst: &Square,
    piece_dst: &OPPieceVo,
    search_part: &SPMainDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_banjo_sq(&sq_dst, "get_movement_by_square_and_piece_on_drop");

    // 手番の先後、駒種類
    let piece_vo_dst = speed_of_light
        .ml_piece_struct_master_vo
        .get_piece_vo(piece_dst);
    let (sn, _kms_dst) = piece_vo_dst.phase_piece_type();

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(
        &search_part
            .get_current_position()
            .get_sn_by_sq(&sq_dst, speed_of_light),
        &sn,
    ) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = MLMovementDto::new();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    //let mut mv_src_hashset : HashSet<Square> = HashSet::<Square>::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_kms_hashset: HashSet<usize> = HashSet::new();
    make_drop_piece_type_by_square_piece(
        &sq_dst,
        piece_dst,
        &search_part,
        &speed_of_light,
        |piece_type_hash| {
            da_kms_hashset.insert(piece_type_hash);
        },
    );
    // 打
    for num_kms_da in da_kms_hashset.iter() {
        let kms_da = num_to_kms(*num_kms_da);

        let movement_hash = MLMovementDto {
            src: Square::from_umasu(SS_SRC_DA),
            dst: (*sq_dst).clone(),
            pro: false,
            drop: kms_da,
        }
        .to_hash();

        gets_movement(movement_hash);
    }
}
