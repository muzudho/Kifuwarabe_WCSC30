//!
//! 指し手選択
//!

extern crate rand;
use rand::Rng;

use super::super::super::controller::common::conv::*;
use super::super::super::controller::communication::usi::*;
use super::super::super::controller::consoles::asserts::*;
use super::super::super::controller::movement_generation::mg_sub_part::*;
use super::super::super::controller::thinking::results::komatori_result::*;
use super::super::super::model::application::application_part::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::ply::*;
use super::super::super::model::master::square::*;
use super::super::super::model::search::search_part::*;
use super::super::super::model::universe::*;
use std::collections::HashSet;

pub fn choice_1ss_by_hashset(ss_hashset: &HashSet<u64>) -> Sasite {
    let index = if ss_hashset.len() == 0 {
        0
    } else {
        rand::thread_rng().gen_range(0, ss_hashset.len())
    };
    let mut i = 0;
    let mut ss_choice_hash = 0;
    for ss_hash in ss_hashset.iter() {
        if i == index {
            ss_choice_hash = *ss_hash;
            break;
        }
        i += 1;
    }
    Sasite::from_hash(ss_choice_hash)
}

/**
 * 王が取られる局面を除く手を選ぶぜ☆（＾～＾）
 */
pub fn select_movement_except_check(
    ss_hashset_input: &mut HashSet<u64>,
    application_part: &ApplicationPart,
    search_part: &SearchPart,
) {
    // 自玉の位置
    let sq_r = search_part.get_king_sq(&Person::Ji).clone();
    g_writeln(&format!("info string My raion {}.", sq_r.to_umasu()));

    // 王手の一覧を取得
    let komatori_result_hashset: HashSet<u64> = lookup_catching_king_on_board(
        &application_part,
        &search_part,
        &search_part.get_phase(&Person::Ai),
        &sq_r,
    );
    if 0 < komatori_result_hashset.len() {
        // 王手されていれば

        // 表示
        g_writeln(&format!(
            "info string My raion is {} OUTED.",
            komatori_result_hashset.len()
        ));
        for komatori_result_hash0 in komatori_result_hashset.iter() {
            let komatori_result = KomatoriResult::from_hash(*komatori_result_hash0);
            // どんな王手か、出力
            g_writeln(&format!("info OUTE: {}.", komatori_result));
        }

        let mut ss_hashset_pickup: HashSet<u64> = HashSet::new();

        // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
        'idea: for hash_ss_potential in ss_hashset_input.iter() {
            let ss_potential = Sasite::from_hash(*hash_ss_potential);
            for komatori_result_hash in komatori_result_hashset.iter() {
                let komatori_result = KomatoriResult::from_hash(*komatori_result_hash);

                assert_banjo_sq(&ss_potential.dst, "(206)Ｓearch_gohoshu_hash");
                match komatori_result.get_result(&ss_potential) {
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
            ss_hashset_pickup.insert(*hash_ss_potential);
        }

        // 振り替え
        ss_hashset_input.clear();
        for hash_ss in ss_hashset_pickup.iter() {
            ss_hashset_input.insert(*hash_ss);
        }
    } else {
        // 王手されていなければ
        g_writeln(&format!("info string My raion is not outed."));
    }
}

/**
 * 王手されていれば、王手を解除しろだぜ☆（＾～＾）
 * 千日手には喜んで飛び込めだぜ☆（＾▽＾）ｗｗｗ
 */
pub fn select_movement_except_suiceid(
    ss_hashset_input: &mut HashSet<u64>,
    universe: &mut Universe,
) {
    // 残すのはここに退避する☆（＾～＾）
    let mut ss_hashset_pickup: HashSet<u64> = HashSet::new();

    // 自玉の位置
    let sq_r = universe
        .get_search_part()
        .get_current_position()
        .get_sq_r(sn_to_num(
            &universe.get_search_part().get_phase(&Person::Ji),
        ))
        .clone();

    // 王手回避カードを発行する
    // TODO 王手が２か所から掛かっていたら、全部回避しないといけない☆

    // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
    'idea: for hash_ss_potential in ss_hashset_input.iter() {
        let ss_potential = Sasite::from_hash(*hash_ss_potential);

        // その手を指してみる
        universe.do_ss(&ss_potential);
        // // 現局面表示
        // let s1 = &universe.kaku_ky( &KyNums::Current );
        // g_writeln( &s1 );

        // 狙われている方の玉の位置
        let sq_r_new = if ss_potential.src.to_umasu() == sq_r.to_umasu() {
            ss_potential.dst.clone() // 狙われていた方の玉が動いた先
        } else {
            sq_r.clone() // 動いていない、狙われていた方の玉の居場所
        };

        // 利きの再計算
        // 有り得る移動元が入る☆（＾～＾）
        let mut attackers: HashSet<Square> = HashSet::<Square>::new();
        get_no_promotion_src_by_sn_sq(
            &universe.get_search_part().get_phase(&Person::Ji), // 指定の升に駒を動かそうとしている手番
            &sq_r_new,                                          // 指定の升
            &universe.get_application_part(),
            &universe.get_search_part(),
            |square| {
                attackers.insert(square);
            },
        );
        get_before_promotion_src_by_sn_sq(
            &universe.get_search_part().get_phase(&Person::Ji), // 指定の升に駒を動かそうとしている手番
            &sq_r_new,                                          // 指定の升
            &universe.get_application_part(),
            &universe.get_search_part(),
            |square| {
                attackers.insert(square);
            },
        );

        // 玉が利きに飛び込んでいるか？
        let jisatusyu = 0 < attackers.len();
        g_writeln(&format!(
            "info {} evaluated => {} attackers. offence={}->{}",
            ss_potential,
            attackers.len(),
            universe.get_search_part().get_phase(&Person::Ji),
            sq_r_new.to_umasu()
        ));
        for sq_atk in attackers.iter() {
            g_writeln(&format!("info ms_atk={}.", sq_atk.to_umasu()));
        }

        // 手を戻す
        universe.undo_ss();
        // // 現局面表示
        // let s2 = &universe.kaku_ky( &KyNums::Current );
        // g_writeln( &s2 );

        if jisatusyu {
            continue 'idea;
        }

        g_writeln(&format!("info SOLUTED ss={}.", ss_potential));
        // 問題を全て解決していれば、入れる
        ss_hashset_pickup.insert(ss_potential.to_hash());
    }
    g_writeln(&format!("info {} solutions.", ss_hashset_pickup.len()));

    // 空っぽにする
    ss_hashset_input.clear();
    // 振り替える
    for hash_ss in ss_hashset_pickup.iter() {
        ss_hashset_input.insert(*hash_ss);
    }
}

