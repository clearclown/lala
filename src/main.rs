use eframe::egui;
use lala::LalaApp;

fn main() -> Result<(), eframe::Error> {
    // Initialize logger for debugging
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Lala - Modern Text Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "lala",
        options,
        Box::new(|cc| Ok(Box::new(LalaApp::new(cc)))),
    )
}
