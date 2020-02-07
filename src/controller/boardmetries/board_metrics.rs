//!
//! 盤上いろいろ☆（＾～＾）
//!
use super::super::super::model::dto::application_part::ap_universe_dto::*;
use super::super::super::model::vo::person::Person;
use super::super::super::model::vo::phase::*;
use super::super::super::model::vo::speed_of_light::*;
use super::super::super::model::vo::square::*;

pub fn is_ji_km_by_sq(sq: &Square, universe: &Universe, speed_of_light: &SpeedOfLight) -> bool {
    match_sn(
        &speed_of_light
            .piece_vo_master
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
