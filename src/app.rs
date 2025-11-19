use crate::cli::StartupMode;
use crate::gui::editor::EditorPanel;
use std::fs;
use std::path::PathBuf;

pub struct LalaApp {
    editor: EditorPanel,
}

impl LalaApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, mode: StartupMode) -> Self {
        let mut editor = EditorPanel::new();

        // Handle startup mode
        match mode {
            StartupMode::OpenFile(path) => {
                // Load the file if it exists
                if let Ok(content) = fs::read_to_string(&path) {
                    let path_str = path.to_string_lossy().to_string();
                    editor.load_file(path_str, content);
                } else {
                    eprintln!("Warning: Could not read file {:?}", path);
                }
            }
            StartupMode::OpenDir(_path) => {
                // TODO: Implement directory opening
                // For now, just start with empty editor
            }
            StartupMode::Empty => {
                // Already initialized with sample content
            }
            _ => {
                // Other modes are handled in main.rs before GUI starts
            }
        }

        Self { editor }
    }
}

impl eframe::App for LalaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor.ui(ui);
        });
    }
}
