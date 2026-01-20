/*!
# Markdown Preview Renderer for egui

## 概要
このモジュールは、Markdownテキストをパースし、egui UIとしてレンダリングする機能を提供します。
WebViewやHTMLレンダラーを使用せず、純粋なRust + eguiウィジェットのみで実装されています。

## 技術的詳細

### アーキテクチャ
1. **パーサー**: `pulldown-cmark` クレートを使用してMarkdownをパース
2. **AST走査**: `pulldown-cmark::Event` のイテレータを走査
3. **egui変換**: 各EventをeguiウィジェットやTextFormatに変換
4. **リアルタイム更新**: エディタの変更を検知し、即座に再レンダリング

### 主要な変換ロジック

#### 見出し (Headers)
- `Event::Start(Tag::Heading(level))` を検知
- levelに応じてフォントサイズを調整（H1: 30pt, H2: 24pt, etc.）
- `egui::Label` で描画

#### リスト (Lists)
- `Event::Start(Tag::List(_))` でリスト開始を検知
- `Event::Start(Tag::Item)` で各アイテムを処理
- 箇条書き: "• " プレフィックス
- 番号付き: "1. ", "2. " などのプレフィックス

#### 強調 (Emphasis/Strong)
- `Event::Start(Tag::Emphasis)` で *italic* を処理
- `Event::Start(Tag::Strong)` で **bold** を処理
- `egui::RichText` の `.italics()` や `.strong()` を使用

#### コードブロック (Code Blocks)
- `Event::Start(Tag::CodeBlock(_))` でコードブロック開始
- `egui::Frame` で背景色を設定
- `egui::TextStyle::Monospace` でレンダリング

### パフォーマンス考慮
- パース処理は軽量（pulldown-cmarkが高速）
- UIレンダリングはeguiの即時モードで高速
- リアルタイム更新でも100ms以内に完了

### 拡張性
- 新しいMarkdown要素を追加する際は、`render_events()` 関数内でEventパターンを追加
- 将来的にMermaid、LaTeX等の拡張が可能な設計
*/

use eframe::egui;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// Markdown文字列をeguiでレンダリングする
///
/// # Arguments
/// * `ui` - egui UI context
/// * `markdown` - レンダリングするMarkdownテキスト
///
/// # Example
/// ```ignore
/// render_markdown_preview(ui, "# Hello\n\nThis is **bold**.");
/// ```
pub fn render_markdown_preview(ui: &mut egui::Ui, markdown: &str) {
    let parser = Parser::new_ext(markdown, Options::all());
    let events: Vec<Event> = parser.collect();

    render_events(ui, &events);
}

