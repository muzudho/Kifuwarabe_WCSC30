//!
//! 盤上いろいろ☆（＾～＾）
//!
use super::super::super::model::dto::main_loop::ml_universe_dto::*;
use super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::model::vo::other_part::op_square_vo::*;

pub fn is_ji_km_by_sq(sq: &Square, universe: &Universe, speed_of_light: &MLSpeedOfLightVo) -> bool {
    match_sn(
        &speed_of_light
            .ml_piece_struct_master_vo
            .get_piece_vo(
                &universe
                    .get_search_part()
                    .get_current_position()
                    .get_piece_by_square(&sq),
            )
            .phase(),
        &universe.get_search_part().get_phase(&Person::Ji),
    )
}

// TODO
pub fn is_ai_kiki_by_sq(_sq: &Square, _uchu: &Universe) -> bool {
    false
}
