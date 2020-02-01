//!
//! 盤上いろいろ☆（＾～＾）
//!
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::square::*;
use super::super::super::model::universe::*;

pub fn is_ji_km_by_sq(sq: &Square, uchu: &Universe) -> bool {
    match_sn(
        &uchu.ky.get_piece_struct_by_sq(&sq).phase(),
        &uchu.get_teban(&Person::Ji),
    )
}

// TODO
pub fn is_ai_kiki_by_sq(_sq: &Square, _uchu: &Universe) -> bool {
    false
}
