//! RTL (Right-to-Left) text support utilities
//!
//! This module provides utilities for detecting and handling RTL text,
//! including Arabic, Hebrew, Persian, and Urdu scripts.

use egui::{text::LayoutJob, Color32, FontId, TextFormat};

/// Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    #[default]
    LeftToRight,
    RightToLeft,
    Mixed,
}

/// Check if a character is RTL
#[inline]
pub fn is_rtl_char(c: char) -> bool {
    // Arabic script: U+0600 - U+06FF, U+0750 - U+077F, U+08A0 - U+08FF
    // Hebrew script: U+0590 - U+05FF
    // Arabic Extended-A: U+08A0 - U+08FF
    // Arabic Presentation Forms-A: U+FB50 - U+FDFF
    // Arabic Presentation Forms-B: U+FE70 - U+FEFF
    matches!(c,
        '\u{0590}'..='\u{05FF}' |  // Hebrew
        '\u{0600}'..='\u{06FF}' |  // Arabic
        '\u{0750}'..='\u{077F}' |  // Arabic Supplement
        '\u{08A0}'..='\u{08FF}' |  // Arabic Extended-A
        '\u{FB50}'..='\u{FDFF}' |  // Arabic Presentation Forms-A
        '\u{FE70}'..='\u{FEFF}'    // Arabic Presentation Forms-B
    )
}

/// Check if a string contains RTL characters
pub fn contains_rtl(text: &str) -> bool {
    text.chars().any(is_rtl_char)
}

/// Detect the primary text direction of a string
pub fn detect_text_direction(text: &str) -> TextDirection {
    let mut rtl_count = 0;
    let mut ltr_count = 0;

    for c in text.chars() {
        if is_rtl_char(c) {
            rtl_count += 1;
        } else if c.is_alphabetic() && !c.is_whitespace() {
            ltr_count += 1;
        }
    }

    if rtl_count == 0 && ltr_count == 0 {
        TextDirection::LeftToRight
    } else if rtl_count > ltr_count {
        TextDirection::RightToLeft
    } else if rtl_count > 0 {
        TextDirection::Mixed
    } else {
        TextDirection::LeftToRight
    }
}

/// Get alignment for text based on its direction
pub fn get_text_alignment(direction: TextDirection) -> egui::Align {
    match direction {
        TextDirection::RightToLeft => egui::Align::RIGHT,
        TextDirection::LeftToRight | TextDirection::Mixed => egui::Align::LEFT,
    }
}

/// Create a layout job with proper RTL support
pub fn create_rtl_aware_layout(
    text: &str,
    font_id: FontId,
    default_color: Color32,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    let direction = detect_text_direction(text);

    // Set the overall layout direction
    job.halign = get_text_alignment(direction);

    // Add the text with proper formatting
    job.append(
        text,
        0.0,
        TextFormat {
            font_id,
            color: default_color,
            ..Default::default()
        },
    );

    job
}

/// RTL-aware text wrapper for UI components
pub struct RtlText<'a> {
    pub text: &'a str,
    pub direction: TextDirection,
}

impl<'a> RtlText<'a> {
    pub fn new(text: &'a str) -> Self {
        let direction = detect_text_direction(text);
        Self { text, direction }
    }

    pub fn is_rtl(&self) -> bool {
        matches!(self.direction, TextDirection::RightToLeft)
    }

    pub fn is_mixed(&self) -> bool {
        matches!(self.direction, TextDirection::Mixed)
    }

    pub fn alignment(&self) -> egui::Align {
        get_text_alignment(self.direction)
    }
}

/// Extension trait for egui::Ui to add RTL-aware text widgets
pub trait RtlUiExt {
    /// Add a label with RTL-aware alignment
    fn rtl_label(&mut self, text: &str) -> egui::Response;

    /// Add a heading with RTL-aware alignment
    fn rtl_heading(&mut self, text: &str) -> egui::Response;
}

impl RtlUiExt for egui::Ui {
    fn rtl_label(&mut self, text: &str) -> egui::Response {
        let rtl_text = RtlText::new(text);
        if rtl_text.is_rtl() {
            self.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| ui.label(text),
            )
            .inner
        } else {
            self.label(text)
        }
    }

    fn rtl_heading(&mut self, text: &str) -> egui::Response {
        let rtl_text = RtlText::new(text);
        if rtl_text.is_rtl() {
            self.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| ui.heading(text),
            )
            .inner
        } else {
            self.heading(text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rtl_char() {
        // Arabic characters
        assert!(is_rtl_char('ا')); // Arabic Alef
        assert!(is_rtl_char('ب')); // Arabic Ba
        assert!(is_rtl_char('ت')); // Arabic Ta

        // Hebrew characters
        assert!(is_rtl_char('א')); // Hebrew Alef
        assert!(is_rtl_char('ב')); // Hebrew Bet

        // Non-RTL characters
        assert!(!is_rtl_char('a'));
        assert!(!is_rtl_char('A'));
        assert!(!is_rtl_char('1'));
        assert!(!is_rtl_char(' '));
        assert!(!is_rtl_char('日')); // Japanese
    }

    #[test]
    fn test_contains_rtl() {
        assert!(contains_rtl("مرحبا"));
        assert!(contains_rtl("Hello مرحبا World"));
        assert!(contains_rtl("שלום"));
        assert!(!contains_rtl("Hello World"));
        assert!(!contains_rtl("こんにちは"));
    }

    #[test]
    fn test_detect_text_direction() {
        // Pure Arabic
        assert_eq!(
            detect_text_direction("مرحبا بالعالم"),
            TextDirection::RightToLeft
        );

        // Pure Hebrew
        assert_eq!(
            detect_text_direction("שלום עולם"),
            TextDirection::RightToLeft
        );

        // Pure English
        assert_eq!(
            detect_text_direction("Hello World"),
            TextDirection::LeftToRight
        );

        // Mixed - more RTL
        assert_eq!(
            detect_text_direction("مرحبا Hello"),
            TextDirection::Mixed
        );

        // Empty string
        assert_eq!(detect_text_direction(""), TextDirection::LeftToRight);

        // Only numbers and spaces
        assert_eq!(detect_text_direction("123 456"), TextDirection::LeftToRight);
    }

    #[test]
    fn test_rtl_text() {
        let arabic = RtlText::new("مرحبا");
        assert!(arabic.is_rtl());
        assert!(!arabic.is_mixed());
        assert_eq!(arabic.alignment(), egui::Align::RIGHT);

        let english = RtlText::new("Hello");
        assert!(!english.is_rtl());
        assert!(!english.is_mixed());
        assert_eq!(english.alignment(), egui::Align::LEFT);

        let mixed = RtlText::new("Hello مرحبا");
        assert!(!mixed.is_rtl());
        assert!(mixed.is_mixed());
    }
}
