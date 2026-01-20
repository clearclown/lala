/// Comprehensive tests for GUI components
///
/// Tests cover:
/// - Theme functionality (light/dark themes)
/// - RTL text detection and handling
/// - Editor panel state management
/// - Preview mode detection
use lala::gui::{TextDirection, RtlText};

// === RTL Text Detection Tests ===

mod rtl_tests {
    use super::*;

    #[test]
    fn test_arabic_text_detection() {
        let arabic = "مرحبا بالعالم";
        let rtl_text = RtlText::new(arabic);
        
        assert!(rtl_text.is_rtl());
        assert!(!rtl_text.is_mixed());
        assert_eq!(rtl_text.direction, TextDirection::RightToLeft);
    }

    #[test]
    fn test_hebrew_text_detection() {
        let hebrew = "שלום עולם";
        let rtl_text = RtlText::new(hebrew);
        
        assert!(rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::RightToLeft);
    }

    #[test]
    fn test_english_text_detection() {
        let english = "Hello World";
        let rtl_text = RtlText::new(english);
        
        assert!(!rtl_text.is_rtl());
        assert!(!rtl_text.is_mixed());
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_mixed_text_detection() {
        let mixed = "Hello مرحبا World";
        let rtl_text = RtlText::new(mixed);
        
        assert!(!rtl_text.is_rtl());
        assert!(rtl_text.is_mixed());
        assert_eq!(rtl_text.direction, TextDirection::Mixed);
    }

    #[test]
    fn test_empty_text_detection() {
        let empty = "";
        let rtl_text = RtlText::new(empty);
        
        assert!(!rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_numbers_only_text() {
        let numbers = "123 456 789";
        let rtl_text = RtlText::new(numbers);
        
        // Numbers alone should default to LTR
        assert!(!rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_japanese_text_is_not_rtl() {
        let japanese = "こんにちは世界";
        let rtl_text = RtlText::new(japanese);
        
        // Japanese is LTR
        assert!(!rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_chinese_text_is_not_rtl() {
        let chinese = "你好世界";
        let rtl_text = RtlText::new(chinese);
        
        // Chinese is LTR
        assert!(!rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_persian_text_detection() {
        // Persian uses Arabic script
        let persian = "سلام دنیا";
        let rtl_text = RtlText::new(persian);
        
        assert!(rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::RightToLeft);
    }

    #[test]
    fn test_urdu_text_detection() {
        // Urdu uses Arabic script
        let urdu = "ہیلو دنیا";
        let rtl_text = RtlText::new(urdu);
        
        assert!(rtl_text.is_rtl());
        assert_eq!(rtl_text.direction, TextDirection::RightToLeft);
    }

    #[test]
    fn test_arabic_with_numbers() {
        let arabic_num = "١٢٣ مرحبا";
        let rtl_text = RtlText::new(arabic_num);
        
        assert!(rtl_text.is_rtl());
    }
}

// === RTL Character Detection Tests ===

mod rtl_char_tests {
    use lala::gui::rtl::{is_rtl_char, contains_rtl};

    #[test]
    fn test_arabic_alef() {
        assert!(is_rtl_char('ا'));
    }

    #[test]
    fn test_arabic_ba() {
        assert!(is_rtl_char('ب'));
    }

    #[test]
    fn test_hebrew_alef() {
        assert!(is_rtl_char('א'));
    }

    #[test]
    fn test_hebrew_bet() {
        assert!(is_rtl_char('ב'));
    }

    #[test]
    fn test_latin_chars_not_rtl() {
        assert!(!is_rtl_char('a'));
        assert!(!is_rtl_char('Z'));
        assert!(!is_rtl_char('0'));
    }

    #[test]
    fn test_contains_rtl_arabic() {
        assert!(contains_rtl("مرحبا"));
        assert!(contains_rtl("Hello مرحبا World"));
    }

    #[test]
    fn test_contains_rtl_hebrew() {
        assert!(contains_rtl("שלום"));
    }

    #[test]
    fn test_contains_rtl_negative() {
        assert!(!contains_rtl("Hello World"));
        assert!(!contains_rtl("こんにちは"));
        assert!(!contains_rtl("12345"));
    }
}

// === Theme Tests ===

mod theme_tests {
    use lala::gui::{custom_light_theme, custom_dark_theme, ThemeMode, EditorColors, MarkdownPreviewColors};

    #[test]
    fn test_light_theme_creation() {
        let theme = custom_light_theme();
        
        // Light theme should have light backgrounds
        assert!(theme.window_fill.r() > 200);
        assert!(theme.window_fill.g() > 200);
        assert!(theme.window_fill.b() > 200);
    }

    #[test]
    fn test_dark_theme_creation() {
        let theme = custom_dark_theme();
        
        // Dark theme should have dark backgrounds
        assert!(theme.window_fill.r() < 50);
        assert!(theme.window_fill.g() < 50);
        assert!(theme.window_fill.b() < 50);
    }

    #[test]
    fn test_theme_mode_default() {
        let default = ThemeMode::default();
        
        // Default should be dark for code editing
        assert_eq!(default, ThemeMode::Dark);
    }

    #[test]
    fn test_editor_colors_light() {
        let colors = EditorColors::light();
        
        // Light colors should have light background
        assert!(colors.background.r() > 200);
        // And dark foreground
        assert!(colors.foreground.r() < 50);
    }

    #[test]
    fn test_editor_colors_dark() {
        let colors = EditorColors::dark();
        
        // Dark colors should have dark background
        assert!(colors.background.r() < 50);
        // And light foreground
        assert!(colors.foreground.r() > 200);
    }

    #[test]
    fn test_markdown_preview_colors_light() {
        let colors = MarkdownPreviewColors::light();
        
        // Light colors should have light background
        assert!(colors.background.r() > 200);
        // Heading should be dark
        assert!(colors.heading.r() < 50);
    }

    #[test]
    fn test_markdown_preview_colors_dark() {
        let colors = MarkdownPreviewColors::dark();
        
        // Dark colors should have dark background
        assert!(colors.background.r() < 50);
        // Heading should be light
        assert!(colors.heading.r() > 200);
    }

    #[test]
    fn test_light_theme_has_proper_shadow() {
        let theme = custom_light_theme();
        
        // Should have shadow for modern look (egui 0.33 API)
        assert!(theme.window_shadow.blur > 0);
    }

    #[test]
    fn test_dark_theme_has_proper_shadow() {
        let theme = custom_dark_theme();
        
        // Should have shadow for modern look (egui 0.33 API)
        assert!(theme.window_shadow.blur > 0);
    }
}

// === Preview Mode Detection Tests ===

mod preview_mode_tests {
    use std::path::PathBuf;

    /// Helper function to detect preview mode from path
    fn detect_preview_mode_from_path(path: Option<&PathBuf>) -> &'static str {
        path.and_then(|p| p.extension())
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "md" | "markdown" => "Markdown",
                "html" | "htm" => "Html",
                "tex" | "latex" => "Latex",
                "mmd" | "mermaid" => "Mermaid",
                _ => "None",
            })
            .unwrap_or("None")
    }

    #[test]
    fn test_detect_markdown_file() {
        let path = PathBuf::from("test.md");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "Markdown");
    }

    #[test]
    fn test_detect_markdown_long_extension() {
        let path = PathBuf::from("test.markdown");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "Markdown");
    }

    #[test]
    fn test_detect_html_file() {
        let path = PathBuf::from("index.html");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "Html");
    }

    #[test]
    fn test_detect_htm_file() {
        let path = PathBuf::from("page.htm");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "Html");
    }

    #[test]
    fn test_detect_latex_file() {
        let path = PathBuf::from("document.tex");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "Latex");
    }

    #[test]
    fn test_detect_mermaid_file() {
        let path = PathBuf::from("diagram.mmd");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "Mermaid");
    }

