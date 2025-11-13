//! # Lala - Modern Text Editor
//!
//! A lightweight and fast text editor written in Rust with egui.

pub mod cli;
pub mod core;
pub mod core_engine;
pub mod file_tree;
pub mod gui;
pub mod search;

pub use gui::LalaApp;
