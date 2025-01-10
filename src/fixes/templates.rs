use std::fs::File;
use std::io::Write;
use crate::types::{Warning, CategoryType};
use super::examples::get_fix_example;

pub fn write_fix_template(file: &mut File, warning: &Warning) -> std::io::Result<()> {
    if let Some(example) = get_fix_example(warning) {
        writeln!(file, "// {}", example.description)?;
        writeln!(file, "\n// Before:\n{}", example.before)?;
        writeln!(file, "\n// After:\n{}", example.after)?;
        writeln!(file, "\n// Explanation: {}", example.explanation)?;
        writeln!(file, "\n// Additional notes:")?;
        for note in example.additional_notes {
            writeln!(file, "// - {}", note)?;
        }
    }
    Ok(())
}

pub fn write_category_templates(file: &mut File, category: CategoryType) -> std::io::Result<()> {
    let header = match category {
        CategoryType::Safety => "Safety Fixes",
        CategoryType::Performance => "Performance Improvements",
        CategoryType::Style => "Style Guidelines",
        CategoryType::Documentation => "Documentation Guidelines",
    };

    writeln!(file, "\n=== {} ===\n", header)?;
    
    // Write category-specific templates
    match category {
        CategoryType::Safety => write_safety_templates(file)?,
        CategoryType::Performance => write_performance_templates(file)?,
        CategoryType::Style => write_style_templates(file)?,
        CategoryType::Documentation => write_documentation_templates(file)?,
    }

    Ok(())
}

fn write_safety_templates(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "// Template for fixing unsafe code:")?;
    writeln!(file, "// 1. Identify the unsafe operation")?;
    writeln!(file, "// 2. Consider safe alternatives")?;
    writeln!(file, "// 3. Add safety documentation if unsafe is necessary")?;
    Ok(())
}

fn write_performance_templates(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "// Template for performance improvements:")?;
    writeln!(file, "// 1. Profile the code")?;
    writeln!(file, "// 2. Identify bottlenecks")?;
    writeln!(file, "// 3. Consider algorithmic improvements")?;
    Ok(())
}

fn write_style_templates(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "// Template for style fixes:")?;
    writeln!(file, "// 1. Follow Rust naming conventions")?;
    writeln!(file, "// 2. Use consistent formatting")?;
    writeln!(file, "// 3. Remove redundant code")?;
    Ok(())
}

fn write_documentation_templates(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "// Template for documentation:")?;
    writeln!(file, "// 1. Add module-level documentation")?;
    writeln!(file, "// 2. Document public items")?;
    writeln!(file, "// 3. Include usage examples")?;
    Ok(())
}