    #[test]
    fn test_detect_unknown_file() {
        let path = PathBuf::from("file.txt");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "None");
    }

    #[test]
    fn test_detect_no_path() {
        assert_eq!(detect_preview_mode_from_path(None), "None");
    }

    #[test]
    fn test_detect_no_extension() {
        let path = PathBuf::from("README");
        assert_eq!(detect_preview_mode_from_path(Some(&path)), "None");
    }
}

// === Markdown Preview Enhanced Tests ===

mod markdown_preview_tests {
    use pulldown_cmark::{Event, Options, Parser, Tag};

    #[test]
    fn test_parse_blockquote() {
        let markdown = "> This is a quote";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::BlockQuote(_)))));
    }

    #[test]
    fn test_parse_nested_blockquote() {
        let markdown = "> Level 1\n>> Level 2";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        // Should contain blockquotes
        let blockquote_count = events.iter()
            .filter(|e| matches!(e, Event::Start(Tag::BlockQuote(_))))
            .count();
        assert!(blockquote_count >= 1);
    }

    #[test]
    fn test_parse_task_list() {
        let markdown = "- [ ] Task 1\n- [x] Task 2";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        // Should contain task list markers
        let has_task_marker = events.iter().any(|e| matches!(e, Event::TaskListMarker(_)));
        assert!(has_task_marker);
    }

    #[test]
    fn test_parse_image() {
        let markdown = "![Alt text](image.png)";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Image { .. }))));
    }

    #[test]
    fn test_parse_strikethrough() {
        let markdown = "~~strikethrough~~";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Strikethrough))));
    }

    #[test]
    fn test_parse_table() {
        let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Table(_)))));
    }

    #[test]
    fn test_parse_footnote() {
        let markdown = "Text[^1]\n\n[^1]: Footnote content";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::FootnoteDefinition(_)))));
    }

    #[test]
    fn test_hard_break() {
        let markdown = "Line 1  \nLine 2";  // Two spaces before newline
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::HardBreak)));
    }

    #[test]
    fn test_soft_break() {
        let markdown = "Line 1\nLine 2";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::SoftBreak)));
    }

    #[test]
    fn test_parse_horizontal_rule() {
        let markdown = "---";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Rule)));
    }

    #[test]
    fn test_rtl_in_markdown() {
        use lala::gui::rtl::contains_rtl;
        
        let markdown = "# مرحبا بالعالم\n\nهذا نص عربي";
        
        assert!(contains_rtl(markdown));
    }
}

