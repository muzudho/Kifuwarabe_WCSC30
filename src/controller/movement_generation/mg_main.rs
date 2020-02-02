//!
//! 現局面を使った指し手生成
//!

use super::super::super::controller::common::conv::*;
use super::super::super::controller::communication::usi::*;
use super::super::super::controller::consoles::asserts::*;
use super::super::super::controller::movement_generation::mg_sub_part::*;
use super::super::super::model::application::application_part::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::Piece;
use super::super::super::model::master::piece_struct::PieceStruct;
use super::super::super::model::master::piece_type::PieceType;
use super::super::super::model::master::piece_type::*;
use super::super::super::model::master::square::*;
use super::super::super::model::search::search_part::*;
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
    application_part: &ApplicationPart,
    search_part: &SearchPart,
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
            let km_src = search_part
                .get_current_position()
                .get_piece_struct_by_sq(&sq_src)
                .piece();

            if match_sn(
                &PieceStruct::from_piece(&km_src).phase(),
                &search_part.get_phase(&Person::Ji),
            ) {
                // 手番の駒

                let mut dst_hashset: HashSet<Square> = HashSet::<Square>::new();
                get_dst_by_sq_km(
                    &sq_src,
                    &km_src,
                    false, // 成らず
                    &search_part,
                    &mut dst_hashset,
                );

                // g_writeln("テスト ポテンシャルムーブ insert_dst_by_sq_km(成らず).");
                // use consoles::visuals::dumps::*;
                // hyoji_sq_hashset( &dst_hashset );

                for sq_dst in &dst_hashset {
                    gets_movement_callback(
                        Sasite {
                            src: sq_src.clone(),
                            dst: sq_dst.clone(),
                            pro: false, // 成らず
                            drop: PieceType::Kara,
                        }
                        .to_hash(),
                    );
                }

                dst_hashset.clear();
                get_dst_by_sq_km(
                    &sq_src,
                    &km_src,
                    true, // 成り
                    &search_part,
                    &mut dst_hashset,
                );
                for sq_dst in &dst_hashset {
                    gets_movement_callback(
                        Sasite {
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
            let km_dst = search_part
                .get_current_position()
                .get_piece_struct_by_sq(&sq_dst)
                .piece();
            match km_dst {
                Piece::Kara => {
                    // 駒が無いところに打つ

                    let mut da_kms_hashset = HashSet::new();
                    for kms_motigoma in MGS_ARRAY.iter() {
                        let ps_motigoma = application_part
                            .get_piece_struct_master()
                            .get_piece_struct_by_phase_and_piece_type(
                                &search_part.get_phase(&Person::Ji),
                                kms_motigoma,
                            );
                        let km_motigoma = ps_motigoma.piece();
                        if 0 < search_part.get_current_position().get_mg(&km_motigoma) {
                            // 駒を持っていれば
                            get_drop_kms_by_sq_km(
                                &sq_dst,
                                &km_motigoma,
                                &search_part,
                                |piece_type_hash| {
                                    da_kms_hashset.insert(piece_type_hash);
                                },
                            );
                        }
                    }
                    for num_kms_da in da_kms_hashset {
                        let kms = num_to_kms(num_kms_da);
                        gets_movement_callback(
                            Sasite {
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

/**
 * 1. 移動先升指定  ms_dst
 * 2. 移動先駒指定  km_dst
 *
 * 盤上の駒の移動の最初の１つ。打を除く
 */
pub fn get_movement_by_square_and_piece_on_board<F1>(
    application_part: &ApplicationPart,
    search_part: &SearchPart,
    sq_dst: &Square,
    km_dst: &Piece,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_banjo_sq(&sq_dst, "Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let ps_dst = PieceStruct::from_piece(&km_dst);
    let (sn, _kms_dst) = ps_dst.phase_piece_type();

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(
        &search_part.get_current_position().get_sn_by_sq(&sq_dst),
        &sn,
    ) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Sasite::new();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    let mut mv_src_hashset: HashSet<Square> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    get_no_promotion_src_by_sq_km(&sq_dst, &ps_dst, &search_part, |square| {
        mv_src_hashset.insert(square);
    });
    for sq_src in &mv_src_hashset {
        assert_banjo_sq(&sq_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成らず)");

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
    get_before_promotion_src_by_sq_km(sq_dst, &ps_dst, &application_part, &search_part, |square| {
        mv_src_hashset.insert(square);
    });
    for sq_src in &mv_src_hashset {
        assert_banjo_sq(&sq_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成り)");

        ss_hash_builder.src = sq_src.clone();
        // 成り
        ss_hash_builder.pro = true;
        ss_hash_builder.drop = PieceType::Kara;
        gets_movement(ss_hash_builder.to_hash());
    }
}
/**
 * 打
*
 * 1. 移動先升指定  ms_dst
 * 2. 移動先駒指定  km_dst
 */
pub fn get_movement_by_square_and_piece_on_drop<F1>(
    search_part: &SearchPart,
    sq_dst: &Square,
    km_dst: &Piece,
    mut gets_movement: F1,
) where
    F1: FnMut(u64),
{
    assert_banjo_sq(&sq_dst, "get_movement_by_square_and_piece_on_drop");

    // 手番の先後、駒種類
    let piece_struct_dst = PieceStruct::from_piece(&km_dst);
    let (sn, _kms_dst) = piece_struct_dst.phase_piece_type();

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(
        &search_part.get_current_position().get_sn_by_sq(&sq_dst),
        &sn,
    ) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Sasite::new();

    ss_hash_builder.dst = (*sq_dst).clone();

    // 移動元の升
    //let mut mv_src_hashset : HashSet<Square> = HashSet::<Square>::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_kms_hashset: HashSet<usize> = HashSet::new();
    get_drop_kms_by_sq_km(&sq_dst, &km_dst, &search_part, |piece_type_hash| {
        da_kms_hashset.insert(piece_type_hash);
    });
    // 打
    for num_kms_da in da_kms_hashset.iter() {
        let kms_da = num_to_kms(*num_kms_da);

        let movement_hash = Sasite {
            src: Square::from_umasu(SS_SRC_DA),
            dst: (*sq_dst).clone(),
            pro: false,
            drop: kms_da,
        }
        .to_hash();

        gets_movement(movement_hash);
    }
}
