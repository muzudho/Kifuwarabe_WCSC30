//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
use super::piece_struct_master::PieceStructMaster;

pub struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    pub piece_struct_master: PieceStructMaster,
}
impl SpeedOfLight {
    pub fn new() -> Self {
        SpeedOfLight {
            piece_struct_master: PieceStructMaster::new(),
        }
    }
}
