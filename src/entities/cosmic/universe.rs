//!
//! Data transfer object.
//!
extern crate rand;

use crate::entities::cosmic::playing::Game;

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Universe {
    pub game: Game,
    /// 対話モード
    pub dialogue_mode: bool,
    /// 読みの最大深さ。
    pub option_max_depth: usize,
    // /// 思考時間の最小秒☆（＾～＾）
    // pub option_min_think_sec: usize,
    // /// 思考時間の最大秒☆（＾～＾）
    // pub option_max_think_sec: usize,
    /// 諦めない深さ☆（＾～＾）読み終わるまで、思考時間を無視するぜ☆（＾～＾）max_depth - 1 より小さくしろだぜ☆（＾～＾）
    pub option_depth_not_to_give_up: usize,
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            option_max_depth: 1,
            option_depth_not_to_give_up: 2,
            // /// min < max にしろだぜ☆（＾～＾）
            // option_min_think_sec: 7,
            // option_max_think_sec: 17,
        }
    }
}
impl Universe {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.game.big_bang();
    }
}
