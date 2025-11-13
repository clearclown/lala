pub mod file_tree;

use file_tree::FileTree;
use std::path::PathBuf;

pub struct GuiApp {
    file_tree: FileTree,
    open_tabs: Vec<EditorTab>,
    active_tab: Option<usize>,
}

#[derive(Clone)]
pub struct EditorTab {
    pub path: PathBuf,
    pub content: String,
    pub modified: bool,
}

impl GuiApp {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            file_tree: FileTree::new(root_path),
            open_tabs: Vec::new(),
            active_tab: None,
        }
    }

    pub fn open_file(&mut self, path: PathBuf) {
        // Check if file is already open
        if let Some(idx) = self.open_tabs.iter().position(|tab| tab.path == path) {
            self.active_tab = Some(idx);
            return;
        }

        // Try to read the file
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                let tab = EditorTab {
                    path,
                    content,
                    modified: false,
                };
                self.open_tabs.push(tab);
                self.active_tab = Some(self.open_tabs.len() - 1);
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Side panel for file tree
        egui::SidePanel::left("file_tree_panel")
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(clicked_file) = self.file_tree.render(ui) {
                    if !clicked_file.is_dir() {
                        self.open_file(clicked_file);
                    }
                }
            });

        // Central panel for editor tabs
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_editor_tabs(ui);
        });
    }
}

impl GuiApp {
    fn render_editor_tabs(&mut self, ui: &mut egui::Ui) {
        if self.open_tabs.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No files open. Select a file from the tree to open it.");
            });
            return;
        }

        // Tab bar
        ui.horizontal(|ui| {
            let mut close_tab: Option<usize> = None;

            for (idx, tab) in self.open_tabs.iter().enumerate() {
                let tab_name = tab
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("untitled");

                let tab_label = if tab.modified {
                    format!("● {}", tab_name)
                } else {
                    tab_name.to_string()
                };

                let is_active = self.active_tab == Some(idx);

                ui.group(|ui| {
                    if ui
                        .selectable_label(is_active, tab_label)
                        .clicked()
                    {
                        self.active_tab = Some(idx);
                    }

                    if ui.small_button("×").clicked() {
                        close_tab = Some(idx);
                    }
                });
            }

            if let Some(idx) = close_tab {
                self.open_tabs.remove(idx);
                if self.active_tab == Some(idx) {
                    self.active_tab = if self.open_tabs.is_empty() {
                        None
                    } else if idx > 0 {
                        Some(idx - 1)
                    } else {
                        Some(0)
                    };
                } else if let Some(active) = self.active_tab {
                    if active > idx {
                        self.active_tab = Some(active - 1);
                    }
                }
            }
        });

        ui.separator();

        // Editor content
        if let Some(active_idx) = self.active_tab {
            if let Some(tab) = self.open_tabs.get_mut(active_idx) {
                ui.label(format!("File: {}", tab.path.display()));
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut tab.content)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY),
                    );

                    if response.changed() {
                        tab.modified = true;
                    }
                });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() && tab.modified {
                        if let Err(e) = std::fs::write(&tab.path, &tab.content) {
                            eprintln!("Failed to save file: {}", e);
                        } else {
                            tab.modified = false;
                        }
                    }

                    if tab.modified {
                        ui.label("Modified");
                    }
                });
            }
        }
    }
}
