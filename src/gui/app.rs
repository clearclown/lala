use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::core_engine::{Buffer, BufferId};
use crate::file_tree::FileTree;
use crate::search::GrepEngine;

use super::search_panel::SearchPanel;
use super::grep_panel::GrepPanel;

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
}

impl LalaApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        // Create initial empty buffer
        let mut buffers = HashMap::new();
        let buffer_id = BufferId(0);
        let empty_text = "".to_string();

        let buffer = Buffer::from_string(buffer_id, empty_text.clone(), None);
        buffers.insert(buffer_id, buffer);

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
        }
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+S: Save file
        if ctx.input(|i| i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::S)) {
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
        if ctx.input(|i| i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::F)) {
            self.show_search_panel = true;
        }

        // Ctrl+H: Open replace panel
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::H)) {
            self.show_search_panel = true;
            self.search_panel.set_replace_mode(true);
        }

        // Ctrl+Shift+F: Open grep panel
        if ctx.input(|i| {
            i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::F)
        }) {
            self.show_grep_panel = true;
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
            let file_path = self.buffers.get(&buffer_id)
                .and_then(|b| b.file_path())
                .cloned();

            if let Some(file_path) = file_path {
                // Save to existing file
                if let Err(e) = std::fs::write(&file_path, &self.current_text) {
                    eprintln!("Failed to save file: {}", e);
                } else {
                    // Update buffer and mark as clean
                    if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                        *buffer = Buffer::from_string(buffer_id, self.current_text.clone(), Some(file_path));
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
                eprintln!("Failed to save file: {}", e);
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
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New (Ctrl+N)").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button("Open (Ctrl+O)").clicked() {
                        self.show_file_dialog = true;
                        ui.close_menu();
                    }
                    if ui.button("Save (Ctrl+S)").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("Save As (Ctrl+Shift+S)").clicked() {
                        self.show_save_as_dialog = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Find (Ctrl+F)").clicked() {
                        self.show_search_panel = true;
                        ui.close_menu();
                    }
                    if ui.button("Replace (Ctrl+H)").clicked() {
                        self.show_search_panel = true;
                        self.search_panel.set_replace_mode(true);
                        ui.close_menu();
                    }
                });

                // Show file status
                if let Some(buffer_id) = self.active_buffer_id {
                    if let Some(buffer) = self.buffers.get(&buffer_id) {
                        let file_name = buffer.file_path()
                            .and_then(|p| p.file_name())
                            .and_then(|n| n.to_str())
                            .unwrap_or("Untitled");

                        let dirty_marker = if self.text_changed { " *" } else { "" };
                        ui.separator();
                        ui.label(format!("{}{}", file_name, dirty_marker));
                    }
                }
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let lines = self.current_text.lines().count();
                let chars = self.current_text.len();
                ui.label(format!("Lines: {} | Characters: {}", lines, chars));

                if self.text_changed {
                    ui.label("| Modified");
                }
            });
        });

        // Main editor - NO PADDING
        egui::CentralPanel::default()
            .frame(egui::Frame::none()) // Remove all frames and padding
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
                                .frame(false) // No frame around text edit
                        );

                        if response.changed() {
                            self.text_changed = true;
                        }
                    });
            });
    }

    fn show_file_dialog(&mut self, ctx: &egui::Context) {
        let mut should_open = None;
        let mut should_close = false;

        let mut is_open = self.show_file_dialog;
        egui::Window::new("Open File")
            .open(&mut is_open)
            .show(ctx, |ui| {
                ui.label("Enter file path:");
                ui.text_edit_singleline(&mut self.file_path_input);

                ui.horizontal(|ui| {
                    if ui.button("Open").clicked() {
                        should_open = Some(PathBuf::from(self.file_path_input.clone()));
                        should_close = true;
                    }
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });

                ui.separator();
                ui.label("Recent:");
                // TODO: Add recent files list
            });

        self.show_file_dialog = is_open;

        if let Some(path) = should_open {
            self.open_file(path);
            self.file_path_input.clear();
        }
        if should_close {
            self.show_file_dialog = false;
            self.file_path_input.clear();
        }
    }

    fn show_save_as_dialog(&mut self, ctx: &egui::Context) {
        let mut should_save = None;
        let mut should_close = false;

        let mut is_open = self.show_save_as_dialog;
        egui::Window::new("Save As")
            .open(&mut is_open)
            .show(ctx, |ui| {
                ui.label("Enter file path:");
                ui.text_edit_singleline(&mut self.file_path_input);

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        should_save = Some(PathBuf::from(self.file_path_input.clone()));
                        should_close = true;
                    }
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });
            });

        self.show_save_as_dialog = is_open;

        if let Some(path) = should_save {
            self.save_file_as(path);
            self.file_path_input.clear();
        }
        if should_close {
            self.show_save_as_dialog = false;
            self.file_path_input.clear();
        }
    }
}

impl eframe::App for LalaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        // Poll grep results
        if self.grep_engine.is_searching() {
            while let Some(result) = self.grep_engine.poll_result() {
                self.grep_panel.add_result(result);
            }
            ctx.request_repaint(); // Keep updating while searching
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
            let mut open = self.show_grep_panel;
            self.grep_panel.show(
                ctx,
                &mut self.grep_engine,
                &self.file_tree,
                &mut self.buffers,
                &mut self.active_buffer_id,
                &mut self.next_buffer_id,
                &mut open,
            );
            self.show_grep_panel = open;
        }

        // Show file dialog
        if self.show_file_dialog {
            self.show_file_dialog(ctx);
        }

        // Show save as dialog
        if self.show_save_as_dialog {
            self.show_save_as_dialog(ctx);
        }
    }
}
