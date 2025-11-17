use crate::core_engine::{Buffer, Range};
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub use_regex: bool,
    #[allow(dead_code)]
    pub whole_word: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            use_regex: false,
            whole_word: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub range: Range,
    pub match_text: String,
}

/// Search for a pattern in a buffer and return all matches
pub fn search_in_buffer(
    buffer: &Buffer,
    pattern: &str,
    options: &SearchOptions,
) -> Result<Vec<SearchResult>, String> {
    if pattern.is_empty() {
        return Ok(Vec::new());
    }

    let content = buffer.content();
    let mut results = Vec::new();

    if options.use_regex {
        // Regex search
        let regex = build_regex(pattern, options)?;

        for mat in regex.find_iter(&content) {
            let start_pos = buffer
                .char_idx_to_position(mat.start())
                .map_err(|e| format!("Failed to convert start position: {}", e))?;
            let end_pos = buffer
                .char_idx_to_position(mat.end())
                .map_err(|e| format!("Failed to convert end position: {}", e))?;

            results.push(SearchResult {
                range: Range::new(start_pos, end_pos),
                match_text: mat.as_str().to_string(),
            });
        }
    } else {
        // Literal search
        let search_pattern = if options.case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        let search_content = if options.case_sensitive {
            content.clone()
        } else {
            content.to_lowercase()
        };

        let mut start = 0;
        while let Some(pos) = search_content[start..].find(&search_pattern) {
            let match_start = start + pos;
            let match_end = match_start + pattern.len();

            let start_pos = buffer
                .char_idx_to_position(match_start)
                .map_err(|e| format!("Failed to convert start position: {}", e))?;
            let end_pos = buffer
                .char_idx_to_position(match_end)
                .map_err(|e| format!("Failed to convert end position: {}", e))?;

            results.push(SearchResult {
                range: Range::new(start_pos, end_pos),
                match_text: content[match_start..match_end].to_string(),
            });

            start = match_end;
        }
    }

    Ok(results)
}

/// Replace text in buffer based on search results
pub fn replace_in_buffer(
    buffer: &mut Buffer,
    pattern: &str,
    replacement: &str,
    options: &SearchOptions,
    replace_all: bool,
) -> Result<usize, String> {
    let results = search_in_buffer(buffer, pattern, options)?;

    if results.is_empty() {
        return Ok(0);
    }

    let count = if replace_all {
        // Replace all occurrences (in reverse order to maintain positions)
        let mut replaced = 0;
        for result in results.iter().rev() {
            buffer.replace_range(result.range, replacement)?;
            replaced += 1;
        }
        replaced
    } else {
        // Replace only the first occurrence
        if let Some(result) = results.first() {
            buffer.replace_range(result.range, replacement)?;
            1
        } else {
            0
        }
    };

    Ok(count)
}

fn build_regex(pattern: &str, options: &SearchOptions) -> Result<Regex, String> {
    RegexBuilder::new(pattern)
        .case_insensitive(!options.case_sensitive)
        .build()
        .map_err(|e| format!("Invalid regex pattern: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_engine::BufferId;

    #[test]
    fn test_search_literal_case_sensitive() {
        let buffer = Buffer::from_string(
            BufferId(0),
            "Hello World\nHello Rust\nhello world\n".to_string(),
            None,
        );

        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
            whole_word: false,
        };

        let results = search_in_buffer(&buffer, "Hello", &options).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].match_text, "Hello");
        assert_eq!(results[1].match_text, "Hello");
    }

    #[test]
    fn test_search_literal_case_insensitive() {
        let buffer = Buffer::from_string(
            BufferId(0),
            "Hello World\nHello Rust\nhello world\n".to_string(),
            None,
        );

        let options = SearchOptions {
            case_sensitive: false,
            use_regex: false,
            whole_word: false,
        };

        let results = search_in_buffer(&buffer, "hello", &options).unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_regex() {
        let buffer = Buffer::from_string(BufferId(0), "t1t t2t t3t test".to_string(), None);

        let options = SearchOptions {
            case_sensitive: true,
            use_regex: true,
            whole_word: false,
        };

        let results = search_in_buffer(&buffer, r"t\dt", &options).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].match_text, "t1t");
        assert_eq!(results[1].match_text, "t2t");
        assert_eq!(results[2].match_text, "t3t");
    }

    #[test]
    fn test_search_regex_invalid() {
        let buffer = Buffer::from_string(BufferId(0), "test".to_string(), None);

        let options = SearchOptions {
            case_sensitive: true,
            use_regex: true,
            whole_word: false,
        };

        let result = search_in_buffer(&buffer, "[invalid", &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_single() {
        let mut buffer =
            Buffer::from_string(BufferId(0), "Hello World Hello Rust".to_string(), None);

        let options = SearchOptions::default();
        let count = replace_in_buffer(&mut buffer, "Hello", "Hi", &options, false).unwrap();

        assert_eq!(count, 1);
        assert_eq!(buffer.content(), "Hi World Hello Rust");
    }

    #[test]
    fn test_replace_all() {
        let mut buffer =
            Buffer::from_string(BufferId(0), "Hello World Hello Rust".to_string(), None);

        let options = SearchOptions::default();
        let count = replace_in_buffer(&mut buffer, "Hello", "Hi", &options, true).unwrap();

        assert_eq!(count, 2);
        assert_eq!(buffer.content(), "Hi World Hi Rust");
    }

    #[test]
    fn test_replace_regex() {
        let mut buffer = Buffer::from_string(BufferId(0), "t1t t2t t3t".to_string(), None);

        let options = SearchOptions {
            case_sensitive: true,
            use_regex: true,
            whole_word: false,
        };

        let count = replace_in_buffer(&mut buffer, r"t\dt", "NUM", &options, true).unwrap();

        assert_eq!(count, 3);
        assert_eq!(buffer.content(), "NUM NUM NUM");
    }
}
