//! 宇宙船の備品だぜ☆（＾～＾）
use crate::entities::logging::{LOGFILE, LOG_ENABLED};
use crate::position::rotation::test_rotation;
use std::io::Write;
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
    pub stopwatch: Instant,
    pub previous: Duration,
    pub first: bool,
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

    /// 表示していいタイミングか？
    pub fn is_printable(&self) -> bool {
        // 初回であれば、3秒経過してから
        (self.first && self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs()) ||
        // そうでなければ、前回より1秒以上経過していれば。
        self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs()
    }
}

pub struct Log {}
impl Log {
    #[allow(dead_code)]
    pub fn write(s: &str) {
        if *LOG_ENABLED.lock().unwrap() {
            // write_allメソッドを使うには use std::io::Write; が必要
            if let Err(_why) = LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
                // 大会向けに、ログ書き込み失敗は出力しないことにする
                // panic!("(Err.148) couldn't write log. : {}",Error::description(&why)),
            }
        }
    }
    #[allow(dead_code)]
    pub fn writeln(s: &str) -> &str {
        if *LOG_ENABLED.lock().unwrap() {
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
