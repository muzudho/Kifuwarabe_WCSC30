//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::model::vo::game_part::gp_piece_type_vo::GPPieceTypeVo;
use crate::model::vo::game_part::gp_piece_vo::GPPieceVo;
use crate::model::vo::main_loop::ml_speed_of_light_vo::MLSpeedOfLightVo;

pub struct SPEvaluationController {}
impl SPEvaluationController {
    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    pub fn evaluate(captured_piece: GPPieceVo, speed_of_light: &MLSpeedOfLightVo) -> i16 {
        let piece_struct = speed_of_light.get_piece_struct_vo(&captured_piece);
        match piece_struct.phase_piece_type().1 {
            GPPieceTypeVo::King => 30000,
            GPPieceTypeVo::Rook => 1000,
            GPPieceTypeVo::Bishop => 900,
            GPPieceTypeVo::Gold => 600,
            GPPieceTypeVo::Silver => 500,
            GPPieceTypeVo::Knight => 300,
            GPPieceTypeVo::Lance => 200,
            GPPieceTypeVo::Pawn => 100,
            GPPieceTypeVo::Dragon => 2000,
            GPPieceTypeVo::Horse => 1900,
            GPPieceTypeVo::PromotedSilver => 500,
            GPPieceTypeVo::PromotedKnight => 300,
            GPPieceTypeVo::PromotedLance => 200,
            GPPieceTypeVo::PromotedPawn => 100,
            _ => 0,
        }
    }
}
