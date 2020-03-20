//!
//! Data transfer object.
//!
extern crate rand;

use crate::controller::io::*;
use crate::model::univ::gam::misc::piece_direction::PieceDirection;
use crate::model::univ::gam::misc::piece_movement::*;
use crate::model::univ::gam::misc::piece_type::PieceType;
use crate::model::univ::gam::misc::piece_type::*;
use crate::model::univ::game::*;
use crate::model::univ::speed_of_light::*;

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
    pub fn is_empty_command(&mut self) -> bool {
        self.vec_command.is_empty()
    }
    pub fn push_command(&mut self, line: &str) {
        self.vec_command.push(format!("{}\n", line));
    }
    pub fn pop_command(&mut self) -> String {
        self.vec_command.pop().unwrap()
    }

    /* ******
     * 盤上 *
     ********/

    // 駒の動きを出力
    pub fn print_kmugoki(&self, speed_of_light: &MLSpeedOfLightVo) {
        for piece_type in PIECE_TYPE_ARRAY.iter() {
            IO::write(&format!("{} ", piece_type));
            self.print_kmugoki_dir(*piece_type, speed_of_light);
            IO::writeln(""); //改行
        }
    }
    pub fn print_kmugoki_dir(&self, piece_type: PieceType, speed_of_light: &MLSpeedOfLightVo) {
        for kmdir in KM_UGOKI.back[speed_of_light
            .get_piece_type_struct_from_piece_type(&piece_type)
            .serial_piece_number]
            .iter()
        {
            match *kmdir {
                PieceDirection::Owari => break,
                _ => IO::write(&format!("{},", kmdir)),
            }
        }
    }
}
