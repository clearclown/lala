/// File Tree Module
///
/// This module provides async directory loading and GUI rendering for a file tree view.
/// It uses the `ignore` crate to respect .gitignore rules and filter out unwanted files.
///
/// ## Architecture
/// - `FileTreeLoader`: Handles async directory scanning in a background thread
/// - `FileTreeNode`: Represents a node in the tree (file or directory)
/// - `FileTree`: Main component that manages the tree state and GUI rendering
///
/// ## Async Loading
/// Directory scanning is done asynchronously using tokio::spawn to prevent UI freezes.
/// Results are sent to the GUI thread via a flume channel.
use flume::{Receiver, Sender};
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents a node in the file tree
#[derive(Debug, Clone)]
pub struct FileTreeNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileTreeNode>,
    pub error: Option<String>,
}

impl FileTreeNode {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Self {
            path,
            name,
            is_dir,
            children: Vec::new(),
            error: None,
        }
    }

    pub fn with_error(path: PathBuf, error: String) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Self {
            path,
            name,
            is_dir: true,
            children: Vec::new(),
            error: Some(error),
        }
    }
}

/// Messages sent from the loader thread to the GUI thread
#[derive(Debug, Clone)]
pub enum LoadMessage {
    /// Directory contents loaded successfully
    DirectoryLoaded {
        path: PathBuf,
        nodes: Vec<FileTreeNode>,
    },
    /// Error loading directory
    Error { path: PathBuf, error: String },
    /// Loading completed
    Complete,
}

/// Handles async directory loading
pub struct FileTreeLoader {
    tx: Sender<LoadMessage>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl FileTreeLoader {
    pub fn new(tx: Sender<LoadMessage>) -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        Self {
            tx,
            runtime: Arc::new(runtime),
        }
    }

    /// Load a directory asynchronously
    pub fn load_directory(&self, path: PathBuf, max_depth: Option<usize>) {
        let tx = self.tx.clone();
        let runtime = self.runtime.clone();

        std::thread::spawn(move || {
            runtime.block_on(async move {
                Self::load_directory_impl(path, max_depth, tx).await;
            });
        });
    }

    async fn load_directory_impl(
        root_path: PathBuf,
        max_depth: Option<usize>,
        tx: Sender<LoadMessage>,
    ) {
        let mut builder = WalkBuilder::new(&root_path);
        builder
            .max_depth(max_depth)
            .follow_links(false) // Don't follow symlinks for security
            .git_ignore(true)
            .git_exclude(true)
            .hidden(false); // Show hidden files except .git

        let walker = builder.build();

        // Group entries by parent directory
        let mut dir_map: HashMap<PathBuf, Vec<FileTreeNode>> = HashMap::new();

        for entry_result in walker {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path().to_path_buf();

                    // Skip the root itself
                    if path == root_path {
                        continue;
                    }

                    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

                    // Skip .git directory
                    if is_dir && path.file_name() == Some(std::ffi::OsStr::new(".git")) {
                        continue;
                    }

                    let parent = path.parent().unwrap_or(&root_path).to_path_buf();
                    let node = FileTreeNode::new(path, is_dir);

                    dir_map.entry(parent).or_default().push(node);
                }
                Err(e) => {
                    let error = format!("{}", e);

                    // Send error message with root path as we can't get specific path from error
                    let _ = tx.send(LoadMessage::Error {
                        path: root_path.clone(),
                        error,
                    });
                }
            }
        }

        // Send loaded directories
        for (parent, mut nodes) in dir_map {
            // Sort: directories first, then files, both alphabetically
            nodes.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            });

            let _ = tx.send(LoadMessage::DirectoryLoaded {
                path: parent,
                nodes,
            });
        }

        let _ = tx.send(LoadMessage::Complete);
    }
}

/// Main file tree component
pub struct FileTree {
    root_path: PathBuf,
    root_node: Option<FileTreeNode>,
    loader: FileTreeLoader,
    rx: Receiver<LoadMessage>,
    expanded_dirs: HashMap<PathBuf, bool>,
    loading: bool,
    dir_contents: HashMap<PathBuf, Vec<FileTreeNode>>,
}

impl FileTree {
    pub fn new(root_path: PathBuf) -> Self {
        let (tx, rx) = flume::unbounded();
        let loader = FileTreeLoader::new(tx);

        Self {
            root_path,
            root_node: None,
            loader,
            rx,
            expanded_dirs: HashMap::new(),
            loading: false,
            dir_contents: HashMap::new(),
        }
    }

    /// Start loading the root directory
    pub fn load(&mut self) {
        if !self.loading {
            self.loading = true;
            self.loader.load_directory(self.root_path.clone(), Some(3)); // Limit depth initially
        }
    }

