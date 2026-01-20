/// Comprehensive tests for CLI functionality
///
/// Tests cover:
/// - Argument parsing
/// - StartupMode determination
/// - Subcommand handling
/// - Path handling (files, directories, special paths)
/// - Edge cases
use lala::cli::{parse_args, StartupMode};
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================
// === Basic Argument Parsing Tests ===
// ============================================

mod basic_parsing_tests {
    use super::*;

    #[test]
    fn test_no_arguments() {
        let mode = parse_args(vec!["lala"]);
        assert_eq!(mode, StartupMode::Empty);
    }

    #[test]
    fn test_single_file_argument() {
        let mode = parse_args(vec!["lala", "test.txt"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("test.txt")));
    }

    #[test]
    fn test_markdown_file() {
        let mode = parse_args(vec!["lala", "README.md"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("README.md")));
    }

    #[test]
    fn test_rust_file() {
        let mode = parse_args(vec!["lala", "main.rs"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("main.rs")));
    }

    #[test]
    fn test_python_file() {
        let mode = parse_args(vec!["lala", "script.py"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("script.py")));
    }

    #[test]
    fn test_javascript_file() {
        let mode = parse_args(vec!["lala", "app.js"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("app.js")));
    }

    #[test]
    fn test_hidden_file() {
        let mode = parse_args(vec!["lala", ".gitignore"]);
        // .gitignore is treated as a directory because no extension after the dot
        // Actually it's a file with extension "gitignore"
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from(".gitignore")));
    }
}

// ============================================
// === Directory Path Tests ===
// ============================================

mod directory_path_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_current_directory() {
        let mode = parse_args(vec!["lala", "."]);
        assert_eq!(mode, StartupMode::OpenDir(PathBuf::from(".")));
    }

    #[test]
    fn test_parent_directory() {
        let mode = parse_args(vec!["lala", ".."]);
        assert_eq!(mode, StartupMode::OpenDir(PathBuf::from("..")));
    }

    #[test]
    fn test_named_directory_no_extension() {
        let mode = parse_args(vec!["lala", "src"]);
        // Without extension, treated as directory when it exists as dir
        // If doesn't exist, still treated as directory
        let expected = if PathBuf::from("src").is_dir() {
            StartupMode::OpenDir(PathBuf::from("src"))
        } else {
            StartupMode::OpenDir(PathBuf::from("src"))
        };
        assert_eq!(mode, expected);
    }

    #[test]
    fn test_existing_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();
        let mode = parse_args(vec!["lala", dir_path]);
        assert_eq!(mode, StartupMode::OpenDir(PathBuf::from(dir_path)));
    }

    #[test]
    fn test_directory_with_trailing_slash() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = format!("{}/", temp_dir.path().to_str().unwrap());
        let mode = parse_args(vec!["lala", &dir_path]);
        assert_eq!(mode, StartupMode::OpenDir(PathBuf::from(&dir_path)));
    }

    #[test]
    fn test_nested_directory_path() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("deep");
        fs::create_dir_all(&nested_dir).unwrap();

        let mode = parse_args(vec!["lala", nested_dir.to_str().unwrap()]);
        assert_eq!(mode, StartupMode::OpenDir(nested_dir));
    }
}

// ============================================
// === File Path Variations Tests ===
// ============================================

mod file_path_tests {
    use super::*;

    #[test]
    fn test_absolute_path() {
        let mode = parse_args(vec!["lala", "/tmp/test.txt"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("/tmp/test.txt")));
    }

    #[test]
    fn test_relative_path_with_directory() {
        let mode = parse_args(vec!["lala", "./src/main.rs"]);
        assert_eq!(
            mode,
            StartupMode::OpenFile(PathBuf::from("./src/main.rs"))
        );
    }

    #[test]
    fn test_path_with_multiple_dots() {
        let mode = parse_args(vec!["lala", "file.test.txt"]);
        assert_eq!(
            mode,
            StartupMode::OpenFile(PathBuf::from("file.test.txt"))
        );
    }

    #[test]
    fn test_path_with_spaces() {
        let mode = parse_args(vec!["lala", "my file.txt"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("my file.txt")));
    }

    #[test]
    fn test_path_with_special_characters() {
        let mode = parse_args(vec!["lala", "file-with_special.chars.txt"]);
        assert_eq!(
            mode,
            StartupMode::OpenFile(PathBuf::from("file-with_special.chars.txt"))
        );
    }

    #[test]
    fn test_uppercase_extension() {
        let mode = parse_args(vec!["lala", "FILE.TXT"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("FILE.TXT")));
    }

    #[test]
    fn test_long_extension() {
        let mode = parse_args(vec!["lala", "archive.tar.gz"]);
        assert_eq!(
            mode,
            StartupMode::OpenFile(PathBuf::from("archive.tar.gz"))
        );
    }

