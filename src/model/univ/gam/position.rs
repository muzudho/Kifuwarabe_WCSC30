//! 探索部
use crate::controller::search_part::sp_number_board_controller::*;
use crate::model::univ::gam::board::*;
use crate::model::univ::gam::misc::movement_builder::*;
use crate::model::univ::gam::misc::phase::*;
use crate::model::univ::gam::misc::piece::*;
use crate::model::univ::gam::misc::piece_type::*;
use crate::model::univ::gam::misc::square::*;

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
}
impl Default for Position {
    fn default() -> Self {
        Position {
            // 現局面
            current_board: Board::default(),
            /// 現在の指し手を作成中。
            current_movement_builder: MovementBuilder::default(),
            /// 利き数（先後別）
            control_count_by_phase: [NumberBoard::default(), NumberBoard::default()],
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
        }
    }
}
impl Position {
    pub fn set_current_movement_source_temporary(&mut self, src: &Square) {
        self.current_movement_builder.src = src.clone()
    }
    pub fn set_current_movement_destination_temporary(&mut self, dst: &Square) {
        self.current_movement_builder.dst = dst.clone()
    }
    pub fn set_current_movement_promote_temporary(&mut self, pro: bool) {
        self.current_movement_builder.pro = pro
    }
    pub fn set_current_movement_drop_temporary(&mut self, piece_type: Option<PieceType>) {
        self.current_movement_builder.drop = piece_type
    }
}
