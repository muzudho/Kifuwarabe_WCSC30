pub struct DialoguePart {
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
}
impl DialoguePart {
    pub fn new() -> Self {
        DialoguePart {
            dialogue_mode: false,
            vec_command: Vec::new(),
        }
    }
}
