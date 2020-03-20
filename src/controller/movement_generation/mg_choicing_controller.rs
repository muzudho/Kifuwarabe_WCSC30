//!
//! 指し手選択
//!

extern crate rand;
use crate::controller::common_use::cu_asserts_controller::*;
use crate::controller::io::*;
use crate::controller::movement_generation::mg_controller::*;
use crate::controller::movement_generation::mg_komatori_result_controller::*;
use crate::model::univ::gam::misc::movement::*;
use crate::model::univ::gam::misc::movement_builder::*;
use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::phase::phase_to_num;
use crate::model::univ::gam::misc::square::*;
use crate::model::univ::game::Game;
use crate::model::univ::speed_of_light::*;
use crate::model::universe::*;
use rand::Rng;
use std::collections::HashSet;
use std::hash::BuildHasher;

/// ハッシュセットから、指し手を１つ選ぶぜ☆（＾～＾）
pub fn choice_1movement_from_hashset<S: BuildHasher>(
    movement_hashset: &HashSet<u64, S>,
) -> MovementBuilder {
    let index = if movement_hashset.is_empty() {
        0
    } else {
        rand::thread_rng().gen_range(0, movement_hashset.len())
    };
    let mut ss_choice_hash = 0;
    for (i, ss_hash) in movement_hashset.iter().enumerate() {
        if i == index {
            ss_choice_hash = *ss_hash;
            break;
        }
    }
    MovementBuilder::from_hash(ss_choice_hash)
}

/**
 * 王が取られる局面を除く手を選ぶぜ☆（＾～＾）
 */
pub fn select_movement_except_check<S: BuildHasher>(
    ss_hashset_input: &mut HashSet<u64, S>,
    game: &Game,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // 自玉の位置
    let sq_r = game.get_king_sq(&Person::Friend).clone();
    // g_writeln(&format!("info string My raion {}.", sq_r.to_usquare()));

    // 王手の一覧を取得
    let komatori_result_hashset: HashSet<u64> = lookup_catching_king_on_board(
        &game.history.get_phase(&Person::Opponent),
        &sq_r,
        &game.position,
        &speed_of_light,
    );
    if !komatori_result_hashset.is_empty() {
        // 王手されていれば

        /*
        // 表示
        g_writeln(&format!(
            "info string My raion is {} OUTED.",
            komatori_result_hashset.len()
        ));
        for komatori_result_hash0 in komatori_result_hashset.iter() {
            let komatori_result = KomatoriResult::from_hash(*komatori_result_hash0);
            // どんな王手か、出力
            g_writeln(&format!("info string OUTE: {}.", komatori_result));
        }
        */

        let mut ss_hashset_pickup: HashSet<u64> = HashSet::new();

        // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
        'idea: for hash_potential_movement in ss_hashset_input.iter() {
            let potential_movement = MovementBuilder::from_hash(*hash_potential_movement);
            for komatori_result_hash in komatori_result_hashset.iter() {
                let komatori_result = KomatoriResult::from_hash(*komatori_result_hash);

                assert_banjo_sq(&potential_movement.dst, "(206)Ｓearch_gohoshu_hash");
                match komatori_result.get_result(&potential_movement, speed_of_light) {
                    KomatoriResultResult::NoneAttacker
                    | KomatoriResultResult::NoneAigoma
                    | KomatoriResultResult::NoneMoved => {
                        // 駒取りが起こらないものだけが解決
                    }
                    _ => {
                        // 解決しないのが１つでもあれば、次のアイデアへ☆（＾～＾）
                        continue 'idea;
                    }
                }
            }

            // 王手を回避している指し手
            ss_hashset_pickup.insert(*hash_potential_movement);
        }

        // 振り替え
        ss_hashset_input.clear();
        for hash_ss in ss_hashset_pickup.iter() {
            ss_hashset_input.insert(*hash_ss);
        }
    } else {
        // 王手されていなければ
        // g_writeln(&"info string My raion is not outed.".to_string());
    }
}

