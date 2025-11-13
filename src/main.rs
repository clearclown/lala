mod gui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Lala - Markdown Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Lala",
        options,
        Box::new(|cc| {
            // Set up custom fonts if needed
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::proportional(24.0),
            );
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(16.0),
            );
            style.text_styles.insert(
                egui::TextStyle::Monospace,
                egui::FontId::monospace(14.0),
            );
            cc.egui_ctx.set_style(style);

            Ok(Box::new(gui::MarkdownEditorApp::new()))
        }),
    )
}
