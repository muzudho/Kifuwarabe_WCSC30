use crate::search::Reason;
use crate::search::Value;
use crate::search::RESIGN_MOVE;
use crate::take1base::Move;

#[derive(Clone)]
pub struct Bestmove {
    pub value: Value,
    pub move_: Move,
    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: Reason,
}
impl Default for Bestmove {
    fn default() -> Self {
        Bestmove {
            value: Value::Lose,
            move_: RESIGN_MOVE,
            // なんの手も無かったぜ☆（＾～＾）
            reason: Reason::NoUpdate,
        }
    }
}
impl Bestmove {
    /// TODO 廃止予定☆（＾～＾）
    pub fn catch_king(&mut self, move_: Move) {
        // 玉を取る手より強い手はないぜ☆（＾～＾）！
        self.move_ = move_;
        self.value = Value::Win;
        self.reason = Reason::KingCatchIsStrongest;
    }
    pub fn update(&mut self, move_: Move, value: Value, reason: Reason) {
        self.move_ = move_;
        self.value = value;
        self.reason = reason;
    }
}
