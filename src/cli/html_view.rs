/// HTML viewer for CLI
///
/// This module renders HTML files in the terminal with:
/// - Colored output
/// - Proper heading hierarchy
/// - Formatted lists and tables
/// - Code blocks with highlighting
/// - Clean text extraction

use colored::*;
use scraper::{Html, Node, Selector};
use std::fmt::Write as FmtWrite;

/// Render HTML to terminal with formatting
pub fn render_html_to_terminal(html_content: &str) {
    let document = Html::parse_document(html_content);

    // Try to render with custom parser first
    if let Ok(formatted) = render_html_custom(&document) {
        print!("{}", formatted);
    } else {
        // Fallback to simple text conversion
        render_html_simple(html_content);
    }
}

/// Custom HTML renderer with rich formatting
fn render_html_custom(document: &Html) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = String::new();

    // Get body content, or root if no body
    let body_selector = Selector::parse("body").unwrap();
    let root = document.select(&body_selector).next()
        .map(|e| e.html())
        .unwrap_or_else(|| document.html());

    let body_doc = Html::parse_fragment(&root);

    // Process different elements
    render_headings(&body_doc, &mut output)?;
    render_paragraphs(&body_doc, &mut output)?;
    render_lists(&body_doc, &mut output)?;
    render_code_blocks(&body_doc, &mut output)?;
    render_tables(&body_doc, &mut output)?;

    Ok(output)
}

/// Render headings (h1-h6)
fn render_headings(document: &Html, output: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    for level in 1..=6 {
        let selector_str = format!("h{}", level);
        let Ok(selector) = Selector::parse(&selector_str) else {
            continue;
        };

        for element in document.select(&selector) {
            let text = element.text().collect::<String>().trim().to_string();
            if text.is_empty() {
                continue;
            }

            writeln!(output)?;
            match level {
                1 => {
                    writeln!(output, "{}", text.bold().bright_blue().underline())?;
                    writeln!(output, "{}", "=".repeat(text.len()).bright_blue())?;
                }
                2 => {
                    writeln!(output, "{}", text.bold().bright_cyan())?;
                    writeln!(output, "{}", "-".repeat(text.len()).bright_cyan())?;
                }
                3 => {
                    writeln!(output, "{}", text.bold().green())?;
                }
                4 => {
                    writeln!(output, "{}", text.bold().yellow())?;
                }
                5 => {
                    writeln!(output, "{}", text.yellow())?;
                }
                6 => {
                    writeln!(output, "{}", text.dimmed())?;
                }
                _ => {}
            }
            writeln!(output)?;
        }
    }

    Ok(())
}

/// Render paragraphs
fn render_paragraphs(document: &Html, output: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let Ok(selector) = Selector::parse("p") else {
        return Ok(());
    };

    for element in document.select(&selector) {
        let text = extract_styled_text(&element);
        if !text.trim().is_empty() {
            writeln!(output, "{}", text)?;
            writeln!(output)?;
        }
    }

    Ok(())
}

/// Extract text with inline styling (bold, italic, code)
fn extract_styled_text(element: &scraper::ElementRef) -> String {
    let mut result = String::new();

    for node in element.children() {
        match node.value() {
            Node::Text(text) => {
                result.push_str(text);
            }
            Node::Element(elem) => {
                let elem_ref = scraper::ElementRef::wrap(node).unwrap();
                let inner_text = elem_ref.text().collect::<String>();

                match elem.name() {
                    "strong" | "b" => {
                        result.push_str(&format!("{}", inner_text.bold()));
                    }
                    "em" | "i" => {
                        result.push_str(&format!("{}", inner_text.italic()));
                    }
                    "code" => {
                        result.push_str(&format!("{}", inner_text.bright_magenta().on_truecolor(50, 50, 50)));
                    }
                    "a" => {
                        if let Some(href) = elem_ref.value().attr("href") {
                            result.push_str(&format!("{} {}",
                                inner_text.bright_blue().underline(),
                                format!("({})", href).dimmed()
                            ));
                        } else {
                            result.push_str(&inner_text);
                        }
                    }
                    _ => {
                        result.push_str(&extract_styled_text(&elem_ref));
                    }
                }
            }
            _ => {}
        }
    }

    result
}

