use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::core_engine::{Buffer, BufferId};
use crate::file_tree::FileTree;
use crate::llm::GeminiClient;
use crate::search::GrepEngine;

use super::grep_panel::GrepPanel;
use super::markdown_preview;
use super::search_panel::SearchPanel;

// Import new modules
use super::dialogs;
use super::menu;
use super::previews::{self, PreviewMode};
use super::theme;

pub struct LalaApp {
    // Core components
    buffers: HashMap<BufferId, Buffer>,
    active_buffer_id: Option<BufferId>,
    next_buffer_id: usize,
    file_tree: FileTree,

    // Editor state
    current_text: String,
    text_changed: bool,

    // Search components
    search_panel: SearchPanel,
    grep_panel: GrepPanel,
    grep_engine: GrepEngine,

    // UI state
    show_search_panel: bool,
    show_grep_panel: bool,
    show_file_dialog: bool,
    show_save_as_dialog: bool,
    file_path_input: String,

    // Preview state
    show_preview: bool,
    preview_mode: PreviewMode,

    // Theme state
    is_light_theme: bool,

    // LLM integration (optional)
    llm_client: Option<GeminiClient>,
    llm_status: String,

    // Settings
    show_settings: bool,
    api_key_input: String,
    ai_enabled: bool,
}

impl LalaApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        // Create initial empty buffer
        let mut buffers = HashMap::new();
        let buffer_id = BufferId(0);
        let empty_text = "".to_string();

        let buffer = Buffer::from_string(buffer_id, empty_text.clone(), None);
        buffers.insert(buffer_id, buffer);

        // Try to initialize LLM client from environment
        let (llm_client, llm_status, ai_enabled) = match GeminiClient::from_env() {
            Ok(client) => (
                Some(client),
                "LLM ready (Gemini 1.5 Flash)".to_string(),
                true,
            ),
            Err(_) => (
                None,
                "LLM not available (set GEMINI_API_KEY)".to_string(),
                false,
            ),
        };

        Self {
            buffers,
            active_buffer_id: Some(buffer_id),
            next_buffer_id: 1,
            file_tree: FileTree::default(),
            current_text: empty_text,
            text_changed: false,
            search_panel: SearchPanel::new(),
            grep_panel: GrepPanel::new(),
            grep_engine: GrepEngine::new(),
            show_search_panel: false,
            show_grep_panel: false,
            show_file_dialog: false,
            show_save_as_dialog: false,
            file_path_input: String::new(),
            show_preview: false,
            preview_mode: PreviewMode::None,
            is_light_theme: false, // Default to dark theme
            llm_client,
            llm_status,
            show_settings: false,
            api_key_input: String::new(),
            ai_enabled,
        }
    }

    fn detect_preview_mode(&self) -> PreviewMode {
        if let Some(buffer_id) = self.active_buffer_id {
            if let Some(buffer) = self.buffers.get(&buffer_id) {
                return previews::detect_preview_mode(buffer.file_path());
            }
        }
        PreviewMode::None
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+S: Save file
        if ctx.input(|i| i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::S))
        {
            self.save_file();
        }

        // Ctrl+Shift+S: Save as
        if ctx.input(|i| i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::S)) {
            self.show_save_as_dialog = true;
        }

        // Ctrl+O: Open file
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::O)) {
            self.show_file_dialog = true;
        }

        // Ctrl+N: New file
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N)) {
            self.new_file();
        }

        // Ctrl+F: Open search panel
        if ctx.input(|i| i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::F))
        {
            self.show_search_panel = true;
        }

        // Ctrl+H: Open replace panel
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::H)) {
            self.show_search_panel = true;
            self.search_panel.set_replace_mode(true);
        }

        // Ctrl+Shift+F: Open grep panel
        if ctx.input(|i| i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::F)) {
            self.show_grep_panel = true;
        }

        // Ctrl+P: Toggle preview
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::P)) {
            self.show_preview = !self.show_preview;
            if self.show_preview {
                // Auto-detect or keep current mode
                let detected = self.detect_preview_mode();
                if detected != PreviewMode::None {
                    self.preview_mode = detected;
                } else if self.preview_mode == PreviewMode::None {
                    // Default to Markdown if no file extension
                    self.preview_mode = PreviewMode::Markdown;
                }
            }
        }

        // Escape: Close panels
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_search_panel = false;
            self.show_grep_panel = false;
            self.show_file_dialog = false;
            self.show_save_as_dialog = false;
        }
    }

    fn save_file(&mut self) {
        if let Some(buffer_id) = self.active_buffer_id {
            let file_path = self
                .buffers
                .get(&buffer_id)
                .and_then(|b| b.file_path())
                .cloned();

            if let Some(file_path) = file_path {
                // Save to existing file
                if let Err(e) = std::fs::write(&file_path, &self.current_text) {
                    eprintln!("Failed to save file: {e}");
                } else {
                    // Update buffer and mark as clean
                    if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                        *buffer = Buffer::from_string(
                            buffer_id,
                            self.current_text.clone(),
                            Some(file_path),
                        );
                        self.text_changed = false;
                    }
                }
            } else {
                // No file path, open save as dialog
                self.show_save_as_dialog = true;
            }
        }
    }

    fn save_file_as(&mut self, path: PathBuf) {
        if let Some(buffer_id) = self.active_buffer_id {
            // Save to new file
            if let Err(e) = std::fs::write(&path, &self.current_text) {
                eprintln!("Failed to save file: {e}");
            } else {
                // Update buffer with new path and mark as clean
                if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                    *buffer = Buffer::from_string(buffer_id, self.current_text.clone(), Some(path));
                    self.text_changed = false;
                }
            }
        }
    }

    fn open_file(&mut self, path: PathBuf) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let buffer_id = BufferId(self.next_buffer_id);
            self.next_buffer_id += 1;

            let buffer = Buffer::from_string(buffer_id, content.clone(), Some(path));
            self.buffers.insert(buffer_id, buffer);
            self.active_buffer_id = Some(buffer_id);
            self.current_text = content;
            self.text_changed = false;
        }
    }

    fn new_file(&mut self) {
        let buffer_id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;

        let buffer = Buffer::from_string(buffer_id, String::new(), None);
        self.buffers.insert(buffer_id, buffer);
        self.active_buffer_id = Some(buffer_id);
        self.current_text = String::new();
        self.text_changed = false;
    }

    fn show_main_editor(&mut self, ctx: &egui::Context) {
        // Update preview mode when file changes
        if self.show_preview && self.preview_mode == PreviewMode::None {
            self.preview_mode = self.detect_preview_mode();
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            let mut new_file = false;
            let mut open_file = false;
            let mut save_file = false;
            let mut save_as = false;
            let mut show_replace = false;

            menu::render_menu_bar(
                ui,
                &mut new_file,
                &mut open_file,
                &mut save_file,
                &mut save_as,
                &mut self.show_search_panel,
                &mut show_replace,
                &mut self.show_grep_panel,
                &mut self.show_settings,
                &mut self.is_light_theme,
                &mut self.show_preview,
                &mut self.preview_mode,
                &self.llm_status,
                &self.llm_client,
                self.ai_enabled,
                &mut self.current_text,
                &mut self.text_changed,
                &self.buffers,
                self.active_buffer_id,
            );

            // Handle menu actions
            if new_file {
                self.new_file();
            }
            if open_file {
                self.show_file_dialog = true;
            }
            if save_file {
                self.save_file();
            }
            if save_as {
                self.show_save_as_dialog = true;
            }
            if show_replace {
                self.search_panel.set_replace_mode(true);
            }
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let lines = self.current_text.lines().count();
                let chars = self.current_text.len();
                ui.label(format!("Lines: {lines} | Characters: {chars}"));

                if self.text_changed {
                    ui.label("| Modified");
                }
            });
        });

        // Main editor with optional preview
        if self.show_preview && self.preview_mode != PreviewMode::None {
            // Split view: Editor on left, Preview on right
            egui::SidePanel::right("preview_panel")
                .default_width(ctx.available_rect().width() * 0.5)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.heading("Preview");
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| match self.preview_mode {
                            PreviewMode::Markdown => {
                                markdown_preview::render_markdown_preview(ui, &self.current_text);
                            }
                            PreviewMode::Html => {
                                previews::render_html_preview(ui, &self.current_text);
                            }
                            PreviewMode::Latex => {
                                previews::render_latex_preview(ui, &self.current_text);
                            }
                            PreviewMode::Mermaid => {
                                previews::render_mermaid_preview(ui, &self.current_text);
                            }
                            PreviewMode::None => {}
                        });
                });
        }

        // Main editor
        let (bg_color, text_color) = if self.is_light_theme {
            (
                egui::Color32::from_rgb(255, 255, 255), // White background
                egui::Color32::from_rgb(0, 0, 0),       // Black text
            )
        } else {
            (
                egui::Color32::from_rgb(30, 30, 30),    // Dark gray background
                egui::Color32::from_rgb(255, 255, 255), // White text
            )
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(bg_color).inner_margin(0.0))
            .show(ctx, |ui| {
                let available_height = ui.available_height();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut self.current_text)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .min_size(egui::vec2(f32::INFINITY, available_height))
                                .frame(false)
                                .background_color(bg_color)
                                .text_color(text_color),
                        );

                        // Request focus on first frame
                        if self.current_text.is_empty() && !self.text_changed {
                            response.request_focus();
                        }

                        if response.changed() {
                            self.text_changed = true;
                        }
                    });
            });
    }
}