/**
 * 千日手の指し手を取り除いた集合を作るぜ☆（＾～＾）
 *
 * ただし、千日手を取り除くと手がない場合は、千日手を選ぶぜ☆（＾～＾）
 */
pub fn select_movement_except_fourfold_repetition(
    ss_hashset_input: &mut HashSet<u64>,
    universe: &mut Universe,
) {
    let mut ss_hashset_pickup = HashSet::new();
    // 指せる手から、千日手が消えている手だけ選んで、集合を作るぜ☆（＾～＾）
    // 'idea:
    for hash_ss_potential in ss_hashset_input.iter() {
        let ss = Sasite::from_hash(*hash_ss_potential);
        //ss_hashset.insert( *hash_ss_potential );

        // その手を指してみる
        universe.do_ss(&ss);
        // 現局面表示
        // let s1 = &universe.kaku_ky( &KyNums::Current );
        // g_writeln( &s1 );

        // 千日手かどうかを判定する☆（＾～＾）
        if universe.count_same_ky() < SENNTITE_NUM {
            ss_hashset_pickup.insert(*hash_ss_potential);
        } else {
            // 千日手
        }

        // 手を戻す FIXME: 打った象が戻ってない？
        universe.undo_ss();
        // 現局面表示
        // let s2 = &universe.kaku_ky( &KyNums::Current );
        // g_writeln( &s2 );
    }

    // ただし、千日手を取り除くと手がない場合は、千日手を選ぶぜ☆（＾～＾）
    if 0 == ss_hashset_pickup.len() {
        return;
    }

    // 振り替え
    ss_hashset_input.clear();
    for hash_ss in ss_hashset_pickup.iter() {
        ss_hashset_input.insert(*hash_ss);
    }
}