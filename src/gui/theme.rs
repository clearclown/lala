use eframe::egui;

/// Creates a custom light theme with improved visibility and modern aesthetics
pub fn custom_light_theme() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();

    // Background colors - clean, modern look
    visuals.window_fill = egui::Color32::from_rgb(252, 252, 252); // Near white
    visuals.panel_fill = egui::Color32::from_rgb(248, 249, 250); // Subtle gray
    visuals.faint_bg_color = egui::Color32::from_rgb(243, 244, 246);
    visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);

    // Widget colors - modern, accessible design
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
    visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(31, 41, 55);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::from_rgb(229, 231, 235);

    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(243, 244, 246);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(55, 65, 81);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.inactive.bg_stroke.color = egui::Color32::from_rgb(209, 213, 219);

    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(229, 231, 235);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(17, 24, 39);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.hovered.bg_stroke.color = egui::Color32::from_rgb(156, 163, 175);

    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(209, 213, 219);
    visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(0, 0, 0);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.active.bg_stroke.color = egui::Color32::from_rgb(107, 114, 128);

    visuals.widgets.open.bg_fill = egui::Color32::from_rgb(243, 244, 246);
    visuals.widgets.open.fg_stroke.color = egui::Color32::from_rgb(17, 24, 39);
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(6);

    // Selection color - modern blue
    visuals.selection.bg_fill = egui::Color32::from_rgb(191, 219, 254);
    visuals.selection.stroke.color = egui::Color32::from_rgb(59, 130, 246);

    // Hyperlink color - accessible blue
    visuals.hyperlink_color = egui::Color32::from_rgb(37, 99, 235);

    // Window stroke - subtle border
    visuals.window_stroke.color = egui::Color32::from_rgb(229, 231, 235);
    visuals.window_stroke.width = 1.0;

    // Window shadow - subtle elevation (egui 0.33 API)
    visuals.window_shadow.offset = [0, 4];
    visuals.window_shadow.blur = 12;
    visuals.window_shadow.spread = 0;
    visuals.window_shadow.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 25);

    // Popup shadow (egui 0.33 API)
    visuals.popup_shadow.offset = [0, 4];
    visuals.popup_shadow.blur = 16;
    visuals.popup_shadow.spread = 0;
    visuals.popup_shadow.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 30);

    // Resize corner - subtle
    visuals.resize_corner_size = 12.0;

    // Text cursor
    visuals.text_cursor.stroke.color = egui::Color32::from_rgb(37, 99, 235);
    visuals.text_cursor.stroke.width = 2.0;

    visuals
}

/// Creates a custom dark theme optimized for Markdown composition
/// with comfortable contrast and modern aesthetics
pub fn custom_dark_theme() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();

    // Background colors - comfortable dark tones
    visuals.window_fill = egui::Color32::from_rgb(24, 24, 27); // Zinc-900
    visuals.panel_fill = egui::Color32::from_rgb(30, 30, 33); // Slightly lighter
    visuals.faint_bg_color = egui::Color32::from_rgb(39, 39, 42); // Zinc-800
    visuals.extreme_bg_color = egui::Color32::from_rgb(17, 17, 19); // Near black

    // Widget colors - modern dark design
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(39, 39, 42);
    visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(212, 212, 216);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::from_rgb(63, 63, 70);

    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(52, 52, 58);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(161, 161, 170);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.inactive.bg_stroke.color = egui::Color32::from_rgb(63, 63, 70);

    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(63, 63, 70);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(244, 244, 245);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.hovered.bg_stroke.color = egui::Color32::from_rgb(82, 82, 91);

    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(82, 82, 91);
    visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(255, 255, 255);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
    visuals.widgets.active.bg_stroke.color = egui::Color32::from_rgb(113, 113, 122);

    visuals.widgets.open.bg_fill = egui::Color32::from_rgb(52, 52, 58);
    visuals.widgets.open.fg_stroke.color = egui::Color32::from_rgb(244, 244, 245);
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(6);

    // Selection color - modern blue on dark
    visuals.selection.bg_fill = egui::Color32::from_rgb(30, 64, 115);
    visuals.selection.stroke.color = egui::Color32::from_rgb(96, 165, 250);

    // Hyperlink color - bright blue for visibility
    visuals.hyperlink_color = egui::Color32::from_rgb(96, 165, 250);

    // Window stroke - subtle border
    visuals.window_stroke.color = egui::Color32::from_rgb(63, 63, 70);
    visuals.window_stroke.width = 1.0;

    // Window shadow - subtle elevation (egui 0.33 API)
    visuals.window_shadow.offset = [0, 4];
    visuals.window_shadow.blur = 16;
    visuals.window_shadow.spread = 0;
    visuals.window_shadow.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80);

    // Popup shadow (egui 0.33 API)
    visuals.popup_shadow.offset = [0, 6];
    visuals.popup_shadow.blur = 20;
    visuals.popup_shadow.spread = 0;
    visuals.popup_shadow.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100);

    // Resize corner - subtle
    visuals.resize_corner_size = 12.0;

    // Text cursor - bright for visibility
    visuals.text_cursor.stroke.color = egui::Color32::from_rgb(96, 165, 250);
    visuals.text_cursor.stroke.width = 2.0;

    visuals
}

