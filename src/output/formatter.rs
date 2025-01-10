use crate::types::{Warning, CategoryType};
use termcolor::Color;

pub struct WarningFormatter<'a> {
    warning: &'a Warning,
}

impl<'a> WarningFormatter<'a> {
    pub fn new(warning: &'a Warning) -> Self {
        Self { warning }
    }

    pub fn format(&self) -> (String, Color) {
        let color = self.get_category_color();
        let priority_marker = self.get_priority_marker();
        let formatted = format!(
            "{} {} in {} (line {})\n    {}\n",
            priority_marker,
            self.warning.category.category_type,
            self.warning.file,
            self.warning.line,
            self.warning.message
        );
        (formatted, color)
    }

    fn get_category_color(&self) -> Color {
        match self.warning.category.category_type {
            CategoryType::Safety => Color::Red,
            CategoryType::Performance => Color::Yellow,
            CategoryType::Style => Color::Blue,
            CategoryType::Documentation => Color::Cyan,
        }
    }

    fn get_priority_marker(&self) -> &'static str {
        match self.warning.category.category_type {
            CategoryType::Safety => "🔴",
            CategoryType::Performance => "🟡",
            CategoryType::Documentation => "🟢",
            CategoryType::Style => "⚪",
        }
    }
}

pub fn format_warning(warning: &Warning) -> (String, Color) {
    let formatter = WarningFormatter::new(warning);
    formatter.format()
}

pub fn format_summary(total: usize, by_category: &[(CategoryType, usize)], input_warnings: usize) -> String {
    let mut summary = format!("\nTotal Warnings: {} (from {} input warnings)\n\n", total, input_warnings);
    summary.push_str("By Category:\n");
    
    for (category, count) in by_category {
        summary.push_str(&format!("  {}: {}\n", category, count));
    }
    
    summary
}

pub fn format_file_path(path: &str, warning_count: usize) -> String {
    format!("\n📁 {} ({} warnings)\n", path, warning_count)
}

pub fn format_code_snippet(code: &str, line_number: u32) -> String {
    let mut formatted = String::new();
    for (i, line) in code.lines().enumerate() {
        let line_num = line_number as usize + i;
        formatted.push_str(&format!("{:>4} | {}\n", line_num, line));
    }
    formatted
} 