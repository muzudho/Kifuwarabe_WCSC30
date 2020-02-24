#![allow(dead_code)]
//!
//! コレクションの内容をダンプ（全部見る）とかだぜ☆（＾～＾）
//!
use super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::model::vo::game_part::gp_phase_vo::Phase;
use super::super::super::model::vo::game_part::gp_phase_vo::*;
use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::model::vo::game_part::gp_piece_vo::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;

/// 利き数表示
pub fn cmd_kikisu(ml_universe_dto: &MLUniverseDto, speed_of_light: &MLSpeedOfLightVo) {
    GPPieces::for_all(&mut |any_piece| {
        g_writeln(&format!("利き数：{}", any_piece));
        let s = ml_universe_dto.print_number_board(&Phase::None, &any_piece, speed_of_light);
        g_writeln(&s);
    });

    for phase in PHASE_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", phase));
        let s = ml_universe_dto.print_number_board(&phase, &GPPieceVo::OwariPiece, speed_of_light);
        g_writeln(&s);
    }
}
