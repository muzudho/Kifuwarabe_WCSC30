//! 探索部
use super::super::super::super::controller::common_use::cu_conv_controller::*;
use super::super::super::super::controller::search_part::sp_number_board_controller::*;
use super::super::super::super::model::dto::main_loop::ml_movement_dto::*;
use super::super::super::super::model::dto::search_part::sp_position_dto::*;
use super::super::super::super::model::vo::game_part::gp_piece_type_vo::*;
use super::super::super::super::model::vo::main_loop::ml_speed_of_light_vo::*;
use super::super::super::super::model::vo::other_part::op_person_vo::*;
use super::super::super::super::model::vo::other_part::op_phase_vo::*;
use super::super::super::super::model::vo::other_part::op_piece_vo::*;
use super::super::super::super::model::vo::other_part::op_ply_vo::*;
use super::super::super::super::model::vo::other_part::op_square_vo::*;

pub struct SPDto {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    ply: i16,
    /// 現局面
    current_position: SPPositionDto,
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hash_history: [u64; TEME_LN],
    /// 取った駒
    pub captured_piece_history: [OPPieceVo; TEME_LN],
    /// 棋譜
    pub moves_history: [MLMovementDto; TEME_LN],
    /// 利きの数（先後別）
    pub effect_count_by_phase: [NumberBoard; SN_LN],
    /// 利きの数（先後付き駒別）
    pub effect_count_by_piece: [NumberBoard; KM_LN],
    // ビジョン・ツリー
    // pub vision_tree_by_phase: [VisionTree; SN_LN],
}
impl Default for SPDto {
    fn default() -> Self {
        SPDto {
            ply: 0,
            // 現局面
            current_position: SPPositionDto::default(),
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
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara,
                OPPieceVo::Kara, //257要素
            ],
            moves_history: [
                // 1行16要素で並べるぜ☆（＾～＾）
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(),
                MLMovementDto::default(), //257要素
            ],
            /// 利き数（先後別）
            effect_count_by_phase: [
                NumberBoard::default(),
                NumberBoard::default(),
                NumberBoard::default(),
            ],
            // 利き数（駒別なので３０個ある）
            effect_count_by_piece: [
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
        }
    }
}
impl SPDto {
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
        use super::super::super::super::model::vo::other_part::op_person_vo::Person::*;
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

    pub fn get_current_position(&self) -> &SPPositionDto {
        &self.current_position
    }
    pub fn get_current_position_mut(&mut self) -> &mut SPPositionDto {
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

    /**
     * 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
     * 手目のカウントが増えたりはしないぜ☆（＾～＾）
     */
    pub fn undo_move(
        &mut self,
        phase: &Phase,
        move2: &MLMovementDto,
        speed_of_light: &MLSpeedOfLightVo,
    ) {
        let cap = self.captured_piece_history[self.get_ply() as usize].clone();

        // 移動先の駒
        let piece186 = if move2.src.to_umasu() == SS_SRC_DA {
            // 打なら
            let piece679 = OPPieceVo::from_phase_piece_type(phase, move2.drop);
            // 自分の持ち駒を増やす
            //let mg = km_to_mg(km);
            //self.add_hand(mg,1);
            self.current_position.add_hand(&piece679, 1, speed_of_light);
            piece679
        } else {
            // 打で無ければ
            if move2.pro {
                // 成ったなら、成る前へ
                speed_of_light
                    .ml_piece_struct_master_vo
                    .get_piece_vo(self.current_position.get_piece_by_square(&move2.dst))
                    .demote()
                    .clone()
            } else {
                self.current_position
                    .get_piece_by_square(&move2.dst)
                    .clone()
            }
        };

        // 移動先の駒を、取った駒（あるいは空）に戻す
        self.current_position.set_piece_by_square(&move2.dst, &cap);
        match cap {
            OPPieceVo::Kara => {}
            _ => {
                // 自分の持ち駒を減らす
                self.current_position.add_hand(
                    speed_of_light
                        .ml_piece_struct_master_vo
                        .get_piece_vo(&cap)
                        .capture(),
                    -1,
                    speed_of_light,
                );
            }
        }

        // 移動元升に、動かした駒を置く
        self.current_position
            .set_piece_by_square(&move2.src, &piece186);
    }

    /// らいおんの位置
    pub fn get_king_sq(&self, jiai: &Person) -> &Square {
        &self
            .current_position
            .get_sq_r(sn_to_num(&self.get_phase(jiai)))
    }

    /// 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
    /// 手目のカウントが増えたりはしないぜ☆（＾～＾）
    ///
    /// return : 取った駒
    pub fn do_move(
        &mut self,
        move1: &MLMovementDto,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> OPPieceVo {
        let phase = self.get_phase(&Person::Ji);

        // 取った駒
        let cap;

        {
            // 動かす駒
            let piece144 = if move1.src.to_umasu() == SS_SRC_DA {
                // 打なら
                // 自分の持ち駒を減らす
                let piece734 = OPPieceVo::from_phase_piece_type(&phase, move1.drop);
                self.current_position
                    .add_hand(&piece734, -1, speed_of_light);
                piece734
            } else {
                // 打で無ければ、元の升の駒を消す。
                let piece152 = if move1.pro {
                    // 成りなら
                    speed_of_light
                        .ml_piece_struct_master_vo
                        .get_piece_vo(self.current_position.get_piece_by_square(&move1.src))
                        .promote()
                        .clone()
                } else {
                    self.current_position
                        .get_piece_by_square(&move1.src)
                        .clone()
                };

                self.current_position
                    .set_piece_by_square(&move1.src, &OPPieceVo::Kara);

                piece152
            };

            // 移動先升に駒があるかどうか
            cap = if let OPPieceVo::Kara = self.current_position.get_piece_by_square(&move1.dst) {
                OPPieceVo::Kara
            } else {
                // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
                let cap764;
                {
                    cap764 = self
                        .current_position
                        .get_piece_by_square(&move1.dst)
                        .clone();
                }

                {
                    let cap773;
                    {
                        cap773 = speed_of_light
                            .ml_piece_struct_master_vo
                            .get_piece_vo(&cap764)
                            .capture()
                            .clone();
                    }
                    self.current_position.add_hand(&cap773, 1, speed_of_light);
                }
                cap764
            };

            // 移動先升に駒を置く
            self.current_position
                .set_piece_by_square(&move1.dst, &piece144);
        }

        cap
    }

    pub fn get_moves_history(&self) -> &[MLMovementDto; TEME_LN] {
        &self.moves_history
    }
    pub fn get_moves_history_mut(&mut self) -> &mut [MLMovementDto; TEME_LN] {
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
    pub fn set_move_drop(&mut self, piece_type: GPPieceTypeVo) {
        self.moves_history[self.get_ply() as usize].drop = piece_type
    }
    pub fn set_cap(&mut self, ply1: usize, km: OPPieceVo) {
        self.captured_piece_history[ply1] = km
    }
    pub fn get_move(&self) -> &MLMovementDto {
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

    /*
    /// 相手の　玉　の位置を覚えます。
    pub fn memory_opponent_king(&mut self, phase: &Phase, opponent_phase: &Phase) {
        self.vision_tree_by_phase[sn_to_num(phase)]
            .set_ai_r(&self.current_position.get_sq_r(sn_to_num(opponent_phase)));
    }
    */
}
