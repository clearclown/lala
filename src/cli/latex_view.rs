/// LaTeX viewer for CLI
///
/// This module renders LaTeX documents in the terminal with:
/// - Unicode approximations of math symbols
/// - Document structure preservation
/// - Syntax-highlighted source code view
/// - Basic equation rendering
///
/// For high-quality rendering, users can compile with:
/// `pdflatex document.tex` or use Overleaf

use colored::*;
use regex::Regex;
use std::collections::HashMap;

/// Render LaTeX to terminal with formatting
pub fn render_latex_to_terminal(latex_content: &str) {
    println!();
    println!("{}", "LaTeX Document".bold().bright_cyan());
    println!("{}", "─".repeat(80).bright_black());
    println!();

    // Parse and render different parts
    render_preamble(latex_content);
    render_title_section(latex_content);
    render_sections(latex_content);
    render_equations(latex_content);
    render_lists(latex_content);

    println!();
    println!("{}", "─".repeat(80).bright_black());
    println!();
    println!("{}", "Note: For full LaTeX compilation:".dimmed());
    println!("{}", "  pdflatex document.tex".dimmed());
    println!("{}", "  or use Overleaf: https://www.overleaf.com".dimmed());
    println!();
}

/// Render preamble information (documentclass, packages)
fn render_preamble(content: &str) {
    let doc_class_re = Regex::new(r"\\documentclass\[?([^\]]*)\]?\{([^}]+)\}").unwrap();
    let package_re = Regex::new(r"\\usepackage\[?([^\]]*)\]?\{([^}]+)\}").unwrap();

    if let Some(cap) = doc_class_re.captures(content) {
        let doc_type = cap.get(2).unwrap().as_str();
        println!("{} {}", "Document Type:".bold(), doc_type.bright_blue());

        if let Some(options) = cap.get(1) {
            if !options.as_str().is_empty() {
                println!("{} {}", "Options:".bold(), options.as_str().dimmed());
            }
        }
    }

    let packages: Vec<_> = package_re.captures_iter(content)
        .map(|cap| cap.get(2).unwrap().as_str())
        .collect();

    if !packages.is_empty() {
        println!("{} {}", "Packages:".bold(), packages.join(", ").dimmed());
    }

    println!();
}

/// Render title, author, date section
fn render_title_section(content: &str) {
    let title_re = Regex::new(r"\\title\{([^}]+)\}").unwrap();
    let author_re = Regex::new(r"\\author\{([^}]+)\}").unwrap();
    let date_re = Regex::new(r"\\date\{([^}]+)\}").unwrap();

    if let Some(cap) = title_re.captures(content) {
        let title = cap.get(1).unwrap().as_str();
        println!("{}", title.bold().bright_blue().underline());
        println!("{}", "=".repeat(title.len()).bright_blue());
        println!();
    }

    if let Some(cap) = author_re.captures(content) {
        let author = cap.get(1).unwrap().as_str();
        println!("{} {}", "Author:".bold(), author.italic());
    }

    if let Some(cap) = date_re.captures(content) {
        let date = cap.get(1).unwrap().as_str();
        println!("{} {}", "Date:".bold(), date.dimmed());
    }

    println!();
}

/// Render sections and subsections
fn render_sections(content: &str) {
    let section_re = Regex::new(r"\\section\{([^}]+)\}").unwrap();
    let subsection_re = Regex::new(r"\\subsection\{([^}]+)\}").unwrap();
    let subsubsection_re = Regex::new(r"\\subsubsection\{([^}]+)\}").unwrap();

    // Extract document body
    let begin_doc = content.find(r"\begin{document}");
    let end_doc = content.find(r"\end{document}");

    let body = if let (Some(start), Some(end)) = (begin_doc, end_doc) {
        &content[start..end]
    } else {
        content
    };

    for line in body.lines() {
        if let Some(cap) = section_re.captures(line) {
            let section = cap.get(1).unwrap().as_str();
            println!();
            println!("{}", section.bold().bright_cyan());
            println!("{}", "-".repeat(section.len()).bright_cyan());
            println!();
        } else if let Some(cap) = subsection_re.captures(line) {
            let subsection = cap.get(1).unwrap().as_str();
            println!();
            println!("  {}", subsection.bold().green());
            println!();
        } else if let Some(cap) = subsubsection_re.captures(line) {
            let subsubsection = cap.get(1).unwrap().as_str();
            println!("    {}", subsubsection.bold().yellow());
            println!();
        } else if !line.trim().starts_with('\\') && !line.trim().is_empty() {
            // Regular text
            let cleaned = clean_latex_text(line);
            if !cleaned.is_empty() {
                println!("{}", cleaned);
            }
        }
    }
}