// === Unicode and Character Handling Tests ===

mod unicode_tests {
    #[test]
    fn test_arabic_numerals() {
        let arabic_numerals = "٠١٢٣٤٥٦٧٨٩";
        assert_eq!(arabic_numerals.chars().count(), 10);
    }

    #[test]
    fn test_arabic_punctuation() {
        let text = "؟،؛";
        assert_eq!(text.chars().count(), 3);
    }

    #[test]
    fn test_combining_characters() {
        // Arabic with diacritics
        let text = "مَرْحَبًا";
        assert!(text.len() > text.chars().count());
    }

    #[test]
    fn test_bidirectional_text() {
        let mixed = "Hello مرحبا World";
        
        // Contains both LTR and RTL
        assert!(mixed.contains("Hello"));
        assert!(mixed.contains("مرحبا"));
    }

    #[test]
    fn test_emoji_with_rtl() {
        let text = "مرحبا 😀";
        assert!(text.contains('😀'));
    }
}

// === Comprehensive RTL Module Tests ===

mod rtl_comprehensive_tests {
    use lala::gui::rtl::{is_rtl_char, contains_rtl, detect_text_direction, TextDirection};
    use lala::gui::RtlText;

    // === Arabic Script Variants ===

    #[test]
    fn test_arabic_basic_letters() {
        // All basic Arabic letters should be RTL
        let letters = "ابتثجحخدذرزسشصضطظعغفقكلمنهوي";
        for c in letters.chars() {
            assert!(is_rtl_char(c), "Expected '{}' to be RTL", c);
        }
    }

