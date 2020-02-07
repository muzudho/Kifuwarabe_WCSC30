//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

extern crate rand;
use rand::Rng;
use std::collections::HashSet;

use super::super::super::controller::go_command::gc_kikisu_controller::*;
use super::super::super::controller::movement_generation::mg_choicing_controller::*;
use super::super::super::controller::movement_generation::mg_main_controller::*;
use super::super::super::model::dto::main_loop::ml_dto::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;

/// 現局面での最善手を返すぜ☆（*＾～＾*）
pub fn think(ml_dto: &mut MLDto, speed_of_light: &MLSpeedOfLightVo) -> MLMovementDto {
    // TODO 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）

    // +----------------------+
    // | 王手放置漏れ回避対策 |
    // +----------------------+

    // 相手の利き升調べ（自殺手防止のため）
    {
        update_effect_count(ml_dto, speed_of_light);
    }

    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
    let mut ss_hashset = HashSet::<u64>::new();

    // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
    get_potential_movement(
        &ml_dto.get_search_part(),
        &speed_of_light,
        |movement_hash| {
            ss_hashset.insert(movement_hash);
        },
    );
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &ss_hashset );

    select_movement_except_check(&mut ss_hashset, &ml_dto.get_search_part(), &speed_of_light);

    // 現局面を見て、ビジョンを作り直せだぜ☆（＾～＾）
    // &ml_dto.remake_visions();
    // insert_rakkansuji(ml_dto);
    // TODO 楽観筋はまだ使ってない☆（＾～＾）

    // 楽観王手の一覧はできているはず。

    // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
    select_movement_except_fourfold_repetition(&mut ss_hashset, ml_dto, speed_of_light);

    // 自殺手は省くぜ☆（＾～＾）
    select_movement_except_suiceid(&mut ss_hashset, ml_dto, speed_of_light);

    if ss_hashset.len() == 0 {
        // 投了
        return MLMovementDto::new();
    } else {
        let index = rand::thread_rng().gen_range(0, ss_hashset.len());
        let mut i = 0;
        for ss_hash in ss_hashset {
            if i == index {
                //let result : MLMovementDto = ss.clone();
                let ss = MLMovementDto::from_hash(ss_hash);
                g_writeln(&format!("info solution:{}.", ss));
                return ss;
            }
            i += 1;
        }

        // 投了
        return MLMovementDto::new();
    }
}
