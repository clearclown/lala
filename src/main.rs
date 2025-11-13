use eframe::egui;

mod core_engine;
mod file_tree;
mod gui;
mod search;

use gui::LalaApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Lala - Advanced Text Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "lala",
        options,
        Box::new(|cc| Ok(Box::new(LalaApp::new(cc)))),
    )
}