/// Theme settings for the application
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ThemeMode {
    Light,
    #[default]
    Dark, // Default to dark theme for code editing
    System, // Future: auto-detect system theme
}

/// Get visuals for the specified theme mode
#[allow(dead_code)]
pub fn get_theme_visuals(mode: ThemeMode) -> egui::Visuals {
    match mode {
        ThemeMode::Light => custom_light_theme(),
        ThemeMode::Dark | ThemeMode::System => custom_dark_theme(),
    }
}

/// Editor color scheme for syntax-aware text display
pub struct EditorColors {
    pub background: egui::Color32,
    pub foreground: egui::Color32,
    pub selection: egui::Color32,
    pub cursor: egui::Color32,
    pub line_number: egui::Color32,
    pub line_number_bg: egui::Color32,
    pub current_line: egui::Color32,
}

impl EditorColors {
    pub fn light() -> Self {
        Self {
            background: egui::Color32::from_rgb(255, 255, 255),
            foreground: egui::Color32::from_rgb(31, 41, 55),
            selection: egui::Color32::from_rgb(191, 219, 254),
            cursor: egui::Color32::from_rgb(37, 99, 235),
            line_number: egui::Color32::from_rgb(156, 163, 175),
            line_number_bg: egui::Color32::from_rgb(248, 249, 250),
            current_line: egui::Color32::from_rgb(243, 244, 246),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: egui::Color32::from_rgb(24, 24, 27),
            foreground: egui::Color32::from_rgb(244, 244, 245),
            selection: egui::Color32::from_rgb(30, 64, 115),
            cursor: egui::Color32::from_rgb(96, 165, 250),
            line_number: egui::Color32::from_rgb(113, 113, 122),
            line_number_bg: egui::Color32::from_rgb(30, 30, 33),
            current_line: egui::Color32::from_rgb(39, 39, 42),
        }
    }
}

/// Markdown preview color scheme
pub struct MarkdownPreviewColors {
    pub background: egui::Color32,
    pub text: egui::Color32,
    pub heading: egui::Color32,
    pub link: egui::Color32,
    pub code_bg: egui::Color32,
    pub code_text: egui::Color32,
    pub blockquote_border: egui::Color32,
    pub blockquote_bg: egui::Color32,
    pub hr: egui::Color32,
}

impl MarkdownPreviewColors {
    pub fn light() -> Self {
        Self {
            background: egui::Color32::from_rgb(255, 255, 255),
            text: egui::Color32::from_rgb(31, 41, 55),
            heading: egui::Color32::from_rgb(17, 24, 39),
            link: egui::Color32::from_rgb(37, 99, 235),
            code_bg: egui::Color32::from_rgb(243, 244, 246),
            code_text: egui::Color32::from_rgb(153, 27, 27),
            blockquote_border: egui::Color32::from_rgb(209, 213, 219),
            blockquote_bg: egui::Color32::from_rgb(249, 250, 251),
            hr: egui::Color32::from_rgb(229, 231, 235),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: egui::Color32::from_rgb(24, 24, 27),
            text: egui::Color32::from_rgb(212, 212, 216),
            heading: egui::Color32::from_rgb(250, 250, 250),
            link: egui::Color32::from_rgb(96, 165, 250),
            code_bg: egui::Color32::from_rgb(39, 39, 42),
            code_text: egui::Color32::from_rgb(248, 113, 113),
            blockquote_border: egui::Color32::from_rgb(63, 63, 70),
            blockquote_bg: egui::Color32::from_rgb(30, 30, 33),
            hr: egui::Color32::from_rgb(63, 63, 70),
        }
    }
}
