//!
//! 駒集合
//!

use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece::*;
use crate::model::univ::game::Game;
use crate::model::univ::speed_of_light::*;
use std::collections::HashSet;

pub struct SPPieceSetDto {
    num_syugo: HashSet<usize>,
}
impl SPPieceSetDto {
    /// 全ての元を含む
    pub fn new_all(speed_of_light: &SpeedOfLight) -> SPPieceSetDto {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        GPPieces::for_all(&mut |any_piece| {
            let ps = speed_of_light.get_piece_struct(&any_piece);
            num_syugo1.insert(ps.serial_piece_number);
        });
        SPPieceSetDto {
            num_syugo: num_syugo1,
        }
    }
    /// 自分相手
    pub fn new_person(
        &self,
        person: &Person,
        game: &Game,
        speed_of_light: &SpeedOfLight,
    ) -> SPPieceSetDto {
        let phase0 = game.history.get_phase(&person);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        GPPieces::for_all(&mut |any_piece| {
            let ps = speed_of_light.get_piece_struct(&any_piece);
            let (phase2, _piece_type) = &ps.phase_piece_type;
            if phase0 == *phase2 {
                num_syugo1.insert(ps.serial_piece_number);
            }
        });
        SPPieceSetDto {
            num_syugo: num_syugo1,
        }
    }
    pub fn remove(&mut self, piece: &Piece, speed_of_light: &SpeedOfLight) {
        self.num_syugo
            .remove(&speed_of_light.get_piece_struct(piece).serial_piece_number);
    }
}
