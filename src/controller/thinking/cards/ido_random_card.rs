#![allow(dead_code)]
//!
//! ランダム移動カード
//!

use super::super::super::super::controller::communication::usi::*;
use super::super::super::super::controller::consoles::asserts::*;
use super::super::super::super::controller::movement_generation::mg_choicing::*;
use super::super::super::super::controller::movement_generation::mg_main::*;
use super::super::super::super::controller::thinking::randommove;
use super::super::super::super::controller::thinking::results::jisatusyu_result::*;
use super::super::super::super::model::master::person::Person;
use super::super::super::super::model::master::piece::Piece;
use super::super::super::super::model::universe::*;
use std::collections::HashSet;

/**
 * ランダム移動
 *
 * km_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(universe: &Universe, km_dst: &Piece) -> Sasite {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1000000 {
        // 移動したい先の升
        let sq_dst = randommove::random_square();
        assert_banjo_sq(&sq_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &universe.get_application_part(),
            &universe.get_search_part(),
            &sq_dst,
            &km_dst,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &universe.get_search_part(),
            &sq_dst,
            &km_dst,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1ss_by_hashset(&ss_hashset);

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    Sasite::new()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(universe: &Universe) -> Sasite {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1000000 {
        // 移動したい先の升
        let sq_dst = randommove::random_square();
        assert_banjo_sq(&sq_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let ps_dst = universe
            .get_application_part()
            .get_piece_struct_master()
            .get_piece_struct_by_phase_and_piece_type(
                &universe.get_search_part().get_phase(&Person::Ji),
                randommove::rnd_kms(),
            );
        let km_dst = ps_dst.piece();

        ss_hashset.clear();
        get_movement_by_square_and_piece_on_board(
            &universe.get_application_part(),
            &universe.get_search_part(),
            &sq_dst,
            &km_dst,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        get_movement_by_square_and_piece_on_drop(
            &universe.get_search_part(),
            &sq_dst,
            &km_dst,
            |movement_hash| {
                ss_hashset.insert(movement_hash);
            },
        );
        let ss = choice_1ss_by_hashset(&ss_hashset);

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu(&universe, &ss) {
            continue 'random;
        }

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    Sasite::new()
}
