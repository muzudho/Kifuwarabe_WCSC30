//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::recording::Movement;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::toy_box::Piece;
use crate::law::speed_of_light::*;

/// 千日手の価値☆（＾～＾）
pub const REPITITION_VALUE: i16 = -300;

pub struct Evaluation {}
impl Evaluation {
    /// 成ったら評価に加点するぜ☆（＾～＾）
    /// 駒得より 評価は下げた方が良さげ☆（＾～＾）
    pub fn from_promotion(cur_depth: usize, source: PieceType, movement: &Movement) -> i16 {
        if movement.promote {
            (match source {
                PieceType::Bishop => 90,
                PieceType::Knight => 20,
                PieceType::Lance => 10,
                PieceType::Pawn => 50,
                PieceType::Rook => 100,
                PieceType::Silver => 40,
                _ => 0,
            }) / (cur_depth as i16)
        } else {
            0
        }
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    ///
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないんで、
    /// 読みの深い所の駒の価値は減らしてやろうぜ☆（＾～＾）？
    ///
    /// * `cur_depth` - １手指すから葉に進めるわけで、必ず 1 は有るから 0除算エラー は心配しなくていいぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// Centi pawn.
    pub fn from_caputured_piece(
        cur_depth: usize,
        captured_piece: Option<Piece>,
        speed_of_light: &SpeedOfLight,
    ) -> i16 {
        if let Some(captured_piece_val) = captured_piece {
            (match captured_piece_val.r#type(speed_of_light) {
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                PieceType::King => 0,
                PieceType::Rook => 1000,
                PieceType::Bishop => 900,
                PieceType::Gold => 600,
                PieceType::Silver => 500,
                PieceType::Knight => 300,
                PieceType::Lance => 200,
                PieceType::Pawn => 100,
                PieceType::Dragon => 2000,
                PieceType::Horse => 1900,
                PieceType::PromotedSilver => 500,
                PieceType::PromotedKnight => 300,
                PieceType::PromotedLance => 200,
                PieceType::PromotedPawn => 100,
            }) / (cur_depth as i16)
        } else {
            0
        }
    }
}