    #[test]
    fn test_arabic_extended_letters() {
        // Arabic extended characters
        let extended = "پچڤگ"; // Persian additions
        for c in extended.chars() {
            assert!(is_rtl_char(c), "Expected '{}' to be RTL", c);
        }
    }

    #[test]
    fn test_arabic_presentation_forms() {
        // Arabic presentation forms (FB50-FDFF, FE70-FEFF)
        let forms = "ﺍﺏﺕﺙ"; // Isolated forms
        for c in forms.chars() {
            assert!(is_rtl_char(c), "Expected presentation form '{}' to be RTL", c);
        }
    }

    // === Hebrew Script Tests ===

    #[test]
    fn test_hebrew_basic_letters() {
        let letters = "אבגדהוזחטיכלמנסעפצקרשת";
        for c in letters.chars() {
            assert!(is_rtl_char(c), "Expected Hebrew '{}' to be RTL", c);
        }
    }

    #[test]
    fn test_hebrew_final_forms() {
        let finals = "ךםןףץ"; // Final forms
        for c in finals.chars() {
            assert!(is_rtl_char(c), "Expected Hebrew final '{}' to be RTL", c);
        }
    }

    // === Direction Detection Tests ===

    #[test]
    fn test_direction_pure_arabic() {
        let text = "مرحبا بالعالم كيف حالك";
        assert_eq!(detect_text_direction(text), TextDirection::RightToLeft);
    }

    #[test]
    fn test_direction_pure_hebrew() {
        let text = "שלום עולם מה נשמע";
        assert_eq!(detect_text_direction(text), TextDirection::RightToLeft);
    }

    #[test]
    fn test_direction_pure_english() {
        let text = "Hello World How Are You";
        assert_eq!(detect_text_direction(text), TextDirection::LeftToRight);
    }

    #[test]
    fn test_direction_mixed_mostly_rtl() {
        // More RTL than LTR
        let text = "مرحبا بالعالم Hello كيف حالك";
        let direction = detect_text_direction(text);
        // RTL dominates
        assert!(matches!(direction, TextDirection::RightToLeft | TextDirection::Mixed));
    }

    #[test]
    fn test_direction_mixed_mostly_ltr() {
        // More LTR than RTL
        let text = "Hello World مرحبا How Are You";
        let direction = detect_text_direction(text);
        // Mixed because both present
        assert!(matches!(direction, TextDirection::LeftToRight | TextDirection::Mixed));
    }

    #[test]
    fn test_direction_whitespace_only() {
        let text = "   \t\n  ";
        assert_eq!(detect_text_direction(text), TextDirection::LeftToRight);
    }

    #[test]
    fn test_direction_punctuation_only() {
        let text = "!@#$%^&*()";
        assert_eq!(detect_text_direction(text), TextDirection::LeftToRight);
    }

    // === RtlText Struct Tests ===

    #[test]
    fn test_rtl_text_alignment_rtl() {
        let text = RtlText::new("مرحبا");
        assert_eq!(text.alignment(), egui::Align::RIGHT);
    }

    #[test]
    fn test_rtl_text_alignment_ltr() {
        let text = RtlText::new("Hello");
        assert_eq!(text.alignment(), egui::Align::LEFT);
    }

    #[test]
    fn test_rtl_text_alignment_mixed() {
        let text = RtlText::new("Hello مرحبا");
        assert_eq!(text.alignment(), egui::Align::LEFT);
    }

    // === Edge Cases ===

