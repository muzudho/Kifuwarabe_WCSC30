//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::daydream::Value;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::toy_box::Piece;
use crate::law::speed_of_light::*;

/// 千日手の価値☆（＾～＾）
pub const REPITITION_VALUE: i16 = -300;

pub struct Evaluation {
    pub value: Value,
}
impl Evaluation {
    pub fn new(value1: Value) -> Self {
        Evaluation { value: value1 }
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    ///
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないんで、
    /// 読みの深い所の駒の価値は減らしてやろうぜ☆（＾～＾）？
    ///
    /// * `cur_depth` - １手指すから葉に進めるわけで、必ず 1 は有るから 0除算エラー は心配しなくていいぜ☆（＾～＾）
    pub fn from_caputured_piece(
        cur_depth: u8,
        captured_piece: Option<Piece>,
        speed_of_light: &SpeedOfLight,
    ) -> Evaluation {
        if let Some(captured_piece_val) = captured_piece {
            let cur_depth = cur_depth as i16;
            match captured_piece_val.r#type(speed_of_light) {
                PieceType::King => Evaluation::new(Value::Win), // 玉を取ったら、評価しないのでここには来ないぜ☆（＾～＾）
                PieceType::Rook => Evaluation::new(Value::CentiPawn(1000 / cur_depth)),
                PieceType::Bishop => Evaluation::new(Value::CentiPawn(900 / cur_depth)),
                PieceType::Gold => Evaluation::new(Value::CentiPawn(600 / cur_depth)),
                PieceType::Silver => Evaluation::new(Value::CentiPawn(500 / cur_depth)),
                PieceType::Knight => Evaluation::new(Value::CentiPawn(300 / cur_depth)),
                PieceType::Lance => Evaluation::new(Value::CentiPawn(200 / cur_depth)),
                PieceType::Pawn => Evaluation::new(Value::CentiPawn(100 / cur_depth)),
                PieceType::Dragon => Evaluation::new(Value::CentiPawn(2000 / cur_depth)),
                PieceType::Horse => Evaluation::new(Value::CentiPawn(1900 / cur_depth)),
                PieceType::PromotedSilver => Evaluation::new(Value::CentiPawn(500 / cur_depth)),
                PieceType::PromotedKnight => Evaluation::new(Value::CentiPawn(300 / cur_depth)),
                PieceType::PromotedLance => Evaluation::new(Value::CentiPawn(200 / cur_depth)),
                PieceType::PromotedPawn => Evaluation::new(Value::CentiPawn(100 / cur_depth)),
            }
        } else {
            Evaluation::new(Value::CentiPawn(0))
        }
    }
}