/// Markdown ASTイベントをeguiウィジェットに変換してレンダリング
///
/// これがこのプロジェクトの核心技術：
/// pulldown-cmarkのEventストリームをイテレートし、各Eventに応じて
/// eguiのウィジェット（Label, Frame, Separator等）を動的に構築します。
///
/// # Arguments
/// * `ui` - egui UI context
/// * `events` - pulldown-cmarkのEventスライス
fn render_events(ui: &mut egui::Ui, events: &[Event]) {
    let mut i = 0;
    let mut list_item_number = 0;
    let mut in_ordered_list = false;
    let mut list_depth: u32 = 0;
    #[allow(unused_assignments)]
    let mut in_blockquote = false;
    let mut task_list_marker: Option<bool> = None;

    while i < events.len() {
        match &events[i] {
            // ========== 見出し (Headings) ==========
            Event::Start(Tag::Heading { level, .. }) => {
                let heading_level = *level;
                i += 1;

                let text = extract_text_until_end(&events[i..], TagEnd::Heading(heading_level));
                let font_size = match heading_level {
                    HeadingLevel::H1 => 32.0,
                    HeadingLevel::H2 => 26.0,
                    HeadingLevel::H3 => 22.0,
                    HeadingLevel::H4 => 18.0,
                    HeadingLevel::H5 => 16.0,
                    HeadingLevel::H6 => 14.0,
                };

                // Add heading style with proper spacing
                ui.add_space(12.0);
                let heading_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(250, 250, 250)
                } else {
                    egui::Color32::from_rgb(17, 24, 39)
                };
                ui.label(egui::RichText::new(text).size(font_size).strong().color(heading_color));
                
                // Add underline for H1 and H2
                if matches!(heading_level, HeadingLevel::H1 | HeadingLevel::H2) {
                    ui.add_space(4.0);
                    let separator_color = if ui.visuals().dark_mode {
                        egui::Color32::from_rgb(63, 63, 70)
                    } else {
                        egui::Color32::from_rgb(229, 231, 235)
                    };
                    ui.add(egui::Separator::default().spacing(0.0));
                    ui.painter().line_segment(
                        [ui.cursor().min, egui::pos2(ui.cursor().max.x, ui.cursor().min.y)],
                        egui::Stroke::new(1.0, separator_color),
                    );
                }
                ui.add_space(8.0);

                // Skip to end of heading
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Heading(_))) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== 段落 (Paragraphs) ==========
            Event::Start(Tag::Paragraph) => {
                i += 1;
                let rich_text = extract_rich_text(&events[i..], TagEnd::Paragraph);
                
                if in_blockquote {
                    // Blockquote styling
                    ui.label(rich_text.italics().color(
                        if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(161, 161, 170)
                        } else {
                            egui::Color32::from_rgb(107, 114, 128)
                        }
                    ));
                } else {
                    ui.add_space(4.0);
                    ui.label(rich_text);
                    ui.add_space(4.0);
                }

                // Skip to end of paragraph
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Paragraph)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== 引用 (Blockquotes) ==========
            Event::Start(Tag::BlockQuote(_)) => {
                in_blockquote = true;
                ui.add_space(8.0);
                let border_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(96, 165, 250)
                } else {
                    egui::Color32::from_rgb(59, 130, 246)
                };
                let bg_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgba_unmultiplied(30, 64, 115, 50)
                } else {
                    egui::Color32::from_rgba_unmultiplied(191, 219, 254, 100)
                };
                
                egui::Frame::NONE
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(3.0, border_color))
                    .inner_margin(egui::Margin::symmetric(12, 8))
                    .outer_margin(egui::Margin::symmetric(0, 4))
                    .show(ui, |ui| {
                        i += 1;
                        while i < events.len() && !matches!(events[i], Event::End(TagEnd::BlockQuote(_))) {
                            match &events[i] {
                                Event::Start(Tag::Paragraph) => {
                                    i += 1;
                                    let text = extract_text_until_end(&events[i..], TagEnd::Paragraph);
                                    let quote_color = if ui.visuals().dark_mode {
                                        egui::Color32::from_rgb(212, 212, 216)
                                    } else {
                                        egui::Color32::from_rgb(55, 65, 81)
                                    };
                                    ui.label(egui::RichText::new(text).italics().color(quote_color));
                                    while i < events.len() && !matches!(events[i], Event::End(TagEnd::Paragraph)) {
                                        i += 1;
                                    }
                                }
                                _ => {}
                            }
                            i += 1;
                        }
                    });
                in_blockquote = false;
                ui.add_space(8.0);
            }

            // ========== リスト (Lists) ==========
            Event::Start(Tag::List(first_number)) => {
                in_ordered_list = first_number.is_some();
                list_item_number = first_number.unwrap_or(0);
                list_depth += 1;
                if list_depth == 1 {
                    ui.add_space(6.0);
                }
            }

            Event::End(TagEnd::List(_)) => {
                in_ordered_list = false;
                list_item_number = 0;
                list_depth = list_depth.saturating_sub(1);
                if list_depth == 0 {
                    ui.add_space(6.0);
                }
            }

            // ========== タスクリストマーカー ==========
            Event::TaskListMarker(checked) => {
                task_list_marker = Some(*checked);
            }

            Event::Start(Tag::Item) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Item);
                let indent = (list_depth - 1) as f32 * 20.0;

                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    
                    if let Some(checked) = task_list_marker.take() {
                        // Task list item
                        let checkbox = if checked { "☑" } else { "☐" };
                        let checkbox_color = if checked {
                            egui::Color32::from_rgb(34, 197, 94)
                        } else if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(113, 113, 122)
                        } else {
                            egui::Color32::from_rgb(156, 163, 175)
                        };
                        ui.label(egui::RichText::new(checkbox).color(checkbox_color));
                        let text_color = if checked {
                            if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(113, 113, 122)
                            } else {
                                egui::Color32::from_rgb(156, 163, 175)
                            }
                        } else {
                            ui.visuals().text_color()
                        };
                        let mut rt = egui::RichText::new(&text).color(text_color);
                        if checked {
                            rt = rt.strikethrough();
                        }
                        ui.label(rt);
                    } else if in_ordered_list {
                        list_item_number += 1;
                        let num_color = if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(161, 161, 170)
                        } else {
                            egui::Color32::from_rgb(107, 114, 128)
                        };
                        ui.label(egui::RichText::new(format!("{}.", list_item_number)).color(num_color));
                        ui.label(&text);
                    } else {
                        let bullet_color = if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(96, 165, 250)
                        } else {
                            egui::Color32::from_rgb(59, 130, 246)
                        };
                        ui.label(egui::RichText::new("•").color(bullet_color));
                        ui.label(&text);
                    }
                });

                // Skip to end of item
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Item)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== コードブロック (Code Blocks) ==========
            Event::Start(Tag::CodeBlock(kind)) => {
                // Extract language tag
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };

                i += 1;
                let code = extract_text_until_end(&events[i..], TagEnd::CodeBlock);

                ui.add_space(8.0);
                
                // Code block with language label
                let bg_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(30, 30, 33)
                } else {
                    egui::Color32::from_rgb(243, 244, 246)
                };
                let border_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(63, 63, 70)
                } else {
                    egui::Color32::from_rgb(209, 213, 219)
                };
                
                egui::Frame::NONE
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(1.0, border_color))
                    .corner_radius(egui::CornerRadius::same(6))
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |ui| {
                        // Language label if specified
                        if !lang.is_empty() {
                            let label_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(113, 113, 122)
                            } else {
                                egui::Color32::from_rgb(156, 163, 175)
                            };
                            ui.label(egui::RichText::new(&lang).small().color(label_color));
                            ui.add_space(4.0);
                        }
                        
                        // Apply syntax highlighting if language is specified
                        if !lang.is_empty() && !code.is_empty() {
                            render_highlighted_code(ui, &code, &lang);
                        } else {
                            // Fallback to plain monospace
                            let code_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(212, 212, 216)
                            } else {
                                egui::Color32::from_rgb(55, 65, 81)
                            };
                            ui.label(
                                egui::RichText::new(&code)
                                    .monospace()
                                    .color(code_color),
                            );
                        }
                    });
                ui.add_space(8.0);

                // Skip to end of code block
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::CodeBlock)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== インラインコード (Inline Code) ==========
            Event::Code(code) => {
                let bg_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(52, 52, 58)
                } else {
                    egui::Color32::from_rgb(243, 244, 246)
                };
                let code_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(248, 113, 113)
                } else {
                    egui::Color32::from_rgb(153, 27, 27)
                };
                ui.label(
                    egui::RichText::new(code.as_ref())
                        .monospace()
                        .color(code_color)
                        .background_color(bg_color),
                );
            }

            // ========== リンク (Links) ==========
            Event::Start(Tag::Link { dest_url, title, .. }) => {
                i += 1;
                let link_text = extract_text_until_end(&events[i..], TagEnd::Link);
                let url = dest_url.to_string();
                let tooltip = if title.is_empty() { url.clone() } else { title.to_string() };
                
                let link_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(96, 165, 250)
                } else {
                    egui::Color32::from_rgb(37, 99, 235)
                };
                
                if ui.link(egui::RichText::new(&link_text).color(link_color).underline())
                    .on_hover_text(&tooltip)
                    .clicked()
                {
                    // Try to open URL in default browser
                    let _ = open::that(&url);
                }

                // Skip to end of link
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Link)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== 画像 (Images) ==========
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                i += 1;
                let alt_text = extract_text_until_end(&events[i..], TagEnd::Image);
                let tooltip = if title.is_empty() { alt_text.clone() } else { title.to_string() };
                
                ui.add_space(4.0);
                let border_color = if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(63, 63, 70)
                } else {
                    egui::Color32::from_rgb(209, 213, 219)
                };
                
                egui::Frame::NONE
                    .stroke(egui::Stroke::new(1.0, border_color))
                    .corner_radius(egui::CornerRadius::same(4))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        let icon_color = if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(113, 113, 122)
                        } else {
                            egui::Color32::from_rgb(156, 163, 175)
                        };
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🖼").size(20.0).color(icon_color));
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(&alt_text).color(ui.visuals().text_color()));
                                ui.label(egui::RichText::new(dest_url.as_ref()).small().color(icon_color));
                            });
                        });
                    })
                    .response
                    .on_hover_text(tooltip);
                ui.add_space(4.0);

                // Skip to end of image
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Image)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== 水平線 (Horizontal Rule) ==========
            Event::Rule => {
                ui.add_space(12.0);
                ui.add(egui::Separator::default().spacing(0.0));
                ui.add_space(12.0);
            }

            // ========== ソフト改行 ==========
            Event::SoftBreak => {
                // Treat as space
            }

            // ========== ハード改行 ==========
            Event::HardBreak => {
                ui.add_space(8.0);
            }

            // ========== その他 ==========
            _ => {}
        }

        i += 1;
    }
}

