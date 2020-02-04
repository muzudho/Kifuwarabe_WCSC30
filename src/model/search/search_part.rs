//! 探索部
use super::super::super::controller::common::conv::*;
use super::super::super::controller::communication::usi::*;
use super::super::super::controller::status::number_board::*;
use super::super::super::controller::thinking::visions::vision_tree::*;
use super::super::super::model::master::person::*;
use super::super::super::model::master::phase::*;
use super::super::super::model::master::piece::*;
use super::super::super::model::master::piece_struct::*;
use super::super::super::model::master::piece_struct_master::PieceStructMaster;
use super::super::super::model::master::piece_type::*;
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
    /// 取った駒
    pub captured_piece_history: [Piece; TEME_LN],
    /// 棋譜
    pub moves_history: [Sasite; TEME_LN],
    /// 利きの数（先後別）
    pub effect_count_by_phase: [NumberBoard; SN_LN],
    /// 利きの数（先後付き駒別）
    pub effect_count_by_piece: [NumberBoard; KM_LN],
    /// ビジョン・ツリー
    pub vision_tree_by_phase: [VisionTree; SN_LN],
    /// 駒構造体・マスター
    piece_struct_master: PieceStructMaster,
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
            /// 取った駒
            captured_piece_history: [
                // 1行16要素で並べるぜ☆（＾～＾）
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara, //257要素
            ],
            moves_history: [
                // 1行16要素で並べるぜ☆（＾～＾）
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(), //257要素
            ],
            /// 利き数（先後別）
            effect_count_by_phase: [NumberBoard::new(), NumberBoard::new(), NumberBoard::new()],
            // 利き数（駒別なので３０個ある）
            effect_count_by_piece: [
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
            ],
            vision_tree_by_phase: [VisionTree::new(), VisionTree::new(), VisionTree::new()],
            piece_struct_master: PieceStructMaster::new(),
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

    pub fn undo_move(&mut self, phase: &Phase, move1: &Sasite) {
        let cap = self.captured_piece_history[self.get_ply() as usize].clone();
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

    pub fn get_moves_history(&self) -> &[Sasite; TEME_LN] {
        &self.moves_history
    }
    pub fn get_moves_history_mut(&mut self) -> &mut [Sasite; TEME_LN] {
        &mut self.moves_history
    }
    /// 棋譜の作成
    pub fn set_move_src(&mut self, src: &Square) {
        self.moves_history[self.get_ply() as usize].src = src.clone()
    }
    pub fn set_move_dst(&mut self, dst: &Square) {
        self.moves_history[self.get_ply() as usize].dst = dst.clone()
    }
    pub fn set_move_pro(&mut self, pro: bool) {
        self.moves_history[self.get_ply() as usize].pro = pro
    }
    pub fn set_move_drop(&mut self, kms: PieceType) {
        self.moves_history[self.get_ply() as usize].drop = kms
    }
    pub fn set_cap(&mut self, ply1: usize, km: Piece) {
        self.captured_piece_history[ply1] = km
    }
    pub fn get_move(&self) -> &Sasite {
        &self.moves_history[self.get_ply() as usize]
    }

    /// 棋譜☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.get_ply() {
            let ss = &self.moves_history[ply as usize];
            s.push_str(&format!("[{}] {}", ply, ss));
        }
        s
    }

    /// 相手の　玉　の位置を覚えます。
    pub fn memory_opponent_king(&mut self, phase: &Phase, opponent_phase: &Phase) {
        self.vision_tree_by_phase[sn_to_num(phase)]
            .set_ai_r(&self.current_position.get_sq_r(sn_to_num(opponent_phase)));
    }

    /// 先後＆駒種類→先後付き駒
    pub fn get_piece_struct_by_phase_and_piece_type(
        &self,
        sn: &Phase,
        kms: &PieceType,
    ) -> &PieceStruct {
        &self
            .piece_struct_master
            .get_piece_struct_by_phase_and_piece_type(sn, kms)
    }
    pub fn get_piece_struct_by_square(&self, square: &Square) -> &PieceStruct {
        &self
            .piece_struct_master
            .get_piece_struct(&self.current_position.get_piece_by_square(square))
    }

    pub fn get_piece_struct(&self, piece: &Piece) -> &PieceStruct {
        &self.piece_struct_master.get_piece_struct(piece)
    }

    pub fn get_piece_struct_master(&self) -> &PieceStructMaster {
        &self.piece_struct_master
    }

    pub fn get_piece_struct_master_mut(&mut self) -> &mut PieceStructMaster {
        &mut self.piece_struct_master
    }
}
