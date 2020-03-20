//! 探索部
use crate::controller::search_part::sp_number_board_controller::*;
use crate::model::dto::search_part::sp_info::SPInfo;
use crate::model::univ::gam::board::*;
use crate::model::univ::gam::movement_builder::*;
use crate::model::univ::gam::phase::*;
use crate::model::univ::gam::piece::*;
use crate::model::univ::gam::piece_type::*;
use crate::model::univ::gam::square::*;

/// ミュータブルなオブジェクト☆（＾～＾）
pub struct Position {
    /// 現局面
    pub current_board: Board,

    /// 現在の指し手を作成中。
    pub current_movement_builder: MovementBuilder,

    /// 利きの数（先後別）
    pub control_count_by_phase: [NumberBoard; PHASE_LN],

    /// 利きの数（先後付き駒別）
    pub control_count_by_piece: [NumberBoard; PIECE_LN],
    // ビジョン・ツリー
    // pub vision_tree_by_phase: [VisionTree; PHASE_LN],
    /// 情報表示担当
    pub info: SPInfo,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            // 現局面
            current_board: Board::default(),
            /// 現在の指し手を作成中。
            current_movement_builder: MovementBuilder::default(),
            /// 利き数（先後別）
            control_count_by_phase: [
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
            ],
            // 利き数（駒別なので３０個ある）
            control_count_by_piece: [
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
            ],
            // vision_tree_by_phase: [VisionTree::default(), VisionTree::default(), VisionTree::default()],
            info: SPInfo::default(),
        }
    }
}
impl Position {
    pub fn get_current_board(&self) -> &Board {
        &self.current_board
    }
    pub fn get_current_board_mut(&mut self) -> &mut Board {
        &mut self.current_board
    }

    pub fn set_current_movement_source_temporary(&mut self, src: &Square) {
        self.current_movement_builder.src = src.clone()
    }
    pub fn set_current_movement_destination_temporary(&mut self, dst: &Square) {
        self.current_movement_builder.dst = dst.clone()
    }
    pub fn set_current_movement_promote_temporary(&mut self, pro: bool) {
        self.current_movement_builder.pro = pro
    }
    pub fn set_current_movement_drop_temporary(&mut self, piece_type: PieceType) {
        self.current_movement_builder.drop = piece_type
    }

    /*
    /// 相手の　玉　の位置を覚えます。
    pub fn memory_opponent_king(&mut self, phase: &Phase, opponent_phase: &Phase) {
        self.vision_tree_by_phase[phase_to_num(phase)]
            .set_ai_r(&self.current_position.get_sq_r(phase_to_num(opponent_phase)));
    }
    */
}
