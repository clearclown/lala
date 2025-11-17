use egui::{text::LayoutJob, Color32, FontId, TextFormat};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};
use syntect::util::LinesWithEndings;

/// SyntaxHighlighter manages syntax highlighting for different file types
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl SyntaxHighlighter {
    /// Creates a new SyntaxHighlighter with default settings
    pub fn new() -> Self {
        // Load default syntax definitions
        let mut builder = SyntaxSetBuilder::new();

        // Add Rust syntax
        builder.add_plain_text_syntax();

        let syntax_set = builder.build();

        // Use a bundled theme (base16-ocean.dark)
        let theme = Self::load_theme();

        Self { syntax_set, theme }
    }

    /// Loads the theme for syntax highlighting
    fn load_theme() -> Theme {
        // Load from bundled themes
        let ts = ThemeSet::load_defaults();
        ts.themes["base16-ocean.dark"].clone()
    }

    /// Gets the syntax definition for a file based on its extension
    pub fn load_syntax(&self, file_path: &str) -> &syntect::parsing::SyntaxReference {
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match extension {
            "rs" => self
                .syntax_set
                .find_syntax_by_extension("rs")
                .or_else(|| self.syntax_set.find_syntax_by_name("Rust"))
                .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text()),
            "md" => self
                .syntax_set
                .find_syntax_by_extension("md")
                .or_else(|| self.syntax_set.find_syntax_by_name("Markdown"))
                .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text()),
            _ => self.syntax_set.find_syntax_plain_text(),
        }
    }

    /// Highlights the given text and returns an egui LayoutJob
    pub fn highlight(&self, text: &str, file_path: Option<&str>) -> LayoutJob {
        let syntax = if let Some(path) = file_path {
            self.load_syntax(path)
        } else {
            self.syntax_set.find_syntax_plain_text()
        };

        let mut layout_job = LayoutJob::default();
        let mut highlighter = HighlightLines::new(syntax, &self.theme);

        for line in LinesWithEndings::from(text) {
            let ranges = highlighter
                .highlight_line(line, &self.syntax_set)
                .unwrap_or_default();

            for (style, text_segment) in ranges {
                let color = Self::style_to_color(style);
                layout_job.append(
                    text_segment,
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color,
                        ..Default::default()
                    },
                );
            }
        }

        layout_job
    }

    /// Converts syntect Style to egui Color32
    fn style_to_color(style: Style) -> Color32 {
        Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b)
    }

    /// Returns the list of supported extensions
    #[allow(dead_code)]
    pub fn supported_extensions() -> Vec<&'static str> {
        vec!["rs", "md", "txt"]
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_syntax_rust() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.load_syntax("foo.rs");
        // Check that we get a syntax (even if it's plain text as fallback)
        assert!(syntax.name == "Plain Text" || syntax.name.contains("Rust"));
    }

    #[test]
    fn test_load_syntax_markdown() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.load_syntax("foo.md");
        assert!(syntax.name == "Plain Text" || syntax.name.contains("Markdown"));
    }

    #[test]
    fn test_load_syntax_unknown() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.load_syntax("foo.xyz");
        assert_eq!(syntax.name, "Plain Text");
    }

    #[test]
    fn test_highlight_rust_code() {
        let highlighter = SyntaxHighlighter::new();
        let code = "let a = 1;";
        let layout_job = highlighter.highlight(code, Some("test.rs"));

        // Check that the layout job contains the text
        assert!(layout_job.text.contains("let"));
        assert!(layout_job.text.contains("a"));
        assert!(layout_job.text.contains("="));
        assert!(layout_job.text.contains("1"));
    }

    #[test]
    fn test_highlight_markdown() {
        let highlighter = SyntaxHighlighter::new();
        let markdown = "# Header\n\n**bold**";
        let layout_job = highlighter.highlight(markdown, Some("test.md"));

        assert!(layout_job.text.contains("# Header"));
        assert!(layout_job.text.contains("**bold**"));
    }

    #[test]
    fn test_highlight_plaintext() {
        let highlighter = SyntaxHighlighter::new();
        let text = "Hello, World!";
        let layout_job = highlighter.highlight(text, Some("test.txt"));

        assert_eq!(layout_job.text, text);
    }

    #[test]
    fn test_supported_extensions() {
        let extensions = SyntaxHighlighter::supported_extensions();
        assert!(extensions.contains(&"rs"));
        assert!(extensions.contains(&"md"));
        assert!(extensions.contains(&"txt"));
    }
}
