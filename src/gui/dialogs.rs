use eframe::egui;
use std::path::PathBuf;

use crate::llm::GeminiClient;

/// File open dialog
pub fn show_file_dialog(
    ctx: &egui::Context,
    show: &mut bool,
    file_path_input: &mut String,
) -> Option<PathBuf> {
    let mut should_open = None;
    let mut should_close = false;

    let mut is_open = *show;
    egui::Window::new("Open File")
        .open(&mut is_open)
        .default_width(500.0)
        .show(ctx, |ui| {
            ui.heading("Open File");
            ui.separator();

            // Quick access buttons
            ui.horizontal(|ui| {
                if ui.button("🏠 Home").clicked() {
                    if let Some(home) = dirs::home_dir() {
                        *file_path_input = home.display().to_string();
                    }
                }
                if ui.button("📁 Documents").clicked() {
                    if let Some(docs) = dirs::document_dir() {
                        *file_path_input = docs.display().to_string();
                    }
                }
                if ui.button("💻 Desktop").clicked() {
                    if let Some(desktop) = dirs::desktop_dir() {
                        *file_path_input = desktop.display().to_string();
                    }
                }
            });

            ui.separator();

            // Current directory
            ui.label("Current working directory:");
            if let Ok(cwd) = std::env::current_dir() {
                if ui.button(cwd.display().to_string()).clicked() {
                    *file_path_input = cwd.display().to_string();
                }
            }

            ui.separator();

            // File path input
            ui.label("File path:");
            ui.text_edit_singleline(file_path_input);

            ui.label("Examples:");
            ui.monospace("  ./README.md");
            ui.monospace("  /home/user/documents/file.txt");
            ui.monospace("  ~/Documents/file.md");

            ui.separator();

            // Directory browser
            ui.label("Recent directories:");
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    // Show some common paths
                    let paths = vec![
                        ("Current Directory", std::env::current_dir().ok()),
                        ("Home", dirs::home_dir()),
                        ("Documents", dirs::document_dir()),
                        ("Downloads", dirs::download_dir()),
                    ];

                    for (name, path_opt) in paths {
                        if let Some(path) = path_opt {
                            if ui
                                .button(format!("📂 {}: {}", name, path.display()))
                                .clicked()
                            {
                                *file_path_input = path.display().to_string();
                            }
                        }
                    }
                });

            ui.separator();

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("✓ Open").clicked() {
                    should_open = Some(PathBuf::from(file_path_input.clone()));
                    should_close = true;
                }
                if ui.button("✗ Cancel").clicked() {
                    should_close = true;
                }
            });
        });

    *show = is_open;

    if should_close {
        *show = false;
        file_path_input.clear();
    }

    should_open
}

/// Save as dialog
pub fn show_save_as_dialog(
    ctx: &egui::Context,
    show: &mut bool,
    file_path_input: &mut String,
) -> Option<PathBuf> {
    let mut should_save = None;
    let mut should_close = false;

    let mut is_open = *show;
    egui::Window::new("Save As")
        .open(&mut is_open)
        .default_width(500.0)
        .show(ctx, |ui| {
            ui.heading("Save As");
            ui.separator();

            // Quick access buttons
            ui.horizontal(|ui| {
                if ui.button("🏠 Home").clicked() {
                    if let Some(home) = dirs::home_dir() {
                        *file_path_input = home.display().to_string() + "/";
                    }
                }
                if ui.button("📁 Documents").clicked() {
                    if let Some(docs) = dirs::document_dir() {
                        *file_path_input = docs.display().to_string() + "/";
                    }
                }
                if ui.button("💻 Desktop").clicked() {
                    if let Some(desktop) = dirs::desktop_dir() {
                        *file_path_input = desktop.display().to_string() + "/";
                    }
                }
            });

            ui.separator();

            // File path input
            ui.label("Save as:");
            ui.text_edit_singleline(file_path_input);

            ui.label("Examples:");
            ui.monospace("  ./myfile.md");
            ui.monospace("  /home/user/documents/newfile.txt");
            ui.monospace("  ~/Documents/document.tex");

            ui.separator();

            // Common locations
            ui.label("Save to common locations:");
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    let paths = vec![
                        ("Current Directory", std::env::current_dir().ok()),
                        ("Home", dirs::home_dir()),
                        ("Documents", dirs::document_dir()),
                        ("Downloads", dirs::download_dir()),
                        ("Desktop", dirs::desktop_dir()),
                    ];

                    for (name, path_opt) in paths {
                        if let Some(path) = path_opt {
                            if ui.button(format!("📂 {name}")).clicked() {
                                *file_path_input = path.display().to_string() + "/";
                            }
                        }
                    }
                });

            ui.separator();

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("💾 Save").clicked() {
                    should_save = Some(PathBuf::from(file_path_input.clone()));
                    should_close = true;
                }
                if ui.button("✗ Cancel").clicked() {
                    should_close = true;
                }
            });
        });

    *show = is_open;

    if should_close {
        *show = false;
        file_path_input.clear();
    }

    should_save
}

/// Settings dialog
pub fn show_settings(
    ctx: &egui::Context,
    show: &mut bool,
    api_key_input: &mut String,
    ai_enabled: &mut bool,
    llm_client: &mut Option<GeminiClient>,
    llm_status: &mut String,
) {
    let mut is_open = *show;
    egui::Window::new("Settings")
        .open(&mut is_open)
        .default_width(500.0)
        .show(ctx, |ui| {
            ui.heading("AI Settings");
            ui.separator();

            // AI Enable/Disable toggle
            ui.horizontal(|ui| {
                ui.label("Enable AI Features:");
                if ui.checkbox(ai_enabled, "").changed() {
                    if !*ai_enabled {
                        *llm_status = "AI features disabled".to_string();
                    } else if llm_client.is_some() {
                        *llm_status = "LLM ready (Gemini 1.5 Flash)".to_string();
                    } else {
                        *llm_status = "Enter API key to enable".to_string();
                    }
                }
            });

            ui.add_space(10.0);

            // API Key input
            ui.label("Gemini API Key:");
            ui.horizontal(|ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(api_key_input)
                        .hint_text("Enter your Gemini API key")
                        .password(true)
                        .desired_width(300.0),
                );

                if ui.button("Apply").clicked()
                    || response.lost_focus() && !api_key_input.is_empty()
                {
                    // Try to create client with new API key
                    match GeminiClient::new(api_key_input.clone()) {
                        Ok(client) => {
                            *llm_client = Some(client);
                            *llm_status = "LLM ready (Gemini 1.5 Flash)".to_string();
                            *ai_enabled = true;
                        }
                        Err(e) => {
                            *llm_status = format!("Error: {e}");
                            *llm_client = None;
                        }
                    }
                }
            });

            ui.add_space(10.0);

            // Status
            ui.label("Status:");
            ui.monospace(llm_status);

            ui.add_space(10.0);

            // Help text
            ui.label("How to get API key:");
            ui.hyperlink_to(
                "Get Gemini API Key →",
                "https://ai.google.dev/tutorials/setup",
            );

            ui.add_space(10.0);
            ui.separator();

            ui.heading("Available AI Features");
            ui.label("• Improve Markdown - Enhance formatting and structure");
            ui.label("• Fix Grammar - Correct spelling and grammar errors");
            ui.label("• Summarize - Create concise summaries");
        });

    *show = is_open;
}
