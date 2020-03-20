//!
//! Data transfer object.
//!
extern crate rand;

use crate::config::*;
use crate::model::univ::gam::movement::*;
use crate::model::univ::gam::piece_type::PieceType;
use crate::model::univ::gam::piece_type::*;
use crate::model::univ::game::*;
use crate::model::vo::main_loop::ml_speed_of_light_vo::*;
use crate::model::vo::other_part::op_piece_direction_vo::PieceDirection;
use crate::model::vo::other_part::op_piece_movement_vo::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

// グローバル定数
//
// 使い方（lazy_static!マクロ）
// ============================
// 定数の値を実行時に決めることができる。
//
// Cargo.toml に１行追記
// > [dependencies]
// > lazy_static = "1.0.0"
//
// main.rs の冒頭あたりに次の２行を記述
// > #[macro_use]
// > extern crate lazy_static;
//
// 「How can I use mutable lazy_static?」
// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
lazy_static! {
    /**
      * ログ・ファイル
      */
    pub static ref LOGFILE: Mutex<File> = {
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        Mutex::new(File::create(Path::new(LOG_FILE_PATH)).unwrap())
    };
}

#[allow(dead_code)]
pub fn g_log(s: &str) {
    if LOG_ENABLE {
        // write_allメソッドを使うには use std::io::Write; が必要
        if let Err(_why) = LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            //panic!("couldn't write log. : {}",Error::description(&why)),
        }
    }
}
#[allow(dead_code)]
pub fn g_write(s: &str) {
    println!("{}", s);
    g_log(s)
}
#[allow(dead_code)]
pub fn g_logln(s: &str) {
    if LOG_ENABLE {
        if let Err(_why) = LOGFILE
            .lock()
            .unwrap()
            .write_all(format!("{}\n", s).as_bytes())
        {}
    }
}
#[allow(dead_code)]
pub fn g_writeln(s: &str) {
    println!("{}", s);
    g_logln(s);
}

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
            g_write(&format!("{} ", piece_type));
            self.print_kmugoki_dir(*piece_type, speed_of_light);
            g_writeln(""); //改行
        }
    }
    pub fn print_kmugoki_dir(&self, piece_type: PieceType, speed_of_light: &MLSpeedOfLightVo) {
        for kmdir in KM_UGOKI.back[speed_of_light
            .get_piece_type_struct_vo_from_piece_type(&piece_type)
            .serial_piece_number]
            .iter()
        {
            match *kmdir {
                PieceDirection::Owari => break,
                _ => g_write(&format!("{},", kmdir)),
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
