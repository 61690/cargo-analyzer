use crate::{
    analysis::{
        charts::{ChartConfig, ChartStyle, create_enhanced_chart},
        trends::{TrendAnalysis, analyze_trends},
        statistics::warning::WarningStatistics,
    },
    parser::AnalysisContext, 
    types::CategoryType,
};
use std::io::{self, Write};
use std::collections::HashMap;

pub struct MarkdownWriter<W: Write> {
    writer: W,
}

impl<W: Write> MarkdownWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn write_header(&mut self, title: &str) -> io::Result<()> {
        writeln!(self.writer, "# {}\n", title)
    }

    pub fn write_summary(&mut self, stats: &WarningStatistics, timestamp: &str) -> io::Result<()> {
        writeln!(self.writer, "## Analysis Summary\n")?;
        writeln!(self.writer, "Total warnings: {}", stats.total_warnings)?;
        writeln!(self.writer, "Files affected: {}\n", stats.files_affected)?;
        
        // Map categories to severity levels based on our definitions
        let mut severity_counts: HashMap<&str, usize> = HashMap::new();
        for (category, count) in &stats.by_category {
            let severity = match category {
                CategoryType::Safety => "Critical",
                CategoryType::Performance => "High",
                CategoryType::Documentation => "Medium",
                CategoryType::Style => "Low",
            };
            *severity_counts.entry(severity).or_default() += count;
        }

        // Create severity distribution data (ordered by severity)
        writeln!(self.writer, "### Warning Distribution by Severity\n")?;
        let severity_data = vec![
            ("Critical".to_string(), *severity_counts.get("Critical").unwrap_or(&0)),
            ("High".to_string(), *severity_counts.get("High").unwrap_or(&0)),
            ("Medium".to_string(), *severity_counts.get("Medium").unwrap_or(&0)),
            ("Low".to_string(), *severity_counts.get("Low").unwrap_or(&0)),
        ];
        self.write_chart("Severity Distribution", &severity_data)?;

        // Add category distribution (sorted by count)
        writeln!(self.writer, "### Warning Distribution by Category\n")?;
        let mut category_data: Vec<(String, usize)> = stats.by_category
            .iter()
            .map(|(k, v)| (format!("{:?}", k), *v))
            .collect();
        category_data.sort_by(|a, b| b.1.cmp(&a.1));  // Sort by count in descending order
        self.write_chart("Category Distribution", &category_data)?;

        // Add subcategory insights (already sorted)
        writeln!(self.writer, "### Top Warning Subcategories\n")?;
        let mut subcategories: Vec<_> = stats.by_subcategory.iter().collect();
        subcategories.sort_by(|a, b| b.1.cmp(a.1));
        for (subcategory, count) in subcategories.iter().take(5) {
            writeln!(self.writer, "- {}: {} warnings", subcategory, count)?;
        }

        writeln!(self.writer, "\nFor detailed fix instructions, see clippy_fix_plan_{}.md", timestamp)?;
        Ok(())
    }

    fn write_chart(&mut self, title: &str, data: &[(String, usize)]) -> io::Result<()> {
        let chart_config = ChartConfig {
            style: ChartStyle::Blocks,
            color: None,
            width: 60,
            show_percentage: true,
        };

        let chart = create_enhanced_chart(data, chart_config);
        writeln!(self.writer, "#### {}\n", title)?;
        writeln!(self.writer, "```")?;
        writeln!(self.writer, "{}", chart)?;
        writeln!(self.writer, "```\n")?;

        // Add detailed percentages
        for (category, count) in data {
            let percentage = (*count as f64 / data.iter().map(|(_, c)| c).sum::<usize>() as f64) * 100.0;
            writeln!(self.writer, "- {}: {} ({:.1}%)", category, count, percentage)?;
        }
        writeln!(self.writer)?;
        Ok(())
    }

    pub fn write_trend_analysis(&mut self, trends: &TrendAnalysis, historical: &[TrendAnalysis]) -> io::Result<()> {
        writeln!(self.writer, "## Trend Analysis\n")?;
        
        if historical.is_empty() {
            writeln!(self.writer, "No historical data available for trend analysis.\n")?;
            writeln!(self.writer, "Current Analysis Summary:")?;
            writeln!(self.writer, "- Total Warnings: {}", trends.total_warnings)?;
            
            // Add category breakdown in severity order
            writeln!(self.writer, "\nWarning Distribution:")?;
            let ordered_categories = [
                (CategoryType::Safety, "CRITICAL"),
                (CategoryType::Performance, "HIGH"),
                (CategoryType::Documentation, "MEDIUM"),
                (CategoryType::Style, "LOW"),
            ];
            
            for (category, severity) in ordered_categories {
                let count = trends.by_category.get(&category).unwrap_or(&0);
                writeln!(self.writer, "- {} ({} Risk): {} warnings", category, severity, count)?;
            }
        } else {
            // Add trend chart
            let trend_data: Vec<(String, usize)> = historical.iter()
                .enumerate()
                .map(|(i, t)| (format!("Analysis {}", i + 1), t.total_warnings))
                .chain(std::iter::once(("Current".to_string(), trends.total_warnings)))
                .collect();
            
            writeln!(self.writer, "### Warning Count Trends\n")?;
            self.write_chart("Historical Trends", &trend_data)?;

            // Add category trend analysis
            writeln!(self.writer, "### Category Trends\n")?;
            let insights = analyze_trends(trends, historical);
            for insight in insights {
                writeln!(self.writer, "- {}", insight)?;
            }

            // Add risk level changes
            writeln!(self.writer, "\n### Risk Level Changes\n")?;
            for category in [CategoryType::Safety, CategoryType::Performance, CategoryType::Documentation, CategoryType::Style] {
                let current = trends.by_category.get(&category).unwrap_or(&0);
                let previous = historical.last()
                    .and_then(|h| h.by_category.get(&category))
                    .unwrap_or(&0);
                
                let change = *current as i64 - *previous as i64;
                let direction = if change > 0 { "increased" } else if change < 0 { "decreased" } else { "unchanged" };
                
                writeln!(self.writer, "- {:?} issues have {} ({:+})", category, direction, change)?;
            }
        }

        Ok(())
    }

    pub fn write_build_info(&mut self, context: &[AnalysisContext]) -> io::Result<()> {
        writeln!(self.writer, "## Build Configuration Analysis\n")?;
        
        let mut crate_stats = HashMap::new();
        for ctx in context {
            if let AnalysisContext::BuildInfo { 
                crate_name,
                features,
                build_config,
                ..
            } = ctx {
                crate_stats.entry(crate_name.clone())
                    .or_insert_with(Vec::new)
                    .push((features.len(), build_config.crate_types.len()));
            }
        }

        // Add build complexity analysis
        writeln!(self.writer, "### Build Complexity Overview\n")?;
        for (crate_name, stats) in &crate_stats {
            let avg_features = stats.iter().map(|(f, _)| f).sum::<usize>() as f64 / stats.len() as f64;
            writeln!(self.writer, "- {}: {:.1} features on average", crate_name, avg_features)?;
        }

        Ok(())
    }

    pub fn write_reports_list(&mut self, timestamp: &str) -> io::Result<()> {
        writeln!(self.writer, "\n=== Generated Reports ===\n")?;
        writeln!(self.writer, "📊 Generated Reports:\n")?;
        
        writeln!(self.writer, "📥 Input:")?;
        writeln!(self.writer, "  clippy_output.json⎯⎯▶ clippy_output.json.............. Raw Clippy warnings\n")?;

        writeln!(self.writer, "📋 Analysis:")?;
        writeln!(self.writer, "  clippy_analysis_{}.md⎯⎯▶ clippy_analysis_{}.md...... Detailed analysis with charts", timestamp, timestamp)?;
        // ... rest of the existing report listings ...
        Ok(())
    }
}

pub fn generate_markdown_report<W: Write>(
    writer: W,
    stats: &WarningStatistics,
    trends: &TrendAnalysis,
    historical: &[TrendAnalysis],
    context: &[AnalysisContext],
) -> io::Result<()> {
    let mut md_writer = MarkdownWriter::new(writer);
    
    // Write header and summary sections
    md_writer.write_header("Clippy Analysis Report")?;
    md_writer.write_summary(stats, &chrono::Local::now().format("%Y%m%d_%H%M%S").to_string())?;

    // Write build configuration
    md_writer.write_build_info(context)?;

    // Write trend analysis
    md_writer.write_trend_analysis(trends, historical)?;

    Ok(())
} 