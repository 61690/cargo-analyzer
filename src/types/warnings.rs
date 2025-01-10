//! Core warning types and related functionality.
//! 
//! This module defines the fundamental types used to represent and process
//! Clippy warnings throughout the analysis process.

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use super::categories::CategoryType;
use super::priorities::Priority;

/// Represents the analysis result of a warning: (severity score, impact description)
pub type WarningAnalysis = (u8, String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warning {
    /// Unique identifier for the warning
    pub id: String,
    /// The specific warning message
    pub message: String,
    /// Category of the warning (style, safety, etc.)
    pub category: CategoryType,
    /// Priority level of the warning
    pub priority: Priority,
    /// File path where the warning was found
    pub file: String,
    /// Line number where the warning was found
    pub line: u32,
    /// Suggested fix for the warning
    pub suggested_fix: Option<String>,
}

impl Warning {
    /// Analyzes the warning to extract additional insights.
    /// 
    /// This method processes the warning's contents to determine:
    /// - Impact on code quality
    /// - Suggested fixes
    /// - Related patterns
    pub fn analyze(&self) -> WarningAnalysis {
        let severity_score = match self.priority {
            Priority::Critical => 5,
            Priority::High => 4,
            Priority::Medium => 3,
            Priority::Low => 2,
            Priority::Trivial => 1,
        };

        let impact_description = match self.category {
            CategoryType::Safety => format!("Safety issue in {} (line {})", self.file, self.line),
            CategoryType::Performance => format!("Performance bottleneck in {} (line {})", self.file, self.line),
            CategoryType::Style => format!("Style improvement needed in {} (line {})", self.file, self.line),
            CategoryType::Documentation => format!("Documentation needed in {} (line {})", self.file, self.line),
        };

        (severity_score, impact_description)
    }
}

#[derive(Debug)]
pub struct FileWarnings {
    pub file_path: PathBuf,
    pub warnings: Vec<Warning>,
}

impl FileWarnings {
    pub fn new(path: String) -> Self {
        FileWarnings {
            file_path: PathBuf::from(path),
            warnings: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    pub fn sort_by_line(&mut self) {
        self.warnings.sort_by_key(|w| w.line);
    }

    pub fn analyze_file(&self) -> Vec<(u8, String)> {
        self.warnings.iter()
            .map(|w| w.analyze())
            .collect()
    }
} 