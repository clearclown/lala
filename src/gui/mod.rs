pub mod editor;
pub mod markdown_preview;

use eframe::egui;

/// Main application struct - feature/gui-base
pub struct MarkdownEditorApp {
    editor: editor::Editor,
    show_preview: bool,
}

impl MarkdownEditorApp {
    pub fn new() -> Self {
        Self {
            editor: editor::Editor::new(),
            show_preview: true,
        }
    }
}

impl Default for MarkdownEditorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for MarkdownEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.editor.clear();
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Preview").clicked() {
                        self.show_preview = !self.show_preview;
                        ui.close_menu();
                    }
                });
            });
        });

        // Main content area with side-by-side editor and preview
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_preview {
                // Split view: Editor on left, Preview on right
                ui.horizontal(|ui| {
                    // Editor panel (left side)
                    ui.allocate_ui(
                        egui::vec2(ui.available_width() * 0.5, ui.available_height()),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_source("editor_scroll")
                                .show(ui, |ui| {
                                    ui.heading("Editor");
                                    self.editor.ui(ui);
                                });
                        },
                    );

                    ui.separator();

                    // Preview panel (right side)
                    ui.allocate_ui(
                        egui::vec2(ui.available_width(), ui.available_height()),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_source("preview_scroll")
                                .show(ui, |ui| {
                                    ui.heading("Preview");
                                    ui.separator();
                                    markdown_preview::render_markdown_preview(
                                        ui,
                                        self.editor.content(),
                                    );
                                });
                        },
                    );
                });
            } else {
                // Editor only (full width)
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Editor");
                    self.editor.ui(ui);
                });
            }
        });
    }
}
