use std::fs::File;
use std::io::Write;
use termcolor::Color;
use crate::analysis::TrendAnalysis;

pub fn write_trend_analysis(file: &mut File, trends: &TrendAnalysis) -> std::io::Result<()> {
    writeln!(file, "\n=== Trend Analysis ===")?;
    
    // Write total warnings
    writeln!(file, "\nTotal Warnings: {}", trends.total_warnings)?;
    
    // Write category breakdown
    writeln!(file, "\nWarnings by Category:")?;
    for (category, count) in &trends.by_category {
        writeln!(file, "  {} - {} issues", category, count)?;
    }

    // Write priority breakdown
    writeln!(file, "\nWarnings by Priority:")?;
    for (priority, count) in &trends.by_priority {
        writeln!(file, "  {:?} - {} warnings", priority, count)?;
    }

    // Write recurring issues
    writeln!(file, "\nRecurring Issues:")?;
    for (issue, count) in &trends.recurring_issues {
        writeln!(file, "  {} - {} occurrences", issue, count)?;
    }

    // Write improvement rate
    writeln!(file, "\nImprovement Rate: {:.1}%", trends.improvement_rate * 100.0)?;

    Ok(())
}

pub fn write_colored_section(file: &mut File, title: &str, content: &str, _color: Color) -> std::io::Result<()> {
    writeln!(file, "=== {} ===\n", title)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn write_warning_report(
    file: &mut File,
    warnings: &[crate::types::Warning],
    show_snippets: bool
) -> std::io::Result<()> {
    writeln!(file, "Warning Report\n")?;

    let mut current_file = String::new();
    for warning in warnings {
        if warning.file != current_file {
            current_file = warning.file.clone();
            writeln!(file, "\nFile: {}", current_file)?;
        }

        let (formatted, _) = super::formatter::format_warning(warning);
        writeln!(file, "{}", formatted)?;

        if show_snippets && warning.suggested_fix.is_some() {
            writeln!(file, "Suggested fix:\n{}\n", warning.suggested_fix.as_ref().unwrap())?;
        }
    }

    Ok(())
}
