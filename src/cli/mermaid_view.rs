/// Mermaid diagram viewer for CLI
///
/// This module renders Mermaid diagrams in the terminal with:
/// - ASCII art diagrams
/// - Syntax-highlighted source code view
/// - Simple flowchart rendering
///
/// For high-quality rendering, users can install mermaid-cli:
/// `npm install -g @mermaid-js/mermaid-cli`

use colored::*;
use regex::Regex;

/// Render Mermaid diagram to terminal
pub fn render_mermaid_to_terminal(mermaid_content: &str) {
    // Detect diagram type
    let diagram_type = detect_diagram_type(mermaid_content);

    println!();
    println!("{}", "Mermaid Diagram".bold().bright_cyan());
    println!("{}", "─".repeat(80).bright_black());
    println!();

    match diagram_type.as_str() {
        "graph" | "flowchart" => render_flowchart(mermaid_content),
        "sequenceDiagram" => render_sequence_diagram(mermaid_content),
        "classDiagram" => render_class_diagram(mermaid_content),
        "stateDiagram" => render_state_diagram(mermaid_content),
        "erDiagram" => render_er_diagram(mermaid_content),
        "gantt" => render_gantt(mermaid_content),
        "pie" => render_pie_chart(mermaid_content),
        _ => render_source_view(mermaid_content),
    }

    println!();
    println!("{}", "─".repeat(80).bright_black());
    println!();
    println!("{}", "Note: For high-quality SVG/PNG output, use:".dimmed());
    println!("{}", "  npm install -g @mermaid-js/mermaid-cli".dimmed());
    println!("{}", "  mmdc -i diagram.mmd -o diagram.svg".dimmed());
    println!();
}

/// Detect Mermaid diagram type
fn detect_diagram_type(content: &str) -> String {
    let first_line = content.lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("");

    if first_line.contains("graph") || first_line.contains("flowchart") {
        "graph".to_string()
    } else if first_line.contains("sequenceDiagram") {
        "sequenceDiagram".to_string()
    } else if first_line.contains("classDiagram") {
        "classDiagram".to_string()
    } else if first_line.contains("stateDiagram") {
        "stateDiagram".to_string()
    } else if first_line.contains("erDiagram") {
        "erDiagram".to_string()
    } else if first_line.contains("gantt") {
        "gantt".to_string()
    } else if first_line.contains("pie") {
        "pie".to_string()
    } else {
        "unknown".to_string()
    }
}

/// Render flowchart as ASCII art
fn render_flowchart(content: &str) {
    println!("{}", "Flowchart:".bold().green());
    println!();

    // Parse nodes and edges
    let node_re = Regex::new(r"(\w+)\[([^\]]+)\]").unwrap();
    let edge_re = Regex::new(r"(\w+)\s*-->\s*(\w+)").unwrap();

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for line in content.lines() {
        // Extract nodes
        for cap in node_re.captures_iter(line) {
            let id = cap.get(1).unwrap().as_str();
            let label = cap.get(2).unwrap().as_str();
            nodes.push((id.to_string(), label.to_string()));
        }

        // Extract edges
        for cap in edge_re.captures_iter(line) {
            let from = cap.get(1).unwrap().as_str();
            let to = cap.get(2).unwrap().as_str();
            edges.push((from.to_string(), to.to_string()));
        }
    }

    // Render nodes
    for (id, label) in &nodes {
        let box_width = label.len() + 4;
        let top_bottom = format!("┌{}┐", "─".repeat(box_width - 2));

        println!("  {}", top_bottom.bright_blue());
        println!("  {} {} {}", "│".bright_blue(), label.bold(), "│".bright_blue());
        println!("  {}", format!("└{}┘", "─".repeat(box_width - 2)).bright_blue());

        // Show edges from this node
        for (from, to) in &edges {
            if from == id {
                println!("      {}", "│".bright_yellow());
                println!("      {}", "▼".bright_yellow());
                println!("    {} {}", "to".dimmed(), to.cyan());
            }
        }
        println!();
    }

    if nodes.is_empty() {
        // Fallback to source view
        render_source_view(content);
    }
}