/// Render equations (inline and display)
fn render_equations(content: &str) {
    let display_math_re = Regex::new(r"\$\$([^$]+)\$\$").unwrap();
    let inline_math_re = Regex::new(r"\$([^$]+)\$").unwrap();
    let equation_env_re = Regex::new(r"\\begin\{equation\}(.*?)\\end\{equation\}").unwrap();

    println!("{}", "Mathematics:".bold().bright_magenta());
    println!();

    // Display math ($$...$$)
    for cap in display_math_re.captures_iter(content) {
        let math = cap.get(1).unwrap().as_str();
        println!("  {}", "Display Equation:".bold());
        println!("    {}", render_math(math).bright_cyan());
        println!();
    }

    // Equation environment
    for cap in equation_env_re.captures_iter(content) {
        let math = cap.get(1).unwrap().as_str();
        println!("  {}", "Equation:".bold());
        println!("    {}", render_math(math).bright_cyan());
        println!();
    }

    // Inline math ($...$)
    let inline_count = inline_math_re.captures_iter(content).count();
    if inline_count > 0 {
        println!("  {} {} inline math expressions", "Found".dimmed(), inline_count);
    }

    println!();
}

/// Render lists (itemize, enumerate)
fn render_lists(content: &str) {
    let itemize_re = Regex::new(r"\\begin\{itemize\}(.*?)\\end\{itemize\}").unwrap();
    let enumerate_re = Regex::new(r"\\begin\{enumerate\}(.*?)\\end\{enumerate\}").unwrap();
    let item_re = Regex::new(r"\\item\s+(.+)").unwrap();

    // Itemize (bullet lists)
    for cap in itemize_re.captures_iter(content) {
        let list_content = cap.get(1).unwrap().as_str();
        println!("{}", "List:".bold());

        for item_cap in item_re.captures_iter(list_content) {
            let item_text = item_cap.get(1).unwrap().as_str();
            println!("  {} {}", "•".bright_green(), clean_latex_text(item_text));
        }
        println!();
    }

    // Enumerate (numbered lists)
    for cap in enumerate_re.captures_iter(content) {
        let list_content = cap.get(1).unwrap().as_str();
        println!("{}", "Numbered List:".bold());

        let mut counter = 1;
        for item_cap in item_re.captures_iter(list_content) {
            let item_text = item_cap.get(1).unwrap().as_str();
            println!("  {}. {}", counter.to_string().bright_yellow(), clean_latex_text(item_text));
            counter += 1;
        }
        println!();
    }
}