    #[test]
    fn test_single_rtl_char() {
        assert!(is_rtl_char('ا'));
        let text = RtlText::new("ا");
        assert!(text.is_rtl());
    }

    #[test]
    fn test_single_ltr_char() {
        assert!(!is_rtl_char('a'));
        let text = RtlText::new("a");
        assert!(!text.is_rtl());
    }

    #[test]
    fn test_rtl_with_numbers() {
        // Numbers are neutral
        let text = "مرحبا 123 عالم";
        assert!(contains_rtl(text));
        let rtl_text = RtlText::new(text);
        assert!(rtl_text.is_rtl());
    }

    #[test]
    fn test_rtl_with_punctuation() {
        let text = "مرحبا! كيف حالك؟";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_yiddish_text() {
        // Yiddish uses Hebrew script
        let text = "אַ גוטן טאָג";
        assert!(contains_rtl(text));
        assert_eq!(detect_text_direction(text), TextDirection::RightToLeft);
    }

    #[test]
    fn test_pashto_text() {
        // Pashto uses Arabic script
        let text = "سلام";
        assert!(contains_rtl(text));
    }

    #[test]
    fn test_kurdish_text() {
        // Kurdish can use Arabic script
        let text = "سڵاو";
        assert!(contains_rtl(text));
    }
}

// === Additional Theme Tests ===

mod theme_comprehensive_tests {
    use lala::gui::{custom_light_theme, custom_dark_theme, EditorColors, MarkdownPreviewColors};

    #[test]
    fn test_light_theme_panel_fill() {
        let theme = custom_light_theme();
        // Panel should be slightly different from window
        assert!(theme.panel_fill.r() > 200);
    }

    #[test]
    fn test_dark_theme_panel_fill() {
        let theme = custom_dark_theme();
        assert!(theme.panel_fill.r() < 50);
    }

    #[test]
    fn test_light_theme_selection_color() {
        let theme = custom_light_theme();
        // Selection should have visible fill
        assert!(theme.selection.bg_fill.a() > 100);
    }

    #[test]
    fn test_dark_theme_selection_color() {
        let theme = custom_dark_theme();
        assert!(theme.selection.bg_fill.a() > 100);
    }

    #[test]
    fn test_light_theme_hyperlink_color() {
        let theme = custom_light_theme();
        // Hyperlink should be blue-ish
        assert!(theme.hyperlink_color.b() > theme.hyperlink_color.r());
    }

    #[test]
    fn test_dark_theme_hyperlink_color() {
        let theme = custom_dark_theme();
        assert!(theme.hyperlink_color.b() > theme.hyperlink_color.r());
    }

    #[test]
    fn test_editor_colors_selection_visible() {
        let light = EditorColors::light();
        let dark = EditorColors::dark();
        
        // Selection should be different from background
        assert_ne!(light.selection, light.background);
        assert_ne!(dark.selection, dark.background);
    }

    #[test]
    fn test_editor_colors_cursor_visible_on_light() {
        let light = EditorColors::light();
        // Cursor should be visible against light background
        let contrast = (light.background.r() as i32 - light.cursor.r() as i32).abs() +
                       (light.background.g() as i32 - light.cursor.g() as i32).abs() +
                       (light.background.b() as i32 - light.cursor.b() as i32).abs();
        assert!(contrast > 200, "Cursor should have sufficient contrast");
    }

    #[test]
    fn test_editor_colors_cursor_visible_on_dark() {
        let dark = EditorColors::dark();
        let contrast = (dark.background.r() as i32 - dark.cursor.r() as i32).abs() +
                       (dark.background.g() as i32 - dark.cursor.g() as i32).abs() +
                       (dark.background.b() as i32 - dark.cursor.b() as i32).abs();
        assert!(contrast > 200, "Cursor should have sufficient contrast");
    }

