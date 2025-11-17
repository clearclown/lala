use super::tab::EditorTabState;

/// アプリケーション全体の状態を管理する構造体
#[derive(Debug)]
pub struct AppState {
    /// 開いているタブのリスト
    tabs: Vec<EditorTabState>,
    /// 現在アクティブなタブのインデックス
    active_tab_index: Option<usize>,
    /// 次に割り当てるタブID
    next_tab_id: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// 新しい AppState を作成する
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_index: None,
            next_tab_id: 0,
        }
    }

    /// 新しいタブを開く
    ///
    /// # Arguments
    /// * `file_name` - ファイル名
    ///
    /// # Returns
    /// 新しく作成されたタブのID
    pub fn open_new_tab(&mut self, file_name: impl Into<String>) -> usize {
        let id = self.next_tab_id;
        self.next_tab_id += 1;

        let tab = EditorTabState::new(id, file_name);
        self.tabs.push(tab);

        // 最初のタブの場合は自動的にアクティブにする
        if self.active_tab_index.is_none() {
            self.active_tab_index = Some(0);
        }

        id
    }

    /// タブを閉じる
    ///
    /// # Arguments
    /// * `index` - 閉じるタブのインデックス
    ///
    /// # Returns
    /// タブが閉じられた場合は true、インデックスが無効な場合は false
    pub fn close_tab(&mut self, index: usize) -> bool {
        if index >= self.tabs.len() {
            return false;
        }

        self.tabs.remove(index);

        // アクティブなタブのインデックスを調整
        if let Some(active_idx) = self.active_tab_index {
            if self.tabs.is_empty() {
                self.active_tab_index = None;
            } else if active_idx >= self.tabs.len() {
                self.active_tab_index = Some(self.tabs.len() - 1);
            } else if active_idx == index && self.tabs.len() > index {
                // 閉じたタブがアクティブだった場合、同じインデックスの新しいタブをアクティブに
                self.active_tab_index = Some(index);
            } else if active_idx > index {
                // 閉じたタブがアクティブなタブより前にあった場合、インデックスを調整
                self.active_tab_index = Some(active_idx - 1);
            }
        }

        true
    }

    /// 現在アクティブなタブへの参照を取得する
    pub fn active_tab(&self) -> Option<&EditorTabState> {
        self.active_tab_index.and_then(|idx| self.tabs.get(idx))
    }

    /// 現在アクティブなタブへの可変参照を取得する
    #[allow(dead_code)]
    pub fn active_tab_mut(&mut self) -> Option<&mut EditorTabState> {
        self.active_tab_index.and_then(|idx| self.tabs.get_mut(idx))
    }

    /// アクティブなタブを設定する
    ///
    /// # Arguments
    /// * `index` - アクティブにするタブのインデックス
    ///
    /// # Returns
    /// インデックスが有効な場合は true、無効な場合は false
    pub fn set_active_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_tab_index = Some(index);
            true
        } else {
            false
        }
    }

    /// すべてのタブへの参照を取得する
    pub fn tabs(&self) -> &[EditorTabState] {
        &self.tabs
    }

    /// 現在アクティブなタブのインデックスを取得する
    pub fn active_tab_index(&self) -> Option<usize> {
        self.active_tab_index
    }

    /// タブの数を取得する
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// タブが空かどうかを確認する
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app_state() {
        let state = AppState::new();
        assert_eq!(state.tab_count(), 0);
        assert!(state.is_empty());
        assert!(state.active_tab_index().is_none());
    }

    #[test]
    fn test_open_new_tab() {
        let mut state = AppState::new();

        let id1 = state.open_new_tab("a.md");
        assert_eq!(id1, 0);
        assert_eq!(state.tab_count(), 1);
        assert_eq!(state.active_tab_index(), Some(0));

        let id2 = state.open_new_tab("b.md");
        assert_eq!(id2, 1);
        assert_eq!(state.tab_count(), 2);

        let tabs = state.tabs();
        assert_eq!(tabs[0].file_name, "a.md");
        assert_eq!(tabs[1].file_name, "b.md");
    }

    #[test]
    fn test_close_tab() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");
        state.open_new_tab("b.md");
        state.open_new_tab("c.md");

        assert_eq!(state.tab_count(), 3);

        // 真ん中のタブを閉じる
        assert!(state.close_tab(1));
        assert_eq!(state.tab_count(), 2);

        let tabs = state.tabs();
        assert_eq!(tabs[0].file_name, "a.md");
        assert_eq!(tabs[1].file_name, "c.md");
    }

    #[test]
    fn test_close_invalid_tab() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");

        // 存在しないインデックスを指定
        assert!(!state.close_tab(10));
        assert_eq!(state.tab_count(), 1);
    }

    #[test]
    fn test_close_last_tab() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");

        assert!(state.close_tab(0));
        assert_eq!(state.tab_count(), 0);
        assert!(state.is_empty());
        assert!(state.active_tab_index().is_none());
    }

    #[test]
    fn test_active_tab() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");
        state.open_new_tab("b.md");

        let active = state.active_tab();
        assert!(active.is_some());
        assert_eq!(active.unwrap().file_name, "a.md");
    }

    #[test]
    fn test_active_tab_mut() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");

        if let Some(tab) = state.active_tab_mut() {
            tab.mark_modified();
        }

        let active = state.active_tab();
        assert!(active.unwrap().is_modified);
    }

    #[test]
    fn test_set_active_tab() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");
        state.open_new_tab("b.md");
        state.open_new_tab("c.md");

        assert!(state.set_active_tab(2));
        assert_eq!(state.active_tab_index(), Some(2));
        assert_eq!(state.active_tab().unwrap().file_name, "c.md");

        // 無効なインデックス
        assert!(!state.set_active_tab(10));
        assert_eq!(state.active_tab_index(), Some(2)); // 変更されない
    }

    #[test]
    fn test_close_active_tab() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");
        state.open_new_tab("b.md");
        state.open_new_tab("c.md");

        state.set_active_tab(1);
        assert_eq!(state.active_tab().unwrap().file_name, "b.md");

        // アクティブなタブを閉じる
        state.close_tab(1);
        assert_eq!(state.tab_count(), 2);

        // アクティブなタブは調整される
        let active = state.active_tab();
        assert!(active.is_some());
    }
}
