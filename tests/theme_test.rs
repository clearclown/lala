/// Comprehensive tests for Theme module
///
/// Tests cover:
/// - ThemeMode enum
/// - EditorColors struct
/// - MarkdownPreviewColors struct
/// - Theme functions
use egui::Color32;
use lala::gui::{EditorColors, MarkdownPreviewColors, ThemeMode};

// ============================================
// === ThemeMode Enum Tests ===
// ============================================

mod theme_mode_tests {
    use super::*;

    #[test]
    fn test_default_is_dark() {
        let mode = ThemeMode::default();
        assert_eq!(mode, ThemeMode::Dark);
    }

    #[test]
    fn test_equality() {
        assert_eq!(ThemeMode::Light, ThemeMode::Light);
        assert_eq!(ThemeMode::Dark, ThemeMode::Dark);
        assert_eq!(ThemeMode::System, ThemeMode::System);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(ThemeMode::Light, ThemeMode::Dark);
        assert_ne!(ThemeMode::Dark, ThemeMode::System);
        assert_ne!(ThemeMode::System, ThemeMode::Light);
    }

    #[test]
    fn test_clone() {
        let mode = ThemeMode::Light;
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_copy() {
        let mode = ThemeMode::Dark;
        let copied = mode;
        assert_eq!(mode, copied);
    }

    #[test]
    fn test_debug() {
        let debug_str = format!("{:?}", ThemeMode::Light);
        assert!(debug_str.contains("Light"));

        let debug_str = format!("{:?}", ThemeMode::Dark);
        assert!(debug_str.contains("Dark"));

        let debug_str = format!("{:?}", ThemeMode::System);
        assert!(debug_str.contains("System"));
    }
}

// ============================================
// === EditorColors Tests ===
// ============================================

mod editor_colors_tests {
    use super::*;

    #[test]
    fn test_light_theme_colors() {
        let colors = EditorColors::light();

        // Background should be light (high RGB values)
        assert_eq!(colors.background, Color32::from_rgb(255, 255, 255));

        // Foreground should be dark (low RGB values)
        assert_eq!(colors.foreground, Color32::from_rgb(31, 41, 55));
    }

    #[test]
    fn test_dark_theme_colors() {
        let colors = EditorColors::dark();

        // Background should be dark (low RGB values)
        assert_eq!(colors.background, Color32::from_rgb(24, 24, 27));

        // Foreground should be light (high RGB values)
        assert_eq!(colors.foreground, Color32::from_rgb(244, 244, 245));
    }

    #[test]
    fn test_light_selection_color() {
        let colors = EditorColors::light();
        assert_eq!(colors.selection, Color32::from_rgb(191, 219, 254));
    }

    #[test]
    fn test_dark_selection_color() {
        let colors = EditorColors::dark();
        assert_eq!(colors.selection, Color32::from_rgb(30, 64, 115));
    }

    #[test]
    fn test_light_cursor_color() {
        let colors = EditorColors::light();
        assert_eq!(colors.cursor, Color32::from_rgb(37, 99, 235));
    }

    #[test]
    fn test_dark_cursor_color() {
        let colors = EditorColors::dark();
        assert_eq!(colors.cursor, Color32::from_rgb(96, 165, 250));
    }

    #[test]
    fn test_light_line_number_colors() {
        let colors = EditorColors::light();
        assert_eq!(colors.line_number, Color32::from_rgb(156, 163, 175));
        assert_eq!(colors.line_number_bg, Color32::from_rgb(248, 249, 250));
    }

    #[test]
    fn test_dark_line_number_colors() {
        let colors = EditorColors::dark();
        assert_eq!(colors.line_number, Color32::from_rgb(113, 113, 122));
        assert_eq!(colors.line_number_bg, Color32::from_rgb(30, 30, 33));
    }

    #[test]
    fn test_light_current_line() {
        let colors = EditorColors::light();
        assert_eq!(colors.current_line, Color32::from_rgb(243, 244, 246));
    }

    #[test]
    fn test_dark_current_line() {
        let colors = EditorColors::dark();
        assert_eq!(colors.current_line, Color32::from_rgb(39, 39, 42));
    }

    #[test]
    fn test_light_vs_dark_contrast() {
        let light = EditorColors::light();
        let dark = EditorColors::dark();

        // Background colors should be different
        assert_ne!(light.background, dark.background);

        // Foreground colors should be different
        assert_ne!(light.foreground, dark.foreground);

        // Selection colors should be different
        assert_ne!(light.selection, dark.selection);
    }
}

// ============================================
// === MarkdownPreviewColors Tests ===
// ============================================

mod markdown_preview_colors_tests {
    use super::*;

    #[test]
    fn test_light_theme_colors() {
        let colors = MarkdownPreviewColors::light();

        // Background should be white
        assert_eq!(colors.background, Color32::from_rgb(255, 255, 255));

        // Text should be dark
        assert_eq!(colors.text, Color32::from_rgb(31, 41, 55));
    }

    #[test]
    fn test_dark_theme_colors() {
        let colors = MarkdownPreviewColors::dark();

        // Background should be dark
        assert_eq!(colors.background, Color32::from_rgb(24, 24, 27));

        // Text should be light
        assert_eq!(colors.text, Color32::from_rgb(212, 212, 216));
    }

    #[test]
    fn test_light_heading_color() {
        let colors = MarkdownPreviewColors::light();
        assert_eq!(colors.heading, Color32::from_rgb(17, 24, 39));
    }

