/// Markdown viewer for CLI
///
/// This module renders Markdown files in the terminal with:
/// - Colored output
/// - Proper heading sizes
/// - Formatted lists
/// - Code blocks with highlighting
/// - Bold and italic text
use colored::*;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Render Markdown to terminal with formatting
pub fn render_markdown_to_terminal(markdown: &str) {
    let parser = Parser::new_ext(markdown, Options::all());
    let events: Vec<Event> = parser.collect();

    render_events(&events);
}

/// Render events to terminal
fn render_events(events: &[Event]) {
    let mut i = 0;
    let mut in_code_block = false;
    let mut in_list = false;
    let mut list_item_number = 0;
    let mut in_ordered_list = false;

    while i < events.len() {
        match &events[i] {
            // Headings
            Event::Start(Tag::Heading { level, .. }) => {
                let heading_level = *level;
                i += 1;

                let text = extract_text_until_end(&events[i..], TagEnd::Heading(heading_level));

                println!(); // Empty line before heading
                match heading_level {
                    HeadingLevel::H1 => {
                        println!("{}", text.bold().bright_blue().underline());
                        println!("{}", "=".repeat(text.len()).bright_blue());
                    }
                    HeadingLevel::H2 => {
                        println!("{}", text.bold().bright_cyan());
                        println!("{}", "-".repeat(text.len()).bright_cyan());
                    }
                    HeadingLevel::H3 => {
                        println!("{}", format!("### {text}").bold().green());
                    }
                    HeadingLevel::H4 => {
                        println!("{}", format!("#### {text}").bold().yellow());
                    }
                    HeadingLevel::H5 => {
                        println!("{}", format!("##### {text}").yellow());
                    }
                    HeadingLevel::H6 => {
                        println!("{}", format!("###### {text}").dimmed());
                    }
                }
                println!(); // Empty line after heading

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Heading(_))) {
                        break;
                    }
                    i += 1;
                }
            }

            // Paragraphs
            Event::Start(Tag::Paragraph) => {
                i += 1;
                let text = extract_styled_text_until_end(&events[i..], TagEnd::Paragraph);
                print!("{text}");
                println!();
                println!(); // Empty line after paragraph

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Paragraph)) {
                        break;
                    }
                    i += 1;
                }
            }

            // Lists
            Event::Start(Tag::List(first_number)) => {
                in_list = true;
                in_ordered_list = first_number.is_some();
                list_item_number = first_number.unwrap_or(0) as usize;
                println!(); // Empty line before list
            }

            Event::End(TagEnd::List(_)) => {
                in_list = false;
                println!(); // Empty line after list
            }

            Event::Start(Tag::Item) => {
                i += 1;
                let text = extract_styled_text_until_end(&events[i..], TagEnd::Item);

                if in_ordered_list {
                    print!("  {}. ", list_item_number.to_string().bright_yellow());
                    list_item_number += 1;
                } else {
                    print!("  {} ", "•".bright_green());
                }
                println!("{text}");

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Item)) {
                        break;
                    }
                    i += 1;
                }
            }

            // Code blocks
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                i += 1;

                let lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };

                let code = extract_text_until_end(&events[i..], TagEnd::CodeBlock);

                println!(); // Empty line before code
                if !lang.is_empty() {
                    println!("{}", format!("```{lang}").dimmed());
                } else {
                    println!("{}", "```".dimmed());
                }

                // Print code with background
                for line in code.lines() {
                    println!("{}", line.on_truecolor(40, 44, 52));
                }

                println!("{}", "```".dimmed());
                println!(); // Empty line after code

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::CodeBlock)) {
                        in_code_block = true;
                        break;
                    }
                    i += 1;
                }
            }

            // Inline code
            Event::Code(code) => {
                print!("{}", code.bright_magenta().on_truecolor(50, 50, 50));
            }

            // Emphasis (italic)
            Event::Start(Tag::Emphasis) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Emphasis);
                print!("{}", text.italic());

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Emphasis)) {
                        break;
                    }
                    i += 1;
                }
            }

            // Strong (bold)
            Event::Start(Tag::Strong) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Strong);
                print!("{}", text.bold());

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Strong)) {
                        break;
                    }
                    i += 1;
                }
            }

            // Blockquote
            Event::Start(Tag::BlockQuote(_)) => {
                i += 1;

                // Collect blockquote content
                let mut blockquote_text = String::new();
                while i < events.len() {
                    match &events[i] {
                        Event::Text(text) => blockquote_text.push_str(text),
                        Event::End(TagEnd::BlockQuote(_)) => break,
                        Event::SoftBreak | Event::HardBreak => blockquote_text.push('\n'),
                        _ => {}
                    }
                    i += 1;
                }

                println!();
                for line in blockquote_text.lines() {
                    println!("{} {}", "│".bright_black(), line.italic().dimmed());
                }
                println!();
            }

            // Links
            Event::Start(Tag::Link { dest_url, .. }) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Link);
                print!("{}", text.bright_blue().underline());
                print!("{}", format!(" ({dest_url})").dimmed());

                // Skip to end tag
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Link)) {
                        break;
                    }
                    i += 1;
                }
            }

            // Horizontal rule
            Event::Rule => {
                println!();
                println!("{}", "─".repeat(80).bright_black());
                println!();
            }

            // Plain text (only if not inside other tags)
            Event::Text(text) => {
                if !in_code_block && !in_list {
                    print!("{text}");
                }
            }

            // Line breaks
            Event::SoftBreak => {
                print!(" ");
            }

            Event::HardBreak => {
                println!();
            }

            _ => {}
        }

        i += 1;
    }
}

/// Extract plain text until end tag
fn extract_text_until_end(events: &[Event], end_tag: TagEnd) -> String {
    let mut text = String::new();

    for event in events {
        match event {
            Event::Text(t) => text.push_str(t),
            Event::Code(t) => text.push_str(t),
            Event::End(tag) if *tag == end_tag => break,
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }

    text.trim().to_string()
}

/// Extract styled text until end tag (preserves formatting)
fn extract_styled_text_until_end(events: &[Event], end_tag: TagEnd) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < events.len() {
        match &events[i] {
            Event::Text(text) => {
                result.push_str(text);
            }
            Event::Code(code) => {
                result.push_str(&format!("{}", code.bright_magenta()));
            }
            Event::Start(Tag::Strong) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Strong);
                result.push_str(&format!("{}", text.bold()));
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Strong)) {
                        break;
                    }
                    i += 1;
                }
            }
            Event::Start(Tag::Emphasis) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Emphasis);
                result.push_str(&format!("{}", text.italic()));
                while i < events.len() {
                    if matches!(&events[i], Event::End(TagEnd::Emphasis)) {
                        break;
                    }
                    i += 1;
                }
            }
            Event::End(tag) if *tag == end_tag => break,
            Event::SoftBreak => result.push(' '),
            Event::HardBreak => result.push('\n'),
            _ => {}
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_rendering() {
        let markdown = r#"
# Heading 1
## Heading 2

This is **bold** and this is *italic*.

- Item 1
- Item 2
- Item 3

1. First
2. Second
3. Third

```rust
fn main() {
    println!("Hello, world!");
}
```

> This is a blockquote

`inline code`

[Link](https://example.com)

---
"#;

        // Just test that it doesn't panic
        render_markdown_to_terminal(markdown);
    }
}
