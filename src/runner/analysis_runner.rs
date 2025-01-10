use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use crate::{
    parser::{WarningParser, AnalysisContext},
    types::{Warning, FileWarnings, CategoryType},
    analysis::{
        trends::TrendAnalysis,
        statistics::warning::WarningStatistics,
    },
    output::{
        color::ColorWriter,
        report::{write_warning_report, write_colored_section},
        markdown::generate_markdown_report,
        fix_plan::FixPlanGenerator,
    },
};

/// Provides the core analysis runner implementation for processing Clippy warnings.

/// Main struct responsible for executing the analysis workflow and generating reports.
/// 
/// The `AnalysisRunner` coordinates the entire analysis process, including:
/// - Parsing Clippy warnings
/// - Generating statistics
/// - Creating reports
/// - Managing historical data
/// 
/// # Example
/// 
/// ```rust
/// use cargo_analyzer::runner::analysis_runner::AnalysisRunner;
/// 
/// let runner = AnalysisRunner::new();
/// runner.run().expect("Analysis failed");
/// ```
pub struct AnalysisRunner {
    color_writer: ColorWriter,
    timestamp: String,
    reports_dir: Option<PathBuf>,
    debug_log: std::io::BufWriter<File>,
}

impl AnalysisRunner {
    /// Creates a new instance of the analysis runner with default configuration.
    pub fn new() -> std::io::Result<Self> {
        let debug_log = std::io::BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("clippy_analyzer_debug.log")?
        );