    #[test]
    fn test_dark_heading_color() {
        let colors = MarkdownPreviewColors::dark();
        assert_eq!(colors.heading, Color32::from_rgb(250, 250, 250));
    }

    #[test]
    fn test_light_link_color() {
        let colors = MarkdownPreviewColors::light();
        assert_eq!(colors.link, Color32::from_rgb(37, 99, 235));
    }

    #[test]
    fn test_dark_link_color() {
        let colors = MarkdownPreviewColors::dark();
        assert_eq!(colors.link, Color32::from_rgb(96, 165, 250));
    }

    #[test]
    fn test_light_code_colors() {
        let colors = MarkdownPreviewColors::light();
        assert_eq!(colors.code_bg, Color32::from_rgb(243, 244, 246));
        assert_eq!(colors.code_text, Color32::from_rgb(153, 27, 27));
    }

    #[test]
    fn test_dark_code_colors() {
        let colors = MarkdownPreviewColors::dark();
        assert_eq!(colors.code_bg, Color32::from_rgb(39, 39, 42));
        assert_eq!(colors.code_text, Color32::from_rgb(248, 113, 113));
    }

    #[test]
    fn test_light_blockquote_colors() {
        let colors = MarkdownPreviewColors::light();
        assert_eq!(colors.blockquote_border, Color32::from_rgb(209, 213, 219));
        assert_eq!(colors.blockquote_bg, Color32::from_rgb(249, 250, 251));
    }

    #[test]
    fn test_dark_blockquote_colors() {
        let colors = MarkdownPreviewColors::dark();
        assert_eq!(colors.blockquote_border, Color32::from_rgb(63, 63, 70));
        assert_eq!(colors.blockquote_bg, Color32::from_rgb(30, 30, 33));
    }

    #[test]
    fn test_light_hr_color() {
        let colors = MarkdownPreviewColors::light();
        assert_eq!(colors.hr, Color32::from_rgb(229, 231, 235));
    }

    #[test]
    fn test_dark_hr_color() {
        let colors = MarkdownPreviewColors::dark();
        assert_eq!(colors.hr, Color32::from_rgb(63, 63, 70));
    }

    #[test]
    fn test_light_vs_dark_contrast() {
        let light = MarkdownPreviewColors::light();
        let dark = MarkdownPreviewColors::dark();

        // All colors should be different between light and dark
        assert_ne!(light.background, dark.background);
        assert_ne!(light.text, dark.text);
        assert_ne!(light.heading, dark.heading);
        assert_ne!(light.link, dark.link);
        assert_ne!(light.code_bg, dark.code_bg);
        assert_ne!(light.code_text, dark.code_text);
        assert_ne!(light.blockquote_border, dark.blockquote_border);
        assert_ne!(light.blockquote_bg, dark.blockquote_bg);
        assert_ne!(light.hr, dark.hr);
    }
}

// ============================================
// === Color Consistency Tests ===
// ============================================

mod color_consistency_tests {
    use super::*;

    #[test]
    fn test_editor_and_preview_light_backgrounds_match() {
        let editor = EditorColors::light();
        let preview = MarkdownPreviewColors::light();

        // Both should use white background for consistency
        assert_eq!(editor.background, preview.background);
    }

    #[test]
    fn test_editor_and_preview_dark_backgrounds_match() {
        let editor = EditorColors::dark();
        let preview = MarkdownPreviewColors::dark();

        // Both should use the same dark background
        assert_eq!(editor.background, preview.background);
    }

    #[test]
    fn test_light_colors_are_readable() {
        let colors = EditorColors::light();

        // Background should be light (R+G+B > 600 for white-ish)
        let bg_brightness = colors.background.r() as u32
            + colors.background.g() as u32
            + colors.background.b() as u32;
        assert!(bg_brightness > 600, "Background should be light");

        // Foreground should be dark (R+G+B < 200 for dark)
        let fg_brightness = colors.foreground.r() as u32
            + colors.foreground.g() as u32
            + colors.foreground.b() as u32;
        assert!(fg_brightness < 200, "Foreground should be dark");
    }

    #[test]
    fn test_dark_colors_are_readable() {
        let colors = EditorColors::dark();

        // Background should be dark (R+G+B < 150)
        let bg_brightness = colors.background.r() as u32
            + colors.background.g() as u32
            + colors.background.b() as u32;
        assert!(bg_brightness < 150, "Background should be dark");

        // Foreground should be light (R+G+B > 600)
        let fg_brightness = colors.foreground.r() as u32
            + colors.foreground.g() as u32
            + colors.foreground.b() as u32;
        assert!(fg_brightness > 600, "Foreground should be light");
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_multiple_instances() {
        let colors1 = EditorColors::light();
        let colors2 = EditorColors::light();

        // Same colors should be created
        assert_eq!(colors1.background, colors2.background);
        assert_eq!(colors1.foreground, colors2.foreground);
    }

    #[test]
    fn test_theme_mode_all_variants() {
        let modes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::System];

        for mode in modes {
            let _ = format!("{:?}", mode);
        }
    }

    #[test]
    fn test_colors_have_full_opacity() {
        let light = EditorColors::light();
        let dark = EditorColors::dark();

        // All colors should have full opacity (alpha = 255)
        assert_eq!(light.background.a(), 255);
        assert_eq!(light.foreground.a(), 255);
        assert_eq!(dark.background.a(), 255);
        assert_eq!(dark.foreground.a(), 255);
    }
}