    #[test]
    fn test_markdown_colors_code_bg_distinct() {
        let light = MarkdownPreviewColors::light();
        let dark = MarkdownPreviewColors::dark();
        
        // Code background should be slightly different from main background
        assert_ne!(light.code_bg, light.background);
        assert_ne!(dark.code_bg, dark.background);
    }

    #[test]
    fn test_markdown_colors_blockquote_visible() {
        let light = MarkdownPreviewColors::light();
        let dark = MarkdownPreviewColors::dark();
        
        // Blockquote border should be visible
        assert!(light.blockquote_border.a() > 100);
        assert!(dark.blockquote_border.a() > 100);
    }
}

// === Markdown Preview Comprehensive Tests ===

mod markdown_comprehensive_tests {
    use pulldown_cmark::{Event, Options, Parser, Tag, HeadingLevel, CodeBlockKind};

    #[test]
    fn test_heading_levels() {
        for level in 1..=6 {
            let markdown = format!("{} Heading {}", "#".repeat(level), level);
            let parser = Parser::new_ext(&markdown, Options::all());
            let events: Vec<Event> = parser.collect();
            
            let has_heading = events.iter().any(|e| matches!(e, Event::Start(Tag::Heading { .. })));
            assert!(has_heading, "Should parse heading level {}", level);
        }
    }

    #[test]
    fn test_code_block_with_language() {
        let markdown = "```rust\nfn main() {}\n```";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        let code_block = events.iter().find(|e| matches!(e, Event::Start(Tag::CodeBlock(_))));
        assert!(code_block.is_some());
        
        if let Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))) = code_block {
            assert_eq!(lang.as_ref(), "rust");
        }
    }

    #[test]
    fn test_code_block_without_language() {
        let markdown = "```\ncode\n```";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::CodeBlock(_)))));
    }

    #[test]
    fn test_inline_code() {
        let markdown = "This is `inline code` here";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        let code = events.iter().find(|e| matches!(e, Event::Code(_)));
        assert!(code.is_some());
        if let Some(Event::Code(text)) = code {
            assert_eq!(text.as_ref(), "inline code");
        }
    }

    #[test]
    fn test_emphasis() {
        let markdown = "*italic*";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Emphasis))));
    }

    #[test]
    fn test_strong() {
        let markdown = "**bold**";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Strong))));
    }

    #[test]
    fn test_link() {
        let markdown = "[link text](https://example.com)";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        let link = events.iter().find(|e| matches!(e, Event::Start(Tag::Link { .. })));
        assert!(link.is_some());
    }

    #[test]
    fn test_unordered_list() {
        let markdown = "- item 1\n- item 2\n- item 3";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        let list = events.iter().find(|e| matches!(e, Event::Start(Tag::List(None))));
        assert!(list.is_some());
    }

    #[test]
    fn test_ordered_list() {
        let markdown = "1. item 1\n2. item 2\n3. item 3";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        let list = events.iter().find(|e| matches!(e, Event::Start(Tag::List(Some(_)))));
        assert!(list.is_some());
    }

    #[test]
    fn test_nested_list() {
        let markdown = "- item 1\n  - nested 1\n  - nested 2\n- item 2";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        let list_count = events.iter()
            .filter(|e| matches!(e, Event::Start(Tag::List(_))))
            .count();
        assert!(list_count >= 2, "Should have nested lists");
    }

    #[test]
    fn test_complex_markdown() {
        let markdown = r#"# Title

Paragraph with **bold** and *italic* and `code`.

## Subtitle

- List item 1
- List item 2

> Blockquote

```rust
fn main() {}
```

[Link](https://example.com)
"#;
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();
        
        // Should have all elements
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Heading { level: HeadingLevel::H1, .. }))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Heading { level: HeadingLevel::H2, .. }))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Strong))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Emphasis))));
        assert!(events.iter().any(|e| matches!(e, Event::Code(_))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::List(_)))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::BlockQuote(_)))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::CodeBlock(_)))));
        assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Link { .. }))));
    }
}