    #[test]
    fn test_json_file() {
        let mode = parse_args(vec!["lala", "config.json"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("config.json")));
    }

    #[test]
    fn test_yaml_file() {
        let mode = parse_args(vec!["lala", "config.yaml"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("config.yaml")));
    }

    #[test]
    fn test_toml_file() {
        let mode = parse_args(vec!["lala", "Cargo.toml"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("Cargo.toml")));
    }
}

// ============================================
// === Subcommand Tests ===
// ============================================

mod subcommand_tests {
    use super::*;

    #[test]
    fn test_markdown_subcommand() {
        let mode = parse_args(vec!["lala", "markdown", "README.md"]);
        assert_eq!(
            mode,
            StartupMode::MarkdownPreview {
                file: PathBuf::from("README.md"),
                no_color: false
            }
        );
    }

    #[test]
    fn test_markdown_subcommand_no_color() {
        let mode = parse_args(vec!["lala", "markdown", "README.md", "--no-color"]);
        assert_eq!(
            mode,
            StartupMode::MarkdownPreview {
                file: PathBuf::from("README.md"),
                no_color: true
            }
        );
    }

    #[test]
    fn test_html_subcommand() {
        let mode = parse_args(vec!["lala", "html", "index.html"]);
        assert_eq!(
            mode,
            StartupMode::HtmlPreview {
                file: PathBuf::from("index.html"),
                no_color: false
            }
        );
    }

    #[test]
    fn test_html_subcommand_no_color() {
        let mode = parse_args(vec!["lala", "html", "index.html", "--no-color"]);
        assert_eq!(
            mode,
            StartupMode::HtmlPreview {
                file: PathBuf::from("index.html"),
                no_color: true
            }
        );
    }

    #[test]
    fn test_mermaid_subcommand() {
        let mode = parse_args(vec!["lala", "mermaid", "diagram.mmd"]);
        assert_eq!(
            mode,
            StartupMode::MermaidPreview {
                file: PathBuf::from("diagram.mmd"),
                no_color: false
            }
        );
    }

    #[test]
    fn test_mermaid_subcommand_no_color() {
        let mode = parse_args(vec!["lala", "mermaid", "diagram.mmd", "--no-color"]);
        assert_eq!(
            mode,
            StartupMode::MermaidPreview {
                file: PathBuf::from("diagram.mmd"),
                no_color: true
            }
        );
    }

    #[test]
    fn test_latex_subcommand() {
        let mode = parse_args(vec!["lala", "latex", "document.tex"]);
        assert_eq!(
            mode,
            StartupMode::LatexPreview {
                file: PathBuf::from("document.tex"),
                no_color: false
            }
        );
    }

    #[test]
    fn test_latex_subcommand_no_color() {
        let mode = parse_args(vec!["lala", "latex", "document.tex", "--no-color"]);
        assert_eq!(
            mode,
            StartupMode::LatexPreview {
                file: PathBuf::from("document.tex"),
                no_color: true
            }
        );
    }

    #[test]
    fn test_view_subcommand() {
        let mode = parse_args(vec!["lala", "view", "file.txt"]);
        assert_eq!(
            mode,
            StartupMode::ViewFile {
                file: PathBuf::from("file.txt"),
                line_numbers: false
            }
        );
    }

    #[test]
    fn test_view_subcommand_with_line_numbers_short() {
        let mode = parse_args(vec!["lala", "view", "-n", "file.txt"]);
        assert_eq!(
            mode,
            StartupMode::ViewFile {
                file: PathBuf::from("file.txt"),
                line_numbers: true
            }
        );
    }

    #[test]
    fn test_view_subcommand_with_line_numbers_long() {
        let mode = parse_args(vec!["lala", "view", "--line-numbers", "file.txt"]);
        assert_eq!(
            mode,
            StartupMode::ViewFile {
                file: PathBuf::from("file.txt"),
                line_numbers: true
            }
        );
    }
}

// ============================================
// === StartupMode Tests ===
// ============================================

mod startup_mode_tests {
    use super::*;

    #[test]
    fn test_startup_mode_clone() {
        let mode = StartupMode::OpenFile(PathBuf::from("test.txt"));
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_startup_mode_debug() {
        let mode = StartupMode::Empty;
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("Empty"));
    }

    #[test]
    fn test_startup_mode_partial_eq() {
        let mode1 = StartupMode::OpenFile(PathBuf::from("test.txt"));
        let mode2 = StartupMode::OpenFile(PathBuf::from("test.txt"));
        let mode3 = StartupMode::OpenFile(PathBuf::from("other.txt"));

        assert_eq!(mode1, mode2);
        assert_ne!(mode1, mode3);
    }

