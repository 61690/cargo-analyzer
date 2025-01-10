//! # Cargo Analyzer
//! 
//! `cargo-analyzer` is a sophisticated static analysis tool that processes and analyzes
//! Clippy warnings to provide enhanced insights, statistics, and fix suggestions for Rust projects.
//! 
//! ## Core Features
//! 
//! - **Warning Analysis**: Parses and categorizes Clippy warnings
//! - **Statistical Analysis**: Generates comprehensive statistics about code quality
//! - **Trend Analysis**: Tracks improvements and regressions over time
//! - **Fix Suggestions**: Provides actionable fix suggestions with examples
//! - **Report Generation**: Creates detailed reports in multiple formats
//! 
//! ## Module Structure
//! 
//! - `analysis`: Statistical analysis and trend tracking
//! - `fixes`: Fix suggestions and example generation
//! - `output`: Report generation and formatting
//! - `parser`: Warning parsing and categorization
//! - `runner`: Analysis execution and workflow management
//! - `types`: Core type definitions and enums
//! 
//! ## Usage Example
//! 
//! ```rust
//! use cargo_analyzer::runner::workflow::ClippyWorkflow;
//! 
//! let workflow = ClippyWorkflow::new();
//! workflow.run().expect("Failed to run analysis");
//! ```

pub mod analysis;
pub mod output;
pub mod runner;
pub mod parser;
pub mod types;
pub mod fixes;

// Re-export commonly used items
pub use types::*;
pub use analysis::*;
pub use output::*;
pub use fixes::*;
pub use parser::*;
pub use runner::*;
