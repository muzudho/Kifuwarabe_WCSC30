//! 情報表示担当☆（＾～＾）
use crate::controller::io::*;
use crate::universe::game::movement::movement_builder::MovementBuilder;
use std::time::{Duration, Instant};

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
        cur_move: &MovementBuilder,
        text: &str,
    ) {
        // 初回か、前回より1秒以上経過していれば。
        if self.first || self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs() {
            // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
            IO::writeln(&format!(
                "info depth {} nodes {} score cp {} currmove {} string {}",
                cur_depth, sum_nodes, best_value, cur_move, text
            ));
            self.first = false;
            self.previous = self.stopwatch.elapsed();
        }
    }

    /// 必ず情報表示
    pub fn print_force_string(&mut self, text: &str) {
        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        IO::writeln(&format!("info string {}", text));
        self.first = false;
        self.previous = self.stopwatch.elapsed();
    }
}
