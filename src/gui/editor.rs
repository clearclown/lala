use crate::core::TextBuffer;
use crate::gui::highlighting::SyntaxHighlighter;
use egui::{ScrollArea, TextEdit};

/// EditorPanel manages the text editor UI and integrates syntax highlighting
pub struct EditorPanel {
    buffer: TextBuffer,
    highlighter: SyntaxHighlighter,
    current_text: String,
}

impl EditorPanel {
    /// Creates a new EditorPanel with sample content
    pub fn new() -> Self {
        // Create sample Rust code for demonstration
        let sample_rust = r#"// Sample Rust file
fn main() {
    let x = 42;
    let message = "Hello, World!";
    println!("{}: {}", message, x);

    for i in 0..10 {
        if i % 2 == 0 {
            println!("Even: {}", i);
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
"#;

        let mut buffer = TextBuffer::from_str(sample_rust);
        buffer.set_file_path("example.rs".to_string());

        Self {
            buffer,
            highlighter: SyntaxHighlighter::new(),
            current_text: sample_rust.to_string(),
        }
    }

    /// Loads a file into the editor
    pub fn load_file(&mut self, file_path: String, content: String) {
        self.buffer = TextBuffer::from_str(&content);
        self.buffer.set_file_path(file_path);
        self.current_text = content;
    }

    /// Renders the editor UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("File:");
            if let Some(path) = self.buffer.file_path() {
                ui.label(path);
            } else {
                ui.label("Untitled");
            }

            ui.separator();

            if ui.button("Load Rust Sample").clicked() {
                self.load_rust_sample();
            }

            if ui.button("Load Markdown Sample").clicked() {
                self.load_markdown_sample();
            }

            if ui.button("Load Plain Text").clicked() {
                self.load_plaintext_sample();
            }
        });

        ui.separator();

        // Use ScrollArea for the editor
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // Use TextEdit with custom layouter for syntax highlighting
                let response = TextEdit::multiline(&mut self.current_text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .layouter(&mut |ui, text, wrap_width| {
                        // Use the highlighter to create a layout job
                        let mut layout_job =
                            self.highlighter.highlight(text, self.buffer.file_path());
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    })
                    .show(ui);

                // Update buffer if text changed
                if response.response.changed() {
                    self.buffer = TextBuffer::from_str(&self.current_text);
                    if let Some(path) = self.buffer.file_path() {
                        self.buffer.set_file_path(path.to_string());
                    }
                }
            });
    }

    /// Loads a Rust sample file
    fn load_rust_sample(&mut self) {
        let sample = r#"// Rust syntax highlighting example
use std::collections::HashMap;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();

    println!("Sum: {}", sum);
    println!("Fib(10): {}", fibonacci(10));

    // String manipulation
    let s = String::from("hello");
    let s2 = format!("{} world", s);

    // Pattern matching
    match sum {
        0..=10 => println!("Small"),
        11..=100 => println!("Medium"),
        _ => println!("Large"),
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("Hello, I'm {} and I'm {} years old", self.name, self.age);
    }
}
"#;
        self.load_file("example.rs".to_string(), sample.to_string());
    }

    /// Loads a Markdown sample file
    fn load_markdown_sample(&mut self) {
        let sample = r#"# Markdown Syntax Highlighting

## Features

This is a **bold** text and this is *italic*.

### Code Blocks

Here's some inline `code` and a code block:

```rust
fn main() {
    println!("Hello from markdown!");
}
```

### Lists

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
- Item 3

### Numbered Lists

1. First item
2. Second item
3. Third item

### Links and Images

[Link to example](https://example.com)

![Image alt text](image.png)

### Quotes

> This is a quote.
> It can span multiple lines.

### Tables

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |

### Horizontal Rule

---

### Emphasis

**Bold text**
*Italic text*
***Bold and italic***
~~Strikethrough~~

### Task Lists

- [x] Completed task
- [ ] Incomplete task
- [ ] Another task
"#;
        self.load_file("example.md".to_string(), sample.to_string());
    }

    /// Loads a plain text sample
    fn load_plaintext_sample(&mut self) {
        let sample = r#"Plain Text Example

This is plain text without any syntax highlighting.
It should be displayed in a single color.

No special formatting is applied to:
- Keywords
- Strings
- Comments
- Numbers

Just plain text.
"#;
        self.load_file("example.txt".to_string(), sample.to_string());
    }
}

impl Default for EditorPanel {
    fn default() -> Self {
        Self::new()
    }
}
