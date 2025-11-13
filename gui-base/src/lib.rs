pub mod gui;

use anyhow::Result;
use gui::GuiApp;
use std::path::PathBuf;

/// Run the GUI application with the specified root path
pub fn run(root_path: PathBuf) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Lala Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Lala Editor",
        options,
        Box::new(|_cc| Ok(Box::new(GuiApp::new(root_path)))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run GUI: {}", e))?;

    Ok(())
}
