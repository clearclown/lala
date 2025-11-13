/*!
# Async Grep Architecture

This module implements asynchronous grep functionality with .gitignore support.

## Architecture Overview

```
┌─────────────────┐
│   GUI Thread    │
│                 │
│  GrepEngine     │
└────────┬────────┘
         │ start_search()
         │
         ▼
┌─────────────────────────────────┐
│    Tokio Async Runtime          │
│                                 │
│  ┌──────────────────────────┐  │
│  │  Walk Directory Tree     │  │
│  │  (ignore crate)          │  │
│  │  - Respects .gitignore   │  │
│  │  - Respects .ignore      │  │
│  └──────────┬───────────────┘  │
│             │                   │
│             ▼                   │
│  ┌──────────────────────────┐  │
│  │  Search Each File        │  │
│  │  - Read file content     │  │
│  │  - Apply regex           │  │
│  │  - Find all matches      │  │
│  └──────────┬───────────────┘  │
│             │                   │
│             │ Send results      │
│             ▼                   │
│  ┌──────────────────────────┐  │
│  │  Flume Channel (mpsc)    │  │
│  └──────────┬───────────────┘  │
└─────────────┼───────────────────┘
              │
              │ Receive results
              ▼
     ┌────────────────────┐
     │   GUI Thread       │
     │                    │
     │  poll_results()    │
     │  - Update UI       │
     │  - Display matches │
     └────────────────────┘
```

## Usage

```rust
use lala::search::grep::{GrepEngine, GrepOptions};

let mut engine = GrepEngine::new();
let options = GrepOptions {
    pattern: "TODO".to_string(),
    case_sensitive: false,
    use_regex: false,
    root_path: "/path/to/project".into(),
};

// Start async search
engine.start_search(options);

// Poll for results in GUI loop
while let Some(result) = engine.poll_result() {
    println!("Found match in {}:{}:{}",
        result.file_path.display(),
        result.line_number,
        result.line_content);
}
```

## Thread Safety

- The flume channel is thread-safe and lock-free
- Search runs on tokio thread pool
- GUI polls results on its main thread
- No blocking operations on GUI thread
*/

use flume::{Receiver, Sender};
use ignore::WalkBuilder;
use regex::{Regex, RegexBuilder};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GrepOptions {
    pub pattern: String,
    pub case_sensitive: bool,
    pub use_regex: bool,
    pub root_path: PathBuf,
    pub file_filter: Option<String>, // e.g., "*.rs" for future extension
}

#[derive(Debug, Clone)]
pub struct GrepResult {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrepStatus {
    Idle,
    Searching,
    Completed,
}

/// Grep engine for async multi-file search with .gitignore support
pub struct GrepEngine {
    status: GrepStatus,
    result_rx: Option<Receiver<GrepResult>>,
    result_count: usize,
}

impl GrepEngine {
    pub fn new() -> Self {
        Self {
            status: GrepStatus::Idle,
            result_rx: None,
            result_count: 0,
        }
    }

    pub fn status(&self) -> GrepStatus {
        self.status
    }

    pub fn result_count(&self) -> usize {
        self.result_count
    }

    /// Start an async grep search
    pub fn start_search(&mut self, options: GrepOptions) {
        let (tx, rx) = flume::unbounded();
        self.result_rx = Some(rx);
        self.status = GrepStatus::Searching;
        self.result_count = 0;

        // Spawn async search task
        tokio::spawn(async move {
            perform_grep(options, tx).await;
        });
    }

    /// Poll for new results (call this from GUI loop)
    pub fn poll_result(&mut self) -> Option<GrepResult> {
        if let Some(rx) = &self.result_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.result_count += 1;
                    Some(result)
                }
                Err(flume::TryRecvError::Empty) => None,
                Err(flume::TryRecvError::Disconnected) => {
                    self.status = GrepStatus::Completed;
                    self.result_rx = None;
                    None
                }
            }
        } else {
            None
        }
    }

    /// Clear results and reset state
    pub fn clear(&mut self) {
        self.result_rx = None;
        self.status = GrepStatus::Idle;
        self.result_count = 0;
    }

    /// Check if search is active
    pub fn is_searching(&self) -> bool {
        self.status == GrepStatus::Searching
    }
}

