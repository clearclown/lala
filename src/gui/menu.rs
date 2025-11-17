use eframe::egui;
use std::collections::HashMap;

use crate::core_engine::{Buffer, BufferId};
use crate::llm::GeminiClient;

use super::previews::PreviewMode;

/// Render the menu bar
#[allow(clippy::too_many_arguments)]
pub fn render_menu_bar(
    ui: &mut egui::Ui,
    new_file: &mut bool,
    open_file: &mut bool,
    save_file: &mut bool,
    save_as: &mut bool,
    show_search: &mut bool,
    show_replace: &mut bool,
    _show_grep: &mut bool,
    show_settings: &mut bool,
    is_light_theme: &mut bool,
    show_preview: &mut bool,
    preview_mode: &mut PreviewMode,
    llm_status: &str,
    llm_client: &Option<GeminiClient>,
    ai_enabled: bool,
    current_text: &mut String,
    text_changed: &mut bool,
    buffers: &HashMap<BufferId, Buffer>,
    active_buffer_id: Option<BufferId>,
) {
    egui::MenuBar::new().ui(ui, |ui| {
        // File menu
        ui.menu_button("File", |ui| {
            if ui.button("New (Ctrl+N)").clicked() {
                *new_file = true;
                ui.close();
            }
            if ui.button("Open (Ctrl+O)").clicked() {
                *open_file = true;
                ui.close();
            }
            if ui.button("Save (Ctrl+S)").clicked() {
                *save_file = true;
                ui.close();
            }
            if ui.button("Save As (Ctrl+Shift+S)").clicked() {
                *save_as = true;
                ui.close();
            }
        });

        // Edit menu
        ui.menu_button("Edit", |ui| {
            if ui.button("Find (Ctrl+F)").clicked() {
                *show_search = true;
                ui.close();
            }
            if ui.button("Replace (Ctrl+H)").clicked() {
                *show_search = true;
                *show_replace = true;
                ui.close();
            }
        });

        // Tools menu with AI features
        ui.menu_button("Tools", |ui| {
            // Settings button
            if ui.button("⚙️ Settings").clicked() {
                *show_settings = true;
                ui.close();
            }

            ui.separator();
            ui.label(llm_status);
            ui.separator();

            let can_use_llm = llm_client.is_some() && ai_enabled;

            // AI text editing features
            if ui
                .add_enabled(can_use_llm, egui::Button::new("🤖 Improve Markdown"))
                .clicked()
            {
                if let Some(client) = llm_client {
                    match client.improve_markdown(current_text) {
                        Ok(improved) => {
                            *current_text = improved;
                            *text_changed = true;
                        }
                        Err(e) => {
                            eprintln!("LLM Error: {e}");
                        }
                    }
                }
                ui.close();
            }

            if ui
                .add_enabled(can_use_llm, egui::Button::new("✨ Fix Grammar"))
                .clicked()
            {
                if let Some(client) = llm_client {
                    match client.improve_markdown(&format!(
                        "Fix grammar and spelling errors in this text:\n\n{current_text}"
                    )) {
                        Ok(improved) => {
                            *current_text = improved;
                            *text_changed = true;
                        }
                        Err(e) => {
                            eprintln!("LLM Error: {e}");
                        }
                    }
                }
                ui.close();
            }

            if ui
                .add_enabled(can_use_llm, egui::Button::new("📝 Summarize"))
                .clicked()
            {
                if let Some(client) = llm_client {
                    match client
                        .improve_markdown(&format!("Summarize this text concisely:\n\n{current_text}"))
                    {
                        Ok(summary) => {
                            *current_text = summary;
                            *text_changed = true;
                        }
                        Err(e) => {
                            eprintln!("LLM Error: {e}");
                        }
                    }
                }
                ui.close();
            }

            if !can_use_llm {
                ui.label("💡 Tip: Enable AI in Settings");
            }
        });

        // View menu
        ui.menu_button("View", |ui| {
            // Theme toggle
            let theme_label = if *is_light_theme {
                "🌙 Dark Theme"
            } else {
                "☀️ Light Theme"
            };
            if ui.button(theme_label).clicked() {
                *is_light_theme = !*is_light_theme;
                ui.close();
            }

            ui.separator();

            let preview_label = if *show_preview {
                "Hide Preview (Ctrl+P)"
            } else {
                "Show Preview (Ctrl+P)"
            };
            if ui.button(preview_label).clicked() {
                *show_preview = !*show_preview;
                ui.close();
            }

            ui.separator();
            ui.label("Preview Mode:");

            if ui
                .selectable_label(*preview_mode == PreviewMode::Markdown, "Markdown")
                .clicked()
            {
                *preview_mode = PreviewMode::Markdown;
                *show_preview = true;
                ui.close();
            }

            if ui
                .selectable_label(*preview_mode == PreviewMode::Html, "HTML")
                .clicked()
            {
                *preview_mode = PreviewMode::Html;
                *show_preview = true;
                ui.close();
            }

            if ui
                .selectable_label(*preview_mode == PreviewMode::Latex, "LaTeX")
                .clicked()
            {
                *preview_mode = PreviewMode::Latex;
                *show_preview = true;
                ui.close();
            }

            if ui
                .selectable_label(*preview_mode == PreviewMode::Mermaid, "Mermaid")
                .clicked()
            {
                *preview_mode = PreviewMode::Mermaid;
                *show_preview = true;
                ui.close();
            }
        });

        // Show file status
        if let Some(buffer_id) = active_buffer_id {
            if let Some(buffer) = buffers.get(&buffer_id) {
                let file_name = buffer
                    .file_path()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled");

                let dirty_marker = if *text_changed { " *" } else { "" };
                ui.separator();
                ui.label(format!("{file_name}{dirty_marker}"));
            }
        }
    });
}
