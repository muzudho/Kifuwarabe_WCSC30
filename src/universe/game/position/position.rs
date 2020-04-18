use crate::universe::game::board::board::*;
use crate::universe::game::board::square::*;
use crate::universe::game::movement::movement_builder::MovementBuilder;
use crate::universe::game::piece::piece_type::*;

/// ミュータブルなオブジェクト☆（＾～＾）
pub struct Position {
    /// 現局面
    pub current_board: Board,

    /// 現在の指し手を作成中。
    pub current_movement_builder: MovementBuilder,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            // 現局面
            current_board: Board::default(),
            /// 現在の指し手を作成中。
            current_movement_builder: MovementBuilder::default(),
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