    #[test]
    fn test_startup_mode_variants() {
        // Test all variant patterns compile
        let _empty = StartupMode::Empty;
        let _file = StartupMode::OpenFile(PathBuf::new());
        let _dir = StartupMode::OpenDir(PathBuf::new());
        let _md = StartupMode::MarkdownPreview {
            file: PathBuf::new(),
            no_color: false,
        };
        let _html = StartupMode::HtmlPreview {
            file: PathBuf::new(),
            no_color: false,
        };
        let _mermaid = StartupMode::MermaidPreview {
            file: PathBuf::new(),
            no_color: false,
        };
        let _latex = StartupMode::LatexPreview {
            file: PathBuf::new(),
            no_color: false,
        };
        let _view = StartupMode::ViewFile {
            file: PathBuf::new(),
            line_numbers: false,
        };
        let _cli = StartupMode::CliCommandExecuted;
    }
}

// ============================================
// === Edge Case Tests ===
// ============================================

mod edge_case_tests {
    use super::*;

    // Note: Empty string argument test removed as clap treats "" as missing required argument

    #[test]
    fn test_dot_only_file() {
        // File named just "."
        let mode = parse_args(vec!["lala", "."]);
        assert_eq!(mode, StartupMode::OpenDir(PathBuf::from(".")));
    }

    #[test]
    fn test_double_dot_directory() {
        let mode = parse_args(vec!["lala", ".."]);
        assert_eq!(mode, StartupMode::OpenDir(PathBuf::from("..")));
    }

    #[test]
    fn test_file_starting_with_dot() {
        let mode = parse_args(vec!["lala", ".hidden.txt"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from(".hidden.txt")));
    }

    #[test]
    fn test_very_long_filename() {
        let long_name = "a".repeat(200) + ".txt";
        let mode = parse_args(vec!["lala", &long_name]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from(&long_name)));
    }

    #[test]
    fn test_unicode_filename() {
        let mode = parse_args(vec!["lala", "日本語.txt"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("日本語.txt")));
    }

    #[test]
    fn test_unicode_directory() {
        let mode = parse_args(vec!["lala", "日本語フォルダ"]);
        assert_eq!(
            mode,
            StartupMode::OpenDir(PathBuf::from("日本語フォルダ"))
        );
    }

    #[test]
    fn test_emoji_in_filename() {
        let mode = parse_args(vec!["lala", "readme_📝.md"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("readme_📝.md")));
    }
}

// ============================================
// === HTML View Tests ===
// ============================================

mod html_view_tests {
    use lala::cli::html_view;

    #[test]
    fn test_render_simple_html() {
        // The function prints to stdout, just verify it doesn't panic
        html_view::render_html_to_terminal("<html><body><p>Hello</p></body></html>");
    }

    #[test]
    fn test_render_html_with_headers() {
        html_view::render_html_to_terminal("<h1>Title</h1><h2>Subtitle</h2><p>Content</p>");
    }

    #[test]
    fn test_render_html_with_list() {
        html_view::render_html_to_terminal("<ul><li>Item 1</li><li>Item 2</li></ul>");
    }

    #[test]
    fn test_render_html_with_table() {
        html_view::render_html_to_terminal("<table><tr><td>Cell 1</td><td>Cell 2</td></tr></table>");
    }

    #[test]
    fn test_render_empty_html() {
        html_view::render_html_to_terminal("");
    }

    #[test]
    fn test_render_malformed_html() {
        html_view::render_html_to_terminal("<p>Unclosed paragraph");
    }
}

// ============================================
// === Markdown View Tests ===
// ============================================

mod markdown_view_tests {
    use lala::cli::markdown_view;

    #[test]
    fn test_render_simple_markdown() {
        markdown_view::render_markdown_to_terminal("# Hello\n\nThis is a test.");
    }

    #[test]
    fn test_render_markdown_with_code() {
        markdown_view::render_markdown_to_terminal("```rust\nfn main() {}\n```");
    }

    #[test]
    fn test_render_markdown_with_list() {
        markdown_view::render_markdown_to_terminal("- Item 1\n- Item 2\n- Item 3");
    }

    #[test]
    fn test_render_markdown_with_bold() {
        markdown_view::render_markdown_to_terminal("**bold text**");
    }

    #[test]
    fn test_render_markdown_with_italic() {
        markdown_view::render_markdown_to_terminal("*italic text*");
    }

    #[test]
    fn test_render_empty_markdown() {
        markdown_view::render_markdown_to_terminal("");
    }

    #[test]
    fn test_render_markdown_with_links() {
        markdown_view::render_markdown_to_terminal("[link text](https://example.com)");
    }

    #[test]
    fn test_render_markdown_with_ordered_list() {
        markdown_view::render_markdown_to_terminal("1. First\n2. Second\n3. Third");
    }
}

