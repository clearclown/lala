/// Comprehensive tests for RTL (Right-to-Left) text support
///
/// Tests cover:
/// - RTL character detection
/// - Text direction detection
/// - RtlText wrapper
/// - Alignment functions
/// - TextDirection enum
use lala::gui::rtl::{contains_rtl, detect_text_direction, is_rtl_char, RtlText, TextDirection};

// ============================================
// === is_rtl_char Tests ===
// ============================================

mod is_rtl_char_tests {
    use super::*;

    #[test]
    fn test_arabic_basic() {
        // Basic Arabic characters
        assert!(is_rtl_char('ا')); // Alef
        assert!(is_rtl_char('ب')); // Ba
        assert!(is_rtl_char('ت')); // Ta
        assert!(is_rtl_char('ث')); // Tha
        assert!(is_rtl_char('ج')); // Jim
        assert!(is_rtl_char('ح')); // Ha
        assert!(is_rtl_char('خ')); // Kha
        assert!(is_rtl_char('د')); // Dal
    }

    #[test]
    fn test_arabic_extended() {
        // Arabic with diacritics
        assert!(is_rtl_char('أ')); // Alef with Hamza above
        assert!(is_rtl_char('إ')); // Alef with Hamza below
        assert!(is_rtl_char('آ')); // Alef with Madda
        assert!(is_rtl_char('ء')); // Hamza
        assert!(is_rtl_char('ؤ')); // Waw with Hamza
        assert!(is_rtl_char('ئ')); // Ya with Hamza
    }

    #[test]
    fn test_hebrew_basic() {
        // Basic Hebrew characters
        assert!(is_rtl_char('א')); // Alef
        assert!(is_rtl_char('ב')); // Bet
        assert!(is_rtl_char('ג')); // Gimel
        assert!(is_rtl_char('ד')); // Dalet
        assert!(is_rtl_char('ה')); // He
        assert!(is_rtl_char('ו')); // Vav
        assert!(is_rtl_char('ז')); // Zayin
        assert!(is_rtl_char('ח')); // Het
    }

    #[test]
    fn test_persian_urdu_chars() {
        // Persian-specific characters
        assert!(is_rtl_char('پ')); // Pe
        assert!(is_rtl_char('چ')); // Che
        assert!(is_rtl_char('ژ')); // Zhe
        assert!(is_rtl_char('گ')); // Gaf
    }

    #[test]
    fn test_non_rtl_latin() {
        assert!(!is_rtl_char('a'));
        assert!(!is_rtl_char('Z'));
        assert!(!is_rtl_char('m'));
        assert!(!is_rtl_char('Q'));
    }

    #[test]
    fn test_non_rtl_numbers() {
        assert!(!is_rtl_char('0'));
        assert!(!is_rtl_char('5'));
        assert!(!is_rtl_char('9'));
    }

    #[test]
    fn test_non_rtl_cjk() {
        // Chinese characters
        assert!(!is_rtl_char('中'));
        assert!(!is_rtl_char('国'));
        assert!(!is_rtl_char('文'));

        // Japanese hiragana
        assert!(!is_rtl_char('あ'));
        assert!(!is_rtl_char('い'));
        assert!(!is_rtl_char('う'));

        // Japanese katakana
        assert!(!is_rtl_char('ア'));
        assert!(!is_rtl_char('イ'));
        assert!(!is_rtl_char('ウ'));

        // Korean
        assert!(!is_rtl_char('한'));
        assert!(!is_rtl_char('국'));
    }

    #[test]
    fn test_non_rtl_punctuation() {
        assert!(!is_rtl_char('.'));
        assert!(!is_rtl_char(','));
        assert!(!is_rtl_char('!'));
        assert!(!is_rtl_char('?'));
        assert!(!is_rtl_char(':'));
        assert!(!is_rtl_char(';'));
    }

    #[test]
    fn test_non_rtl_whitespace() {
        assert!(!is_rtl_char(' '));
        assert!(!is_rtl_char('\t'));
        assert!(!is_rtl_char('\n'));
        assert!(!is_rtl_char('\r'));
    }

    #[test]
    fn test_non_rtl_special() {
        assert!(!is_rtl_char('@'));
        assert!(!is_rtl_char('#'));
        assert!(!is_rtl_char('$'));
        assert!(!is_rtl_char('%'));
        assert!(!is_rtl_char('&'));
        assert!(!is_rtl_char('*'));
    }

    #[test]
    fn test_emoji() {
        assert!(!is_rtl_char('😀'));
        assert!(!is_rtl_char('🎉'));
        assert!(!is_rtl_char('❤'));
    }
}

// ============================================
// === contains_rtl Tests ===
// ============================================

mod contains_rtl_tests {
    use super::*;

