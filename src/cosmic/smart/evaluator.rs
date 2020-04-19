//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::toy_box::Piece;
use crate::law::speed_of_light::*;

pub struct Evaluation {
    pub value: i16,
}
impl Evaluation {
    pub fn new(value1: i16) -> Self {
        Evaluation { value: value1 }
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    pub fn from_caputured_piece(
        captured_piece_o: Option<Piece>,
        speed_of_light: &SpeedOfLight,
    ) -> Evaluation {
        if let Some(captured_piece) = captured_piece_o {
            let captured_ps = speed_of_light.piece_chart(&captured_piece);
            match captured_ps.phase_piece_type.1 {
                PieceType::King => Evaluation::new(25000), // 玉を取ったら、評価しないのでここには来ないぜ☆（＾～＾）
                PieceType::Rook => Evaluation::new(1000),
                PieceType::Bishop => Evaluation::new(900),
                PieceType::Gold => Evaluation::new(600),
                PieceType::Silver => Evaluation::new(500),
                PieceType::Knight => Evaluation::new(300),
                PieceType::Lance => Evaluation::new(200),
                PieceType::Pawn => Evaluation::new(100),
                PieceType::Dragon => Evaluation::new(2000),
                PieceType::Horse => Evaluation::new(1900),
                PieceType::PromotedSilver => Evaluation::new(500),
                PieceType::PromotedKnight => Evaluation::new(300),
                PieceType::PromotedLance => Evaluation::new(200),
                PieceType::PromotedPawn => Evaluation::new(100),
            }
        } else {
            Evaluation::new(0)
        }
    }
}
