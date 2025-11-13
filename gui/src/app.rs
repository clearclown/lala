//! Main application state and UI logic

use crate::editor_view::EditorView;
use core_engine::Editor;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Main application state
pub struct LalaApp {
    /// The active editor
    editor: Arc<Mutex<Editor>>,
    /// Editor view state
    editor_view: EditorView,
    /// Status message
    status_message: String,
    /// Async runtime for file operations
    runtime: tokio::runtime::Runtime,
}

impl LalaApp {
    /// Create a new application
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let editor = Arc::new(Mutex::new(Editor::with_text("Welcome to Lala!\n\nStart typing to edit text.\n\nKeyboard shortcuts:\n- Ctrl+Z: Undo\n- Ctrl+Y or Ctrl+Shift+Z: Redo\n- Ctrl+S: Save\n")));

        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        Self {
            editor: editor.clone(),
            editor_view: EditorView::new(editor),
            status_message: String::new(),
            runtime,
        }
    }

    /// Handle keyboard shortcuts
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let runtime_handle = self.runtime.handle().clone();

        // Ctrl+Z: Undo
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Z))) {
            let editor = self.editor.clone();
            runtime_handle.block_on(async move {
                let mut ed = editor.lock().await;
                ed.undo();
            });
        }

        // Ctrl+Y or Ctrl+Shift+Z: Redo
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Y)))
            || ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL.plus(egui::Modifiers::SHIFT), egui::Key::Z)))
        {
            let editor = self.editor.clone();
            runtime_handle.block_on(async move {
                let mut ed = editor.lock().await;
                ed.redo();
            });
        }

        // Ctrl+S: Save
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S))) {
            let editor = self.editor.clone();
            runtime_handle.block_on(async move {
                let mut ed = editor.lock().await;
                match ed.save_file().await {
                    Ok(_) => {
                        // Successfully saved
                    }
                    Err(core_engine::EditorError::NoFilePath) => {
                        // Save to default path
                        if let Err(e) = ed.save_to_file("untitled.txt").await {
                            eprintln!("Failed to save: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to save: {}", e);
                    }
                }
            });
        }
    }
}

impl eframe::App for LalaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts first
        self.handle_shortcuts(ctx);

        // Top panel with menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        // TODO: Implement new file
                        ui.close_menu();
                    }
                    if ui.button("Open...").clicked() {
                        // TODO: Implement open file dialog
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        // Trigger save via shortcut handler
                        ctx.input_mut(|i| {
                            i.events.push(egui::Event::Key {
                                key: egui::Key::S,
                                physical_key: None,
                                pressed: true,
                                repeat: false,
                                modifiers: egui::Modifiers::CTRL,
                            });
                        });
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo (Ctrl+Z)").clicked() {
                        let editor = self.editor.clone();
                        let runtime_handle = self.runtime.handle().clone();
                        runtime_handle.block_on(async move {
                            let mut ed = editor.lock().await;
                            ed.undo();
                        });
                        ui.close_menu();
                    }
                    if ui.button("Redo (Ctrl+Y)").clicked() {
                        let editor = self.editor.clone();
                        let runtime_handle = self.runtime.handle().clone();
                        runtime_handle.block_on(async move {
                            let mut ed = editor.lock().await;
                            ed.redo();
                        });
                        ui.close_menu();
                    }
                });
            });
        });

        // Bottom panel with status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let editor = self.editor.clone();
                let runtime_handle = self.runtime.handle().clone();

                runtime_handle.block_on(async {
                    let ed = editor.lock().await;

                    // Show modified indicator
                    if ed.is_modified() {
                        ui.label("*");
                    }

                    // Show file name or "Untitled"
                    let file_name = ed.file_name().unwrap_or("Untitled");
                    ui.label(file_name);

                    ui.separator();

                    // Show cursor position
                    let cursor_pos = ed.cursor().position();
                    ui.label(format!("Position: {}", cursor_pos));

                    ui.separator();

                    // Show character count
                    ui.label(format!("Characters: {}", ed.buffer().len_chars()));
                });

                if !self.status_message.is_empty() {
                    ui.separator();
                    ui.label(&self.status_message);
                }
            });
        });

        // Central panel with editor
        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor_view.ui(ui);
        });

        // Request repaint for smooth updates
        ctx.request_repaint();
    }
}