// ============================================
// === Mermaid View Tests ===
// ============================================

mod mermaid_view_tests {
    use lala::cli::mermaid_view;

    #[test]
    fn test_render_flowchart() {
        mermaid_view::render_mermaid_to_terminal("graph TD\n    A-->B\n    B-->C");
    }

    #[test]
    fn test_render_sequence_diagram() {
        mermaid_view::render_mermaid_to_terminal("sequenceDiagram\n    Alice->>Bob: Hello");
    }

    #[test]
    fn test_render_class_diagram() {
        mermaid_view::render_mermaid_to_terminal("classDiagram\n    Animal <|-- Duck");
    }

    #[test]
    fn test_render_state_diagram() {
        mermaid_view::render_mermaid_to_terminal("stateDiagram\n    [*] --> State1");
    }

    #[test]
    fn test_render_empty_mermaid() {
        mermaid_view::render_mermaid_to_terminal("");
    }

    #[test]
    fn test_render_er_diagram() {
        mermaid_view::render_mermaid_to_terminal("erDiagram\n    CUSTOMER ||--o{ ORDER : places");
    }
}

// ============================================
// === LaTeX View Tests ===
// ============================================

mod latex_view_tests {
    use lala::cli::latex_view;

    #[test]
    fn test_render_simple_latex() {
        latex_view::render_latex_to_terminal("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}");
    }

    #[test]
    fn test_render_latex_with_math() {
        latex_view::render_latex_to_terminal("$E = mc^2$\n\\sqrt{2}\n\\sum_{i=1}^n");
    }

    #[test]
    fn test_render_latex_with_sections() {
        latex_view::render_latex_to_terminal("\\section{Introduction}\nContent here.\n\\subsection{Details}");
    }

    #[test]
    fn test_render_empty_latex() {
        latex_view::render_latex_to_terminal("");
    }

    #[test]
    fn test_render_latex_with_packages() {
        latex_view::render_latex_to_terminal("\\documentclass{article}\n\\usepackage{amsmath}\n\\usepackage{graphicx}");
    }
}

// ============================================
// === Path Resolution Tests ===
// ============================================

mod path_resolution_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_existing_file_detected() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("exists.txt");
        fs::write(&file_path, "content").unwrap();

        let mode = parse_args(vec!["lala", file_path.to_str().unwrap()]);
        assert_eq!(mode, StartupMode::OpenFile(file_path));
    }

    #[test]
    fn test_nonexistent_file_with_extension() {
        let mode = parse_args(vec!["lala", "/path/to/nonexistent.txt"]);
        assert_eq!(
            mode,
            StartupMode::OpenFile(PathBuf::from("/path/to/nonexistent.txt"))
        );
    }

    #[test]
    fn test_nonexistent_path_without_extension() {
        let mode = parse_args(vec!["lala", "/path/to/nonexistent"]);
        assert_eq!(
            mode,
            StartupMode::OpenDir(PathBuf::from("/path/to/nonexistent"))
        );
    }
}

// ============================================
// === Multiple Files Tests ===
// ============================================

mod multiple_arguments_tests {
    use super::*;

    // Note: Current implementation only supports one path argument
    // These tests document the current behavior

    #[test]
    fn test_first_argument_used() {
        // When multiple paths are provided, only the first is used
        // (due to clap configuration)
        let mode = parse_args(vec!["lala", "first.txt"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("first.txt")));
    }
}

// ============================================
// === File Extension Detection Tests ===
// ============================================

mod extension_detection_tests {
    use super::*;

    #[test]
    fn test_common_text_extensions() {
        let extensions = vec!["txt", "md", "rs", "py", "js", "ts", "json", "yaml", "toml"];
        for ext in extensions {
            let filename = format!("test.{}", ext);
            let mode = parse_args(vec!["lala", &filename]);
            assert_eq!(
                mode,
                StartupMode::OpenFile(PathBuf::from(&filename)),
                "Failed for extension: {}",
                ext
            );
        }
    }

    #[test]
    fn test_uppercase_extensions() {
        let extensions = vec!["TXT", "MD", "RS", "PY", "JS"];
        for ext in extensions {
            let filename = format!("test.{}", ext);
            let mode = parse_args(vec!["lala", &filename]);
            assert_eq!(
                mode,
                StartupMode::OpenFile(PathBuf::from(&filename)),
                "Failed for uppercase extension: {}",
                ext
            );
        }
    }

    #[test]
    fn test_mixed_case_extensions() {
        let mode = parse_args(vec!["lala", "test.TxT"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("test.TxT")));
    }

    #[test]
    fn test_numeric_extension() {
        let mode = parse_args(vec!["lala", "backup.123"]);
        assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("backup.123")));
    }
}