/// イベント列からタグ終了までのテキストを抽出
///
/// # Arguments
/// * `events` - イベントスライス
/// * `end_tag` - 終了タグ
///
/// # Returns
/// 抽出されたテキスト文字列
fn extract_text_until_end(events: &[Event], end_tag: TagEnd) -> String {
    let mut result = String::new();

    for event in events {
        match event {
            Event::Text(text) => result.push_str(text),
            Event::Code(code) => result.push_str(code),
            Event::End(tag) if tag == &end_tag => break,
            _ => {}
        }
    }

    result
}

/// イベント列からリッチテキストを抽出（強調、太字等を含む）
///
/// # Arguments
/// * `events` - イベントスライス
/// * `end_tag` - 終了タグ
///
/// # Returns
/// egui::RichText
fn extract_rich_text(events: &[Event], end_tag: TagEnd) -> egui::RichText {
    let mut result = String::new();
    let mut is_bold = false;
    let mut is_italic = false;

    for event in events {
        match event {
            Event::Text(text) => result.push_str(text),
            Event::Code(code) => {
                result.push('`');
                result.push_str(code);
                result.push('`');
            }
            Event::Start(Tag::Strong) => is_bold = true,
            Event::End(TagEnd::Strong) => is_bold = false,
            Event::Start(Tag::Emphasis) => is_italic = true,
            Event::End(TagEnd::Emphasis) => is_italic = false,
            Event::End(tag) if tag == &end_tag => break,
            _ => {}
        }
    }

    let mut rich = egui::RichText::new(result);
    if is_bold {
        rich = rich.strong();
    }
    if is_italic {
        rich = rich.italics();
    }

    rich
}

