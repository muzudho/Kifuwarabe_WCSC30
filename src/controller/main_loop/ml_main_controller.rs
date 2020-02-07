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

/// 指し手
pub fn hyoji_ss_hashset(ss_hashset: &HashSet<u64>) {
    g_writeln(&format!("ss_hashset.len()={}", ss_hashset.len()));
    // 辞書順ソート
    let mut vec_ss_str = Vec::new();
    for ss_hash in ss_hashset {
        let ss = MLMovementDto::from_hash(*ss_hash);
        let ss_str = format!("{}", ss);
        vec_ss_str.push(ss_str);
    }
    //vec_ss_str.sort();
    vec_ss_str.sort_by(|a_str, b_str| {
        let a_arr: Vec<_> = a_str.chars().collect();
        let b_arr: Vec<_> = b_str.chars().collect();
        use std::cmp::min;
        let len = min(a_arr.len(), b_arr.len());

        use std::cmp::Ordering;
        for i in 0..len {
            if a_arr[i] < b_arr[i] {
                //g_writeln(&format!( "[{}] a_arr {} < b_arr {}", i, a_arr[i], b_arr[i] ));
                return Ordering::Greater;
            } else if b_arr[i] < a_arr[i] {
                //g_writeln(&format!( "[{}] a_arr {} > b_arr {}", i, a_arr[i], b_arr[i] ));
                return Ordering::Less;
            }
        }

        if a_arr.len() < b_arr.len() {
            //g_writeln(&format!( "len a_arr {} < b_arr {}", a_arr.len(), b_arr.len() ));
            Ordering::Greater
        } else if b_arr.len() < a_arr.len() {
            //g_writeln(&format!( "len a_arr {} > b_arr {}", a_arr.len(), b_arr.len() ));
            Ordering::Less
        } else {
            //g_writeln(&format!( "len a_arr {} = b_arr {}", a_arr.len(), b_arr.len() ));
            Ordering::Equal
        }
    });
    vec_ss_str.reverse();

    let mut i = 0;
    for ss_str in vec_ss_str {
        g_writeln(&format!("[{}] {}", i, ss_str));
        i += 1;
    }
}

/// 利き数表示
pub fn cmd_kikisu(ml_dto: &MLDto, speed_of_light: &MLSpeedOfLightVo) {
    for pc in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", pc));
        let s = ml_dto.kaku_number_board(&Phase::Owari, pc, speed_of_light);
        g_writeln(&s);
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = ml_dto.kaku_number_board(&sn, &OPPieceVo::Owari, speed_of_light);
        g_writeln(&s);
    }
}
