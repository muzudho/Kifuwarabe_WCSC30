//! 宇宙船の備品だぜ☆（＾～＾）
use crate::config::*;
use crate::cosmic::daydream::Value;
use crate::cosmic::recording::Movement;
use crate::cosmic::smart::square::test_rotation;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// ちゆり「望遠鏡だぜ☆」
/// 夢見　「何も見えないんだけど？」
/// ちゆり「そうか、残念だな……☆」
pub struct Telescope {}
impl Telescope {
    pub fn look() {
        test_rotation();
    }
}

/// PV表示、または 文字列表示だぜ☆（＾～＾）
pub enum PvString {
    /// 思考を開始してからのミリ秒と、読み筋。
    PV(u128, String),
    String(String),
}

/// 行き先表示案内板だぜ☆（＾～＾）
/// 読み筋とか表示されてるぜ☆（＾～＾）
pub struct DestinationDisplay {
    /// 情報用のストップウォッチ
    stopwatch: Instant,
    previous: Duration,
    first: bool,
}
impl Default for DestinationDisplay {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        DestinationDisplay {
            stopwatch: stopwatch1,
            previous: stopwatch1.elapsed(),
            first: true,
        }
    }
}
impl DestinationDisplay {
    /// ストップウォッチを初期化します。
    pub fn clear(&mut self) {
        self.stopwatch = Instant::now();
        self.previous = self.stopwatch.elapsed();
        self.first = true;
    }

    pub fn is_printable(&self) -> bool {
        // 初回か、前回より1秒以上経過していれば。
        self.first || self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs()
    }
    /// 情報表示
    pub fn print(
        &mut self,
        cur_depth: Option<usize>,
        state_nodes_nps: Option<(u64, u64)>,
        value: Option<Value>,
        movement: Option<Movement>,
        pv_string: &Option<PvString>,
    ) {
        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        Beam::shoot(&format!(
            "info{}{}{}{} currmove {}{}",
            // 思考を開始してからのミリ秒☆（＾～＾）
            if let Some(pv_string_val) = pv_string {
                match pv_string_val {
                    PvString::PV(msec, _pv) => format!(" time {}", msec),
                    PvString::String(_x) => "".to_string(),
                }
            } else {
                "".to_string()
            },
            if let Some(num) = cur_depth {
                // 単に読み筋の長さ☆（＾～＾）
                format!(" depth {}", num)
            } else {
                "".to_string()
            },
            if let Some((state_node, nps)) = state_nodes_nps {
                format!(" nodes {} nps {}", state_node, nps)
            } else {
                "".to_string()
            },
            //if let Some(centi_pawn) = value {
            if let Some(value_val) = value {
                match value_val {
                    Value::Win => {
                        // 自分が勝つ
                        " score mate +".to_string()
                    }
                    Value::Lose => {
                        // 自分が負ける
                        " score mate -".to_string()
                    }
                    Value::CentiPawn(num) => format!(" score cp {}", num),
                }
            } else {
                "".to_string()
            },
            if let Some(movement_val) = movement {
                format!("{}", movement_val)
            } else {
                "".to_string()
            },
            if let Some(pv_string_val) = pv_string {
                match pv_string_val {
                    PvString::PV(_sec, pv) => format!(" pv {}", pv),
                    PvString::String(x) => format!(" string {}", x),
                }
            } else {
                "".to_string()
            }
        ));
        self.first = false;
        self.previous = self.stopwatch.elapsed();
    }
}

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

pub struct Log {}
impl Log {
    #[allow(dead_code)]
    pub fn write(s: &str) {
        if LOG_ENABLE {
            // write_allメソッドを使うには use std::io::Write; が必要
            if let Err(_why) = LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
                // 大会向けに、ログ書き込み失敗は出力しないことにする
                // panic!("(Err.148) couldn't write log. : {}",Error::description(&why)),
            }
        }
    }
    #[allow(dead_code)]
    pub fn writeln(s: &str) -> &str {
        if LOG_ENABLE {
            if let Err(_why) = LOGFILE
                .lock()
                .unwrap()
                .write_all(format!("{}\n", s).as_bytes())
            {}
        }
        s
    }

    #[allow(dead_code)]
    pub fn graffiti(s: &str) {
        Log::writeln(&format!("Debug   | {}", s));
    }
}

pub struct Beam {}
impl Beam {
    #[allow(dead_code)]
    pub fn shot(s: &str) {
        println!("{}", s);
        Log::write(s)
    }
    #[allow(dead_code)]
    pub fn shoot(s: &str) {
        println!("{}", s);
        Log::writeln(s);
    }

    /// panic! で強制終了する前に、ヤケクソで読み筋欄に表示できないかトライするぜ☆（＾～＾）
    #[allow(dead_code)]
    pub fn trouble(s: &str) -> String {
        let s2 = Log::writeln(&format!("info string panic! {}", s)).to_string();
        println!("{}", s2);
        s2
    }
}
