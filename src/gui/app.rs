use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::core_engine::{Buffer, BufferId};
use crate::file_tree::FileTree;
use crate::search::GrepEngine;
use crate::llm::GeminiClient;

use super::search_panel::SearchPanel;
use super::grep_panel::GrepPanel;
use super::markdown_preview;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PreviewMode {
    Markdown,
    Html,
    Latex,
    Mermaid,
    None,
}

pub struct LalaApp {
    // Core components
    buffers: HashMap<BufferId, Buffer>,
    active_buffer_id: Option<BufferId>,
    next_buffer_id: usize,
    file_tree: FileTree,

    // Editor state
    current_text: String,
    text_changed: bool,

    // Search components
    search_panel: SearchPanel,
    grep_panel: GrepPanel,
    grep_engine: GrepEngine,

    // UI state
    show_search_panel: bool,
    show_grep_panel: bool,
    show_file_dialog: bool,
    show_save_as_dialog: bool,
    file_path_input: String,

    // Preview state
    show_preview: bool,
    preview_mode: PreviewMode,

    // Theme state
    is_light_theme: bool,

    // LLM integration (optional)
    llm_client: Option<GeminiClient>,
    llm_status: String,
}

impl LalaApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        // Create initial empty buffer
        let mut buffers = HashMap::new();
        let buffer_id = BufferId(0);
        let empty_text = "".to_string();

        let buffer = Buffer::from_string(buffer_id, empty_text.clone(), None);
        buffers.insert(buffer_id, buffer);

        // Try to initialize LLM client from environment
        let (llm_client, llm_status) = match GeminiClient::from_env() {
            Ok(client) => (Some(client), "LLM ready (Gemini 1.5 Flash)".to_string()),
            Err(_) => (None, "LLM not available (set GEMINI_API_KEY)".to_string()),
        };

        Self {
            buffers,
            active_buffer_id: Some(buffer_id),
            next_buffer_id: 1,
            file_tree: FileTree::default(),
            current_text: empty_text,
            text_changed: false,
            search_panel: SearchPanel::new(),
            grep_panel: GrepPanel::new(),
            grep_engine: GrepEngine::new(),
            show_search_panel: false,
            show_grep_panel: false,
            show_file_dialog: false,
            show_save_as_dialog: false,
            file_path_input: String::new(),
            show_preview: false,
            preview_mode: PreviewMode::None,
            is_light_theme: false, // Default to dark theme
            llm_client,
            llm_status,
        }
    }

    fn detect_preview_mode(&self) -> PreviewMode {
        if let Some(buffer_id) = self.active_buffer_id {
            if let Some(buffer) = self.buffers.get(&buffer_id) {
                if let Some(path) = buffer.file_path() {
                    if let Some(ext) = path.extension() {
                        return match ext.to_str() {
                            Some("md") | Some("markdown") => PreviewMode::Markdown,
                            Some("html") | Some("htm") => PreviewMode::Html,
                            Some("tex") | Some("latex") => PreviewMode::Latex,
                            Some("mmd") | Some("mermaid") => PreviewMode::Mermaid,
                            _ => PreviewMode::None,
                        };
                    }
                }
            }
        }
        PreviewMode::None
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+S: Save file
        if ctx.input(|i| i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::S)) {
            self.save_file();
        }

        // Ctrl+Shift+S: Save as
        if ctx.input(|i| i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::S)) {
            self.show_save_as_dialog = true;
        }

        // Ctrl+O: Open file
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::O)) {
            self.show_file_dialog = true;
        }

        // Ctrl+N: New file
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::N)) {
            self.new_file();
        }

        // Ctrl+F: Open search panel
        if ctx.input(|i| i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::F)) {
            self.show_search_panel = true;
        }

        // Ctrl+H: Open replace panel
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::H)) {
            self.show_search_panel = true;
            self.search_panel.set_replace_mode(true);
        }

        // Ctrl+Shift+F: Open grep panel
        if ctx.input(|i| {
            i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::F)
        }) {
            self.show_grep_panel = true;
        }

        // Ctrl+P: Toggle preview
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::P)) {
            self.show_preview = !self.show_preview;
            if self.show_preview {
                // Auto-detect or keep current mode
                let detected = self.detect_preview_mode();
                if detected != PreviewMode::None {
                    self.preview_mode = detected;
                } else if self.preview_mode == PreviewMode::None {
                    // Default to Markdown if no file extension
                    self.preview_mode = PreviewMode::Markdown;
                }
            }
        }

        // Escape: Close panels
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_search_panel = false;
            self.show_grep_panel = false;
            self.show_file_dialog = false;
            self.show_save_as_dialog = false;
        }
    }

    fn save_file(&mut self) {
        if let Some(buffer_id) = self.active_buffer_id {
            let file_path = self.buffers.get(&buffer_id)
                .and_then(|b| b.file_path())
                .cloned();

            if let Some(file_path) = file_path {
                // Save to existing file
                if let Err(e) = std::fs::write(&file_path, &self.current_text) {
                    eprintln!("Failed to save file: {}", e);
                } else {
                    // Update buffer and mark as clean
                    if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                        *buffer = Buffer::from_string(buffer_id, self.current_text.clone(), Some(file_path));
                        self.text_changed = false;
                    }
                }
            } else {
                // No file path, open save as dialog
                self.show_save_as_dialog = true;
            }
        }
    }

    fn save_file_as(&mut self, path: PathBuf) {
        if let Some(buffer_id) = self.active_buffer_id {
            // Save to new file
            if let Err(e) = std::fs::write(&path, &self.current_text) {
                eprintln!("Failed to save file: {}", e);
            } else {
                // Update buffer with new path and mark as clean
                if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                    *buffer = Buffer::from_string(buffer_id, self.current_text.clone(), Some(path));
                    self.text_changed = false;
                }
            }
        }
    }

    fn open_file(&mut self, path: PathBuf) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let buffer_id = BufferId(self.next_buffer_id);
            self.next_buffer_id += 1;

            let buffer = Buffer::from_string(buffer_id, content.clone(), Some(path));
            self.buffers.insert(buffer_id, buffer);
            self.active_buffer_id = Some(buffer_id);
            self.current_text = content;
            self.text_changed = false;
        }
    }

    fn new_file(&mut self) {
        let buffer_id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;

        let buffer = Buffer::from_string(buffer_id, String::new(), None);
        self.buffers.insert(buffer_id, buffer);
        self.active_buffer_id = Some(buffer_id);
        self.current_text = String::new();
        self.text_changed = false;
    }

    fn show_main_editor(&mut self, ctx: &egui::Context) {
        // Update preview mode when file changes
        if self.show_preview && self.preview_mode == PreviewMode::None {
            self.preview_mode = self.detect_preview_mode();
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New (Ctrl+N)").clicked() {
                        self.new_file();
                        ui.close();
                    }
                    if ui.button("Open (Ctrl+O)").clicked() {
                        self.show_file_dialog = true;
                        ui.close();
                    }
                    if ui.button("Save (Ctrl+S)").clicked() {
                        self.save_file();
                        ui.close();
                    }
                    if ui.button("Save As (Ctrl+Shift+S)").clicked() {
                        self.show_save_as_dialog = true;
                        ui.close();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Find (Ctrl+F)").clicked() {
                        self.show_search_panel = true;
                        ui.close();
                    }
                    if ui.button("Replace (Ctrl+H)").clicked() {
                        self.show_search_panel = true;
                        self.search_panel.set_replace_mode(true);
                        ui.close();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    ui.label(&self.llm_status);
                    ui.separator();

                    let can_use_llm = self.llm_client.is_some();

                    if ui.add_enabled(can_use_llm, egui::Button::new("🤖 Improve Markdown")).clicked() {
                        if let Some(client) = &self.llm_client {
                            match client.improve_markdown(&self.current_text) {
                                Ok(improved) => {
                                    self.current_text = improved;
                                    self.text_changed = true;
                                }
                                Err(e) => {
                                    eprintln!("LLM Error: {}", e);
                                }
                            }
                        }
                        ui.close();
                    }

                    if !can_use_llm {
                        ui.label("💡 Tip: Set GEMINI_API_KEY to enable");
                    }
                });

                ui.menu_button("View", |ui| {
                    // Theme toggle
                    let theme_label = if self.is_light_theme {
                        "🌙 Dark Theme"
                    } else {
                        "☀️ Light Theme"
                    };
                    if ui.button(theme_label).clicked() {
                        self.is_light_theme = !self.is_light_theme;
                        ui.close();
                    }

                    ui.separator();

                    let preview_label = if self.show_preview {
                        "Hide Preview (Ctrl+P)"
                    } else {
                        "Show Preview (Ctrl+P)"
                    };
                    if ui.button(preview_label).clicked() {
                        self.show_preview = !self.show_preview;
                        if self.show_preview {
                            // Auto-detect or keep current mode
                            let detected = self.detect_preview_mode();
                            if detected != PreviewMode::None {
                                self.preview_mode = detected;
                            } else if self.preview_mode == PreviewMode::None {
                                // Default to Markdown if no file extension
                                self.preview_mode = PreviewMode::Markdown;
                            }
                        }
                        ui.close();
                    }

                    ui.separator();
                    ui.label("Preview Mode:");

                    if ui.selectable_label(self.preview_mode == PreviewMode::Markdown, "Markdown").clicked() {
                        self.preview_mode = PreviewMode::Markdown;
                        self.show_preview = true;
                        ui.close();
                    }

                    if ui.selectable_label(self.preview_mode == PreviewMode::Html, "HTML").clicked() {
                        self.preview_mode = PreviewMode::Html;
                        self.show_preview = true;
                        ui.close();
                    }

                    if ui.selectable_label(self.preview_mode == PreviewMode::Latex, "LaTeX").clicked() {
                        self.preview_mode = PreviewMode::Latex;
                        self.show_preview = true;
                        ui.close();
                    }

                    if ui.selectable_label(self.preview_mode == PreviewMode::Mermaid, "Mermaid").clicked() {
                        self.preview_mode = PreviewMode::Mermaid;
                        self.show_preview = true;
                        ui.close();
                    }
                });

                // Show file status
                if let Some(buffer_id) = self.active_buffer_id {
                    if let Some(buffer) = self.buffers.get(&buffer_id) {
                        let file_name = buffer.file_path()
                            .and_then(|p| p.file_name())
                            .and_then(|n| n.to_str())
                            .unwrap_or("Untitled");

                        let dirty_marker = if self.text_changed { " *" } else { "" };
                        ui.separator();
                        ui.label(format!("{}{}", file_name, dirty_marker));
                    }
                }
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let lines = self.current_text.lines().count();
                let chars = self.current_text.len();
                ui.label(format!("Lines: {} | Characters: {}", lines, chars));

                if self.text_changed {
                    ui.label("| Modified");
                }
            });
        });

        // Main editor with optional preview
        if self.show_preview && self.preview_mode != PreviewMode::None {
            // Split view: Editor on left, Preview on right
            egui::SidePanel::right("preview_panel")
                .default_width(ctx.available_rect().width() * 0.5)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.heading("Preview");
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            match self.preview_mode {
                                PreviewMode::Markdown => {
                                    markdown_preview::render_markdown_preview(ui, &self.current_text);
                                }
                                PreviewMode::Html => {
                                    self.render_html_preview(ui);
                                }
                                PreviewMode::Latex => {
                                    self.render_latex_preview(ui);
                                }
                                PreviewMode::Mermaid => {
                                    self.render_mermaid_preview(ui);
                                }
                                PreviewMode::None => {}
                            }
                        });
                });
        }

        // Main editor - NO PADDING
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE) // Remove all frames and padding
            .show(ctx, |ui| {
                let available_height = ui.available_height();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        // TextEdit in egui 0.33+ automatically supports IME on all platforms
                        // No manual IME configuration is needed
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut self.current_text)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .min_size(egui::vec2(f32::INFINITY, available_height))
                                .frame(false) // No frame around text edit
                        );

                        // Request focus on first frame to ensure IME works immediately
                        if self.current_text.is_empty() && !self.text_changed {
                            response.request_focus();
                        }

                        if response.changed() {
                            self.text_changed = true;
                            // Don't auto-detect preview mode on text change
                            // This prevents the preview from disappearing when editing
                            // The user can manually change preview mode via View menu
                        }
                    });
            });
    }

    fn render_html_preview(&self, ui: &mut egui::Ui) {
        ui.heading("HTML Preview");
        ui.separator();

        // Parse HTML and display as formatted text
        use scraper::{Html, Selector};

        let document = Html::parse_document(&self.current_text);

        // Extract and display title
        if let Ok(selector) = Selector::parse("title") {
            for element in document.select(&selector) {
                ui.heading(element.text().collect::<String>());
                ui.separator();
            }
        }

        // Extract and display body content
        if let Ok(selector) = Selector::parse("body") {
            for element in document.select(&selector) {
                let text = element.text().collect::<Vec<_>>().join(" ");
                ui.label(text);
            }
        } else {
            // If no body tag, just show all text
            ui.label(&self.current_text);
        }
    }

    fn render_latex_preview(&self, ui: &mut egui::Ui) {
        ui.heading("LaTeX Preview");
        ui.separator();

        // Simple LaTeX rendering with Unicode substitution
        let mut preview_text = self.current_text.clone();

        // Common LaTeX symbols to Unicode
        let substitutions = vec![
            (r"\alpha", "α"), (r"\beta", "β"), (r"\gamma", "γ"), (r"\delta", "δ"),
            (r"\epsilon", "ε"), (r"\theta", "θ"), (r"\lambda", "λ"), (r"\mu", "μ"),
            (r"\pi", "π"), (r"\sigma", "σ"), (r"\phi", "φ"), (r"\omega", "ω"),
            (r"\sqrt", "√"), (r"\int", "∫"), (r"\sum", "∑"), (r"\prod", "∏"),
            (r"\partial", "∂"), (r"\nabla", "∇"), (r"\infty", "∞"),
            (r"\pm", "±"), (r"\times", "×"), (r"\div", "÷"),
            (r"\leq", "≤"), (r"\geq", "≥"), (r"\neq", "≠"), (r"\approx", "≈"),
        ];

        for (latex, unicode) in substitutions {
            preview_text = preview_text.replace(latex, unicode);
        }

        ui.label(&preview_text);
    }

    fn render_mermaid_preview(&self, ui: &mut egui::Ui) {
        ui.heading("Mermaid Diagram");
        ui.separator();

        // Simple ASCII art rendering
        ui.label("Diagram Preview:");
        ui.separator();

        let lines: Vec<&str> = self.current_text.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("graph") || trimmed.starts_with("flowchart") {
                continue;
            }

            // Simple node rendering
            if trimmed.contains("-->") || trimmed.contains("->") {
                let parts: Vec<&str> = trimmed.split("-->").collect();
                if parts.len() == 2 {
                    ui.horizontal(|ui| {
                        ui.monospace(format!("[{}]", parts[0].trim()));
                        ui.label("→");
                        ui.monospace(format!("[{}]", parts[1].trim()));
                    });
                }
            } else {
                ui.monospace(trimmed);
            }
        }
    }

    fn show_file_dialog(&mut self, ctx: &egui::Context) {
        let mut should_open = None;
        let mut should_close = false;

        let mut is_open = self.show_file_dialog;
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
                            self.file_path_input = home.display().to_string();
                        }
                    }
                    if ui.button("📁 Documents").clicked() {
                        if let Some(docs) = dirs::document_dir() {
                            self.file_path_input = docs.display().to_string();
                        }
                    }
                    if ui.button("💻 Desktop").clicked() {
                        if let Some(desktop) = dirs::desktop_dir() {
                            self.file_path_input = desktop.display().to_string();
                        }
                    }
                });

                ui.separator();

                // Current directory
                ui.label("Current working directory:");
                if let Ok(cwd) = std::env::current_dir() {
                    if ui.button(cwd.display().to_string()).clicked() {
                        self.file_path_input = cwd.display().to_string();
                    }
                }

                ui.separator();

                // File path input
                ui.label("File path:");
                ui.text_edit_singleline(&mut self.file_path_input);

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
                                if ui.button(format!("📂 {}: {}", name, path.display())).clicked() {
                                    self.file_path_input = path.display().to_string();
                                }
                            }
                        }
                    });

                ui.separator();

                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("✓ Open").clicked() {
                        should_open = Some(PathBuf::from(self.file_path_input.clone()));
                        should_close = true;
                    }
                    if ui.button("✗ Cancel").clicked() {
                        should_close = true;
                    }
                });
            });

        self.show_file_dialog = is_open;

        if let Some(path) = should_open {
            self.open_file(path);
            self.file_path_input.clear();
        }
        if should_close {
            self.show_file_dialog = false;
            self.file_path_input.clear();
        }
    }

    fn show_save_as_dialog(&mut self, ctx: &egui::Context) {
        let mut should_save = None;
        let mut should_close = false;

        let mut is_open = self.show_save_as_dialog;
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
                            self.file_path_input = home.display().to_string() + "/";
                        }
                    }
                    if ui.button("📁 Documents").clicked() {
                        if let Some(docs) = dirs::document_dir() {
                            self.file_path_input = docs.display().to_string() + "/";
                        }
                    }
                    if ui.button("💻 Desktop").clicked() {
                        if let Some(desktop) = dirs::desktop_dir() {
                            self.file_path_input = desktop.display().to_string() + "/";
                        }
                    }
                });

                ui.separator();

                // File path input
                ui.label("Save as:");
                ui.text_edit_singleline(&mut self.file_path_input);

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
                                if ui.button(format!("📂 {}", name)).clicked() {
                                    self.file_path_input = path.display().to_string() + "/";
                                }
                            }
                        }
                    });

                ui.separator();

                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() {
                        should_save = Some(PathBuf::from(self.file_path_input.clone()));
                        should_close = true;
                    }
                    if ui.button("✗ Cancel").clicked() {
                        should_close = true;
                    }
                });
            });

        self.show_save_as_dialog = is_open;

        if let Some(path) = should_save {
            self.save_file_as(path);
            self.file_path_input.clear();
        }
        if should_close {
            self.show_save_as_dialog = false;
            self.file_path_input.clear();
        }
    }
}