impl eframe::App for LalaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.is_light_theme {
            ctx.set_visuals(theme::custom_light_theme());
        } else {
            ctx.set_visuals(egui::Visuals::dark());
        }

        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        // Poll grep results
        if self.grep_engine.is_searching() {
            while let Some(result) = self.grep_engine.poll_result() {
                self.grep_panel.add_result(result);
            }
            ctx.request_repaint();
        }

        // Show main editor
        self.show_main_editor(ctx);

        // Show search panel
        if self.show_search_panel {
            if let Some(buffer_id) = self.active_buffer_id {
                if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                    let mut open = self.show_search_panel;
                    self.search_panel.show(ctx, buffer, &mut open);
                    self.show_search_panel = open;
                }
            }
        }

        // Show grep panel
        if self.show_grep_panel {
            self.grep_panel.show(
                ctx,
                &mut self.grep_engine,
                &self.file_tree,
                &mut self.buffers,
                &mut self.active_buffer_id,
                &mut self.next_buffer_id,
                &mut self.show_grep_panel,
            );
        }

        // Show dialogs
        if self.show_file_dialog {
            if let Some(path) =
                dialogs::show_file_dialog(ctx, &mut self.show_file_dialog, &mut self.file_path_input)
            {
                self.open_file(path);
            }
        }

        if self.show_save_as_dialog {
            if let Some(path) = dialogs::show_save_as_dialog(
                ctx,
                &mut self.show_save_as_dialog,
                &mut self.file_path_input,
            ) {
                self.save_file_as(path);
            }
        }

        if self.show_settings {
            dialogs::show_settings(
                ctx,
                &mut self.show_settings,
                &mut self.api_key_input,
                &mut self.ai_enabled,
                &mut self.llm_client,
                &mut self.llm_status,
            );
        }
    }
}
