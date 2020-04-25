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
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            option_max_depth: 1,
            option_min_think_sec: 1,
            option_max_think_sec: 1,
        }
    }
}
impl Universe {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.game.big_bang();
    }
}