/**
 * 王手されていれば、王手を解除しろだぜ☆（＾～＾）
 * 千日手には喜んで飛び込めだぜ☆（＾▽＾）ｗｗｗ
 */
pub fn select_movement_except_suiceid<S: BuildHasher>(
    ss_hashset_input: &mut HashSet<u64, S>,
    ml_universe_dto: &mut Universe,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // 残すのはここに退避する☆（＾～＾）
    let mut ss_hashset_pickup: HashSet<u64> = HashSet::new();

    // 自玉の位置
    let sq_r = ml_universe_dto
        .game
        .position
        .current_board
        .get_sq_r(phase_to_num(
            &ml_universe_dto.game.history.get_phase(&Person::Friend),
        ))
        .clone();

    // 王手回避カードを発行する
    // TODO 王手が２か所から掛かっていたら、全部回避しないといけない☆

    // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
    'idea: for hash_potential_movement in ss_hashset_input.iter() {
        let potential_movement = Movement::from_hash(*hash_potential_movement);

        // その手を指してみる
        ml_universe_dto.do_move(&potential_movement, speed_of_light);
        // // 現局面表示
        // let s1 = &ml_universe_dto.print_ky( &PosNums::Current );
        // g_writeln( &s1 );

        // 狙われている方の玉の位置
        let sq_r_new = if potential_movement.source.to_usquare() == sq_r.to_usquare() {
            potential_movement.destination.clone() // 狙われていた方の玉が動いた先
        } else {
            sq_r.clone() // 動いていない、狙われていた方の玉の居場所
        };

        // 利きの再計算
        // 有り得る移動元が入る☆（＾～＾）
        let mut attackers: HashSet<Square> = HashSet::<Square>::new();
        lookup_no_promotion_source_by_phase_square(
            &ml_universe_dto.game.history.get_phase(&Person::Friend), // 指定の升に駒を動かそうとしている手番
            &sq_r_new,                                                // 指定の升
            &ml_universe_dto.game.position.current_board,
            &speed_of_light,
            |square| {
                attackers.insert(square);
            },
        );
        lookup_before_promotion_source_by_phase_square(
            &ml_universe_dto.game.history.get_phase(&Person::Friend), // 指定の升に駒を動かそうとしている手番
            &sq_r_new,                                                // 指定の升
            &ml_universe_dto.game.position.current_board,
            &speed_of_light,
            |square| {
                attackers.insert(square);
            },
        );

        // 玉が利きに飛び込んでいるか？
        let jisatusyu = !attackers.is_empty();
        IO::writeln(&format!(
            "info string {} evaluated => {} attackers. offence={}->{}",
            potential_movement,
            attackers.len(),
            ml_universe_dto.game.history.get_phase(&Person::Friend),
            sq_r_new.to_usquare()
        ));
        for sq_atk in attackers.iter() {
            IO::writeln(&format!("info string ms_atk={}.", sq_atk.to_usquare()));
        }

        // 手を戻す
        ml_universe_dto.undo_move(speed_of_light);
        // // 現局面表示
        // let s2 = &ml_universe_dto.print_ky( &PosNums::Current );
        // g_writeln( &s2 );

        if jisatusyu {
            continue 'idea;
        }

        IO::writeln(&format!(
            "info string SOLUTED movement={}.",
            potential_movement
        ));
        // 問題を全て解決していれば、入れる
        ss_hashset_pickup.insert(potential_movement.to_hash(speed_of_light));
    }
    IO::writeln(&format!(
        "info string {} solutions.",
        ss_hashset_pickup.len()
    ));

    // 空っぽにする
    ss_hashset_input.clear();
    // 振り替える
    for hash_ss in ss_hashset_pickup.iter() {
        ss_hashset_input.insert(*hash_ss);
    }
}