    #[test]
    fn test_pure_arabic() {
        assert!(contains_rtl("مرحبا"));
        assert!(contains_rtl("السلام عليكم"));
        assert!(contains_rtl("أهلا وسهلا"));
    }

    #[test]
    fn test_pure_hebrew() {
        assert!(contains_rtl("שלום"));
        assert!(contains_rtl("בוקר טוב"));
        assert!(contains_rtl("ערב טוב"));
    }

    #[test]
    fn test_mixed_arabic_english() {
        assert!(contains_rtl("Hello مرحبا World"));
        assert!(contains_rtl("مرحبا Hello"));
        assert!(contains_rtl("Test أهلا Test"));
    }

    #[test]
    fn test_mixed_hebrew_english() {
        assert!(contains_rtl("Hello שלום"));
        assert!(contains_rtl("שלום World"));
    }

    #[test]
    fn test_no_rtl() {
        assert!(!contains_rtl("Hello World"));
        assert!(!contains_rtl(""));
        assert!(!contains_rtl("   "));
        assert!(!contains_rtl("12345"));
    }

    #[test]
    fn test_no_rtl_cjk() {
        assert!(!contains_rtl("日本語"));
        assert!(!contains_rtl("中文"));
        assert!(!contains_rtl("한국어"));
    }

    #[test]
    fn test_single_rtl_char() {
        assert!(!contains_rtl("a")); // a is not RTL
        assert!(contains_rtl("ا")); // Arabic Alef
        assert!(contains_rtl("א")); // Hebrew Alef
    }

    #[test]
    fn test_rtl_with_numbers() {
        assert!(contains_rtl("رقم 123"));
        assert!(contains_rtl("123 رقم"));
    }

    #[test]
    fn test_rtl_with_punctuation() {
        assert!(contains_rtl("مرحبا!"));
        assert!(contains_rtl("?שלום"));
    }
}

// ============================================
// === detect_text_direction Tests ===
// ============================================

mod detect_text_direction_tests {
    use super::*;

    #[test]
    fn test_pure_arabic_rtl() {
        assert_eq!(
            detect_text_direction("مرحبا بالعالم"),
            TextDirection::RightToLeft
        );
        assert_eq!(
            detect_text_direction("السلام عليكم ورحمة الله"),
            TextDirection::RightToLeft
        );
    }

    #[test]
    fn test_pure_hebrew_rtl() {
        assert_eq!(
            detect_text_direction("שלום עולם"),
            TextDirection::RightToLeft
        );
        assert_eq!(
            detect_text_direction("ברוכים הבאים"),
            TextDirection::RightToLeft
        );
    }

    #[test]
    fn test_pure_english_ltr() {
        assert_eq!(
            detect_text_direction("Hello World"),
            TextDirection::LeftToRight
        );
        assert_eq!(
            detect_text_direction("The quick brown fox"),
            TextDirection::LeftToRight
        );
    }

    #[test]
    fn test_mixed_more_rtl() {
        // When RTL characters exceed LTR characters, returns Mixed
        assert_eq!(
            detect_text_direction("مرحبا Hello"),
            TextDirection::Mixed
        );
    }

    #[test]
    fn test_mixed_more_ltr() {
        // When LTR characters exceed RTL characters
        assert_eq!(
            detect_text_direction("Hello World مرحبا"),
            TextDirection::Mixed
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(detect_text_direction(""), TextDirection::LeftToRight);
    }

    #[test]
    fn test_numbers_only() {
        assert_eq!(
            detect_text_direction("123456"),
            TextDirection::LeftToRight
        );
    }

    #[test]
    fn test_spaces_only() {
        assert_eq!(
            detect_text_direction("     "),
            TextDirection::LeftToRight
        );
    }

    #[test]
    fn test_punctuation_only() {
        assert_eq!(
            detect_text_direction("...!!!???"),
            TextDirection::LeftToRight
        );
    }

    #[test]
    fn test_single_rtl_char() {
        assert_eq!(
            detect_text_direction("ا"),
            TextDirection::RightToLeft
        );
    }

    #[test]
    fn test_single_ltr_char() {
        assert_eq!(
            detect_text_direction("A"),
            TextDirection::LeftToRight
        );
    }
}

// ============================================
// === TextDirection Enum Tests ===
// ============================================

mod text_direction_tests {
    use super::*;

    #[test]
    fn test_default() {
        let dir = TextDirection::default();
        assert_eq!(dir, TextDirection::LeftToRight);
    }

    #[test]
    fn test_equality() {
        assert_eq!(TextDirection::LeftToRight, TextDirection::LeftToRight);
        assert_eq!(TextDirection::RightToLeft, TextDirection::RightToLeft);
        assert_eq!(TextDirection::Mixed, TextDirection::Mixed);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(TextDirection::LeftToRight, TextDirection::RightToLeft);
        assert_ne!(TextDirection::RightToLeft, TextDirection::Mixed);
        assert_ne!(TextDirection::Mixed, TextDirection::LeftToRight);
    }

