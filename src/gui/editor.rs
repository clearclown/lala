use eframe::egui;

/// Text editor component - feature/basic-editing
/// Provides a simple multi-line text editor with change detection
pub struct Editor {
    content: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: String::from("# Welcome to Lala\n\nStart typing your **Markdown** here!\n\n## Features\n\n- Real-time preview\n- GitHub-style rendering\n- Code blocks\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n\n### Lists\n\n1. First item\n2. Second item\n3. Third item\n\n*Italic text* and **bold text** are supported.\n"),
        }
    }

    /// Clear the editor content
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// Get the current content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Render the editor UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let response = ui.add(
            egui::TextEdit::multiline(&mut self.content)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_width(f32::INFINITY)
                .desired_rows(30),
        );

        // Request repaint if the text changed (for real-time preview update)
        if response.changed() {
            ui.ctx().request_repaint();
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_new() {
        let editor = Editor::new();
        assert!(!editor.content().is_empty());
    }

    #[test]
    fn test_editor_clear() {
        let mut editor = Editor::new();
        editor.clear();
        assert!(editor.content().is_empty());
    }

    #[test]
    fn test_editor_content() {
        let editor = Editor::new();
        let content = editor.content();
        assert!(content.contains("Welcome"));
    }
}
