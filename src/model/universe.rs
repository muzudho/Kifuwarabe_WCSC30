//!
//! Data transfer object.
//!
extern crate rand;

use crate::controller::io::*;
use crate::model::univ::gam::misc::movement::*;
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
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Game::default(),
            dialogue_mode: false,
            vec_command: Vec::new(),
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

    // 入れた指し手の通り指すぜ☆（＾～＾）
    pub fn do_move(&mut self, movement: &Movement, speed_of_light: &MLSpeedOfLightVo) {
        // もう入っているかも知れないが、棋譜に入れる☆
        let ply = self.game.history.ply;
        self.game.set_current_movement(movement);
        let cap;
        {
            cap = self.game.do_move(movement, speed_of_light);
        }
        self.game.set_cap(ply as usize, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.game.create_ky1_hash(speed_of_light);
        self.game.set_current_position_hash(ky_hash);

        self.game.history.ply += 1;
    }

    pub fn undo_move(&mut self, speed_of_light: &MLSpeedOfLightVo) -> bool {
        if 0 < self.game.history.ply {
            // 棋譜から読取、手目も減る
            self.game.history.ply -= 1;
            // let phase = self.sp_earth_dto.get_phase(&Person::Friend);
            let ss = &self.game.get_move().clone();
            self.game.undo_move(/*&phase,*/ ss, speed_of_light);
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}
