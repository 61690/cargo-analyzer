use std::io::Write;
use std::collections::{HashMap, HashSet};
use crate::{
    types::{Warning, CategoryType, Priority},
    fixes::{examples::get_fix_example, suggestions::generate_fix_suggestion},
    analysis::{
        statistics::warning::WarningStatistics,
        charts::{ChartConfig, ChartStyle, create_enhanced_chart},
    },
};

pub struct FixPlanGenerator<W: Write> {
    writer: W,
}

#[derive(Default)]
struct CategoryStats<'a> {
    count: usize,
    files: HashSet<String>,
    subcategories: HashMap<String, Vec<&'a Warning>>,
}

impl<W: Write> FixPlanGenerator<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn generate_plan(&mut self, warnings: &[Warning]) -> std::io::Result<()> {
        self.write_header()?;
        self.write_overview()?;
        
        // Generate and write statistics
        let stats = WarningStatistics::from_warnings(warnings, warnings.iter()
            .map(|w| w.file.clone())
            .collect::<HashSet<_>>()
            .len());
        self.write_statistics(&stats)?;
        
        self.write_risk_levels()?;

        // Group warnings by priority and category
        let mut priority_groups: HashMap<Priority, HashMap<CategoryType, CategoryStats>> = HashMap::new();
        
        for warning in warnings {
            let priority = match warning.category {
                CategoryType::Safety => Priority::Critical,
                CategoryType::Performance => Priority::High,
                CategoryType::Style => Priority::Low,
                CategoryType::Documentation => Priority::Medium,
            };

            let stats = priority_groups
                .entry(priority)
                .or_default()
                .entry(warning.category)
                .or_default();
            
            stats.count += 1;
            stats.files.insert(warning.file.clone());
            stats.subcategories
                .entry(warning.message.clone())
                .or_default()
                .push(warning);
        }

        // Generate sections by priority
        for priority in [Priority::Critical, Priority::High, Priority::Medium, Priority::Low] {
            if let Some(categories) = priority_groups.get(&priority) {
                self.write_priority_section(priority, categories)?;
            }
        }