impl Default for GrepEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Perform the actual grep search (runs on tokio thread pool)
async fn perform_grep(options: GrepOptions, tx: Sender<GrepResult>) {
    let pattern_result = if options.use_regex {
        RegexBuilder::new(&options.pattern)
            .case_insensitive(!options.case_sensitive)
            .build()
    } else {
        // Escape regex special characters for literal search
        let escaped = regex::escape(&options.pattern);
        RegexBuilder::new(&escaped)
            .case_insensitive(!options.case_sensitive)
            .build()
    };

    let regex = match pattern_result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to build regex: {}", e);
            return;
        }
    };

    // Use ignore crate to walk directory with .gitignore support
    let walker = WalkBuilder::new(&options.root_path)
        .hidden(false) // Show hidden files
        .git_ignore(true) // Respect .gitignore
        .git_global(true) // Respect global .gitignore
        .git_exclude(true) // Respect .git/info/exclude
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Skip directories
        if entry.file_type().is_some_and(|ft| ft.is_dir()) {
            continue;
        }

        let path = entry.path();

        // Apply file filter if specified
        if let Some(filter) = &options.file_filter {
            if !matches_filter(path, filter) {
                continue;
            }
        }

        // Search in file
        if let Err(e) = search_file(path, &regex, &tx).await {
            eprintln!("Error searching {}: {}", path.display(), e);
        }
    }

    // Channel will be dropped here, signaling completion
}

/// Search a single file for matches
async fn search_file(
    path: &Path,
    regex: &Regex,
    tx: &Sender<GrepResult>,
) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(path)?;

    for (line_idx, line) in content.lines().enumerate() {
        for mat in regex.find_iter(line) {
            let result = GrepResult {
                file_path: path.to_path_buf(),
                line_number: line_idx + 1, // 1-indexed for display
                column: mat.start() + 1,   // 1-indexed for display
                line_content: line.to_string(),
                match_start: mat.start(),
                match_end: mat.end(),
            };

            // If sending fails, search was cancelled
            if tx.send(result).is_err() {
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Check if file matches the filter pattern (e.g., "*.rs")
fn matches_filter(path: &Path, filter: &str) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            // Simple extension matching
            if let Some(filter_ext) = filter.strip_prefix("*.") {
                return ext_str == filter_ext;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_grep_basic() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("test1.txt");
        let file2 = temp_dir.path().join("test2.txt");

        fs::write(&file1, "Hello World\nHello Rust\n").unwrap();
        fs::write(&file2, "Goodbye World\n").unwrap();

        let mut engine = GrepEngine::new();
        let options = GrepOptions {
            pattern: "Hello".to_string(),
            case_sensitive: true,
            use_regex: false,
            root_path: temp_dir.path().to_path_buf(),
            file_filter: None,
        };

        engine.start_search(options);

        // Give some time for async search
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut results = Vec::new();
        while let Some(result) = engine.poll_result() {
            results.push(result);
        }

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].match_start, 0);
        assert_eq!(results[0].match_end, 5);
    }

    #[tokio::test]
    async fn test_grep_regex() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("test.txt");

        fs::write(&file, "t1t t2t t3t\n").unwrap();

        let mut engine = GrepEngine::new();
        let options = GrepOptions {
            pattern: r"t\dt".to_string(),
            case_sensitive: true,
            use_regex: true,
            root_path: temp_dir.path().to_path_buf(),
            file_filter: None,
        };

        engine.start_search(options);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut results = Vec::new();
        while let Some(result) = engine.poll_result() {
            results.push(result);
        }

        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_grep_case_insensitive() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("test.txt");

        fs::write(&file, "Hello\nhello\nHELLO\n").unwrap();

        let mut engine = GrepEngine::new();
        let options = GrepOptions {
            pattern: "hello".to_string(),
            case_sensitive: false,
            use_regex: false,
            root_path: temp_dir.path().to_path_buf(),
            file_filter: None,
        };

        engine.start_search(options);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut results = Vec::new();
        while let Some(result) = engine.poll_result() {
            results.push(result);
        }

        assert_eq!(results.len(), 3);
    }
}
