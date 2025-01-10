use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CategoryType {
    Safety,
    Performance,
    Style,
    Documentation,
}

impl CategoryType {
    pub fn description(&self) -> &'static str {
        match self {
            CategoryType::Safety => "Safety and correctness issues",
            CategoryType::Performance => "Performance optimizations",
            CategoryType::Style => "Code style and maintainability",
            CategoryType::Documentation => "Documentation completeness",
        }
    }

    pub fn priority_level(&self) -> u8 {
        match self {
            CategoryType::Safety => 4,
            CategoryType::Performance => 3,
            CategoryType::Style => 2,
            CategoryType::Documentation => 1,
        }
    }
}

impl fmt::Display for CategoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CategoryType::Safety => write!(f, "Safety"),
            CategoryType::Performance => write!(f, "Performance"),
            CategoryType::Style => write!(f, "Style"),
            CategoryType::Documentation => write!(f, "Documentation"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct WarningCategory {
    pub category_type: CategoryType,
    pub subcategory: String,
}

impl WarningCategory {
    pub fn new(category_type: CategoryType, subcategory: String) -> Self {
        Self {
            category_type,
            subcategory,
        }
    }

    pub fn full_description(&self) -> String {
        format!("{}: {}", self.category_type, self.subcategory)
    }
} 