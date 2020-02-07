use super::super::super::model::vo::other_part::op_square_vo::*;

/**
 * 升に数が書いている将棋盤
 */
pub struct NumberBoard {
    /**
     * 10の位を筋、1の位を段とする。
     * 0筋、0段は未使用
     */
    ban: [i8; BAN_SIZE],
}
impl NumberBoard {
    pub fn new() -> NumberBoard {
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
    pub fn clear(&mut self) {
        self.ban = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
    }
    pub fn add_su_by_sq(&mut self, sq: &Square, su: i8) {
        self.ban[sq.to_umasu()] += su
    }
    pub fn get_su_by_sq(&self, sq: &Square) -> i8 {
        self.ban[sq.to_umasu()]
    }
    // #[allow(dead_code)]
    pub fn set_su_by_sq(&mut self, sq: &Square, su: i8) {
        self.ban[sq.to_umasu()] = su
    }
}