    #[test]
    fn test_clone() {
        let dir = TextDirection::RightToLeft;
        let cloned = dir.clone();
        assert_eq!(dir, cloned);
    }

    #[test]
    fn test_copy() {
        let dir = TextDirection::Mixed;
        let copied = dir;
        assert_eq!(dir, copied);
    }

    #[test]
    fn test_debug() {
        let debug_str = format!("{:?}", TextDirection::LeftToRight);
        assert!(debug_str.contains("LeftToRight"));

        let debug_str = format!("{:?}", TextDirection::RightToLeft);
        assert!(debug_str.contains("RightToLeft"));

        let debug_str = format!("{:?}", TextDirection::Mixed);
        assert!(debug_str.contains("Mixed"));
    }
}

// ============================================
// === RtlText Tests ===
// ============================================

mod rtl_text_tests {
    use super::*;

    #[test]
    fn test_new_arabic() {
        let rtl_text = RtlText::new("مرحبا");
        assert_eq!(rtl_text.text, "مرحبا");
        assert_eq!(rtl_text.direction, TextDirection::RightToLeft);
    }

    #[test]
    fn test_new_hebrew() {
        let rtl_text = RtlText::new("שלום");
        assert_eq!(rtl_text.text, "שלום");
        assert_eq!(rtl_text.direction, TextDirection::RightToLeft);
    }

    #[test]
    fn test_new_english() {
        let rtl_text = RtlText::new("Hello");
        assert_eq!(rtl_text.text, "Hello");
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_new_mixed() {
        let rtl_text = RtlText::new("Hello مرحبا");
        assert_eq!(rtl_text.text, "Hello مرحبا");
        assert_eq!(rtl_text.direction, TextDirection::Mixed);
    }

    #[test]
    fn test_new_empty() {
        let rtl_text = RtlText::new("");
        assert_eq!(rtl_text.text, "");
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_is_rtl_true() {
        let rtl_text = RtlText::new("مرحبا");
        assert!(rtl_text.is_rtl());
    }

    #[test]
    fn test_is_rtl_false() {
        let rtl_text = RtlText::new("Hello");
        assert!(!rtl_text.is_rtl());
    }

    #[test]
    fn test_is_mixed_true() {
        let rtl_text = RtlText::new("Hello מרחבא");
        assert!(rtl_text.is_mixed());
    }

    #[test]
    fn test_is_mixed_false() {
        let rtl_text = RtlText::new("מרחבא");
        assert!(!rtl_text.is_mixed());
    }

    #[test]
    fn test_alignment_rtl() {
        let rtl_text = RtlText::new("مرحبا");
        assert_eq!(rtl_text.alignment(), egui::Align::RIGHT);
    }

    #[test]
    fn test_alignment_ltr() {
        let rtl_text = RtlText::new("Hello");
        assert_eq!(rtl_text.alignment(), egui::Align::LEFT);
    }

    #[test]
    fn test_alignment_mixed() {
        let rtl_text = RtlText::new("Hello מרחבא");
        // Mixed defaults to LEFT alignment
        assert_eq!(rtl_text.alignment(), egui::Align::LEFT);
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_very_long_rtl_text() {
        let long_arabic = "مرحبا ".repeat(1000);
        assert!(contains_rtl(&long_arabic));
        assert_eq!(
            detect_text_direction(&long_arabic),
            TextDirection::RightToLeft
        );
    }

    #[test]
    fn test_very_long_mixed_text() {
        let mixed = "Hello مرحبا ".repeat(500);
        assert!(contains_rtl(&mixed));
        // Direction depends on character counts
        let dir = detect_text_direction(&mixed);
        assert!(matches!(dir, TextDirection::Mixed | TextDirection::RightToLeft));
    }

    #[test]
    fn test_rtl_with_newlines() {
        let text = "مرحبا\nשלום\nHello";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_rtl_with_tabs() {
        let text = "مرحبا\tשלום\tHello";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_rtl_in_code_comment() {
        let text = "// مرحبا - Arabic comment";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_url_with_rtl() {
        let text = "https://example.com/مرحبا";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_markdown_with_rtl() {
        let text = "# مرحبا\n\n**שלום**";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_null_char_in_text() {
        let text = "Hello\0مرحبا";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_unicode_boundary() {
        // Test characters at Unicode block boundaries
        assert!(is_rtl_char('\u{0590}')); // First Hebrew
        assert!(is_rtl_char('\u{05FF}')); // Last Hebrew (approx)
        assert!(is_rtl_char('\u{0600}')); // First Arabic
        assert!(is_rtl_char('\u{06FF}')); // Last Arabic (approx)
    }
}
