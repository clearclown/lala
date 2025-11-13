use eframe::egui;
use lala::cli::{markdown_view, parse_args_default, StartupMode};
use lala::LalaApp;
use std::fs;
use std::process;

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
        ..Default::default()
    };

    eframe::run_native(
        "lala",
        options,
        Box::new(|cc| Ok(Box::new(LalaApp::new(cc)))),
    )?;

    Ok(())
}
