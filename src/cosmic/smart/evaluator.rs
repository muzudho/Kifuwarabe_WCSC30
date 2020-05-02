//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::playing::Game;
use crate::law::generate_move::Piece;
use crate::law::generate_move::Ways;

/// TODO 千日手の価値☆（＾～＾） ENGIN OPTIONにしたいぜ☆（＾～＾）
pub const REPITITION_VALUE: isize = -300;

pub struct Evaluation {
    // 指し手がいっぱいあることを評価する重み☆（＾～＾）1000分率☆（＾～＾）
    many_ways_weight: isize,
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    komawari_weight: isize,
    /// 成りの重み☆（＾～＾）1000分率☆（＾～＾）
    promotion_weight: isize,
    // 駒割だぜ☆（＾～＾）
    piece_allocation_value: isize,
    /// 成り駒ボーナスだぜ☆（＾～＾）
    promotion_value: isize,
    /// 指し手生成でその升に移動したら、先手なら＋１、後手なら－１しろだぜ☆（＾～＾）
    ways_value: isize,
}
impl Evaluation {
    pub fn new(many_ways_weight: isize, komawari_weight: isize, promotion_weight: isize) -> Self {
        Evaluation {
            many_ways_weight: many_ways_weight,
            komawari_weight: komawari_weight,
            promotion_weight: promotion_weight,
            piece_allocation_value: 0,
            promotion_value: 0,
            ways_value: 0,
        }
    }
    pub fn centi_pawn(&self) -> isize {
        self.ways() + self.komawari() + self.promotion()
    }
    pub fn ways(&self) -> isize {
        self.many_ways_weight * self.ways_value / 1000
    }
    pub fn komawari(&self) -> isize {
        self.komawari_weight * self.piece_allocation_value / 1000
    }
    pub fn promotion(&self) -> isize {
        self.promotion_weight * self.promotion_value / 1000
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
        source_piece: &Option<Piece>,
        captured_piece: &Option<Piece>,
        promotion: bool,
    ) -> (isize, isize) {
        // 取った駒の価値を評価するぜ☆（＾～＾）
        let delta_captured_piece = Evaluation::caputured_piece_value(captured_piece);
        self.piece_allocation_value += delta_captured_piece;

        // 成り駒を取って降格させたら、成り駒評価値追加だぜ☆（＾～＾）
        let delta_promotion = if let Some(captured_piece_val) = captured_piece {
            if captured_piece_val.meaning
                .r#type()
                .promoted()
            {
                captured_piece_val.meaning.hand_address().r#type().promotion_value()
            } else {
                0
            }
        } else {
            0
        }
        // 進めた駒が成っても、評価値追加だぜ☆（＾～＾）
        +
        if let Some(source_piece_val) = source_piece {
            if promotion {
                source_piece_val.meaning.hand_address().r#type().promotion_value()
            } else {
                0
            }
        } else {
            // 打なら成りは無いぜ☆（＾～＾）
            0
        };
        self.promotion_value += delta_promotion;

        (delta_captured_piece, delta_promotion)
    }

    pub fn before_undo_move(&mut self, delta_captured_piece: isize, delta_promotion: isize) {
        // 1手戻すぜ☆（＾～＾）
        self.piece_allocation_value -= delta_captured_piece;
        self.promotion_value -= delta_promotion;
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないのは、駒割ではなく、別の方法で対応してくれだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// Centi pawn.
    fn caputured_piece_value(captured_piece: &Option<Piece>) -> isize {
        if let Some(captured_piece_val) = captured_piece {
            captured_piece_val
                .meaning
                .hand_address()
                .r#type()
                .captured_value()
        } else {
            0
        }
    }

    pub fn add_control(&mut self, sign: isize, ways: &Ways) {
        // 駒を動かせたんなら、利きが広いと考えるぜ☆（＾～＾）
        self.ways_value += sign * ways.len() as isize;
    }
}
