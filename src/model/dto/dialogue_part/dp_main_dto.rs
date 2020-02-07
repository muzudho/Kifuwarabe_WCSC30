pub struct DPMainDto {
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
}
impl DPMainDto {
    pub fn new() -> Self {
        DPMainDto {
            dialogue_mode: false,
            vec_command: Vec::new(),
        }
    }
}
