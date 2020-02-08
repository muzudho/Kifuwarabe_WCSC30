//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
use super::super::main_loop::ml_piece_struct_master_vo::MLPieceStructMasterVo;

pub struct MLSpeedOfLightVo {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    pub ml_piece_struct_master_vo: MLPieceStructMasterVo,
}
impl Default for MLSpeedOfLightVo {
    fn default() -> Self {
        MLSpeedOfLightVo {
            ml_piece_struct_master_vo: MLPieceStructMasterVo::default(),
        }
    }
}
