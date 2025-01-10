use crate::types::{Warning, CategoryType};

#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub code: String,
    pub explanation: String,
    pub confidence: f32,
}

pub fn generate_fix_suggestion(warning: &Warning) -> Option<FixSuggestion> {
    let subcategory = warning.message.split_whitespace().next().unwrap_or("");
    
    // First try to get a specific fix based on the message
    if let Some(fix) = get_specific_fix(subcategory) {
        return Some(fix);
    }

    // Fall back to category-based suggestions
    match warning.category {
        CategoryType::Style => generate_style_suggestion(warning),
        CategoryType::Safety => generate_safety_suggestion(warning),
        CategoryType::Performance => generate_performance_suggestion(warning),
        CategoryType::Documentation => generate_documentation_suggestion(warning),
    }
}

fn get_specific_fix(clippy_code: &str) -> Option<FixSuggestion> {
    match clippy_code {
        "use_self" => Some(FixSuggestion {
            code: "Replace type name with `Self`".to_string(),
            explanation: "Using `Self` instead of the type name makes the code more maintainable".to_string(),
            confidence: 0.95,
        }),
        "missing_errors_doc" => Some(FixSuggestion {
            code: r#"/// # Errors
/// This function will return an error if:
/// - The input is invalid
/// - The operation fails"#.to_string(),
            explanation: "Document possible error conditions for Result-returning functions".to_string(),
            confidence: 0.9,
        }),
        // Add more specific fixes...
        _ => None,
    }
}

fn generate_performance_suggestion(warning: &Warning) -> Option<FixSuggestion> {
    let subcategory = warning.message.split_whitespace().next().unwrap_or("");
    match subcategory {
        "Allocation" => Some(FixSuggestion {
            code: "// Consider using a pre-allocated buffer\nlet mut buffer = Vec::with_capacity(size);".to_string(),
            explanation: "Pre-allocating memory can reduce reallocations".to_string(),
            confidence: 0.8,
        }),
        "Locking" => Some(FixSuggestion {
            code: "// Consider using a more granular lock\nlet data = { let guard = lock.read(); guard.clone() };".to_string(),
            explanation: "Reducing lock scope can improve concurrency".to_string(),
            confidence: 0.7,
        }),
        _ => None,
    }
}

fn generate_safety_suggestion(warning: &Warning) -> Option<FixSuggestion> {
    let subcategory = warning.message.split_whitespace().next().unwrap_or("");
    match subcategory {
        "UnsafeCode" => Some(FixSuggestion {
            code: "// Consider using safe alternatives\nslice.get(index).copied()".to_string(),
            explanation: "Using safe alternatives reduces the risk of undefined behavior".to_string(),
            confidence: 0.9,
        }),
        "ResourceLeak" => Some(FixSuggestion {
            code: "// Use RAII patterns\nlet _guard = resource.lock();".to_string(),
            explanation: "RAII ensures resources are properly managed".to_string(),
            confidence: 0.85,
        }),
        _ => None,
    }
}

fn generate_style_suggestion(warning: &Warning) -> Option<FixSuggestion> {
    let subcategory = warning.message.split_whitespace().next().unwrap_or("");
    match subcategory {
        "NamingConvention" => Some(FixSuggestion {
            code: "// Follow Rust naming conventions\npub struct MyType {}".to_string(),
            explanation: "Using standard naming conventions improves code readability".to_string(),
            confidence: 0.95,
        }),
        "UnusedCode" => Some(FixSuggestion {
            code: "// Remove or use the unused item\n#[allow(dead_code)]".to_string(),
            explanation: "Removing unused code improves maintainability".to_string(),
            confidence: 0.9,
        }),
        _ => None,
    }
}

fn generate_documentation_suggestion(warning: &Warning) -> Option<FixSuggestion> {
    let subcategory = warning.message.split_whitespace().next().unwrap_or("");
    match subcategory {
        "MissingDocs" => Some(FixSuggestion {
            code: "/// Brief description of the item\n/// \n/// # Examples\n/// ```\n/// // Add example here\n/// ```".to_string(),
            explanation: "Adding documentation helps users understand the code".to_string(),
            confidence: 0.95,
        }),
        "ErrorDocs" => Some(FixSuggestion {
            code: "/// # Errors\n/// Returns an error if:".to_string(),
            explanation: "Documenting error conditions helps users handle errors".to_string(),
            confidence: 0.9,
        }),
        _ => None,
    }
} 