//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::model::univ::gam::piece::GPPieceVo;
use crate::model::univ::gam::piece_type::GPPieceTypeVo;
use crate::model::vo::main_loop::ml_speed_of_light_vo::MLSpeedOfLightVo;

pub struct SPEvaluationController {}
impl SPEvaluationController {
    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    /// ライオンを取ったら勝ちだぜ☆（＾～＾）
    pub fn evaluate(captured_piece: GPPieceVo, speed_of_light: &MLSpeedOfLightVo) -> (i16, bool) {
        let piece_struct = speed_of_light.get_piece_struct_vo(&captured_piece);
        match piece_struct.phase_piece_type().1 {
            GPPieceTypeVo::King => (25000, true),
            GPPieceTypeVo::Rook => (1000, false),
            GPPieceTypeVo::Bishop => (900, false),
            GPPieceTypeVo::Gold => (600, false),
            GPPieceTypeVo::Silver => (500, false),
            GPPieceTypeVo::Knight => (300, false),
            GPPieceTypeVo::Lance => (200, false),
            GPPieceTypeVo::Pawn => (100, false),
            GPPieceTypeVo::Dragon => (2000, false),
            GPPieceTypeVo::Horse => (1900, false),
            GPPieceTypeVo::PromotedSilver => (500, false),
            GPPieceTypeVo::PromotedKnight => (300, false),
            GPPieceTypeVo::PromotedLance => (200, false),
            GPPieceTypeVo::PromotedPawn => (100, false),
            _ => (0, false),
        }
    }
}
