pub mod buffer_search;
pub mod grep;

pub use buffer_search::{replace_in_buffer, search_in_buffer, SearchOptions, SearchResult};
pub use grep::{GrepEngine, GrepOptions, GrepResult, GrepStatus};