/// Render math with Unicode approximations
fn render_math(math: &str) -> String {
    let mut result = math.to_string();

    // Create symbol map
    let mut symbols = HashMap::new();

    // Greek letters (lowercase)
    symbols.insert(r"\alpha", "α");
    symbols.insert(r"\beta", "β");
    symbols.insert(r"\gamma", "γ");
    symbols.insert(r"\delta", "δ");
    symbols.insert(r"\epsilon", "ε");
    symbols.insert(r"\zeta", "ζ");
    symbols.insert(r"\eta", "η");
    symbols.insert(r"\theta", "θ");
    symbols.insert(r"\lambda", "λ");
    symbols.insert(r"\mu", "μ");
    symbols.insert(r"\pi", "π");
    symbols.insert(r"\rho", "ρ");
    symbols.insert(r"\sigma", "σ");
    symbols.insert(r"\tau", "τ");
    symbols.insert(r"\phi", "φ");
    symbols.insert(r"\chi", "χ");
    symbols.insert(r"\psi", "ψ");
    symbols.insert(r"\omega", "ω");

    // Greek letters (uppercase)
    symbols.insert(r"\Gamma", "Γ");
    symbols.insert(r"\Delta", "Δ");
    symbols.insert(r"\Theta", "Θ");
    symbols.insert(r"\Lambda", "Λ");
    symbols.insert(r"\Pi", "Π");
    symbols.insert(r"\Sigma", "Σ");
    symbols.insert(r"\Phi", "Φ");
    symbols.insert(r"\Psi", "Ψ");
    symbols.insert(r"\Omega", "Ω");

    // Math operators
    symbols.insert(r"\sum", "Σ");
    symbols.insert(r"\prod", "Π");
    symbols.insert(r"\int", "∫");
    symbols.insert(r"\oint", "∮");
    symbols.insert(r"\partial", "∂");
    symbols.insert(r"\nabla", "∇");
    symbols.insert(r"\infty", "∞");
    symbols.insert(r"\pm", "±");
    symbols.insert(r"\mp", "∓");
    symbols.insert(r"\times", "×");
    symbols.insert(r"\div", "÷");
    symbols.insert(r"\neq", "≠");
    symbols.insert(r"\leq", "≤");
    symbols.insert(r"\geq", "≥");
    symbols.insert(r"\approx", "≈");
    symbols.insert(r"\equiv", "≡");
    symbols.insert(r"\propto", "∝");
    symbols.insert(r"\in", "∈");
    symbols.insert(r"\notin", "∉");
    symbols.insert(r"\subset", "⊂");
    symbols.insert(r"\supset", "⊃");
    symbols.insert(r"\cup", "∪");
    symbols.insert(r"\cap", "∩");
    symbols.insert(r"\emptyset", "∅");
    symbols.insert(r"\forall", "∀");
    symbols.insert(r"\exists", "∃");
    symbols.insert(r"\neg", "¬");
    symbols.insert(r"\wedge", "∧");
    symbols.insert(r"\vee", "∨");
    symbols.insert(r"\rightarrow", "→");
    symbols.insert(r"\leftarrow", "←");
    symbols.insert(r"\leftrightarrow", "↔");
    symbols.insert(r"\Rightarrow", "⇒");
    symbols.insert(r"\Leftarrow", "⇐");
    symbols.insert(r"\Leftrightarrow", "⇔");

    // Replace symbols
    for (latex, unicode) in symbols {
        result = result.replace(latex, unicode);
    }

    // Handle sqrt
    let sqrt_re = Regex::new(r"\\sqrt\{([^}]+)\}").unwrap();
    result = sqrt_re.replace_all(&result, "√($1)").to_string();

    // Handle frac
    let frac_re = Regex::new(r"\\frac\{([^}]+)\}\{([^}]+)\}").unwrap();
    result = frac_re.replace_all(&result, "($1)/($2)").to_string();

    // Handle superscript (simple cases)
    result = result.replace("^2", "²");
    result = result.replace("^3", "³");
    result = result.replace("^{2}", "²");
    result = result.replace("^{3}", "³");

    // Handle text commands
    result = result.replace(r"\text", "");

    // Clean up braces
    result = result.replace('{', "").replace('}', "");

    result.trim().to_string()
}

/// Clean LaTeX text (remove commands, preserve content)
fn clean_latex_text(text: &str) -> String {
    let mut result = text.to_string();

    // Remove common formatting commands
    result = result.replace(r"\textbf{", "");
    result = result.replace(r"\textit{", "");
    result = result.replace(r"\emph{", "");
    result = result.replace(r"\underline{", "");
    result = result.replace('}', "");

    // Remove comments
    if let Some(comment_pos) = result.find('%') {
        result = result[..comment_pos].to_string();
    }

    result.trim().to_string()
}

/// Render LaTeX without colors (plain text)
pub fn render_latex_plain(latex_content: &str) {
    println!("LaTeX Document");
    println!("{}", "─".repeat(80));
    println!();
    println!("{}", latex_content);
    println!();
    println!("{}", "─".repeat(80));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_rendering() {
        let latex = r#"
\documentclass{article}
\usepackage{amsmath}

\title{Sample Document}
\author{John Doe}
\date{\today}

\begin{document}

\maketitle

\section{Introduction}

This is a sample LaTeX document with math: $E = mc^2$.

\subsection{Equations}

Display math:
$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

\section{Lists}

\begin{itemize}
\item First item
\item Second item
\end{itemize}

\end{document}
"#;

        render_latex_to_terminal(latex);
    }

    #[test]
    fn test_math_rendering() {
        assert!(render_math(r"\alpha + \beta").contains('α'));
        assert!(render_math(r"\sum_{i=1}^n").contains('Σ'));
        assert!(render_math(r"\sqrt{2}").contains('√'));
    }
}
