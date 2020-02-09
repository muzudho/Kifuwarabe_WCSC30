#![allow(dead_code)]
//!
//! コレクションの内容をダンプ（全部見る）とかだぜ☆（＾～＾）
//!
use super::super::super::model::dto::main_loop::ml_dto::*;
use super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_phase_vo::Phase;
use super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::model::vo::other_part::op_piece_vo::OPPieceVo;
use super::super::super::model::vo::other_part::op_piece_vo::*;
use std::collections::HashSet;
use std::hash::BuildHasher;

/// 指し手
pub fn hyoji_ss_hashset<S: BuildHasher>(ss_hashset: &HashSet<u64, S>) {
    g_writeln(&format!("ss_hashset.len()={}", ss_hashset.len()));
    // 辞書順ソート
    let mut vec_ss_str = Vec::new();
    for ss_hash in ss_hashset {
        let ss = MLMovementDto::from_hash(*ss_hash);
        let ss_str = format!("{}", ss);
        vec_ss_str.push(ss_str);
    }
    //vec_ss_str.sort();
    vec_ss_str.sort_by(|y_str, x_str| {
        let y_arr: Vec<_> = y_str.chars().collect();
        let x_arr: Vec<_> = x_str.chars().collect();
        use std::cmp::min;
        let len = min(y_arr.len(), x_arr.len());

        use std::cmp::Ordering;
        for i in 0..len {
            match x_arr[i].cmp(&y_arr[i]) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }

        // Returns Ordering::Greater, Ordering::Less, Ordering::Equal.
        x_arr.len().cmp(&y_arr.len())
    });
    vec_ss_str.reverse();

    for (i, ss_str) in vec_ss_str.into_iter().enumerate() {
        g_writeln(&format!("[{}] {}", i, ss_str));
    }
}

/// 利き数表示
pub fn cmd_kikisu(ml_dto: &MLDto, speed_of_light: &MLSpeedOfLightVo) {
    for pc in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", pc));
        let s = ml_dto.kaku_number_board(&Phase::Owari, pc, speed_of_light);
        g_writeln(&s);
    }

    for phase in PHASE_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", phase));
        let s = ml_dto.kaku_number_board(&phase, &OPPieceVo::Owari, speed_of_light);
        g_writeln(&s);
    }
}
