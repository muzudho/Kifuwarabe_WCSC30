//!
//! 盤上いろいろ☆（＾～＾）
//!
use super::super::super::controller::status::uchu::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece_struct::PieceStruct;
use super::super::super::model::master::place::*;

pub fn is_ji_km_by_ms(ms: umasu, uchu: &Uchu) -> bool {
    let km = uchu.ky.get_km_by_ms(ms);
    let (sn, _kms) = PieceStruct::from_piece(&km).phase_piece_type();
    match_sn(&sn, &uchu.get_teban(&Person::Ji))
}

// TODO
pub fn is_ai_kiki_by_ms(_ms: umasu, _uchu: &Uchu) -> bool {
    false
}