/// Render lists (ul, ol)
fn render_lists(document: &Html, output: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    // Unordered lists
    let Ok(ul_selector) = Selector::parse("ul") else {
        return Ok(());
    };
    let Ok(li_selector) = Selector::parse("li") else {
        return Ok(());
    };

    for ul in document.select(&ul_selector) {
        writeln!(output)?;
        for li in ul.select(&li_selector) {
            let text = li.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                writeln!(output, "  {} {}", "•".bright_green(), text)?;
            }
        }
        writeln!(output)?;
    }

    // Ordered lists
    let Ok(ol_selector) = Selector::parse("ol") else {
        return Ok(());
    };

    for ol in document.select(&ol_selector) {
        writeln!(output)?;
        let mut counter = 1;
        for li in ol.select(&li_selector) {
            let text = li.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                writeln!(output, "  {}. {}", counter.to_string().bright_yellow(), text)?;
                counter += 1;
            }
        }
        writeln!(output)?;
    }

    Ok(())
}

/// Render code blocks (pre, code)
fn render_code_blocks(document: &Html, output: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let Ok(selector) = Selector::parse("pre") else {
        return Ok(());
    };

    for pre in document.select(&selector) {
        let code = pre.text().collect::<String>();
        if code.trim().is_empty() {
            continue;
        }

        writeln!(output)?;
        writeln!(output, "{}", "```".dimmed())?;

        for line in code.lines() {
            writeln!(output, "{}", line.on_truecolor(40, 44, 52))?;
        }

        writeln!(output, "{}", "```".dimmed())?;
        writeln!(output)?;
    }

    Ok(())
}

/// Render tables
fn render_tables(document: &Html, output: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let Ok(table_selector) = Selector::parse("table") else {
        return Ok(());
    };
    let Ok(th_selector) = Selector::parse("th") else {
        return Ok(());
    };
    let Ok(tr_selector) = Selector::parse("tr") else {
        return Ok(());
    };
    let Ok(td_selector) = Selector::parse("td") else {
        return Ok(());
    };

    for table in document.select(&table_selector) {
        writeln!(output)?;
        writeln!(output, "{}", "Table:".bold().cyan())?;

        // Headers
        let mut headers = Vec::new();
        for th in table.select(&th_selector) {
            headers.push(th.text().collect::<String>().trim().to_string());
        }

        if !headers.is_empty() {
            writeln!(output, "  {}", headers.join(" | ").bold())?;
            writeln!(output, "  {}", "-".repeat(headers.len() * 10))?;
        }

        // Rows
        for tr in table.select(&tr_selector) {
            let mut cells = Vec::new();
            for td in tr.select(&td_selector) {
                cells.push(td.text().collect::<String>().trim().to_string());
            }

            if !cells.is_empty() {
                writeln!(output, "  {}", cells.join(" | "))?;
            }
        }

        writeln!(output)?;
    }

    Ok(())
}

/// Simple HTML to text fallback
fn render_html_simple(html_content: &str) {
    let text = html2text::from_read(html_content.as_bytes(), 80);
    println!("{}", text);
}

/// Render HTML without colors (plain text)
pub fn render_html_plain(html_content: &str) {
    let text = html2text::from_read(html_content.as_bytes(), 80);
    println!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_rendering() {
        let html = r#"
<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body>
    <h1>Main Title</h1>
    <h2>Subtitle</h2>

    <p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>

    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>

    <ol>
        <li>First</li>
        <li>Second</li>
    </ol>

    <pre><code>
fn main() {
    println!("Hello!");
}
    </code></pre>

    <table>
        <tr><th>Name</th><th>Age</th></tr>
        <tr><td>Alice</td><td>30</td></tr>
        <tr><td>Bob</td><td>25</td></tr>
    </table>
</body>
</html>
"#;

        // Just test that it doesn't panic
        render_html_to_terminal(html);
        render_html_plain(html);
    }

    #[test]
    fn test_simple_html() {
        let html = "<h1>Title</h1><p>Content</p>";
        render_html_to_terminal(html);
    }
}
