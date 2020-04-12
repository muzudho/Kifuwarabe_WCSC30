//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece_type::PieceType;
use crate::model::univ::speed_of_light::MLSpeedOfLightVo;

pub struct Evaluation {
    pub score: i16,
    pub king_catch: bool,
}
impl Evaluation {
    pub fn new(score1: i16, king_catch1: bool) -> Self {
        Evaluation {
            score: score1,
            king_catch: king_catch1,
        }
    }
}

pub struct SPEvaluationController {}
impl SPEvaluationController {
    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    /// ライオンを取ったら勝ちだぜ☆（＾～＾）
    pub fn evaluate(
        captured_piece_o: Option<Piece>,
        speed_of_light: &MLSpeedOfLightVo,
    ) -> Evaluation {
        if let Some(captured_piece) = captured_piece_o {
            let captured_ps = speed_of_light.get_piece_struct(&captured_piece);
            match captured_ps.phase_piece_type.1 {
                PieceType::King => Evaluation::new(25000, true),
                PieceType::Rook => Evaluation::new(1000, false),
                PieceType::Bishop => Evaluation::new(900, false),
                PieceType::Gold => Evaluation::new(600, false),
                PieceType::Silver => Evaluation::new(500, false),
                PieceType::Knight => Evaluation::new(300, false),
                PieceType::Lance => Evaluation::new(200, false),
                PieceType::Pawn => Evaluation::new(100, false),
                PieceType::Dragon => Evaluation::new(2000, false),
                PieceType::Horse => Evaluation::new(1900, false),
                PieceType::PromotedSilver => Evaluation::new(500, false),
                PieceType::PromotedKnight => Evaluation::new(300, false),
                PieceType::PromotedLance => Evaluation::new(200, false),
                PieceType::PromotedPawn => Evaluation::new(100, false),
            }
        } else {
            Evaluation::new(0, false)
        }
    }
}
