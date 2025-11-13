/// core-engine のダミー実装
/// 将来的に feature/core-engine で実装される予定
#[derive(Debug, Clone)]
pub struct EditorEngine {
    /// エディタのID
    pub id: usize,
    /// ドキュメントの内容（ダミー）
    #[allow(dead_code)]
    pub content: String,
}

impl EditorEngine {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            content: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_content(id: usize, content: String) -> Self {
        Self { id, content }
    }
}
