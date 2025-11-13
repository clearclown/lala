use eframe::egui;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::core_engine::{Buffer, BufferId};
use crate::file_tree::FileTree;
use crate::search::{GrepEngine, GrepOptions, GrepResult, GrepStatus};

pub struct GrepPanel {
    // Search state
    search_query: String,
    case_sensitive: bool,
    use_regex: bool,
    search_path: String,

    // Results
    results: Vec<GrepResult>,

    // Error state
    error_message: Option<String>,
}

impl GrepPanel {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            case_sensitive: true,
            use_regex: false,
            search_path: ".".to_string(),
            results: Vec::new(),
            error_message: None,
        }
    }

    pub fn add_result(&mut self, result: GrepResult) {
        self.results.push(result);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        grep_engine: &mut GrepEngine,
        file_tree: &FileTree,
        buffers: &mut HashMap<BufferId, Buffer>,
        active_buffer_id: &mut Option<BufferId>,
        next_buffer_id: &mut usize,
        open: &mut bool,
    ) {
        egui::Window::new("Grep - Search in Files")
            .open(open)
            .default_width(600.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                self.show_content(
                    ui,
                    grep_engine,
                    file_tree,
                    buffers,
                    active_buffer_id,
                    next_buffer_id,
                );
            });
    }

    fn show_content(
        &mut self,
        ui: &mut egui::Ui,
        grep_engine: &mut GrepEngine,
        file_tree: &FileTree,
        buffers: &mut HashMap<BufferId, Buffer>,
        active_buffer_id: &mut Option<BufferId>,
        next_buffer_id: &mut usize,
    ) {
        // Search input
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });

        ui.horizontal(|ui| {
            ui.label("Path:");
            ui.text_edit_singleline(&mut self.search_path);

            if ui.button("📁").clicked() {
                // Use file tree root as default
                self.search_path = file_tree.root().to_string_lossy().to_string();
            }
        });

        ui.separator();

        // Options
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.case_sensitive, "Case sensitive");
            ui.checkbox(&mut self.use_regex, "Regex");
        });

        ui.separator();

        // Search button
        ui.horizontal(|ui| {
            let can_search = !self.search_query.is_empty()
                && grep_engine.status() != GrepStatus::Searching;

            if ui
                .add_enabled(can_search, egui::Button::new("🔍 Search"))
                .clicked()
            {
                self.start_search(grep_engine);
            }

            if ui
                .add_enabled(
                    grep_engine.status() == GrepStatus::Searching,
                    egui::Button::new("⏹ Stop"),
                )
                .clicked()
            {
                grep_engine.clear();
                self.error_message = Some("Search cancelled".to_string());
            }

            if ui.button("🗑 Clear").clicked() {
                self.results.clear();
                grep_engine.clear();
                self.error_message = None;
            }
        });

        ui.separator();

        // Status
        match grep_engine.status() {
            GrepStatus::Idle => {
                if let Some(error) = &self.error_message {
                    ui.colored_label(egui::Color32::YELLOW, error);
                }
            }
            GrepStatus::Searching => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(format!("Searching... Found {} results", grep_engine.result_count()));
                });
            }
            GrepStatus::Completed => {
                ui.label(format!(
                    "✓ Search completed - {} results found",
                    grep_engine.result_count()
                ));
            }
        }

        ui.separator();

        // Results list
        if self.results.is_empty() {
            ui.label("No results yet. Start a search to see matches.");
        } else {
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    self.show_results(ui, buffers, active_buffer_id, next_buffer_id);
                });
        }
    }

    fn show_results(
        &mut self,
        ui: &mut egui::Ui,
        buffers: &mut HashMap<BufferId, Buffer>,
        active_buffer_id: &mut Option<BufferId>,
        next_buffer_id: &mut usize,
    ) {
        // Group results by file (clone to avoid borrow conflicts)
        let mut results_by_file: HashMap<PathBuf, Vec<GrepResult>> = HashMap::new();
        for result in &self.results {
            results_by_file
                .entry(result.file_path.clone())
                .or_default()
                .push(result.clone());
        }

        for (file_path, file_results) in results_by_file.iter() {
            ui.push_id(file_path, |ui| {
                egui::CollapsingHeader::new(format!(
                    "📄 {} ({} matches)",
                    file_path.display(),
                    file_results.len()
                ))
                .default_open(true)
                .show(ui, |ui| {
                    for result in file_results {
                        self.show_result_item(
                            ui,
                            result,
                            buffers,
                            active_buffer_id,
                            next_buffer_id,
                        );
                    }
                });
            });
        }
    }

    fn show_result_item(
        &mut self,
        ui: &mut egui::Ui,
        result: &GrepResult,
        buffers: &mut HashMap<BufferId, Buffer>,
        active_buffer_id: &mut Option<BufferId>,
        next_buffer_id: &mut usize,
    ) {
        ui.horizontal(|ui| {
            // Line number
            ui.label(format!("{}:{}", result.line_number, result.column));

            // Preview with highlighted match
            let before = &result.line_content[..result.match_start];
            let matched = &result.line_content[result.match_start..result.match_end];
            let after = &result.line_content[result.match_end..];

            let preview = before.trim_start().to_string();
            ui.label(preview);
            ui.colored_label(egui::Color32::YELLOW, matched);
            ui.label(after);

            // Jump button
            if ui.small_button("→").clicked() {
                self.jump_to_result(
                    result,
                    buffers,
                    active_buffer_id,
                    next_buffer_id,
                );
            }
        });
    }

    fn start_search(&mut self, grep_engine: &mut GrepEngine) {
        self.results.clear();
        self.error_message = None;

        let search_path = if self.search_path.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&self.search_path)
        };

        // Validate path
        if !search_path.exists() {
            self.error_message = Some(format!("Path does not exist: {}", search_path.display()));
            return;
        }

        let options = GrepOptions {
            pattern: self.search_query.clone(),
            case_sensitive: self.case_sensitive,
            use_regex: self.use_regex,
            root_path: search_path,
            file_filter: None,
        };

        grep_engine.start_search(options);
    }

    fn jump_to_result(
        &mut self,
        result: &GrepResult,
        buffers: &mut HashMap<BufferId, Buffer>,
        active_buffer_id: &mut Option<BufferId>,
        next_buffer_id: &mut usize,
    ) {
        // Try to read the file
        let content = match fs::read_to_string(&result.file_path) {
            Ok(c) => c,
            Err(e) => {
                self.error_message = Some(format!(
                    "Failed to open file {}: {}",
                    result.file_path.display(),
                    e
                ));
                return;
            }
        };

        // Find existing buffer or create new one
        let buffer_id = BufferId(*next_buffer_id);
        *next_buffer_id += 1;

        let buffer = Buffer::from_string(
            buffer_id,
            content,
            Some(result.file_path.clone()),
        );

        buffers.insert(buffer_id, buffer);
        *active_buffer_id = Some(buffer_id);

        self.error_message = Some(format!(
            "Opened {} at line {}",
            result.file_path.display(),
            result.line_number
        ));
    }
}
