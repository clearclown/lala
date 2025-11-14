use eframe::egui;
use lala::cli::{html_view, latex_view, markdown_view, mermaid_view, parse_args_default, StartupMode};
use lala::LalaApp;
use std::fs;
use std::process;

/// Setup custom fonts to support CJK (Chinese, Japanese, Korean) characters
fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontFamily;
    use std::sync::Arc;

    let mut fonts = egui::FontDefinitions::default();

    // Load system fonts that support Japanese characters
    // Try to load CJK fonts from system if available
    if let Ok(font_data) = std::fs::read("/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf")
        .or_else(|_| std::fs::read("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"))
        .or_else(|_| std::fs::read("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc"))
        .or_else(|_| std::fs::read("/System/Library/Fonts/Hiragino Sans GB.ttc"))
        .or_else(|_| std::fs::read("/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc"))
    {
        fonts.font_data.insert(
            "NotoSansCJK".to_owned(),
            Arc::new(egui::FontData::from_owned(font_data)),
        );

        // Add to Proportional family (for UI)
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "NotoSansCJK".to_owned());

        // Add to Monospace family (for code editor)
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "NotoSansCJK".to_owned());
    }

    // If Noto Sans CJK is not available, try other common Japanese fonts
    if !fonts.font_data.contains_key("NotoSansCJK") {
        // Try IPAGothic (common on Linux)
        if let Ok(font_data) = std::fs::read("/usr/share/fonts/truetype/fonts-japanese-gothic.ttf")
            .or_else(|_| std::fs::read("/usr/share/fonts/opentype/ipafont-gothic/ipag.ttf"))
        {
            fonts.font_data.insert(
                "IPAGothic".to_owned(),
                Arc::new(egui::FontData::from_owned(font_data)),
            );

            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "IPAGothic".to_owned());

            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "IPAGothic".to_owned());
        }
    }

    ctx.set_fonts(fonts);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger for debugging
    env_logger::init();

    // Parse command line arguments
    let mode = parse_args_default();

    // Handle CLI commands (don't start GUI)
    match mode {
        StartupMode::MarkdownPreview { file, no_color } => {
            // Read markdown file
            let content = fs::read_to_string(&file).unwrap_or_else(|err| {
                eprintln!("Error reading file {:?}: {}", file, err);
                process::exit(1);
            });

            // Render to terminal
            if no_color {
                println!("{}", content);
            } else {
                markdown_view::render_markdown_to_terminal(&content);
            }

            return Ok(());
        }

        StartupMode::HtmlPreview { file, no_color } => {
            // Read HTML file
            let content = fs::read_to_string(&file).unwrap_or_else(|err| {
                eprintln!("Error reading file {:?}: {}", file, err);
                process::exit(1);
            });

            // Render to terminal
            if no_color {
                html_view::render_html_plain(&content);
            } else {
                html_view::render_html_to_terminal(&content);
            }

            return Ok(());
        }

        StartupMode::MermaidPreview { file, no_color } => {
            // Read Mermaid file
            let content = fs::read_to_string(&file).unwrap_or_else(|err| {
                eprintln!("Error reading file {:?}: {}", file, err);
                process::exit(1);
            });

            // Render to terminal
            if no_color {
                mermaid_view::render_mermaid_plain(&content);
            } else {
                mermaid_view::render_mermaid_to_terminal(&content);
            }

            return Ok(());
        }

        StartupMode::LatexPreview { file, no_color } => {
            // Read LaTeX file
            let content = fs::read_to_string(&file).unwrap_or_else(|err| {
                eprintln!("Error reading file {:?}: {}", file, err);
                process::exit(1);
            });

            // Render to terminal
            if no_color {
                latex_view::render_latex_plain(&content);
            } else {
                latex_view::render_latex_to_terminal(&content);
            }

            return Ok(());
        }

        StartupMode::ViewFile {
            file,
            line_numbers,
        } => {
            // Read file
            let content = fs::read_to_string(&file).unwrap_or_else(|err| {
                eprintln!("Error reading file {:?}: {}", file, err);
                process::exit(1);
            });

            // Display with or without line numbers
            if line_numbers {
                for (i, line) in content.lines().enumerate() {
                    println!("{:>6} {}", i + 1, line);
                }
            } else {
                print!("{}", content);
            }

            return Ok(());
        }

        StartupMode::CliCommandExecuted => {
            // CLI command was executed, don't start GUI
            return Ok(());
        }

        // For OpenFile, OpenDir, and Empty, start GUI
        _ => {}
    }

    // Start GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Lala - Modern Text Editor"),
        // Note: IME input is enabled by default in egui
        ..Default::default()
    };

    eframe::run_native(
        "lala",
        options,
        Box::new(|cc| {
            // Setup fonts to support CJK (Chinese, Japanese, Korean) characters
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(LalaApp::new(cc)))
        }),
    )?;

    Ok(())
}
