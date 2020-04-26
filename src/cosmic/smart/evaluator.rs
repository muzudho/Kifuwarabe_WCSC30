//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::smart::features::{HandAddressType, PieceMeaning, PieceType};
use crate::cosmic::toy_box::PieceNum;
use crate::law::speed_of_light::SpeedOfLight;

/// 千日手の価値☆（＾～＾）
pub const REPITITION_VALUE: i16 = -300;

pub struct Evaluation {
    // 盤面をカバーする利きの多さの重み☆（＾～＾）1000分率☆（＾～＾）
    board_coverage_weight: i32,
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    komawari_weight: i32,
    /// 成りの重み☆（＾～＾）1000分率☆（＾～＾）
    promotion_weight: i32,
    // 駒割だぜ☆（＾～＾）
    piece_allocation_value: i16,
    /// 成り駒ボーナスだぜ☆（＾～＾）
    promotion_value: i16,
}
impl Evaluation {
    pub fn new(board_coverage_weight: i32, komawari_weight: i32, promotion_weight: i32) -> Self {
        Evaluation {
            board_coverage_weight: board_coverage_weight,
            komawari_weight: komawari_weight,
            promotion_weight: promotion_weight,
            piece_allocation_value: 0,
            promotion_value: 0,
        }
    }
    pub fn centi_pawn(&self, board_coverage_value: i16) -> i16 {
        self.komawari() + self.board_coverage(board_coverage_value) + self.promotion()
    }
    pub fn board_coverage(&self, board_coverage_value: i16) -> i16 {
        ((self.board_coverage_weight * board_coverage_value as i32) / 1000) as i16
    }
    pub fn komawari(&self) -> i16 {
        (self.komawari_weight * self.piece_allocation_value as i32 / 1000) as i16
    }
    pub fn promotion(&self) -> i16 {
        (self.promotion_weight * self.promotion_value as i32 / 1000) as i16
    }

    pub fn before_search(&mut self) {
        // ひっくり返すぜ☆（＾～＾）
        self.piece_allocation_value *= -1;
        self.promotion_value *= -1;
    }

    pub fn after_search(&mut self) {
        // ひっくり返すぜ☆（＾～＾）
        self.piece_allocation_value *= -1;
        self.promotion_value *= -1;
    }

    pub fn after_do_move(
        &mut self,
        source_piece: &Option<(PieceMeaning, PieceNum)>,
        captured_piece: &Option<(PieceMeaning, PieceNum)>,
        promotion: bool,
        speed_of_light: &SpeedOfLight,
    ) -> (i16, i16) {
        // 取った駒の価値を評価するぜ☆（＾～＾）
        let captured_piece_centi_pawn =
            Evaluation::from_caputured_piece(captured_piece, speed_of_light);
        self.piece_allocation_value += captured_piece_centi_pawn;

        // pvリストに１手入れてから評価しろだぜ☆（＾～＾）0除算エラーを防げるぜ☆（＾～＾）
        let delta_promotion_bonus = if let Some(source_piece_val) = source_piece {
            Evaluation::from_promotion(source_piece_val.0.r#type(&speed_of_light), promotion)
        } else {
            // 打なら成りは無いぜ☆（＾～＾）
            0
        };
        self.promotion_value += delta_promotion_bonus;

        (captured_piece_centi_pawn, delta_promotion_bonus)
    }

    pub fn before_undo_move(&mut self, captured_piece_centi_pawn: i16, delta_promotion_bonus: i16) {
        // 1手戻すぜ☆（＾～＾）
        self.piece_allocation_value -= captured_piece_centi_pawn;
        self.promotion_value -= delta_promotion_bonus;
    }

    /// 成ったら評価に加点するぜ☆（＾～＾）
    /// 駒得より 評価は下げた方が良さげ☆（＾～＾）
    pub fn from_promotion(source: PieceType, promotion: bool) -> i16 {
        if promotion {
            match source {
                PieceType::Bishop => 90,
                PieceType::Knight => 20,
                PieceType::Lance => 10,
                PieceType::Pawn => 50,
                PieceType::Rook => 100,
                PieceType::Silver => 40,
                _ => 0,
            }
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
    fn from_caputured_piece(
        captured_piece: &Option<(PieceMeaning, PieceNum)>,
        speed_of_light: &SpeedOfLight,
    ) -> i16 {
        if let Some(captured_piece_val) = captured_piece {
            match captured_piece_val
                .0
                .hand_address(speed_of_light)
                .r#type(speed_of_light)
            {
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                HandAddressType::King => 0,
                // 駒割は取ったときにカウントしているので、成りを考慮しないぜ☆（＾～＾）
                HandAddressType::Rook => 1000,
                HandAddressType::Bishop => 900,
                HandAddressType::Gold => 600,
                HandAddressType::Silver => 500,
                HandAddressType::Knight => 300,
                HandAddressType::Lance => 200,
                HandAddressType::Pawn => 100,
            }
        } else {
            0
        }
    }
}
