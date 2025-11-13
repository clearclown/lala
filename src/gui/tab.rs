use crate::core::EditorEngine;

/// エディタタブの状態を管理する構造体
#[derive(Debug, Clone)]
pub struct EditorTabState {
    /// タブのID
    pub id: usize,
    /// ファイル名
    pub file_name: String,
    /// 変更されているかどうか（未保存フラグ）
    pub is_modified: bool,
    /// core-engine のインスタンス
    pub engine: EditorEngine,
}

impl EditorTabState {
    /// 新しいタブを作成する
    pub fn new(id: usize, file_name: impl Into<String>) -> Self {
        let file_name = file_name.into();
        Self {
            id,
            file_name: file_name.clone(),
            is_modified: false,
            engine: EditorEngine::new(id),
        }
    }

    /// タブのタイトルを取得する（未保存の場合は「*」を付ける）
    pub fn title(&self) -> String {
        if self.is_modified {
            format!("{}*", self.file_name)
        } else {
            self.file_name.clone()
        }
    }

    /// タブを変更済みとしてマークする
    #[allow(dead_code)]
    pub fn mark_modified(&mut self) {
        self.is_modified = true;
    }

    /// タブを未変更としてマークする
    #[allow(dead_code)]
    pub fn mark_saved(&mut self) {
        self.is_modified = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tab() {
        let tab = EditorTabState::new(0, "test.md");
        assert_eq!(tab.id, 0);
        assert_eq!(tab.file_name, "test.md");
        assert!(!tab.is_modified);
    }

    #[test]
    fn test_title_without_modification() {
        let tab = EditorTabState::new(0, "test.md");
        assert_eq!(tab.title(), "test.md");
    }

    #[test]
    fn test_title_with_modification() {
        let mut tab = EditorTabState::new(0, "test.md");
        tab.mark_modified();
        assert_eq!(tab.title(), "test.md*");
    }

    #[test]
    fn test_mark_modified_and_saved() {
        let mut tab = EditorTabState::new(0, "test.md");
        assert!(!tab.is_modified);

        tab.mark_modified();
        assert!(tab.is_modified);

        tab.mark_saved();
        assert!(!tab.is_modified);
    }
}
