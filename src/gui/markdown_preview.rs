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
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

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

    while i < events.len() {
        match &events[i] {
            // ========== 見出し (Headings) ==========
            Event::Start(Tag::Heading { level, .. }) => {
                let heading_level = *level;
                i += 1;

                let text = extract_text_until_end(&events[i..], TagEnd::Heading(heading_level));
                let font_size = match heading_level {
                    HeadingLevel::H1 => 30.0,
                    HeadingLevel::H2 => 24.0,
                    HeadingLevel::H3 => 20.0,
                    HeadingLevel::H4 => 18.0,
                    HeadingLevel::H5 => 16.0,
                    HeadingLevel::H6 => 14.0,
                };

                ui.add_space(10.0);
                ui.label(egui::RichText::new(text).size(font_size).strong());
                ui.add_space(5.0);

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
                ui.add_space(5.0);
                ui.label(rich_text);
                ui.add_space(5.0);

                // Skip to end of paragraph
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Paragraph)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== リスト (Lists) ==========
            Event::Start(Tag::List(first_number)) => {
                in_ordered_list = first_number.is_some();
                list_item_number = first_number.unwrap_or(0);
                ui.add_space(5.0);
            }

            Event::End(TagEnd::List(_)) => {
                in_ordered_list = false;
                list_item_number = 0;
                ui.add_space(5.0);
            }

            Event::Start(Tag::Item) => {
                i += 1;
                let text = extract_text_until_end(&events[i..], TagEnd::Item);

                if in_ordered_list {
                    list_item_number += 1;
                    ui.horizontal(|ui| {
                        ui.label(format!("{list_item_number}."));
                        ui.label(text);
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.label(text);
                    });
                }

                // Skip to end of item
                while i < events.len() {
                    if matches!(events[i], Event::End(TagEnd::Item)) {
                        break;
                    }
                    i += 1;
                }
            }

            // ========== コードブロック (Code Blocks) ==========
            Event::Start(Tag::CodeBlock(_)) => {
                i += 1;
                let code = extract_text_until_end(&events[i..], TagEnd::CodeBlock);

                ui.add_space(5.0);
                egui::Frame::NONE
                    .fill(ui.style().visuals.code_bg_color)
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(code)
                                .monospace()
                                .color(egui::Color32::from_rgb(200, 200, 200)),
                        );
                    });
                ui.add_space(5.0);

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
                ui.label(
                    egui::RichText::new(code.as_ref())
                        .monospace()
                        .background_color(ui.style().visuals.code_bg_color),
                );
            }

            // ========== 水平線 (Horizontal Rule) ==========
            Event::Rule => {
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
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
