use crate::gui::editor::EditorPanel;

pub struct LalaApp {
    editor: EditorPanel,
}

impl LalaApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            editor: EditorPanel::new(),
        }
    }
}

impl eframe::App for LalaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor.ui(ui);
        });
    }
}
