use std::path::{Path, PathBuf};

/// File tree for navigating the file system
#[derive(Debug, Clone)]
pub struct FileTree {
    root: PathBuf,
}

impl FileTree {
    #[allow(dead_code)]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    #[allow(dead_code)]
    pub fn set_root(&mut self, root: PathBuf) {
        self.root = root;
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
}
