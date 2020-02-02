//! 対話部

pub struct DialoguePart {
    /// 対話モード
    pub dialogue_mode: bool,
    /// コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
}
impl DialoguePart {
    pub fn new() -> Self {
        DialoguePart {
            /// 偽ならUSIプロトコル通信。
            dialogue_mode: false,
            vec_command: Vec::new(),
        }
    }

    /// コマンド・バッファー
    pub fn is_empty_command(&mut self) -> bool {
        self.vec_command.len() == 0
    }
    pub fn push_command(&mut self, line: &String) {
        self.vec_command.push(format!("{}\n", line));
    }
    pub fn pop_command(&mut self) -> String {
        self.vec_command.pop().unwrap()
    }
}
