pub mod buffer_search;
pub mod grep;

pub use buffer_search::{SearchOptions, SearchResult, search_in_buffer, replace_in_buffer};
pub use grep::{GrepEngine, GrepOptions, GrepResult, GrepStatus};