        Ok(())
    }

    fn write_statistics(&mut self, stats: &WarningStatistics) -> std::io::Result<()> {
        writeln!(self.writer, "## Summary\n")?;
        writeln!(self.writer, "Total warnings: {}", stats.total_warnings)?;
        writeln!(self.writer, "Files affected: {}\n", stats.files_affected)?;
        
        writeln!(self.writer, "### Category Breakdown\n")?;
        
        // Create category distribution chart
        let warning_counts: Vec<(String, usize)> = stats.by_category
            .iter()
            .map(|(k, v)| (format!("{:?}", k), *v))
            .collect();

        let chart_config = ChartConfig {
            style: ChartStyle::Blocks,
            color: None,
            width: 50,
            show_percentage: true,
        };

        let chart = create_enhanced_chart(&warning_counts, chart_config);
        
        writeln!(self.writer, "```")?;
        writeln!(self.writer, "{}", chart)?;
        writeln!(self.writer, "```\n")?;

        // Write detailed statistics
        for (category, count) in &stats.by_category {
            let percentage = (*count as f64 / stats.total_warnings as f64) * 100.0;
            writeln!(self.writer, "- {:?}: {} ({:.1}%)", category, count, percentage)?;
        }
        writeln!(self.writer)?;

        Ok(())
    }

    fn write_priority_section(
        &mut self, 
        priority: Priority,
        categories: &HashMap<CategoryType, CategoryStats>
    ) -> std::io::Result<()> {
        writeln!(self.writer, "\n# {} Priority Warnings (Risk Level: {})\n", 
            priority, 
            priority.severity_score()
        )?;

        for (category, stats) in categories {
            self.write_category_section(category, stats)?;
        }

        Ok(())
    }

    fn write_category_section(
        &mut self, 
        category: &CategoryType,
        stats: &CategoryStats
    ) -> std::io::Result<()> {
        writeln!(self.writer, "## {} Issues\n", category)?;
        writeln!(self.writer, "**Frequency**: {} occurrences", stats.count)?;
        writeln!(self.writer, "**Affected Files**: {} files\n", stats.files.len())?;

        for (subcategory, warnings) in &stats.subcategories {
            self.write_subcategory_section(category, subcategory, warnings)?;
        }

        Ok(())
    }

    fn write_subcategory_section(
        &mut self,
        category: &CategoryType,
        subcategory: &str,
        warnings: &[&Warning]
    ) -> std::io::Result<()> {
        writeln!(self.writer, "### {}\n", subcategory)?;
        
        // Write impact assessment
        self.write_impact_assessment(category, warnings)?;

        // Get both example and specific fix
        if let Some(example) = get_fix_example(&warnings[0]) {
            writeln!(self.writer, "#### Fix Template\n")?;
            writeln!(self.writer, "```rust")?;
            writeln!(self.writer, "{}", example.before)?;
            writeln!(self.writer, "\n// After applying fix:\n")?;
            writeln!(self.writer, "{}", example.after)?;
            writeln!(self.writer, "```\n")?;
        }

        // Add specific fix suggestion if available
        if let Some(fix) = generate_fix_suggestion(&warnings[0]) {
            writeln!(self.writer, "#### Specific Fix\n")?;
            writeln!(self.writer, "```rust")?;
            writeln!(self.writer, "{}", fix.code)?;
            writeln!(self.writer, "```\n")?;
            writeln!(self.writer, "Confidence: {:.0}%\n", fix.confidence * 100.0)?;
        }

        // List all occurrences with more detail
        writeln!(self.writer, "#### All Occurrences\n")?;
        for warning in warnings {
            writeln!(self.writer, "**{}:{}**", warning.file, warning.line)?;
            writeln!(self.writer, "```")?;
            writeln!(self.writer, "Message: {}", warning.message)?;
            
            // Format child messages properly
            let child_messages = warning.message.lines()
                .find(|line| line.contains("Child messages:"))
                .map(|line| line.to_string());  // Create owned String

            if let Some(msg) = child_messages {
                let msg_content = msg.replace("Child messages: ", "")
                    .trim_matches(|c| c == '[' || c == ']' || c == '"')
                    .to_string();
                let messages = msg_content.split("\", \"").collect::<Vec<_>>();
                
                writeln!(self.writer, "\nChild Messages:")?;
                for msg in messages {
                    writeln!(self.writer, "- {}", msg)?;
                }
            }
            writeln!(self.writer, "```\n")?;
        }

        Ok(())
    }

    fn write_header(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "# Comprehensive Fix Priority Plan\n")?;
        writeln!(self.writer, "This plan outlines all detected issues, prioritized by risk level and impact.\n")
    }

    fn write_overview(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "## Overview\n")?;
        writeln!(self.writer, "This plan covers all warning types, prioritized by risk level and frequency.\n")
    }

    fn write_risk_levels(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "## Risk Level Definitions\n")?;
        writeln!(self.writer, "- 5: Critical - Immediate action required (safety issues, potential bugs)")?;
        writeln!(self.writer, "- 4: High - Should be fixed soon (correctness issues, performance problems)")?;
        writeln!(self.writer, "- 3: Medium - Plan to fix (maintainability issues)")?;
        writeln!(self.writer, "- 2: Low - Fix when convenient (style issues)")?;
        writeln!(self.writer, "- 1: Trivial - Optional fixes\n")
    }

    fn write_impact_assessment(
        &mut self,
        category: &CategoryType,
        warnings: &[&Warning]
    ) -> std::io::Result<()> {
        // Match severity and impact based on both category AND priority
        let (severity, impact) = match (category, warnings[0].priority) {
            (CategoryType::Safety, Priority::Critical) => 
                ("CRITICAL SAFETY ISSUE", "Could cause production failures or security vulnerabilities."),
            (CategoryType::Safety, _) => 
                ("SAFETY CONCERN", "May affect program correctness."),
            (CategoryType::Performance, Priority::High) => 
                ("HIGH PERFORMANCE IMPACT", "May affect system responsiveness and resource usage."),
            (CategoryType::Performance, _) => 
                ("PERFORMANCE CONCERN", "Could impact efficiency."),
            (CategoryType::Style, _) => 
                ("MAINTAINABILITY CONCERN", "Affects code readability and maintenance."),
            (CategoryType::Documentation, _) => 
                ("DOCUMENTATION GAP", "Impacts code understanding and usability."),
        };

        let pattern = if warnings.len() > 5 {
            "Widespread issue affecting multiple files - consider systematic fix."
        } else {
            "Isolated occurrences - can be fixed individually."
        };

        writeln!(self.writer, "**Risk Assessment**: {}: {}", severity, warnings[0].message)?;
        writeln!(self.writer, "**Impact**: {}", impact)?;
        writeln!(self.writer, "**Pattern**: {}\n", pattern)
    }
} 