/// Render sequence diagram
fn render_sequence_diagram(content: &str) {
    println!("{}", "Sequence Diagram:".bold().green());
    println!();

    let participant_re = Regex::new(r"participant\s+(\w+)").unwrap();
    let message_re = Regex::new(r"(\w+)\s*->>?\s*(\w+)\s*:\s*(.+)").unwrap();

    let mut participants = Vec::new();

    // Extract participants
    for line in content.lines() {
        if let Some(cap) = participant_re.captures(line) {
            let name = cap.get(1).unwrap().as_str();
            participants.push(name.to_string());
        }
    }

    // Show participants
    if !participants.is_empty() {
        for participant in &participants {
            print!("  {}  ", participant.bold().cyan());
        }
        println!();
        for _ in &participants {
            print!("  {}  ", "│".dimmed());
        }
        println!("\n");
    }

    // Show messages
    for line in content.lines() {
        if let Some(cap) = message_re.captures(line) {
            let from = cap.get(1).unwrap().as_str();
            let to = cap.get(2).unwrap().as_str();
            let message = cap.get(3).unwrap().as_str();

            println!("  {} {} {}: {}",
                from.bright_green(),
                "─→".bright_yellow(),
                to.bright_green(),
                message.italic()
            );
        }
    }

    if participants.is_empty() {
        render_source_view(content);
    }

    println!();
}

/// Render class diagram
fn render_class_diagram(content: &str) {
    println!("{}", "Class Diagram:".bold().green());
    println!();

    let class_re = Regex::new(r"class\s+(\w+)").unwrap();
    let method_re = Regex::new(r"\+(\w+)\(([^)]*)\)").unwrap();
    let field_re = Regex::new(r"\+(\w+)\s*:\s*(\w+)").unwrap();

    for line in content.lines() {
        if let Some(cap) = class_re.captures(line) {
            let class_name = cap.get(1).unwrap().as_str();
            println!("  ┌{}┐", "─".repeat(class_name.len() + 2));
            println!("  │ {} │", class_name.bold().bright_blue());
            println!("  ├{}┤", "─".repeat(class_name.len() + 2));
        } else if let Some(cap) = field_re.captures(line) {
            let field_name = cap.get(1).unwrap().as_str();
            let field_type = cap.get(2).unwrap().as_str();
            println!("  │ + {}: {} │", field_name.green(), field_type.dimmed());
        } else if let Some(cap) = method_re.captures(line) {
            let method_name = cap.get(1).unwrap().as_str();
            let params = cap.get(2).unwrap().as_str();
            println!("  │ + {}({}) │", method_name.yellow(), params.dimmed());
        }
    }

    println!();
    render_source_view(content);
}

/// Render state diagram
fn render_state_diagram(content: &str) {
    println!("{}", "State Diagram:".bold().green());
    println!();

    let state_re = Regex::new(r"(\w+)\s*-->\s*(\w+)").unwrap();

    for line in content.lines() {
        if let Some(cap) = state_re.captures(line) {
            let from_state = cap.get(1).unwrap().as_str();
            let to_state = cap.get(2).unwrap().as_str();

            println!("  [ {} ] {} [ {} ]",
                from_state.bold().cyan(),
                "─→".bright_yellow(),
                to_state.bold().cyan()
            );
        }
    }

    println!();
    render_source_view(content);
}

/// Render ER diagram
fn render_er_diagram(content: &str) {
    println!("{}", "Entity-Relationship Diagram:".bold().green());
    println!();

    let entity_re = Regex::new(r"(\w+)\s+\{").unwrap();
    let relationship_re = Regex::new(r"(\w+)\s+\|.*\|\s+(\w+)").unwrap();

    println!("{}", "Entities:".bold());
    for line in content.lines() {
        if let Some(cap) = entity_re.captures(line) {
            let entity = cap.get(1).unwrap().as_str();
            println!("  {} {}", "▪".bright_blue(), entity.bold().cyan());
        }
    }

    println!();
    println!("{}", "Relationships:".bold());
    for line in content.lines() {
        if let Some(cap) = relationship_re.captures(line) {
            let entity1 = cap.get(1).unwrap().as_str();
            let entity2 = cap.get(2).unwrap().as_str();
            println!("  {} {} {}",
                entity1.bright_green(),
                "←→".bright_yellow(),
                entity2.bright_green()
            );
        }
    }

    println!();
    render_source_view(content);
}

