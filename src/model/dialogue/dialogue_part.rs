pub struct DialoguePart {
    /// 対話モード
    pub dialogue_mode: bool,
}
impl DialoguePart {
    pub fn new() -> Self {
        DialoguePart {
            dialogue_mode: false,
        }
    }
}
