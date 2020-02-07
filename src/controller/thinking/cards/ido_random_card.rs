#![allow(dead_code)]
//!
//! ランダム移動カード
//!

use super::super::super::super::controller::consoles::asserts::*;
use super::super::super::super::controller::movement_generation::mg_choicing::*;
use super::super::super::super::controller::movement_generation::mg_main::*;
use super::super::super::super::controller::thinking::randommove;
use super::super::super::super::controller::thinking::results::jisatusyu_result::*;
use super::super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use std::collections::HashSet;

/**
 * ランダム移動
 *
 * piece_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(
    universe: &Universe,
    piece_dst: &OPPieceVo,
    speed_of_light: &MLSpeedOfLightVo,
) -> MLMovementDto {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1000000 {
        // 移動したい先の升
        let sq_dst = randommove::random_square();
        assert_banjo_sq(&sq_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_dst,
            piece_dst.clone(),
            &universe.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &sq_dst,
            piece_dst,
            &universe.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1movement_from_hashset(&ss_hashset);

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    MLMovementDto::new()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(universe: &Universe, speed_of_light: &MLSpeedOfLightVo) -> MLMovementDto {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1000000 {
        // 移動したい先の升
        let sq_dst = randommove::random_square();
        assert_banjo_sq(&sq_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let ps_dst = speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo_by_phase_and_piece_type(
                &universe.get_search_part().get_phase(&Person::Ji),
                randommove::rnd_kms(),
            );
        let piece_dst = ps_dst.piece();

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_dst,
            piece_dst.clone(),
            &universe.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &sq_dst,
            piece_dst,
            &universe.get_search_part(),
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1movement_from_hashset(&ss_hashset);

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu(&universe, &ss, speed_of_light) {
            continue 'random;
        }

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    MLMovementDto::new()
}
