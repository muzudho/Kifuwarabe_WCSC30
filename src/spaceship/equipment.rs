//! 宇宙船の備品だぜ☆（＾～＾）
use crate::cosmic::shogi::recording::Movement;
use crate::cosmic::smart::square::test_rotation;
use crate::white_hole::io::*;
use std::time::{Duration, Instant};

/// テストをここに詰め込んであるぜ☆（＾～＾）
pub struct Telescope {}
impl Telescope {
    pub fn look() {
        test_rotation();
    }
}

/// 情報表示担当☆（＾～＾）
pub struct Info {
    /// 情報用のストップウォッチ
    stopwatch: Instant,
    previous: Duration,
    first: bool,
}
impl Default for Info {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        Info {
            stopwatch: stopwatch1,
            previous: stopwatch1.elapsed(),
            first: true,
        }
    }
}
impl Info {
    /// ストップウォッチを初期化します。
    pub fn clear(&mut self) {
        self.stopwatch = Instant::now();
        self.previous = self.stopwatch.elapsed();
        self.first = true;
    }

    /// 情報表示
    pub fn print(
        &mut self,
        cur_depth: u16,
        sum_nodes: u64,
        best_value: i16,
        cur_move: &Movement,
        text: &str,
        forcely: bool,
    ) {
        // 初回か、前回より1秒以上経過していれば。
        if forcely || self.first || self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs()
        {
            // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
            IO::writeln(&format!(
                "info depth {} nodes {} score cp {} currmove {} string {}",
                cur_depth, sum_nodes, best_value, cur_move, text
            ));
            self.first = false;
            self.previous = self.stopwatch.elapsed();
        }
    }
}
