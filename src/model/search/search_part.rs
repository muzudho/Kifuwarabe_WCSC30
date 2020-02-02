//! 探索部
use super::super::super::controller::common::conv::*;
use super::super::super::controller::communication::usi::*;
use super::super::super::model::master::person::*;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::*;
use super::super::super::model::master::ply::*;
use super::super::super::model::master::square::*;
use super::super::super::model::search::position::*;

pub struct SearchPart {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    ply: i16,
    /// 現局面
    current_position: Position,
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hash_history: [u64; TEME_LN],
}
impl SearchPart {
    pub fn new() -> Self {
        SearchPart {
            ply: 0,
            // 現局面
            current_position: Position::new(),
            position_hash_history: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, //257手目 TODO 321手に 拡張したいぜ☆（＾～＾）
            ],
        }
    }
    pub fn add_ply(&mut self, ply1: i16) {
        self.ply += ply1
    }
    pub fn get_ply(&self) -> i16 {
        self.ply
    }
    pub fn set_ply(&mut self, ply1: i16) {
        self.ply = ply1
    }

    /// 手番
    pub fn get_phase(&self, jiai: &Person) -> Phase {
        use super::super::super::model::master::person::Person::*;
        match *jiai {
            Ji => {
                // 手番
                if self.ply % 2 == 0 {
                    Phase::Sen
                } else {
                    Phase::Go
                }
            }
            Ai => {
                // 相手番
                if self.ply % 2 == 0 {
                    Phase::Go
                } else {
                    Phase::Sen
                }
            }
            _ => Phase::Owari,
        }
    }

    pub fn get_current_position(&self) -> &Position {
        &self.current_position
    }
    pub fn get_current_position_mut(&mut self) -> &mut Position {
        &mut self.current_position
    }

    pub fn get_position_hash_history(&self) -> &[u64; TEME_LN] {
        &self.position_hash_history
    }
    pub fn get_position_hash_history_mut(&mut self) -> &mut [u64; TEME_LN] {
        &mut self.position_hash_history
    }

    pub fn get_current_position_hash(&mut self) -> u64 {
        self.position_hash_history[self.ply as usize]
    }
    pub fn set_current_position_hash(&mut self, hash: u64) {
        self.position_hash_history[self.ply as usize] = hash;
    }

    pub fn undo_move(&mut self, phase: &Phase, move1: &Sasite, cap: &Piece) {
        self.current_position.undo_sasite(phase, move1, cap);
    }

    /// らいおんの位置
    pub fn get_king_sq(&self, jiai: &Person) -> &Square {
        &self
            .current_position
            .get_sq_r(sn_to_num(&self.get_phase(jiai)))
    }

    /// TODO 返り値がコピーになってる☆（＾～＾）？
    pub fn do_move(&mut self, move1: &Sasite) -> Piece {
        self.current_position
            .do_sasite(&self.get_phase(&Person::Ji), move1)
    }
}