impl eframe::App for LalaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.is_light_theme {
            ctx.set_visuals(egui::Visuals::light());
        } else {
            ctx.set_visuals(egui::Visuals::dark());
        }

        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        // Poll grep results
        if self.grep_engine.is_searching() {
            while let Some(result) = self.grep_engine.poll_result() {
                self.grep_panel.add_result(result);
            }
            ctx.request_repaint(); // Keep updating while searching
        }

        // Show main editor
        self.show_main_editor(ctx);

        // Show search panel
        if self.show_search_panel {
            if let Some(buffer_id) = self.active_buffer_id {
                if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                    let mut open = self.show_search_panel;
                    self.search_panel.show(ctx, buffer, &mut open);
                    self.show_search_panel = open;
                }
            }
        }

        // Show grep panel
        if self.show_grep_panel {
            let mut open = self.show_grep_panel;
            self.grep_panel.show(
                ctx,
                &mut self.grep_engine,
                &self.file_tree,
                &mut self.buffers,
                &mut self.active_buffer_id,
                &mut self.next_buffer_id,
                &mut open,
            );
            self.show_grep_panel = open;
        }

        // Show file dialog
        if self.show_file_dialog {
            self.show_file_dialog(ctx);
        }

        // Show save as dialog
        if self.show_save_as_dialog {
            self.show_save_as_dialog(ctx);
        }
    }
}
