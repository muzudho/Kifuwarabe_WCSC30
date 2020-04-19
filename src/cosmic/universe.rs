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
    /// 読みの最大深さ
    pub option_max_depth: u16,
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            option_max_depth: 1,
        }
    }
}
impl Universe {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.game.big_bang();
    }
}