/// Render Gantt chart
fn render_gantt(content: &str) {
    println!("{}", "Gantt Chart:".bold().green());
    println!();

    let task_re = Regex::new(r"(\w+)\s*:\s*(.+)").unwrap();

    for line in content.lines() {
        if line.contains("section") {
            let section = line.replace("section", "").trim().to_string();
            println!();
            println!("  {}", section.bold().bright_cyan());
            println!("  {}", "─".repeat(section.len()).bright_cyan());
        } else if let Some(cap) = task_re.captures(line) {
            let task_name = cap.get(1).unwrap().as_str();
            let task_details = cap.get(2).unwrap().as_str();
            println!("    {} {}: {}",
                "▪".bright_green(),
                task_name.bold(),
                task_details.dimmed()
            );
        }
    }

    println!();
}

/// Render pie chart
fn render_pie_chart(content: &str) {
    println!("{}", "Pie Chart:".bold().green());
    println!();

    let data_re = Regex::new(r#""([^"]+)"\s*:\s*(\d+)"#).unwrap();

    let mut total = 0;
    let mut slices = Vec::new();

    for line in content.lines() {
        if let Some(cap) = data_re.captures(line) {
            let label = cap.get(1).unwrap().as_str();
            let value: u32 = cap.get(2).unwrap().as_str().parse().unwrap_or(0);
            total += value;
            slices.push((label.to_string(), value));
        }
    }

    for (label, value) in slices {
        let percentage = if total > 0 {
            value as f64 / total as f64 * 100.0
        } else {
            0.0
        };

        let bar_width = (percentage / 2.0) as usize;
        let bar = "█".repeat(bar_width);

        println!("  {} {}: {} ({:.1}%)",
            "▪".bright_blue(),
            label.bold(),
            bar.bright_green(),
            percentage
        );
    }

    println!();
}

/// Render source code view with syntax highlighting
fn render_source_view(content: &str) {
    println!("{}", "Source Code:".bold().dimmed());
    println!();

    for line in content.lines() {
        if line.trim().is_empty() {
            println!();
            continue;
        }

        if line.contains("graph") || line.contains("flowchart") ||
           line.contains("Diagram") || line.contains("gantt") || line.contains("pie") {
            println!("  {}", line.bold().bright_magenta());
        } else if line.contains("-->") || line.contains("->>") {
            println!("  {}", line.bright_yellow());
        } else if line.contains("participant") || line.contains("section") || line.contains("class") {
            println!("  {}", line.bright_cyan());
        } else {
            println!("  {}", line.dimmed());
        }
    }
}

/// Render Mermaid without colors (plain text)
pub fn render_mermaid_plain(mermaid_content: &str) {
    println!("Mermaid Diagram");
    println!("{}", "─".repeat(80));
    println!();
    println!("{}", mermaid_content);
    println!();
    println!("{}", "─".repeat(80));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flowchart_rendering() {
        let mermaid = r#"
graph TD
    A[Start] --> B[Process]
    B --> C[End]
"#;

        render_mermaid_to_terminal(mermaid);
    }

    #[test]
    fn test_sequence_diagram() {
        let mermaid = r#"
sequenceDiagram
    participant Alice
    participant Bob
    Alice->>Bob: Hello Bob
    Bob->>Alice: Hello Alice
"#;

        render_mermaid_to_terminal(mermaid);
    }

    #[test]
    fn test_diagram_type_detection() {
        assert_eq!(detect_diagram_type("graph TD"), "graph");
        assert_eq!(detect_diagram_type("sequenceDiagram"), "sequenceDiagram");
        assert_eq!(detect_diagram_type("classDiagram"), "classDiagram");
    }
}