        Ok(Self {
            color_writer: ColorWriter::new(),
            timestamp: chrono::Local::now().format("%Y%m%d_%H%M%S").to_string(),
            reports_dir: None,
            debug_log,
        })
    }

    /// Creates a new instance with a custom reports directory.
    /// 
    /// # Arguments
    /// 
    /// * `reports_dir` - Path to the directory where reports will be stored
    pub fn new_with_reports_dir(reports_dir: Option<PathBuf>) -> std::io::Result<Self> {
        let debug_log = std::io::BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("clippy_analyzer_debug.log")?
        );

        Ok(Self {
            color_writer: ColorWriter::new(),
            timestamp: chrono::Local::now().format("%Y%m%d_%H%M%S").to_string(),
            reports_dir,
            debug_log,
        })
    }

    pub fn set_timestamp(&mut self, timestamp: &str) {
        self.timestamp = timestamp.to_string();
    }

    /// Executes the complete analysis workflow.
    /// 
    /// # Returns
    /// 
    /// Returns `Result<(), Error>` indicating success or failure of the analysis
    /// 
    /// # Errors
    /// 
    /// Will return an error if:
    /// - File parsing fails
    /// - Report generation fails
    /// - Output directory is not writable
    pub fn run(&mut self, input_path: &str) -> std::io::Result<()> {
        self.debug_log("Starting Clippy Analyzer")?;
        
        // Remove screen clearing
        // print!("\x1B[2J\x1B[1;1H");
        
        self.color_writer.write_header("Clippy Analyzer")?;
        
        // Create timestamped input file in reports directory
        self.debug_log("Creating output file...")?;
        let (mut output_file, file_path) = self.create_output_file("output")?;
        
        // Run clippy and capture its output
        let output = std::process::Command::new("cargo")
            .current_dir(std::env::current_dir()?)
            .args(["clippy", "--message-format=json"])
            .output()?;
            
        // Write clippy output to our file
        output_file.write_all(&output.stdout)?;
        
        let input_path = file_path.to_str().unwrap_or(input_path);
        self.debug_log(&format!("Analyzing input file: {}", input_path))?;
        writeln!(self.color_writer.writer(), "\nAnalyzing {}...\n", input_path)?;

        // Parse warnings and context
        let (warnings, file_warnings, context) = match WarningParser::parse_file(input_path) {
            Ok((w, fw, ctx)) => (w, fw, ctx),
            Err(e) => {
                self.color_writer.write_error(&format!("Failed to parse file: {}", e))?;
                return Ok(());
            }
        };

        if warnings.is_empty() {
            self.color_writer.write_error("No valid warnings were parsed from the input file!")?;
            return Ok(());
        }

        // Generate statistics and validate
        let stats = WarningStatistics::from_warnings(&warnings, file_warnings.len());
        
        // Validate warning counts
        let total_by_category: usize = stats.by_category.values().sum();
        let total_by_priority: usize = stats.by_priority.values().sum();
        if total_by_category != stats.total_warnings || total_by_priority != stats.total_warnings {
            self.color_writer.write_error("Warning count mismatch detected in analysis!\n")?;
            return Ok(());
        }

        // Show summary immediately
        self.write_terminal_summary(&stats)?;

        // Generate reports silently
        let (mut report_file, _) = self.create_output_file("report")?;
        let (mut summary_file, _) = self.create_output_file("summary")?;
        let (mut analysis_file, _) = self.create_output_file("analysis")?;
        let (mut json_file, _) = self.create_output_file("warnings_json")?;
        let (mut csv_file, _) = self.create_output_file("warnings_csv")?;
        let (mut fix_plan_file, _) = self.create_output_file("fix_plan")?;

        let historical_trends = self.load_historical_trends()?;
        let trend_analysis = TrendAnalysis::default();
        let trend = historical_trends.last().unwrap_or(&trend_analysis);

        self.generate_reports(
            &warnings,
            &file_warnings,
            &stats,
            trend,
            &historical_trends,
            &context,
            &mut report_file,
            &mut summary_file,
            &mut analysis_file,
            &mut json_file,
            &mut csv_file,
            &mut fix_plan_file,
        )?;

        // Add separator before success message
        writeln!(self.color_writer.writer(), "\n{}\n", "=".repeat(50))?;

        // Show success message with file links (without clearing screen)
        self.write_success_message()?;
        Ok(())
    }

    fn debug_log(&mut self, message: &str) -> std::io::Result<()> {
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f");
        writeln!(self.debug_log, "[{}] {}", timestamp, message)?;
        self.debug_log.flush()?;
        Ok(())
    }

    fn create_output_file(&mut self, name: &str) -> std::io::Result<(File, PathBuf)> {
        self.debug_log(&format!("\n=== Creating Output File: {} ===", name))?;
        
        let reports_dir = match &self.reports_dir {
            Some(dir) => {
                let dir_clone = dir.clone();
                self.debug_log(&format!("📁 Using specified reports directory: {:?}", dir))?;
                dir_clone
            },
            None => {
                self.debug_log("⚠️  No reports directory specified, using current directory")?;
                PathBuf::from(".")
            }
        };

        self.debug_log(&format!("📁 Ensuring directory exists: {:?}", reports_dir))?;
        std::fs::create_dir_all(&reports_dir)?;

        let filename = format!("clippy_{}_{}.{}", name, self.timestamp, self.get_extension(name));
        let file_path = reports_dir.join(&filename);
        
        self.debug_log(&format!("📝 Creating file: {:?}", file_path))?;
        let file = File::create(&file_path)?;
        self.debug_log("✅ File created successfully")?;
        self.debug_log("=== File Creation Complete ===\n")?;

        Ok((file, file_path))
    }

    fn get_extension(&self, name: &str) -> &str {
        match name {
            "output" => "json",
            "analysis" | "fix_plan" | "report" => "md",
            "summary" => "html",
            "warnings_json" => "json",
            "warnings_csv" => "csv",
            _ => "txt",
        }
    }

    fn load_historical_trends(&self) -> std::io::Result<Vec<TrendAnalysis>> {
        let path = "clippy_historical.json";
        if let Ok(file) = File::open(path) {
            let reader = std::io::BufReader::new(file);
            Ok(serde_json::from_reader(reader).unwrap_or_default())
        } else {
            Ok(Vec::new())
        }
    }

    fn generate_reports(
        &mut self,
        warnings: &[Warning],
        file_warnings: &HashMap<String, FileWarnings>,
        stats: &WarningStatistics,
        trends: &TrendAnalysis,
        historical_trends: &[TrendAnalysis],
        context: &[AnalysisContext],
        report_file: &mut File,
        summary_file: &mut File,
        markdown_file: &mut File,
        json_file: &mut File,
        csv_file: &mut File,
        fix_plan_file: &mut File,
    ) -> std::io::Result<()> {
        // Write CSV header
        writeln!(csv_file, "File,Line,Category,Message,Priority,Suggested Fix")?;

        // Write warnings to CSV
        for warning in warnings {
            writeln!(
                csv_file,
                "{},{},{:?},{},{:?},{}",
                warning.file,
                warning.line,
                warning.category,
                warning.message.replace(",", ";"),  // Escape commas
                warning.priority,
                warning.suggested_fix.as_ref().unwrap_or(&String::new()).replace(",", ";")
            )?;
        }

        // Write JSON output
        serde_json::to_writer_pretty(json_file, &warnings)?;

        // Write markdown report
        generate_markdown_report(
            markdown_file,
            stats,
            trends,
            &historical_trends,
            context,
        )?;

        // Write summary statistics
        self.write_summary(summary_file, stats)?;

        // Write detailed report
        self.write_detailed_report(report_file, warnings, file_warnings, stats, trends)?;

        let mut fix_plan_generator = FixPlanGenerator::new(fix_plan_file);
        fix_plan_generator.generate_plan(warnings)?;

        Ok(())
    }

    fn write_success_message(&mut self) -> std::io::Result<()> {
        // Always use reports_dir or default to "analysis_reports"
        let base_dir = self.reports_dir
            .as_ref()
            .map(|p| p.to_owned())
            .unwrap_or_else(|| PathBuf::from("analysis_reports"));

        // Get absolute path
        let base_dir = if base_dir.is_relative() {
            std::env::current_dir()?.join(base_dir)
        } else {
            base_dir
        };

        let base_dir = base_dir.to_string_lossy().to_string();
        
        self.color_writer.write_header("Generated Reports")?;
        writeln!(self.color_writer.writer())?;
        self.color_writer.write_success("📊 Generated Reports:\n")?;
        
        // Define file groups with icons and descriptions
        let file_groups = [
            ("📋", "Analysis", vec![
                ("analysis", "md", "Detailed analysis with charts"),
                ("fix_plan", "md", "Fix suggestions and priorities"),
            ]),
            ("📝", "Reports", vec![
                ("report", "md", "File-by-file analysis"),
                ("summary", "html", "Interactive overview"),
            ]),
            ("📦", "Data", vec![
                ("warnings_csv", "csv", "CSV format"),
                ("warnings_json", "json", "JSON format"),
            ]),
        ];

        // Write each group
        for (icon, group_name, files) in file_groups {
            writeln!(self.color_writer.writer(), "\n{} {}:", icon, group_name)?;
            
            for (name, ext, desc) in files {
                let filename = format!("clippy_{}_{}.{}", name, self.timestamp, ext);
                let filepath = format!("{}/{}", base_dir, filename);
                
                writeln!(
                    self.color_writer.writer(),
                    "  \x1B]8;;file://{}\x1B\\{}⎯⎯▶ {}\x1B]8;;\x1B\\ {}",
                    filepath,
                    filename,
                    if filename.len() >= 40 {
                        format!("{}...", &filename[..37])
                    } else {
                        format!("{:.<40}", filename)
                    },
                    desc
                )?;
            }
        }

        writeln!(self.color_writer.writer())?;
        self.color_writer.write_success("✨ Analysis complete! Click on any file to open it.\n")?;
        Ok(())
    }

    fn write_summary(&self, file: &mut File, stats: &WarningStatistics) -> std::io::Result<()> {
        writeln!(file, "Total Warnings: {}", stats.total_warnings)?;
        writeln!(file, "Files Affected: {}", stats.files_affected)?;
        writeln!(file, "\nCategory Distribution:")?;
        
        for (category, count) in &stats.by_category {
            let percentage = (*count as f64 / stats.total_warnings as f64) * 100.0;
            writeln!(file, "{:?}: {} ({:.1}%)", category, count, percentage)?;
        }

        writeln!(file, "\nPriority Distribution:")?;
        for (priority, count) in &stats.by_priority {
            writeln!(file, "{:?}: {}", priority, count)?;
        }

        writeln!(file, "\nSubcategory Distribution:")?;
        for (subcategory, count) in &stats.by_subcategory {
            writeln!(file, "{}: {}", subcategory, count)?;
        }

        Ok(())
    }

    fn write_detailed_report(
        &self,
        file: &mut File,
        warnings: &[Warning],
        file_warnings: &HashMap<String, FileWarnings>,
        stats: &WarningStatistics,
        trends: &TrendAnalysis,
    ) -> std::io::Result<()> {
        let (safety, perf, style, docs) = stats.get_detailed_stats();

        // Write file-by-file analysis
        writeln!(file, "File-by-File Analysis\n")?;
        writeln!(file, "===================\n")?;
        
        for (file_path, file_warnings) in file_warnings {
            writeln!(file, "File: {}", file_path)?;
            writeln!(file, "Total warnings: {}", file_warnings.warnings.len())?;
            
            let analysis_results = file_warnings.analyze_file();
            for (severity, impact) in analysis_results {
                writeln!(file, "- [{}] {}", severity, impact)?;
            }
            writeln!(file)?;
        }

        // Write detailed statistics sections
        write_colored_section(file, "Safety Issues", &format!("{:#?}", safety), termcolor::Color::Red)?;
        write_colored_section(file, "Performance Issues", &format!("{:#?}", perf), termcolor::Color::Yellow)?;
        write_colored_section(file, "Style Issues", &format!("{:#?}", style), termcolor::Color::Blue)?;
        write_colored_section(file, "Documentation Issues", &format!("{:#?}", docs), termcolor::Color::Green)?;

        // Write trend analysis
        write_colored_section(file, "Trend Analysis", &format!("{:#?}", trends), termcolor::Color::Magenta)?;

        // Write all warnings with their full details
        writeln!(file, "\nDetailed Warning List\n")?;
        write_warning_report(file, warnings, true)?;

        Ok(())
    }

    fn write_terminal_summary(&mut self, stats: &WarningStatistics) -> std::io::Result<()> {
        writeln!(self.color_writer.writer(), "\n📊 Analysis Summary:\n")?;
        
        // Input file stats
        writeln!(self.color_writer.writer(), "📥 Input:")?;
        self.color_writer.write_colored(
            &format!("- Raw warnings from clippy: {} warnings\n", stats.total_input_warnings),
            termcolor::Color::White,
        )?;
        writeln!(self.color_writer.writer())?;

        // Validation stats
        writeln!(self.color_writer.writer(), "🔍 Validation:")?;
        let total_by_category: usize = stats.by_category.values().sum();
        let total_by_priority: usize = stats.by_priority.values().sum();
        self.color_writer.write_colored(
            &format!("- Category total: {} warnings\n", total_by_category),
            termcolor::Color::White,
        )?;
        self.color_writer.write_colored(
            &format!("- Priority total: {} warnings\n", total_by_priority),
            termcolor::Color::White,
        )?;
        self.color_writer.write_colored(
            &format!("- Total warnings: {}\n", stats.total_warnings),
            termcolor::Color::White,
        )?;
        writeln!(self.color_writer.writer())?;

        // Total counts
        writeln!(self.color_writer.writer(), "📈 Overview:")?;
        self.color_writer.write_colored(
            &format!("- Total Warnings: {}\n", stats.total_warnings),
            termcolor::Color::White,
        )?;
        self.color_writer.write_colored(
            &format!("- Files Affected: {}\n", stats.files_affected),
            termcolor::Color::White,
        )?;

        // Category breakdown
        writeln!(self.color_writer.writer(), "\n📈 Warning Distribution:")?;
        let ordered_categories = [
            (CategoryType::Safety, "🔴", termcolor::Color::Red),
            (CategoryType::Performance, "🟡", termcolor::Color::Yellow),
            (CategoryType::Documentation, "🟢", termcolor::Color::Cyan),
            (CategoryType::Style, "⚪", termcolor::Color::Blue),
        ];

        for (category, icon, color) in ordered_categories {
            if let Some(&count) = stats.by_category.get(&category) {
                let percentage = (count as f64 / stats.total_warnings as f64) * 100.0;
                self.color_writer.write_colored(
                    &format!("{} {:?}: {} ({:.1}%)\n", 
                        icon, category, count, percentage),
                    color,
                )?;
            }
        }

        // Top subcategories
        writeln!(self.color_writer.writer(), "\n🔍 Top Warning Types:")?;
        let mut subcategories: Vec<_> = stats.by_subcategory.iter().collect();
        subcategories.sort_by(|a, b| b.1.cmp(a.1));
        
        for (subcategory, count) in subcategories.iter().take(5) {
            self.color_writer.write_colored(
                &format!("- {}: {} warnings\n", subcategory, count),
                termcolor::Color::White,
            )?;
        }

        writeln!(self.color_writer.writer())?;
        Ok(())
    }


} 