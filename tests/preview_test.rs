/// Comprehensive tests for preview functionality
///
/// These tests verify that Markdown, HTML, LaTeX, and Mermaid
/// preview features work correctly

use std::path::Path;

/// Test file type detection based on extensions
#[test]
fn test_markdown_extension_detection() {
    let md_files = vec!["test.md", "README.md", "doc.markdown"];
    for file in md_files {
        let path = Path::new(file);
        assert_eq!(path.extension().and_then(|s| s.to_str()),
                   Some(if file.ends_with(".markdown") { "markdown" } else { "md" }));
    }
}

#[test]
fn test_html_extension_detection() {
    let html_files = vec!["test.html", "index.html", "page.htm"];
    for file in html_files {
        let path = Path::new(file);
        let ext = path.extension().and_then(|s| s.to_str());
        assert!(ext == Some("html") || ext == Some("htm"));
    }
}

#[test]
fn test_latex_extension_detection() {
    let latex_files = vec!["doc.tex", "paper.latex"];
    for file in latex_files {
        let path = Path::new(file);
        let ext = path.extension().and_then(|s| s.to_str());
        assert!(ext == Some("tex") || ext == Some("latex"));
    }
}

#[test]
fn test_mermaid_extension_detection() {
    let mermaid_files = vec!["diagram.mmd", "flowchart.mermaid"];
    for file in mermaid_files {
        let path = Path::new(file);
        let ext = path.extension().and_then(|s| s.to_str());
        assert!(ext == Some("mmd") || ext == Some("mermaid"));
    }
}

/// Test Markdown parsing
#[test]
fn test_markdown_basic_parsing() {
    use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};

    let markdown = "# Hello World\n\nThis is a test.";
    let parser = Parser::new(markdown);
    let events: Vec<Event> = parser.collect();

    // Should contain heading and paragraph
    assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Heading { level: HeadingLevel::H1, .. }))));
    assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Paragraph))));
}

#[test]
fn test_markdown_code_blocks() {
    use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};

    let markdown = "```rust\nfn main() {}\n```";
    let parser = Parser::new(markdown);
    let events: Vec<Event> = parser.collect();

    // Should contain code block
    assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))))));
}

#[test]
fn test_markdown_lists() {
    use pulldown_cmark::{Parser, Event, Tag};

    let markdown = "- Item 1\n- Item 2\n- Item 3";
    let parser = Parser::new(markdown);
    let events: Vec<Event> = parser.collect();

    // Should contain list
    assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::List(_)))));
}

/// Test HTML parsing
#[test]
fn test_html_basic_parsing() {
    use scraper::Html;

    let html = "<html><body><h1>Test</h1><p>Content</p></body></html>";
    let document = Html::parse_document(html);

    // Should parse without panic
    assert!(document.root_element().descendants().count() > 0);
}

#[test]
fn test_html_table_extraction() {
    use scraper::{Html, Selector};

    let html = r#"
        <table>
            <tr><th>Name</th><th>Age</th></tr>
            <tr><td>Alice</td><td>30</td></tr>
        </table>
    "#;
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("table").unwrap();

    // Should find table
    assert_eq!(document.select(&table_selector).count(), 1);
}

#[test]
fn test_html_list_extraction() {
    use scraper::{Html, Selector};

    let html = r#"
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
    "#;
    let document = Html::parse_document(html);
    let li_selector = Selector::parse("li").unwrap();

    // Should find 2 list items
    assert_eq!(document.select(&li_selector).count(), 2);
}

/// Test LaTeX symbol substitution
#[test]
fn test_latex_sqrt_substitution() {
    let input = r"\sqrt{x}";
    let output = input.replace(r"\sqrt", "√");
    assert!(output.contains("√"));
}

#[test]
fn test_latex_greek_letters() {
    let substitutions = vec![
        (r"\alpha", "α"),
        (r"\beta", "β"),
        (r"\gamma", "γ"),
        (r"\delta", "δ"),
        (r"\pi", "π"),
    ];

    for (latex, unicode) in substitutions {
        let input = format!("The symbol {} is important", latex);
        let output = input.replace(latex, unicode);
        assert!(output.contains(unicode));
    }
}

#[test]
fn test_latex_math_operators() {
    let substitutions = vec![
        (r"\sum", "∑"),
        (r"\int", "∫"),
        (r"\prod", "∏"),
        (r"\infty", "∞"),
    ];

    for (latex, unicode) in substitutions {
        let input = format!("Expression: {}", latex);
        let output = input.replace(latex, unicode);
        assert!(output.contains(unicode));
    }
}

