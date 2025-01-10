use std::process;
use cargo_analyzer::runner::workflow::run_analysis;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Starting Clippy Analyzer...");
    
    // Try basic file writing first
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("clippy_basic.log")
        .expect("Failed to open log file");

    writeln!(file, "=== Starting Clippy Analyzer ===")
        .expect("Failed to write to log");
    
    println!("Created log file");
    
    if let Err(e) = run_analysis() {
        writeln!(file, "Error: {}", e).expect("Failed to write error");
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

