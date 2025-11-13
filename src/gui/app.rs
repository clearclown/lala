use eframe::egui;
use std::collections::HashMap;

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

    // Search components
    search_panel: SearchPanel,
    grep_panel: GrepPanel,
    grep_engine: GrepEngine,

    // UI state
    show_search_panel: bool,
    show_grep_panel: bool,
}

impl LalaApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        // Create initial buffer with sample text
        let mut buffers = HashMap::new();
        let buffer_id = BufferId(0);
        let sample_text = "Welcome to Lala Editor!\n\
            \n\
            This is a sample text file for testing search functionality.\n\
            \n\
            Some example patterns:\n\
            - TODO: Implement feature A\n\
            - TODO: Fix bug B\n\
            - FIXME: Refactor code C\n\
            \n\
            Numbers: t1t, t2t, t3t\n\
            \n\
            Press Ctrl+F to search\n\
            Press Ctrl+H to replace\n\
            Press Ctrl+Shift+F for grep search\n\
            ";

        let buffer = Buffer::from_string(buffer_id, sample_text.to_string(), None);
        buffers.insert(buffer_id, buffer);

        Self {
            buffers,
            active_buffer_id: Some(buffer_id),
            next_buffer_id: 1,
            file_tree: FileTree::default(),
            search_panel: SearchPanel::new(),
            grep_panel: GrepPanel::new(),
            grep_engine: GrepEngine::new(),
            show_search_panel: false,
            show_grep_panel: false,
        }
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+F: Open search panel
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::F)) {
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
        }
    }

    fn show_main_editor(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lala - Advanced Text Editor");

            ui.separator();

            if let Some(buffer_id) = self.active_buffer_id {
                if let Some(buffer) = self.buffers.get(&buffer_id) {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut buffer.content())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .desired_rows(30),
                        );
                    });

                    ui.separator();
                    ui.label(format!(
                        "Lines: {} | Characters: {}",
                        buffer.line_count(),
                        buffer.content().len()
                    ));
                } else {
                    ui.label("No buffer found");
                }
            } else {
                ui.label("No active buffer");
            }
        });
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

        // Show help overlay
        egui::TopBottomPanel::bottom("help_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Shortcuts:");
                ui.label("Ctrl+F: Search");
                ui.label("Ctrl+H: Replace");
                ui.label("Ctrl+Shift+F: Grep");
                ui.label("Esc: Close panels");
            });
        });
    }
}
