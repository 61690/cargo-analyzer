use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub enum SafetySubcategory {
    TypeCasting,
    UnsafeCode,
    ResourceLeak,
    ConcurrencyIssue,
    Other,
}

impl SafetySubcategory {
    pub fn description(&self) -> &'static str {
        match self {
            SafetySubcategory::TypeCasting => "Unsafe type casting operations",
            SafetySubcategory::UnsafeCode => "Usage of unsafe blocks",
            SafetySubcategory::ResourceLeak => "Potential resource leaks",
            SafetySubcategory::ConcurrencyIssue => "Thread safety concerns",
            SafetySubcategory::Other => "Other safety issues",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub enum PerfSubcategory {
    Allocation,
    Locking,
    Cloning,
    Iteration,
    Other,
}

impl PerfSubcategory {
    pub fn description(&self) -> &'static str {
        match self {
            PerfSubcategory::Allocation => "Memory allocation patterns",
            PerfSubcategory::Locking => "Lock contention issues",
            PerfSubcategory::Cloning => "Unnecessary cloning",
            PerfSubcategory::Iteration => "Inefficient iteration",
            PerfSubcategory::Other => "Other performance issues",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub enum StyleSubcategory {
    NamingConvention,
    CodeStructure,
    UnusedCode,
    Formatting,
    Other,
}

impl StyleSubcategory {
    pub fn description(&self) -> &'static str {
        match self {
            StyleSubcategory::NamingConvention => "Naming convention violations",
            StyleSubcategory::CodeStructure => "Code structure improvements",
            StyleSubcategory::UnusedCode => "Unused code elements",
            StyleSubcategory::Formatting => "Code formatting issues",
            StyleSubcategory::Other => "Other style issues",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub enum DocSubcategory {
    MissingDocs,
    ErrorDocs,
    ExampleDocs,
    DeprecationNotice,
    Other,
}

impl DocSubcategory {
    pub fn description(&self) -> &'static str {
        match self {
            DocSubcategory::MissingDocs => "Missing documentation",
            DocSubcategory::ErrorDocs => "Error documentation",
            DocSubcategory::ExampleDocs => "Example documentation",
            DocSubcategory::DeprecationNotice => "Deprecation notices",
            DocSubcategory::Other => "Other documentation issues",
        }
    }
}

// Implement Display for all subcategories
impl fmt::Display for SafetySubcategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl fmt::Display for PerfSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl fmt::Display for StyleSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl fmt::Display for DocSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
} 