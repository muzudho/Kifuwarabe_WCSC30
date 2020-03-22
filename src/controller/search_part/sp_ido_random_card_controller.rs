#![allow(dead_code)]
//!
//! ランダム移動カード
//!

use super::super::super::controller::common_use::cu_asserts_controller::*;
use super::super::super::controller::common_use::cu_random_move_controller;
use super::super::super::controller::movement_generation::mg_choicing_controller::*;
use super::super::super::controller::movement_generation::mg_controller::*;
use super::super::super::controller::search_part::sp_jisatusyu_result_controller::*;
use crate::model::univ::gam::misc::movement_builder::*;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::speed_of_light::*;
use crate::model::universe::*;
use std::collections::HashSet;

/**
 * ランダム移動
 *
 * piece_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(
    ml_universe_dto: &Universe,
    piece_dst: &Piece,
    speed_of_light: &MLSpeedOfLightVo,
) -> MovementBuilder {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1_000_000 {
        // 移動したい先の升
        let sq_dst = cu_random_move_controller::random_square();
        assert_banjo_sq(&sq_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_dst,
            piece_dst.clone(),
            &ml_universe_dto.game.position,
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &sq_dst,
            piece_dst,
            &ml_universe_dto.game.position,
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
    MovementBuilder::default()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(
    ml_universe_dto: &Universe,
    speed_of_light: &MLSpeedOfLightVo,
) -> MovementBuilder {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1_000_000 {
        // 移動したい先の升
        let sq_dst = cu_random_move_controller::random_square();
        assert_banjo_sq(&sq_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let ps_dst = speed_of_light.get_piece_struct_by_phase_and_piece_type(
            &ml_universe_dto.game.history.get_phase(&Person::Friend),
            *cu_random_move_controller::random_piece_type(),
        );
        let piece_dst = &ps_dst.piece;

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &sq_dst,
            piece_dst.clone(),
            &ml_universe_dto.game.position,
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &sq_dst,
            piece_dst,
            &ml_universe_dto.game.position,
            &speed_of_light,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1movement_from_hashset(&ss_hashset);

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu(&ml_universe_dto, &ss, speed_of_light) {
            continue 'random;
        }

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    MovementBuilder::default()
}
