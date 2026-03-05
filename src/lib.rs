//! LeetCode CLI Library
//!
//! This library provides the core functionality for the LeetCode CLI tool.

pub mod api;
pub mod commands;
pub mod config;
pub mod problem;
pub mod solutions;
pub mod template;

// Re-export commonly used types
pub use api::LeetCodeClient;
pub use config::Config;
pub use problem::{Problem, ProblemDetail, ProblemList};
