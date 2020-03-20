#![allow(dead_code)]
//!
//! コレクションの内容をダンプ（全部見る）とかだぜ☆（＾～＾）
//!
use crate::controller::io::*;
use crate::model::univ::gam::misc::phase::Phase;
use crate::model::univ::gam::misc::phase::*;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece::*;
use crate::model::univ::speed_of_light::*;
use crate::model::universe::*;

/// 利き数表示
pub fn cmd_kikisu(universe: &Universe, speed_of_light: &MLSpeedOfLightVo) {
    GPPieces::for_all(&mut |any_piece| {
        IO::writeln(&format!("利き数：{}", any_piece));
        let s = universe
            .game
            .print_number_board(&Phase::None, &any_piece, speed_of_light);
        IO::writeln(&s);
    });

    for phase in PHASE_ARRAY.iter() {
        IO::writeln(&format!("利き数：{}", phase));
        let s = universe
            .game
            .print_number_board(&phase, &Piece::OwariPiece, speed_of_light);
        IO::writeln(&s);
    }
}
