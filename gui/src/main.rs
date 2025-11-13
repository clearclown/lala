//! Lala - A simple text editor built with egui and core-engine

mod app;
mod editor_view;

use app::LalaApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lala Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(LalaApp::new(cc)))),
    )
}
