//! 情報表示担当☆（＾～＾）
use crate::model::dto::main_loop::ml_movement_dto::MLMovementDto;
use crate::model::dto::main_loop::ml_universe_dto::g_writeln;
use std::time::{Duration, Instant};

pub struct SPInfo {
    /// 情報用のストップウォッチ
    stopwatch: Instant,
    previous: Duration,
    first: bool,
}
impl Default for SPInfo {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        SPInfo {
            stopwatch: stopwatch1,
            previous: stopwatch1.elapsed(),
            first: true,
        }
    }
}
impl SPInfo {
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
        resign_move: &MLMovementDto,
    ) {
        // 初回か、前回より1秒以上経過していれば。
        if self.first || self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs() {
            // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
            g_writeln(&format!(
                "info depth {} nodes {} score cp {} currmove {}",
                cur_depth, sum_nodes, best_value, resign_move
            ));
            self.first = false;
            self.previous = self.stopwatch.elapsed();
        }
    }
}
