mod app;
mod core;
mod gui;

use app::LalaApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Lala Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Lala Editor",
        options,
        Box::new(|cc| Ok(Box::new(LalaApp::new(cc)))),
    )
}
