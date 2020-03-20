#![allow(dead_code)]
//!
//! コレクションの内容をダンプ（全部見る）とかだぜ☆（＾～＾）
//!
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::model::univ::gam::phase::Phase;
use crate::model::univ::gam::phase::*;
use crate::model::univ::gam::piece::GPPieceVo;
use crate::model::univ::gam::piece::*;
use crate::model::universe::*;

/// 利き数表示
pub fn cmd_kikisu(universe: &Universe, speed_of_light: &MLSpeedOfLightVo) {
    GPPieces::for_all(&mut |any_piece| {
        g_writeln(&format!("利き数：{}", any_piece));
        let s = universe
            .game
            .print_number_board(&Phase::None, &any_piece, speed_of_light);
        g_writeln(&s);
    });

    for phase in PHASE_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", phase));
        let s = universe
            .game
            .print_number_board(&phase, &GPPieceVo::OwariPiece, speed_of_light);
        g_writeln(&s);
    }
}