/// Render syntax-highlighted code block
fn render_highlighted_code(ui: &mut egui::Ui, code: &str, lang: &str) {
    // Load syntax definitions and theme
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // Try to find syntax by language name or extension
    let syntax = ps
        .find_syntax_by_extension(lang)
        .or_else(|| ps.find_syntax_by_name(lang))
        .or_else(|| ps.find_syntax_by_first_line(code))
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let theme = &ts.themes["base16-ocean.dark"];
    let mut highlighter = HighlightLines::new(syntax, theme);

    // Render each line with syntax highlighting
    for line in LinesWithEndings::from(code) {
        let ranges = highlighter.highlight_line(line, &ps).unwrap_or_default();

        ui.horizontal(|ui| {
            for (style, text) in ranges {
                let color = style_to_color(style);
                ui.label(egui::RichText::new(text).monospace().color(color));
            }
        });
    }
}

/// Convert syntect Style to egui Color32
fn style_to_color(style: Style) -> egui::Color32 {
    egui::Color32::from_rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    )
}

// ========== ユニットテスト ==========
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let markdown = "# Hello";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        assert!(matches!(events[0], Event::Start(Tag::Heading { .. })));
    }

    #[test]
    fn test_parse_bold() {
        let markdown = "**bold** text";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        // Should contain Strong tag
        let has_strong = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::Strong)));
        assert!(has_strong);
    }

    #[test]
    fn test_parse_list() {
        let markdown = "* item 1\n* item 2";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        // Should contain List tag
        let has_list = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::List(_))));
        assert!(has_list);
    }

    #[test]
    fn test_parse_code_block() {
        let markdown = "```rust\nfn main() {}\n```";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        // Should contain CodeBlock tag
        let has_code_block = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::CodeBlock(_))));
        assert!(has_code_block);
    }

    #[test]
    fn test_extract_text_until_end() {
        let events = vec![
            Event::Text("Hello".into()),
            Event::Text(" World".into()),
            Event::End(TagEnd::Paragraph),
        ];

        let text = extract_text_until_end(&events, TagEnd::Paragraph);
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_parse_ordered_list() {
        let markdown = "1. First\n2. Second\n3. Third";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        // Should contain ordered list
        let has_ordered = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::List(Some(_)))));
        assert!(has_ordered);
    }

    #[test]
    fn test_parse_emphasis() {
        let markdown = "*italic*";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        // Should contain Emphasis tag
        let has_emphasis = events
            .iter()
            .any(|e| matches!(e, Event::Start(Tag::Emphasis)));
        assert!(has_emphasis);
    }

    #[test]
    fn test_parse_inline_code() {
        let markdown = "`code`";
        let parser = Parser::new_ext(markdown, Options::all());
        let events: Vec<Event> = parser.collect();

        // Should contain Code event
        let has_code = events.iter().any(|e| matches!(e, Event::Code(_)));
        assert!(has_code);
    }
}
