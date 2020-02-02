//!
//! 深い考えだぜ☆（＾～＾）
//!

extern crate rand;
use rand::Rng;
use std::collections::HashSet;

use super::super::super::controller::boardmetries::hypothesis::atamakin;
use super::super::super::controller::communication::usi::*;
use super::super::super::controller::movement_generation::mg_choicing::*;
use super::super::super::controller::movement_generation::mg_main::*;
use super::super::super::controller::status::mediators::med_kikisu::*;
use super::super::super::controller::thinking::visions::vision_tree::*;
use super::super::super::model::universe::*;

/**
 * 現局面での最善手を返すぜ☆（*＾～＾*）
 */
pub fn think(mut universe: &mut Universe) -> Sasite {
    // TODO 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）

    // +----------------------+
    // | 王手放置漏れ回避対策 |
    // +----------------------+

    // 相手の利き升調べ（自殺手防止のため）
    update_effect_count(&mut universe);

    g_writeln(&format!("info test is_s={}", atamakin::is_s(&universe)));

    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
    let mut ss_hashset = HashSet::<u64>::new();

    // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
    get_potential_movement(
        &universe.get_application_part(),
        &universe.get_search_part(),
        |movement_hash| {
            ss_hashset.insert(movement_hash);
        },
    );
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &ss_hashset );

    select_movement_except_check(
        &mut ss_hashset,
        &universe.get_application_part(),
        &universe.get_search_part(),
    );

    // 現局面を見て、ビジョンを作り直せだぜ☆（＾～＾）
    &universe.remake_visions();
    insert_rakkansuji(&mut universe);
    // TODO 楽観筋はまだ使ってない☆（＾～＾）

    // 楽観王手の一覧はできているはず。

    // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
    select_movement_except_fourfold_repetition(&mut ss_hashset, &mut universe);

    // 自殺手は省くぜ☆（＾～＾）
    select_movement_except_suiceid(&mut ss_hashset, universe);

    if ss_hashset.len() == 0 {
        // 投了
        return Sasite::new();
    } else {
        let index = rand::thread_rng().gen_range(0, ss_hashset.len());
        let mut i = 0;
        for ss_hash in ss_hashset {
            if i == index {
                //let result : Sasite = ss.clone();
                let ss = Sasite::from_hash(ss_hash);
                g_writeln(&format!("info solution:{}.", ss));
                return ss;
            }
            i += 1;
        }

        // 投了
        return Sasite::new();
    }
}