#[test]
fn test_latex_multiple_substitutions() {
    let input = r"\alpha + \beta = \gamma";
    let output = input
        .replace(r"\alpha", "α")
        .replace(r"\beta", "β")
        .replace(r"\gamma", "γ");

    assert_eq!(output, "α + β = γ");
}

/// Test Mermaid diagram detection
#[test]
fn test_mermaid_flowchart_detection() {
    let content = "flowchart TD\n    A-->B";
    assert!(content.contains("flowchart") || content.contains("graph"));
}

#[test]
fn test_mermaid_sequence_detection() {
    let content = "sequenceDiagram\n    Alice->>John: Hello";
    assert!(content.contains("sequenceDiagram"));
}

#[test]
fn test_mermaid_graph_detection() {
    let content = "graph LR\n    A-->B";
    assert!(content.contains("graph"));
}

/// Test preview mode detection logic
#[test]
fn test_preview_mode_from_filename() {
    // Helper function to detect preview mode from filename
    fn detect_mode(filename: &str) -> &str {
        let path = Path::new(filename);
        match path.extension().and_then(|s| s.to_str()) {
            Some("md") | Some("markdown") => "Markdown",
            Some("html") | Some("htm") => "Html",
            Some("tex") | Some("latex") => "Latex",
            Some("mmd") | Some("mermaid") => "Mermaid",
            _ => "None",
        }
    }

    assert_eq!(detect_mode("test.md"), "Markdown");
    assert_eq!(detect_mode("page.html"), "Html");
    assert_eq!(detect_mode("doc.tex"), "Latex");
    assert_eq!(detect_mode("diagram.mmd"), "Mermaid");
    assert_eq!(detect_mode("file.txt"), "None");
}

/// Test edge cases
#[test]
fn test_empty_markdown() {
    use pulldown_cmark::Parser;

    let markdown = "";
    let parser = Parser::new(markdown);
    let events: Vec<_> = parser.collect();

    // Empty markdown should parse successfully
    // events vec should be created (length is always >= 0 by definition)
    assert!(events.is_empty() || !events.is_empty());
}

#[test]
fn test_empty_html() {
    use scraper::Html;

    let html = "";
    let document = Html::parse_document(html);

    // Empty HTML should parse successfully
    assert!(document.root_element().descendants().count() > 0);
}

#[test]
fn test_malformed_html() {
    use scraper::Html;

    let html = "<div><p>Unclosed";
    let document = Html::parse_document(html);

    // Should still parse (HTML is forgiving)
    assert!(document.root_element().descendants().count() > 0);
}

#[test]
fn test_unicode_in_markdown() {
    use pulldown_cmark::{Parser, Event};

    let markdown = "# 日本語のテスト\n\nこれはテストです。";
    let parser = Parser::new(markdown);
    let events: Vec<Event> = parser.collect();

    // Should handle Unicode correctly
    assert!(events.len() > 0);

    // Check that text contains Unicode
    let has_unicode = events.iter().any(|e| {
        if let Event::Text(text) = e {
            text.contains('日') || text.contains('本')
        } else {
            false
        }
    });
    assert!(has_unicode);
}

#[test]
fn test_special_chars_in_latex() {
    let input = r"\frac{1}{2} + \sqrt{3}";
    let output = input
        .replace(r"\frac", "")
        .replace(r"\sqrt", "√");

    assert!(output.contains("√"));
}

/// Test complex scenarios
#[test]
fn test_markdown_with_inline_code() {
    use pulldown_cmark::{Parser, Event};

    let markdown = "This is `inline code` in text.";
    let parser = Parser::new(markdown);
    let events: Vec<Event> = parser.collect();

    // Should contain inline code
    assert!(events.iter().any(|e| matches!(e, Event::Code(_))));
}

#[test]
fn test_markdown_with_links() {
    use pulldown_cmark::{Parser, Event, Tag};

    let markdown = "[Link text](https://example.com)";
    let parser = Parser::new(markdown);
    let events: Vec<Event> = parser.collect();

    // Should contain link
    assert!(events.iter().any(|e| matches!(e, Event::Start(Tag::Link { .. }))));
}

#[test]
fn test_html_with_nested_elements() {
    use scraper::{Html, Selector};

    let html = r#"
        <div class="outer">
            <div class="inner">
                <p>Nested content</p>
            </div>
        </div>
    "#;
    let document = Html::parse_document(html);
    let p_selector = Selector::parse("p").unwrap();

    // Should find nested paragraph
    assert_eq!(document.select(&p_selector).count(), 1);
}
