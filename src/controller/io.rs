use crate::config::*;
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
    /// ログ・ファイルのミューテックス（排他制御）
    pub static ref LOGFILE: Mutex<File> = {
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        Mutex::new(File::create(Path::new(LOG_FILE_PATH)).unwrap())
    };
}

pub struct IO {}
impl IO {
    #[allow(dead_code)]
    pub fn log(s: &str) {
        if LOG_ENABLE {
            // write_allメソッドを使うには use std::io::Write; が必要
            if let Err(_why) = LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
                // 大会向けに、ログ書き込み失敗は出力しないことにする
                //panic!("couldn't write log. : {}",Error::description(&why)),
            }
        }
    }
    #[allow(dead_code)]
    pub fn write(s: &str) {
        println!("{}", s);
        IO::log(s)
    }
    #[allow(dead_code)]
    pub fn logln(s: &str) {
        if LOG_ENABLE {
            if let Err(_why) = LOGFILE
                .lock()
                .unwrap()
                .write_all(format!("{}\n", s).as_bytes())
            {}
        }
    }
    #[allow(dead_code)]
    pub fn writeln(s: &str) {
        println!("{}", s);
        IO::logln(s);
    }
}
