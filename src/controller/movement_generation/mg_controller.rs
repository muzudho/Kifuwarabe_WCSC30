//!
//! 現局面を使った指し手生成
//!

use super::super::super::controller::common_use::cu_asserts_controller::*;
use super::super::super::controller::movement_generation::mg_sub_part_controller::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::dto::search_part::sp_earth_dto::*;
use super::super::super::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::model::vo::game_part::gp_square_and_piece_vo::GPSquareAndPieceVo;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
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
    sp_earth_dto: &SPEarthDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement_callback: F1,
) where
    F1: FnMut(u64),
{
    // +----------------+
    // | 盤上の駒の移動 |
    // +----------------+
    // (段)
    for rank_src in 1..10 {
        // (筋)
        for file_src in 1..10 {
            let source_of_sqp;
            {
                let sq_src = Square::from_file_rank(file_src, rank_src);
                source_of_sqp = GPSquareAndPieceVo::new(
                    &sq_src,
                    sp_earth_dto
                        .get_current_position()
                        .get_piece_by_square(&sq_src),
                );
            }

            if &speed_of_light
                .get_piece_struct_vo(&source_of_sqp.piece)
                .phase()
                == &sp_earth_dto.get_phase(&Person::Friend)
            {
                // 手番の駒

                let mut dst_hashset: HashSet<Square> = HashSet::<Square>::new();
                make_destination_by_square_piece(
                    &source_of_sqp,
                    false, // 成らず
                    &sp_earth_dto,
                    &speed_of_light,
                    &mut dst_hashset,
                );

                // g_writeln("テスト ポテンシャルムーブ insert_dst_by_sq_km(成らず).");
                // use consoles::visuals::dumps::*;
                // hyoji_sq_hashset( &dst_hashset );

                for sq_dst in &dst_hashset {
                    gets_movement_callback(
                        MLMovementDto {
                            src: source_of_sqp.square.clone(),
                            dst: sq_dst.clone(),
                            pro: false, // 成らず
                            drop: GPPieceTypeVo::KaraPieceType,
                        }
                        .to_hash(speed_of_light),
                    );
                }

                dst_hashset.clear();
                make_destination_by_square_piece(
                    &source_of_sqp,
                    true, // 成り
                    &sp_earth_dto,
                    &speed_of_light,
                    &mut dst_hashset,
                );
                for sq_dst in &dst_hashset {
                    gets_movement_callback(
                        MLMovementDto {
                            src: source_of_sqp.square.clone(),
                            dst: sq_dst.clone(),
                            pro: true, // 成り
                            drop: GPPieceTypeVo::KaraPieceType,
                        }
                        .to_hash(speed_of_light),
                    );
                }
            }
        }
    }

    // +----+
    // | 打 |
    // +----+
    // (段)
    for rank_dst in 1..10 {
        // (筋)
        for file_dst in 1..10 {
            let destination_of_sqp;
            {
                let sq_dst = Square::from_file_rank(file_dst, rank_dst);
                destination_of_sqp = GPSquareAndPieceVo::new(
                    &sq_dst,
                    sp_earth_dto
                        .get_current_position()
                        .get_piece_by_square(&sq_dst),
                );
            }

            if let GPPieceVo::NonePiece = destination_of_sqp.piece {
                // 駒が無いところに打つ
                let mut da_piece_type_hashset = HashSet::new();
                for piece_type_motigoma in MGS_ARRAY.iter() {
                    let ps_motigoma = speed_of_light.get_piece_struct_vo_by_phase_and_piece_type(
                        &sp_earth_dto.get_phase(&Person::Friend),
                        *piece_type_motigoma,
                    );
                    let pc_motigoma = ps_motigoma.piece();
                    if 0 < sp_earth_dto
                        .get_current_position()
                        .get_hand(pc_motigoma, speed_of_light)
                    {
                        // 駒を持っていれば
                        make_drop_piece_type_by_square_piece(
                            &destination_of_sqp.square,
                            pc_motigoma,
                            &sp_earth_dto.get_current_position(),
                            &speed_of_light,
                            |piece_type_hash| {
                                da_piece_type_hashset.insert(piece_type_hash);
                            },
                        );
                    }
                }
                for num_piece_type_da in da_piece_type_hashset {
                    let piece_type = num_to_piece_type(num_piece_type_da);
                    gets_movement_callback(
                        MLMovementDto {
                            src: Square::from_umasu(SS_SRC_DA),     // 駒大
                            dst: destination_of_sqp.square.clone(), // どの升へ行きたいか
                            pro: false,                             // 打に成りは無し
                            drop: piece_type,                       // 打った駒種類
                        }
                        .to_hash(speed_of_light),
                    );
                }
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
    piece_dst: GPPieceVo,
    sp_earth_dto: &SPEarthDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_banjo_sq(&sq_dst, "Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let ps_dst = speed_of_light.get_piece_struct_vo(&piece_dst);
    let (phase, _piece_type_dst) = ps_dst.phase_piece_type();

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if sp_earth_dto
        .get_current_position()
        .get_phase_by_sq(&sq_dst, speed_of_light)
        == *phase
    {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = MLMovementDto::default();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    let mut mv_src_hashset: HashSet<Square> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    make_no_promotion_source_by_square_and_piece(
        &sq_dst,
        &ps_dst,
        &sp_earth_dto,
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
        ss_hash_builder.drop = GPPieceTypeVo::KaraPieceType;
        gets_movement(ss_hash_builder.to_hash(speed_of_light));
    }

    // +--------------+
    // | 盤上（成り） |
    // +--------------+
    mv_src_hashset.clear();
    make_before_promotion_source_by_square_piece(
        sq_dst,
        &ps_dst,
        &sp_earth_dto,
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
        ss_hash_builder.drop = GPPieceTypeVo::KaraPieceType;
        gets_movement(ss_hash_builder.to_hash(speed_of_light));
    }
}

/// 打
///
/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  piece_dst
pub fn get_movement_by_square_and_piece_on_drop<F1>(
    sq_dst: &Square,
    piece_dst: &GPPieceVo,
    sp_earth_dto: &SPEarthDto,
    speed_of_light: &MLSpeedOfLightVo,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_banjo_sq(&sq_dst, "get_movement_by_square_and_piece_on_drop");

    // 手番の先後、駒種類
    let piece_vo_dst = speed_of_light.get_piece_struct_vo(piece_dst);
    let (phase, _piece_type_dst) = piece_vo_dst.phase_piece_type();

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if sp_earth_dto
        .get_current_position()
        .get_phase_by_sq(&sq_dst, speed_of_light)
        == *phase
    {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = MLMovementDto::default();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    //let mut mv_src_hashset : HashSet<Square> = HashSet::<Square>::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_piece_type_hashset: HashSet<usize> = HashSet::new();
    make_drop_piece_type_by_square_piece(
        &sq_dst,
        piece_dst,
        &sp_earth_dto.get_current_position(),
        &speed_of_light,
        |piece_type_hash| {
            da_piece_type_hashset.insert(piece_type_hash);
        },
    );
    // 打
    for num_piece_type_da in da_piece_type_hashset.iter() {
        let piece_type_da = num_to_piece_type(*num_piece_type_da);

        let movement_hash = MLMovementDto {
            src: Square::from_umasu(SS_SRC_DA),
            dst: (*sq_dst).clone(),
            pro: false,
            drop: piece_type_da,
        }
        .to_hash(speed_of_light);

        gets_movement(movement_hash);
    }
}
