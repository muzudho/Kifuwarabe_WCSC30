//! 探索部

pub struct SearchPart {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    ply: i16,
}
impl SearchPart {
    pub fn new() -> Self {
        SearchPart { ply: 0 }
    }
    pub fn add_ply(&mut self, ply1: i16) {
        self.ply += ply1
    }
    pub fn set_ply(&mut self, ply1: i16) {
        self.ply = ply1
    }
    pub fn get_ply(&self) -> i16 {
        self.ply
    }
}
