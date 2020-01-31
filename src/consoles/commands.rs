//!
//! コマンド一覧
//!

use super::super::controller::status::uchu::*;
use super::super::model::master::phase::Phase;
use super::super::model::master::phase::*;
use super::super::model::master::piece::Piece;
use super::super::model::master::piece::*;

/**
 * 利き数表示
 */
pub fn cmd_kikisu(uchu: &Uchu) {
    for km in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", km));
        let s = uchu.kaku_number_board(&Phase::Owari, &km);
        g_writeln(&s);
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = uchu.kaku_number_board(&sn, &Piece::Owari);
        g_writeln(&s);
    }
}
