//! # GUI モジュール
//!
//! このモジュールは、egui/eframe を使用したテキストエディタのGUI基盤を提供します。
//!
//! ## 全体構成
//!
//! - `AppState`: アプリケーション全体の状態を管理し、タブのリストを保持します
//! - `EditorTabState`: 個々のタブの状態（ファイル名、変更フラグ、core-engineインスタンス）を管理します
//! - `EditorApp`: eframe::App を実装し、GUI の描画とイベント処理を行います
//!
//! ## レイアウト構成
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │ Top Menu Bar (将来用)                │
//! ├────┬────────────────────────────────┤
//! │    │ Tab Bar                        │
//! │ S  ├────────────────────────────────┤
//! │ i  │                                │
//! │ d  │ Main Editor Area               │
//! │ e  │                                │
//! │ b  │                                │
//! │ a  │                                │
//! │ r  │                                │
//! │    │                                │
//! │ (  ├────────────────────────────────┤
//! │ 将  │ Status Bar (将来用)             │
//! │ 来  └────────────────────────────────┘
//! │ 用)
//! └────┘
//! ```
//!
//! ## 拡張性
//!
//! 他のフィーチャー（file-tree, basic-editingなど）がこのGUI骨格を拡張できるように、
//! EditorApp は各領域用のメソッドを提供する予定です：
//!
//! - `show_sidebar(&mut self, ui: &mut egui::Ui)` - サイドバー領域
//! - `show_editor(&mut self, ui: &mut egui::Ui)` - エディタ領域
//! - `show_status_bar(&mut self, ui: &mut egui::Ui)` - ステータスバー領域

pub mod app_state;
pub mod tab;

use app_state::AppState;
use eframe::egui;

/// エディタアプリケーションのメイン構造体
///
/// eframe::App トレイトを実装し、GUI の描画とイベント処理を行います。
pub struct EditorApp {
    /// アプリケーション状態
    state: AppState,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorApp {
    /// 新しい EditorApp を作成する
    pub fn new() -> Self {
        let mut state = AppState::new();

        // 初期タブを追加（デモ用）
        state.open_new_tab("Welcome.md");

        Self { state }
    }

    /// タブバーを描画する
    ///
    /// # Arguments
    /// * `ui` - egui の UI コンテキスト
    fn show_tab_bar(&mut self, ui: &mut egui::Ui) {
        // タブ情報を事前に収集（借用の競合を避けるため）
        let tab_titles: Vec<_> = self.state.tabs()
            .iter()
            .map(|tab| tab.title())
            .collect();
        let active_index = self.state.active_tab_index();

        let mut tab_to_close: Option<usize> = None;
        let mut tab_to_activate: Option<usize> = None;
        let mut new_tab_requested = false;

        ui.horizontal(|ui| {
            for (index, title) in tab_titles.iter().enumerate() {
                let is_active = Some(index) == active_index;

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // タブボタン
                        let button = egui::Button::new(title)
                            .selected(is_active);

                        if ui.add(button).clicked() {
                            tab_to_activate = Some(index);
                        }

                        // 閉じるボタン
                        if ui.small_button("✕").clicked() {
                            tab_to_close = Some(index);
                        }
                    });
                });
            }

            // 新しいタブボタン
            if ui.button("+ New Tab").clicked() {
                new_tab_requested = true;
            }
        });

        // UIクロージャの外で状態を更新
        if let Some(index) = tab_to_activate {
            self.state.set_active_tab(index);
        }

        if let Some(index) = tab_to_close {
            self.state.close_tab(index);
        }

        if new_tab_requested {
            let tab_count = self.state.tab_count();
            self.state.open_new_tab(format!("Untitled-{}", tab_count + 1));
        }
    }

    /// サイドバーを描画する（将来用）
    ///
    /// # Arguments
    /// * `ui` - egui の UI コンテキスト
    #[allow(unused)]
    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.label("Sidebar");
        ui.label("(file-tree will be here)");
    }

    /// メインエディタ領域を描画する
    ///
    /// # Arguments
    /// * `ui` - egui の UI コンテキスト
    fn show_editor(&mut self, ui: &mut egui::Ui) {
        if let Some(tab) = self.state.active_tab() {
            ui.heading(&tab.file_name);
            ui.separator();

            ui.label(format!("Tab ID: {}", tab.id));
            ui.label(format!("Modified: {}", tab.is_modified));
            ui.label(format!("Engine ID: {}", tab.engine.id));

            ui.separator();
            ui.label("Editor content will be here");
            ui.label("(basic-editing feature will implement this)");
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No tab open. Click '+ New Tab' to create one.");
            });
        }
    }

    /// ステータスバーを描画する（将来用）
    ///
    /// # Arguments
    /// * `ui` - egui の UI コンテキスト
    #[allow(unused)]
    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Status Bar");
            if let Some(tab) = self.state.active_tab() {
                ui.separator();
                ui.label(format!("Active: {}", tab.file_name));
            }
        });
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // トップメニューバー（将来用）
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Menu Bar (placeholder for future)");
            });
        });

        // 左サイドバー（将来用、現在は非表示）
        // egui::SidePanel::left("sidebar").show(ctx, |ui| {
        //     self.show_sidebar(ui);
        // });

        // 下部ステータスバー（将来用、現在は非表示）
        // egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        //     self.show_status_bar(ui);
        // });

        // メインパネル
        egui::CentralPanel::default().show(ctx, |ui| {
            // タブバー
            egui::TopBottomPanel::top("tab_bar")
                .show_inside(ui, |ui| {
                    self.show_tab_bar(ui);
                });

            // エディタ領域
            egui::CentralPanel::default().show_inside(ui, |ui| {
                self.show_editor(ui);
            });
        });
    }
}
