//!
//! Data transfer object.
//!
extern crate rand;

use crate::cosmic::game::game::Game;

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Universe {
    pub game: Game,
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
    /// 読みの最大深さ
    pub option_max_depth: u16,
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            vec_command: Vec::new(),
            option_max_depth: 1,
        }
    }
}
impl Universe {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.game.big_bang();
    }

    /* **********************
     * コマンド・バッファー *
     ************************/
    pub fn is_empty_command(&self) -> bool {
        self.vec_command.is_empty()
    }

    /*
    pub fn push_command(&mut self, line: &str) {
        self.vec_command.push(format!("{}\n", line));
    }
    */
    pub fn pop_command(&mut self) -> String {
        self.vec_command.pop().unwrap()
    }
}
