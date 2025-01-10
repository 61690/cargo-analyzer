use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use super::categories::{WarningCategory, CategoryType};
use super::priorities::Priority;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warning {
    pub category: WarningCategory,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub priority: Priority,
    pub suggested_fix: Option<String>,
}

impl Warning {
    pub fn analyze(&self) -> (u8, String) {
        let severity_score = match self.priority {
            Priority::Critical => 5,
            Priority::High => 4,
            Priority::Medium => 3,
            Priority::Low => 2,
            Priority::Trivial => 1,
        };

        let impact_description = match self.category.category_type {
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