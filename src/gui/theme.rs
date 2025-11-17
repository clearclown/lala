use eframe::egui;

/// Creates a custom light theme with improved visibility
pub fn custom_light_theme() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();

    // Background colors - light and clean
    visuals.window_fill = egui::Color32::from_rgb(250, 250, 250); // Very light gray
    visuals.panel_fill = egui::Color32::from_rgb(245, 245, 245); // Slightly darker gray
    visuals.faint_bg_color = egui::Color32::from_rgb(240, 240, 240);

    // Widget colors - clear and visible with dark text for high contrast
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
    visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(30, 30, 30);

    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(240, 240, 240);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(40, 40, 40);

    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(220, 230, 245);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(0, 0, 0);

    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(200, 220, 240);
    visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(0, 0, 0);

    // Selection color - clear blue highlight
    visuals.selection.bg_fill = egui::Color32::from_rgb(180, 210, 255);
    visuals.selection.stroke.color = egui::Color32::from_rgb(100, 150, 220);

    // Hyperlink color - visible blue
    visuals.hyperlink_color = egui::Color32::from_rgb(0, 100, 200);

    // Window stroke - subtle border
    visuals.window_stroke.color = egui::Color32::from_rgb(200, 200, 200);

    visuals
}
