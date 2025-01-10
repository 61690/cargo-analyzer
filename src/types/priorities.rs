use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Priority {
    Critical,  // Safety issues, potential bugs
    High,      // Performance issues
    Medium,    // Code structure and maintainability
    Low,       // Style and documentation
    Trivial,   // Optional fixes
}

impl Priority {
    pub fn severity_score(&self) -> u8 {
        match self {
            Priority::Critical => 5,
            Priority::High => 4,
            Priority::Medium => 3,
            Priority::Low => 2,
            Priority::Trivial => 1,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Priority::Critical => "Critical: Must be fixed immediately",
            Priority::High => "High: Should be fixed soon",
            Priority::Medium => "Medium: Should be fixed when possible",
            Priority::Low => "Low: Consider fixing when convenient",
            Priority::Trivial => "Trivial: Optional fixes",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Critical => write!(f, "Critical"),
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Low => write!(f, "Low"),
            Priority::Trivial => write!(f, "Trivial"),
        }
    }
} 