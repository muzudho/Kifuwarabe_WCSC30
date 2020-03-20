use crate::model::univ::gam::square::*;

/**
 * 升に数が書いている将棋盤
 */
pub struct NumberBoard {
    /**
     * 10の位を筋、1の位を段とする。
     * 0筋、0段は未使用
     */
    ban: [i8; BOARD_MEMORY_AREA],
}
impl Default for NumberBoard {
    fn default() -> NumberBoard {
        NumberBoard {
            // 盤上
            ban: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        }
    }
}
impl NumberBoard {
    pub fn clear(&mut self) {
        self.ban = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
    }
    pub fn add_count_by_square(&mut self, sq: &Square, su: i8) {
        self.ban[sq.to_usquare()] += su
    }
    pub fn get_number_by_square(&self, sq: &Square) -> i8 {
        self.ban[sq.to_usquare()]
    }
    pub fn set_number_by_square(&mut self, sq: &Square, su: i8) {
        self.ban[sq.to_usquare()] = su
    }
}
