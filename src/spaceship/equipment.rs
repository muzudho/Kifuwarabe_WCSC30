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

    pub fn is_printable(&self) -> bool {
        // 初回か、前回より1秒以上経過していれば。
        self.first || self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs()
    }
    /// 情報表示
    pub fn print(
        &mut self,
        cur_depth: u16,
        sum_nodes: u64,
        value: Option<i16>,
        lion_catch: Option<u16>,
        movement_hash: u64,
        text: &str,
    ) {
        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        IO::writeln(&format!(
            "info depth {} nodes {}{} currmove {} string {}",
            cur_depth,
            sum_nodes,
            if let Some(centi_pawn) = value {
                format!(" score cp {}", centi_pawn)
            } else if let Some(lion_catch_num) = lion_catch {
                let mate: i32 = if lion_catch_num % 2 == 0 {
                    // 偶数ならマイナスにするぜ☆（＾～＾）
                    -(lion_catch_num as i32)
                } else {
                    lion_catch_num as i32
                };
                format!(" score mate {}", mate)
            } else {
                "".to_string()
            },
            Movement::from_hash(movement_hash),
            text
        ));
        self.first = false;
        self.previous = self.stopwatch.elapsed();
    }
}