    /// Process messages from the loader
    pub fn update(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                LoadMessage::DirectoryLoaded { path, nodes } => {
                    self.dir_contents.insert(path.clone(), nodes);

                    // If this is the root, create the root node
                    if path == self.root_path && self.root_node.is_none() {
                        self.root_node = Some(FileTreeNode::new(self.root_path.clone(), true));
                    }
                }
                LoadMessage::Error { path, error } => {
                    // Store error in dir_contents
                    let error_node = FileTreeNode::with_error(path.clone(), error);
                    self.dir_contents.insert(path, vec![error_node]);
                }
                LoadMessage::Complete => {
                    self.loading = false;
                }
            }
        }
    }

    /// Render the file tree GUI
    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<PathBuf> {
        let mut clicked_file: Option<PathBuf> = None;

        self.update();

        if self.root_node.is_none() && !self.loading {
            self.load();
        }

        ui.heading("Files");
        ui.separator();

        if self.loading && self.root_node.is_none() {
            ui.spinner();
            ui.label("Loading...");
        } else if let Some(root) = &self.root_node {
            let root_name = root.name.clone();
            let root_path = root.path.clone();

            // Render root
            clicked_file = self.render_node(ui, &root_path, &root_name, true);
        }

        clicked_file
    }

    fn render_node(
        &mut self,
        ui: &mut egui::Ui,
        path: &Path,
        name: &str,
        is_dir: bool,
    ) -> Option<PathBuf> {
        let mut clicked_file: Option<PathBuf> = None;

        if is_dir {
            let is_expanded = self.expanded_dirs.get(path).copied().unwrap_or(false);

            let header_response = egui::CollapsingHeader::new(format!("📁 {}", name))
                .id_source(path.to_string_lossy().to_string())
                .default_open(false)
                .open(Some(is_expanded))
                .show(ui, |ui| {
                    // Get children for this directory (clone to avoid borrow checker issues)
                    if let Some(children) = self.dir_contents.get(path).cloned() {
                        for child in &children {
                            if let Some(error) = &child.error {
                                ui.label(format!("❌ {}: {}", child.name, error));
                            } else if let Some(file) =
                                self.render_node(ui, &child.path, &child.name, child.is_dir)
                            {
                                clicked_file = Some(file);
                            }
                        }
                    }
                });

            // Update expanded state
            self.expanded_dirs
                .insert(path.to_path_buf(), header_response.openness == 1.0);
        } else {
            // Render file
            if ui.button(format!("📄 {}", name)).clicked() {
                clicked_file = Some(path.to_path_buf());
            }
        }

        clicked_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_file_tree_node_creation() {
        let path = PathBuf::from("/test/file.txt");
        let node = FileTreeNode::new(path.clone(), false);

        assert_eq!(node.name, "file.txt");
        assert_eq!(node.path, path);
        assert!(!node.is_dir);
        assert!(node.error.is_none());
    }

    #[test]
    fn test_file_tree_node_with_error() {
        let path = PathBuf::from("/test/dir");
        let error = "Access Denied".to_string();
        let node = FileTreeNode::with_error(path.clone(), error.clone());

        assert_eq!(node.name, "dir");
        assert!(node.is_dir);
        assert_eq!(node.error, Some(error));
    }

    #[test]
    fn test_file_tree_loader_directory_structure() {
        // Create a temporary directory structure
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        fs::create_dir(root.join("dir1")).unwrap();
        fs::write(root.join("dir1").join("file1.txt"), "content").unwrap();
        fs::write(root.join("file2.txt"), "content").unwrap();
        fs::write(root.join("file3.txt"), "content3").unwrap();

        let (tx, rx) = flume::unbounded();
        let loader = FileTreeLoader::new(tx);

        loader.load_directory(root.to_path_buf(), Some(5));

        // Collect messages
        let mut messages = Vec::new();
        for _ in 0..100 {
            // Wait a bit for async loading
            if let Ok(msg) = rx.recv_timeout(std::time::Duration::from_millis(100)) {
                messages.push(msg);
                if matches!(messages.last(), Some(LoadMessage::Complete)) {
                    break;
                }
            }
        }

        // Verify we got some directory loaded messages
        let dir_loaded_count = messages
            .iter()
            .filter(|m| matches!(m, LoadMessage::DirectoryLoaded { .. }))
            .count();

        assert!(dir_loaded_count > 0, "Should have loaded at least one directory");

        // Verify files and directories are present
        let mut all_nodes = Vec::new();
        for msg in &messages {
            if let LoadMessage::DirectoryLoaded { nodes, .. } = msg {
                all_nodes.extend(nodes.clone());
            }
        }

        // Check that we have the expected files
        assert!(
            all_nodes.iter().any(|n| n.name == "dir1"),
            "dir1 should be present"
        );
        assert!(
            all_nodes.iter().any(|n| n.name == "file1.txt"),
            "file1.txt should be present"
        );
        assert!(
            all_nodes.iter().any(|n| n.name == "file2.txt"),
            "file2.txt should be present"
        );
    }

    #[test]
    fn test_file_tree_filtering() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create .git directory (should be filtered)
        fs::create_dir(root.join(".git")).unwrap();
        fs::write(root.join(".git").join("config"), "git config").unwrap();

        fs::write(root.join("visible.txt"), "visible").unwrap();

        let (tx, rx) = flume::unbounded();
        let loader = FileTreeLoader::new(tx);

        loader.load_directory(root.to_path_buf(), Some(5));

        // Collect messages
        let mut all_nodes = Vec::new();
        for _ in 0..100 {
            if let Ok(msg) = rx.recv_timeout(std::time::Duration::from_millis(100)) {
                if let LoadMessage::DirectoryLoaded { nodes, .. } = msg {
                    all_nodes.extend(nodes);
                } else if matches!(msg, LoadMessage::Complete) {
                    break;
                }
            }
        }

        // Verify .git is filtered out
        for node in &all_nodes {
            assert_ne!(node.name, ".git", ".git directory should be filtered");
        }

        // Verify visible.txt is present
        assert!(
            all_nodes.iter().any(|n| n.name == "visible.txt"),
            "visible.txt should be present"
        );
    }
}
