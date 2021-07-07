use crate::search::CentiPawn;
use crate::search::MoveEx;
use crate::search::Reason;
use crate::search::RESIGN_MOVE;
use crate::take1base::Move;

impl Default for MoveEx {
    fn default() -> Self {
        MoveEx {
            value: i16::MIN,
            move_: RESIGN_MOVE,
            // なんの手も無かったぜ☆（＾～＾）
            reason: Reason::NoUpdate,
        }
    }
}
impl MoveEx {
    /// TODO 廃止予定☆（＾～＾）
    pub fn catch_king(&mut self, move_: Move) {
        // 玉を取る手より強い手はないぜ☆（＾～＾）！
        self.move_ = move_;
        self.value = i16::MAX;
        self.reason = Reason::KingCatchIsStrongest;
    }
    pub fn update(&mut self, move_: Move, value: CentiPawn, reason: Reason) {
        self.move_ = move_;
        self.value = value;
        self.reason = reason;
    }
}
