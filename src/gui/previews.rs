use eframe::egui;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PreviewMode {
    Markdown,
    Html,
    Latex,
    Mermaid,
    None,
}

/// Detect preview mode based on file extension
pub fn detect_preview_mode(file_path: Option<&PathBuf>) -> PreviewMode {
    if let Some(path) = file_path {
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
    PreviewMode::None
}

/// Render HTML preview
pub fn render_html_preview(ui: &mut egui::Ui, text: &str) {
    ui.heading("HTML Preview");
    ui.separator();

    // Parse HTML and display as formatted text
    use scraper::{Html, Selector};

    let document = Html::parse_document(text);

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
        ui.label(text);
    }
}

/// Render LaTeX preview
pub fn render_latex_preview(ui: &mut egui::Ui, text: &str) {
    ui.heading("LaTeX Preview");
    ui.separator();

    // Simple LaTeX rendering with Unicode substitution
    let mut preview_text = text.to_string();

    // Common LaTeX symbols to Unicode
    let substitutions = vec![
        (r"\alpha", "α"),
        (r"\beta", "β"),
        (r"\gamma", "γ"),
        (r"\delta", "δ"),
        (r"\epsilon", "ε"),
        (r"\theta", "θ"),
        (r"\lambda", "λ"),
        (r"\mu", "μ"),
        (r"\pi", "π"),
        (r"\sigma", "σ"),
        (r"\phi", "φ"),
        (r"\omega", "ω"),
        (r"\sqrt", "√"),
        (r"\int", "∫"),
        (r"\sum", "∑"),
        (r"\prod", "∏"),
        (r"\partial", "∂"),
        (r"\nabla", "∇"),
        (r"\infty", "∞"),
        (r"\pm", "±"),
        (r"\times", "×"),
        (r"\div", "÷"),
        (r"\leq", "≤"),
        (r"\geq", "≥"),
        (r"\neq", "≠"),
        (r"\approx", "≈"),
    ];

    for (latex, unicode) in substitutions {
        preview_text = preview_text.replace(latex, unicode);
    }

    ui.label(&preview_text);
}

/// Render Mermaid preview
pub fn render_mermaid_preview(ui: &mut egui::Ui, text: &str) {
    ui.heading("Mermaid Diagram");
    ui.separator();

    // Simple ASCII art rendering
    ui.label("Diagram Preview:");
    ui.separator();

    let lines: Vec<&str> = text.lines().collect();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("graph")
            || trimmed.starts_with("flowchart")
        {
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
