use eframe::egui;

use crate::core_engine::Buffer;
use crate::search::{replace_in_buffer, search_in_buffer, SearchOptions, SearchResult};

pub struct SearchPanel {
    // Search state
    search_query: String,
    replace_query: String,
    case_sensitive: bool,
    use_regex: bool,
    replace_mode: bool,

    // Results
    current_results: Vec<SearchResult>,
    current_match_index: usize,

    // Error state
    error_message: Option<String>,
}

impl SearchPanel {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            replace_query: String::new(),
            case_sensitive: true,
            use_regex: false,
            replace_mode: false,
            current_results: Vec::new(),
            current_match_index: 0,
            error_message: None,
        }
    }

    pub fn set_replace_mode(&mut self, enabled: bool) {
        self.replace_mode = enabled;
    }

    pub fn show(&mut self, ctx: &egui::Context, buffer: &mut Buffer, open: &mut bool) {
        egui::Window::new(if self.replace_mode {
            "Replace"
        } else {
            "Search"
        })
        .open(open)
        .default_width(400.0)
        .show(ctx, |ui| {
            self.show_content(ui, buffer);
        });
    }

    fn show_content(&mut self, ui: &mut egui::Ui, buffer: &mut Buffer) {
        ui.horizontal(|ui| {
            ui.label("Search:");
            let response = ui.text_edit_singleline(&mut self.search_query);

            // Auto-search on text change
            if response.changed() {
                self.perform_search(buffer);
            }

            // Focus on first show
            if ui.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.command) {
                response.request_focus();
            }
        });

        if self.replace_mode {
            ui.horizontal(|ui| {
                ui.label("Replace:");
                ui.text_edit_singleline(&mut self.replace_query);
            });
        }

        ui.separator();

        // Options
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut self.case_sensitive, "Case sensitive")
                .changed()
            {
                self.perform_search(buffer);
            }

            if ui.checkbox(&mut self.use_regex, "Regex").changed() {
                self.perform_search(buffer);
            }

            if ui.checkbox(&mut self.replace_mode, "Replace").changed() {
                // Mode switched, keep current search
            }
        });

        ui.separator();

        // Error message
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
            ui.separator();
        }

        // Results info
        if !self.current_results.is_empty() {
            ui.label(format!(
                "Found {} match(es) - Current: {}/{}",
                self.current_results.len(),
                self.current_match_index + 1,
                self.current_results.len()
            ));

            // Display current match details
            if let Some(result) = self.current_results.get(self.current_match_index) {
                ui.label(format!(
                    "Line {}, Column {} - {}",
                    result.range.start.line + 1,
                    result.range.start.col + 1,
                    result.match_text
                ));
            }

            ui.separator();

            // Navigation buttons
            ui.horizontal(|ui| {
                if ui.button("⬆ Previous").clicked() {
                    self.go_to_previous();
                }

                if ui.button("⬇ Next").clicked() {
                    self.go_to_next();
                }
            });

            // Replace buttons
            if self.replace_mode {
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Replace").clicked() {
                        self.replace_current(buffer);
                    }

                    if ui.button("Replace All").clicked() {
                        self.replace_all(buffer);
                    }
                });
            }
        } else if !self.search_query.is_empty() {
            ui.label("No matches found");
        }
    }

    fn perform_search(&mut self, buffer: &Buffer) {
        if self.search_query.is_empty() {
            self.current_results.clear();
            self.current_match_index = 0;
            self.error_message = None;
            return;
        }

        let options = SearchOptions {
            case_sensitive: self.case_sensitive,
            use_regex: self.use_regex,
            whole_word: false,
        };

        match search_in_buffer(buffer, &self.search_query, &options) {
            Ok(results) => {
                self.current_results = results;
                self.current_match_index = 0;
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Search error: {}", e));
                self.current_results.clear();
            }
        }
    }

    fn go_to_next(&mut self) {
        if !self.current_results.is_empty() {
            self.current_match_index = (self.current_match_index + 1) % self.current_results.len();
        }
    }

    fn go_to_previous(&mut self) {
        if !self.current_results.is_empty() {
            if self.current_match_index == 0 {
                self.current_match_index = self.current_results.len() - 1;
            } else {
                self.current_match_index -= 1;
            }
        }
    }

    fn replace_current(&mut self, buffer: &mut Buffer) {
        let options = SearchOptions {
            case_sensitive: self.case_sensitive,
            use_regex: self.use_regex,
            whole_word: false,
        };

        match replace_in_buffer(
            buffer,
            &self.search_query,
            &self.replace_query,
            &options,
            false,
        ) {
            Ok(count) => {
                if count > 0 {
                    self.error_message = Some(format!("Replaced {} occurrence(s)", count));
                    // Refresh search after replace
                    self.perform_search(buffer);
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Replace error: {}", e));
            }
        }
    }

    fn replace_all(&mut self, buffer: &mut Buffer) {
        let options = SearchOptions {
            case_sensitive: self.case_sensitive,
            use_regex: self.use_regex,
            whole_word: false,
        };

        match replace_in_buffer(
            buffer,
            &self.search_query,
            &self.replace_query,
            &options,
            true,
        ) {
            Ok(count) => {
                self.error_message = Some(format!("Replaced {} occurrence(s)", count));
                // Refresh search after replace
                self.perform_search(buffer);
            }
            Err(e) => {
                self.error_message = Some(format!("Replace error: {}", e));
            }
        }
    }
}
