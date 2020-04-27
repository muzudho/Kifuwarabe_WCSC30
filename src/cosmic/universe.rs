//!
//! Data transfer object.
//!
extern crate rand;

use crate::cosmic::playing::Game;

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Universe {
    pub game: Game,
    /// 対話モード
    pub dialogue_mode: bool,
    /// 読みの最大深さ。最大で 255 だぜ☆（＾～＾） そんな要らんだろ☆（＾～＾）
    pub option_max_depth: u8,
    /// 思考時間の最小秒☆（＾～＾）
    pub option_min_think_sec: u64,
    /// 思考時間の最大秒☆（＾～＾）
    pub option_max_think_sec: u64,
    /// 盤面をカバーする利きの多さ☆（＾～＾）1000分率☆（＾～＾）
    pub option_board_coverage_weight: i32,
    /// 諦めない深さ☆（＾～＾）読み終わるまで、思考時間を無視するぜ☆（＾～＾）max_depth - 1 より小さくしろだぜ☆（＾～＾）
    pub option_depth_not_to_give_up: u8,
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    pub option_komawari_weight: i32,
    /// 成りの重み☆（＾～＾）1000分率☆（＾～＾）
    pub option_promotion_weight: i32,
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            option_max_depth: 1,
            option_depth_not_to_give_up: 2,
            /// min < max にしろだぜ☆（＾～＾）
            option_min_think_sec: 7,
            option_max_think_sec: 17,
            option_board_coverage_weight: 1000,
            option_komawari_weight: 1000,
            option_promotion_weight: 1000,
        }
    }
}
impl Universe {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.game.big_bang();
    }
}
