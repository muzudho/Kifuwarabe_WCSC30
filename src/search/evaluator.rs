//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::search::PieceEx;
use crate::take1base::Move;

/// TODO 千日手の価値☆（＾～＾） ENGIN OPTIONにしたいぜ☆（＾～＾）
pub const REPITITION_VALUE: isize = -300;

pub struct Evaluation {
    /// 指し手がいっぱいあることを評価する重み☆（＾～＾）1000分率☆（＾～＾）
    many_ways_weight: isize,
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    material_advantage_weight: isize,
    /// 成りの重み☆（＾～＾）1000分率☆（＾～＾）
    promotion_weight: isize,
    /// 駒割だぜ☆（＾～＾）
    piece_allocation_value: isize,
    /// 成り駒ボーナスだぜ☆（＾～＾）
    promotion_value: isize,
    /// 指し手生成でその升に移動したら、先手なら＋１、後手なら－１しろだぜ☆（＾～＾）
    ways_value: isize,
}
impl Evaluation {
    pub fn new(
        many_ways_weight: isize,
        material_advantage_weight: isize,
        promotion_weight: isize,
    ) -> Self {
        Evaluation {
            many_ways_weight: many_ways_weight,
            material_advantage_weight: material_advantage_weight,
            promotion_weight: promotion_weight,
            piece_allocation_value: 0,
            promotion_value: 0,
            ways_value: 0,
        }
    }
    pub fn centi_pawn(&self) -> isize {
        self.move_list() + self.material_advantage() + self.promotion()
    }
    pub fn move_list(&self) -> isize {
        self.many_ways_weight * self.ways_value / 1000
    }
    pub fn material_advantage(&self) -> isize {
        self.material_advantage_weight * self.piece_allocation_value / 1000
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
        from_pc_ex: &Option<PieceEx>,
        captured_pc_ex: &Option<PieceEx>,
        promotion: bool,
    ) -> (isize, isize) {
        // 取った駒の価値を評価するぜ☆（＾～＾）
        let delta_captured_piece = Evaluation::caputured_piece_value(captured_pc_ex);
        self.piece_allocation_value += delta_captured_piece;

        // 成り駒を取って降格させたら、成り駒評価値追加だぜ☆（＾～＾）
        let delta_promotion = if let Some(captured_piece_val) = captured_pc_ex {
            if captured_piece_val.piece
                .type_()
                .promoted()
            {
                captured_piece_val.piece.hand_address().type_().promotion_value()
            } else {
                0
            }
        } else {
            0
        }
        // 進めた駒が成っても、評価値追加だぜ☆（＾～＾）
        +
        if let Some(source_piece_val) = from_pc_ex {
            if promotion {
                source_piece_val.piece.hand_address().type_().promotion_value()
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
    fn caputured_piece_value(captured_pc_ex: &Option<PieceEx>) -> isize {
        if let Some(captured_piece_val) = captured_pc_ex {
            captured_piece_val
                .piece
                .hand_address()
                .type_()
                .captured_value()
        } else {
            0
        }
    }

    pub fn add_control(&mut self, sign: isize, move_list: &Vec<Move>) {
        // 駒を動かせたんなら、利きが広いと考えるぜ☆（＾～＾）
        self.ways_value += sign * move_list.len() as isize;
    }
}