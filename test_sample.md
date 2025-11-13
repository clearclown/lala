# Markdown Syntax Highlighting Test

## Introduction

This is a **test file** for *markdown* syntax highlighting. Let's see if ***bold and italic*** work correctly.

## Code Blocks

Here's some inline `code` and a code block:

```rust
fn main() {
    println!("Hello from markdown!");
    let x = 42;
}
```

```python
def hello():
    print("Hello, Python!")
    return True
```

## Lists

### Unordered Lists

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
    - Deeply nested item
- Item 3

### Ordered Lists

1. First item
2. Second item
3. Third item
   1. Sub-item 3.1
   2. Sub-item 3.2

## Links and Images

[Link to Rust website](https://www.rust-lang.org/)

[Link with title](https://example.com "Example Site")

![Alt text for image](image.png)

## Blockquotes

> This is a blockquote.
> It can span multiple lines.
>
> > This is a nested blockquote.

## Tables

| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |

## Horizontal Rules

---

***

___

## Text Formatting

**Bold text**

*Italic text*

***Bold and italic***

~~Strikethrough~~

`Inline code`

## Task Lists

- [x] Completed task
- [x] Another completed task
- [ ] Incomplete task
- [ ] Another incomplete task

## Escaping

Use backslash to escape: \*not italic\* \[not a link\]

## HTML (if supported)

<div style="color: red;">
Red text using HTML
</div>

## Footnotes

Here's a sentence with a footnote[^1].

[^1]: This is the footnote content.

## Definition Lists

Term 1
: Definition 1

Term 2
: Definition 2a
: Definition 2b
