//! Editor view component that integrates egui UI with core-engine
//!
//! # Text Synchronization Strategy
//!
//! This module implements text synchronization between egui's TextEdit widget
//! and core-engine's Rope-based buffer using "方針A" (Approach A) as specified:
//!
//! 1. Convert Rope to String for display in TextEdit
//! 2. Detect changes after user input
//! 3. Send changes to core-engine via insert_char/delete_range APIs
//! 4. Synchronize cursor position bidirectionally
//!
//! ## Performance Considerations
//!
//! This approach is simple but may have performance issues with large files
//! due to String conversion overhead. If performance becomes a problem, we can
//! migrate to "方針B" (Approach B) which uses egui's LayoutJob/Galley APIs
//! to render directly from the Rope structure.
//!
//! ## Cursor Synchronization
//!
//! The cursor position is synchronized in both directions:
//! - egui -> core-engine: When user clicks or navigates with keyboard
//! - core-engine -> egui: After undo/redo operations
//!
//! ## Input Handling
//!
//! User input is handled through egui's TextEdit events:
//! - Character insertion: Detected by comparing text before/after
//! - Deletion: Detected similarly and mapped to delete_range
//! - The changes are then applied to core-engine's buffer

use core_engine::Editor;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Editor view state
pub struct EditorView {
    /// Reference to the core editor
    editor: Arc<Mutex<Editor>>,
    /// Cached text for egui TextEdit (converted from Rope)
    cached_text: String,
    /// Last known cursor position in egui
    cached_cursor_pos: usize,
}

impl EditorView {
    /// Create a new editor view
    pub fn new(editor: Arc<Mutex<Editor>>) -> Self {
        Self {
            editor,
            cached_text: String::new(),
            cached_cursor_pos: 0,
        }
    }

    /// Render the editor UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Get current state from core-engine
        let runtime = tokio::runtime::Handle::current();
        let editor = self.editor.clone();

        let (current_text, current_cursor) = runtime.block_on(async {
            let ed = editor.lock().await;
            (ed.text(), ed.cursor().position())
        });

        // Update cached text if it differs from core-engine
        // (This happens after undo/redo operations)
        if self.cached_text != current_text {
            self.cached_text = current_text;
            self.cached_cursor_pos = current_cursor;
        }

        // Create a TextEdit widget
        let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
            let mut layout_job = egui::text::LayoutJob::simple_singleline(
                text.to_owned(),
                egui::FontId::monospace(14.0),
                ui.style().visuals.text_color(),
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        let output = egui::TextEdit::multiline(&mut self.cached_text)
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .desired_rows(25)
            .layouter(&mut layouter)
            .show(ui);

        // Process text changes
        if output.response.changed() {
            let new_text = self.cached_text.clone();

            // Detect what changed and update core-engine
            let editor = self.editor.clone();
            runtime.spawn(async move {
                let mut ed = editor.lock().await;
                let old_text = ed.text();

                // Simple approach: Find the difference and apply it
                // This is a naive implementation; a more sophisticated diff
                // algorithm could be used for better performance
                if new_text != old_text {
                    // Find the common prefix
                    let prefix_len = old_text
                        .chars()
                        .zip(new_text.chars())
                        .take_while(|(a, b)| a == b)
                        .count();

                    // Find the common suffix
                    let old_suffix = &old_text.chars().skip(prefix_len).collect::<String>();
                    let new_suffix = &new_text.chars().skip(prefix_len).collect::<String>();

                    let suffix_len = old_suffix
                        .chars()
                        .rev()
                        .zip(new_suffix.chars().rev())
                        .take_while(|(a, b)| a == b)
                        .count();

                    let old_middle_len = old_text.chars().count() - prefix_len - suffix_len;
                    let new_middle: String = new_text
                        .chars()
                        .skip(prefix_len)
                        .take(new_text.chars().count() - prefix_len - suffix_len)
                        .collect();

                    // Apply the change to core-engine
                    if old_middle_len > 0 {
                        // Delete old content
                        ed.delete_range(prefix_len..prefix_len + old_middle_len);
                    }

                    if !new_middle.is_empty() {
                        // Set cursor to insertion point
                        ed.set_cursor_position(prefix_len);
                        // Insert new content
                        ed.insert_text(&new_middle);
                    }
                }
            });
        }

        // Sync cursor position from egui to core-engine
        if let Some(cursor_range) = output.cursor_range {
            let new_cursor_pos = cursor_range.primary.ccursor.index;
            if new_cursor_pos != self.cached_cursor_pos {
                self.cached_cursor_pos = new_cursor_pos;

                let editor = self.editor.clone();
                runtime.block_on(async move {
                    let mut ed = editor.lock().await;
                    ed.set_cursor_position(new_cursor_pos);
                });
            }
        }
    }
}
