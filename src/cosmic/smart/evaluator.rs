//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::toy_box::Piece;
use crate::law::speed_of_light::*;

/// 勝利の価値☆（＾～＾）
pub const WIN_VALUE: i16 = 32000;
/// 敗北の価値☆（＾～＾）
pub const LOSE_VALUE: i16 = -32000;
/// 千日手の価値☆（＾～＾）
pub const REPITITION_VALUE: i16 = -300;

pub struct Evaluation {
    pub value: i16,
}
impl Evaluation {
    pub fn new(value1: i16) -> Self {
        Evaluation { value: value1 }
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    ///
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないんで、
    /// 読みの深い所の駒の価値は減らしてやろうぜ☆（＾～＾）？
    pub fn from_caputured_piece(
        cur_depth: u8,
        captured_piece_o: Option<Piece>,
        speed_of_light: &SpeedOfLight,
    ) -> Evaluation {
        // 0 除算を避けたいだけだぜ☆（＾～＾）
        let denom = (cur_depth + 1) as i16;
        if let Some(captured_piece_val) = captured_piece_o {
            match captured_piece_val.r#type(speed_of_light) {
                PieceType::King => Evaluation::new(WIN_VALUE), // 玉を取ったら、評価しないのでここには来ないぜ☆（＾～＾）
                PieceType::Rook => Evaluation::new((1000 / denom) as i16),
                PieceType::Bishop => Evaluation::new((900 / denom) as i16),
                PieceType::Gold => Evaluation::new((600 / denom) as i16),
                PieceType::Silver => Evaluation::new((500 / denom) as i16),
                PieceType::Knight => Evaluation::new((300 / denom) as i16),
                PieceType::Lance => Evaluation::new((200 / denom) as i16),
                PieceType::Pawn => Evaluation::new((100 / denom) as i16),
                PieceType::Dragon => Evaluation::new((2000 / denom) as i16),
                PieceType::Horse => Evaluation::new((1900 / denom) as i16),
                PieceType::PromotedSilver => Evaluation::new((500 / denom) as i16),
                PieceType::PromotedKnight => Evaluation::new((300 / denom) as i16),
                PieceType::PromotedLance => Evaluation::new((200 / denom) as i16),
                PieceType::PromotedPawn => Evaluation::new((100 / denom) as i16),
            }
        } else {
            Evaluation::new(0)
        }
    }
}
