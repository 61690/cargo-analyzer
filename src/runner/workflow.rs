use std::process::Command;
use std::io::{self, Write};
use std::path::PathBuf;
use clap::{Parser, ArgAction};
use super::analysis_runner::AnalysisRunner;

#[derive(Parser)]
#[command(name = "cargo-analyzer")]
#[command(about = "Analyze Clippy warnings and generate detailed reports")]
pub struct CliArgs {
    #[arg(long, default_value = "clippy_output.json")]
    output_file: String,

    #[arg(long, value_name = "DIR")]
    working_dir: Option<PathBuf>,

    #[arg(long, value_name = "DIR")]
    reports_dir: Option<PathBuf>,

    #[arg(long, action=ArgAction::SetTrue)]
    workspace: bool,

    #[arg(long, action=ArgAction::SetTrue)]
    all_features: bool,

    #[arg(long, action=ArgAction::SetTrue)]
    all_targets: bool,
}

pub struct ClippyWorkflow {
    cargo_args: Vec<String>,
}

impl ClippyWorkflow {
    pub fn new(args: CliArgs) -> Self {
        let mut cargo_args = Vec::new();
        if args.workspace { cargo_args.push("--workspace".to_string()); }
        if args.all_features { cargo_args.push("--all-features".to_string()); }
        if args.all_targets { cargo_args.push("--all-targets".to_string()); }

        Self { cargo_args }
    }

    pub fn run(&self) -> io::Result<()> {
        // Create debug log file
        let debug_log = std::fs::File::create("clippy_analyzer_debug.log")?;
        let mut log = std::io::BufWriter::new(debug_log);
        writeln!(log, "=== CLIPPY ANALYZER DEBUG LOG ===\n")?;

        // Get current directory
        let current_dir = std::env::current_dir()?;
        writeln!(log, "Current directory: {:?}", current_dir)?;
        log.flush()?;

        // Create analysis_reports directory
        let reports_dir = current_dir.join("analysis_reports");
        writeln!(log, "\nCreating reports directory: {:?}", reports_dir)?;
        std::fs::create_dir_all(&reports_dir)?;
        writeln!(log, "Reports directory created")?;
        log.flush()?;

        // Generate timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        writeln!(log, "\nTimestamp: {}", timestamp)?;
        log.flush()?;

        // Create clippy output path
        let output_path = reports_dir.join(format!("clippy_output_{}.json", timestamp));
        writeln!(log, "Clippy output path: {:?}", output_path)?;
        log.flush()?;

        // Run clippy
        writeln!(log, "\nRunning cargo clippy")?;
        writeln!(log, "Command: cargo clippy {} --message-format=json", self.cargo_args.join(" "))?;
        log.flush()?;

        let status = Command::new("cargo")
            .args(["clippy"])
            .args(&self.cargo_args)
            .args(["--message-format=json"])
            .stdout(std::fs::File::create(&output_path)?)
            .status()?;

        if !status.success() {
            writeln!(log, "Clippy command failed!")?;
            log.flush()?;
            return Err(io::Error::new(io::ErrorKind::Other, "Clippy command failed"));
        }

        writeln!(log, "Clippy completed successfully")?;
        writeln!(log, "Output file size: {} bytes", std::fs::metadata(&output_path)?.len())?;
        log.flush()?;

        // Run analyzer
        writeln!(log, "\nStarting analysis")?;
        writeln!(log, "Reports directory: {:?}", reports_dir)?;
        writeln!(log, "Reading from: {:?}", output_path)?;
        log.flush()?;

        let mut analyzer = AnalysisRunner::new_with_reports_dir(Some(reports_dir.clone()))?;
        analyzer.set_timestamp(&timestamp);
        analyzer.run(output_path.to_str().unwrap())?;

        // List files
        writeln!(log, "\nFinal contents of reports directory:")?;
        for entry in std::fs::read_dir(&reports_dir)? {
            let entry = entry?;
            writeln!(log, "  - {:?} ({} bytes)", 
                entry.file_name(),
                entry.metadata()?.len()
            )?;
        }
        log.flush()?;

        writeln!(log, "\nWorkflow completed")?;
        log.flush()?;
        Ok(())
    }
}

pub fn run_analysis() -> io::Result<()> {
    // Skip "cargo" and "analyzer" from args when run as cargo subcommand
    let args = CliArgs::parse_from(std::env::args().skip(2));
    ClippyWorkflow::new(args).run()
} 