//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
use super::piece_vo_master::PieceVoMaster;

pub struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    pub piece_vo_master: PieceVoMaster,
}
impl SpeedOfLight {
    pub fn new() -> Self {
        SpeedOfLight {
            piece_vo_master: PieceVoMaster::new(),
        }
    }
}
