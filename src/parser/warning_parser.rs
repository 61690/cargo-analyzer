use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use serde::Deserialize;
use crate::types::{
    Warning, FileWarnings,
    categories::{CategoryType, WarningCategory},
    priorities::Priority,
};

#[derive(Debug, Deserialize)]
struct CompilerMessage {
    reason: String,
    package_id: Option<String>,
    manifest_path: Option<String>,
    target: Option<CompilerTarget>,
    message: Option<DiagnosticMessage>,
    features: Option<Vec<String>>,
    filenames: Option<Vec<String>>,
    executable: Option<String>,
    fresh: Option<bool>,
    profile: Option<BuildProfile>,
}

#[derive(Debug, Deserialize)]
struct CompilerTarget {
    kind: Vec<String>,
    crate_types: Vec<String>,
    name: String,
    src_path: String,
    edition: String,
    doc: Option<bool>,
    doctest: Option<bool>,
    test: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DiagnosticMessage {
    code: Option<DiagnosticCode>,
    level: String,
    message: String,
    spans: Vec<DiagnosticSpan>,
    children: Vec<DiagnosticMessage>,
    rendered: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DiagnosticCode {
    code: String,
}

#[derive(Debug, Deserialize)]
struct DiagnosticSpan {
    file_name: String,
    line_start: u32,
    line_end: u32,
    column_start: u32,
    column_end: u32,
}

#[derive(Debug)]
pub enum AnalysisContext {
    Warning(Warning),
    BuildInfo {
        crate_name: String,
        features: Vec<String>,
        build_config: BuildConfig,
        artifacts: Vec<String>,
        manifest_path: String,
    },
    BuildScript {
        package: String,
        success: bool,
        output: Option<String>,
        linked_libs: Vec<String>,
        linked_paths: Vec<String>,
    },
}

#[derive(Debug)]
pub struct BuildConfig {
    pub edition: String,
    pub opt_level: String,
    pub debug: bool,
    pub test_mode: bool,
    pub crate_types: Vec<String>,
    pub is_doc: bool,
    pub is_doctest: bool,
    pub profile: Option<BuildProfile>,
    pub kind: Vec<String>,
    pub name: String,
    pub src_path: String,
}

#[derive(Debug, Deserialize)]
pub struct BuildProfile {
    pub opt_level: String,
    pub debuginfo: u32,
    pub debug_assertions: bool,
    pub overflow_checks: bool,
    pub test: bool,
}

#[derive(Debug)]
struct BuildInfo {
    crate_name: String,
    features: Vec<String>,
    manifest_path: String,
    filenames: Vec<String>,
    config: BuildConfig,
}

#[derive(Debug)]
struct BuildScriptInfo {
    package: String,
    success: bool,
    output: Option<String>,
}

pub struct WarningParser {
    files: HashMap<String, FileWarnings>,
}

impl WarningParser {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn parse_file(input_path: &str) 
        -> std::io::Result<(Vec<Warning>, HashMap<String, FileWarnings>, Vec<AnalysisContext>)> 
    {
        let file = File::open(input_path)?;
        let reader = BufReader::new(file);
        let mut parser = Self::new();
        let mut warnings = Vec::new();
        let mut context = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(message) = serde_json::from_str::<CompilerMessage>(&line) {
                    if let Some(ctx) = parser.parse_compiler_message(message) {
                        match &ctx {
                            AnalysisContext::Warning(warning) => {
                                parser.files
                                    .entry(warning.file.clone())
                                    .or_insert_with(|| FileWarnings::new(warning.file.clone()))
                                    .add_warning(warning.clone());
                                warnings.push(warning.clone());
                            }
                            _ => {}
                        }
                        context.push(ctx);
                    }
                }
            }
        }

        Ok((warnings, parser.files, context))
    }

    fn parse_compiler_message(&mut self, msg: CompilerMessage) -> Option<AnalysisContext> {
        match msg.reason.as_str() {
            "compiler-artifact" => self.parse_artifact_message(msg)
                .map(|info| AnalysisContext::BuildInfo {
                    crate_name: info.crate_name,
                    features: info.features,
                    build_config: info.config,
                    artifacts: info.filenames,
                    manifest_path: info.manifest_path,
                }),
            "compiler-message" => self.parse_diagnostic_message(msg)
                .map(AnalysisContext::Warning),
            "build-script-executed" => self.parse_build_script_message(msg)
                .map(|info| AnalysisContext::BuildScript {
                    package: info.package,
                    success: info.success,
                    output: info.output,
                    linked_libs: Vec::new(),
                    linked_paths: Vec::new(),
                }),
            _ => None,
        }
    }

    fn parse_artifact_message(&self, msg: CompilerMessage) -> Option<BuildInfo> {
        let target = msg.target?;
        let package_id = msg.package_id?;
        let profile = msg.profile?;
        let manifest_path = msg.manifest_path?;
        let filenames = msg.filenames.unwrap_or_default();
        
        Some(BuildInfo {
            crate_name: package_id.split('#').next()?.split('/').last()?.to_string(),
            features: msg.features.unwrap_or_default(),
            manifest_path,
            filenames,
            config: BuildConfig {
                edition: target.edition,
                crate_types: target.crate_types,
                opt_level: profile.opt_level.clone(),
                debug: profile.debuginfo > 0,
                test_mode: target.test.unwrap_or(false),
                is_doc: target.doc.unwrap_or(false),
                is_doctest: target.doctest.unwrap_or(false),
                profile: Some(profile),
                kind: target.kind,
                name: target.name,
                src_path: target.src_path,
            },
        })
    }

    fn parse_diagnostic_message(&self, msg: CompilerMessage) -> Option<Warning> {
        let diagnostic = msg.message?;
        let span = diagnostic.spans.first()?;
        
        let clippy_code = diagnostic.code.as_ref()
            .map(|c| c.code.clone())
            .unwrap_or_else(|| "unknown".to_string());

        let (suggestion, explanations) = if let Some(ref rendered) = diagnostic.rendered {
            self.parse_clippy_suggestion(rendered)
        } else {
            (None, Vec::new())
        };

        let child_messages: Vec<String> = diagnostic.children.iter()
            .filter(|child| !child.message.starts_with("for further information"))
            .map(|child| child.message.clone())
            .collect();

        let location = format!(
            "{}:{}-{}:{}-{}",
            span.line_start,
            span.column_start,
            span.line_end,
            span.column_end,
            span.file_name
        );

        let message = diagnostic.message.clone();
        
        Some(Warning {
            message: format!(
                "{}\nLocation: {}\nExplanation: {}\nChild messages: {:?}", 
                message,
                location,
                explanations.join("\n"),
                child_messages
            ),
            file: span.file_name.clone(),
            line: span.line_start,
            category: WarningCategory::new(
                self.categorize_clippy_warning(&clippy_code),
                clippy_code
            ),
            priority: self.determine_message_priority(&diagnostic),
            suggested_fix: suggestion,
        })
    }

    fn parse_clippy_suggestion(&self, rendered: &str) -> (Option<String>, Vec<String>) {
        let lines: Vec<&str> = rendered.lines().collect();
        let mut suggestion = None;
        let mut explanations = Vec::new();

        for line in lines {
            if line.contains("help: ") {
                suggestion = Some(line.replace("help: ", "").trim().to_string());
            } else if line.starts_with("= help:") {
                explanations.push(line.replace("= help:", "").trim().to_string());
            }
        }

        (suggestion, explanations)
    }

    fn categorize_clippy_warning(&self, code: &str) -> CategoryType {
        match code {
            c if c.contains("use_self") || c.contains("redundant") => CategoryType::Style,
            c if c.contains("unsafe") || c.contains("mut") => CategoryType::Safety,
            c if c.contains("perf") || c.contains("box") => CategoryType::Performance,
            c if c.contains("doc") || c.contains("missing") => CategoryType::Documentation,
            _ => CategoryType::Style,
        }
    }

    fn parse_build_script_message(&self, msg: CompilerMessage) -> Option<BuildScriptInfo> {
        let package_id = msg.package_id?;
        
        Some(BuildScriptInfo {
            package: package_id,
            success: msg.fresh.unwrap_or(false),
            output: msg.executable,
        })
    }

    fn determine_message_priority(&self, diagnostic: &DiagnosticMessage) -> Priority {
        match diagnostic.level.as_str() {
            "error" => Priority::Critical,
            "warning" => {
                if diagnostic.message.contains("unsafe") || 
                   diagnostic.message.contains("security") {
                    Priority::High
                } else {
                    Priority::Medium
                }
            },
            _ => Priority::Low,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_artifact_message() {
        let json = r#"{
            "reason": "compiler-artifact",
            "package_id": "my-package",
            "manifest_path": "/path/to/Cargo.toml",
            "target": {
                "kind": ["lib"],
                "name": "my_lib",
                "src_path": "/path/to/src/lib.rs",
                "edition": "2021"
            }
        }"#;

        let msg: CompilerMessage = serde_json::from_str(json).unwrap();
        let mut parser = WarningParser::new();
        
        if let Some(AnalysisContext::BuildInfo { crate_name, .. }) = parser.parse_compiler_message(msg) {
            assert_eq!(crate_name, "my-package");
        } else {
            panic!("Expected BuildInfo variant");
        }
    }

    #[test]
    fn test_parse_diagnostic_message() {
        let json = r#"{
            "reason": "compiler-message",
            "message": {
                "code": {"code": "dead_code"},
                "level": "warning",
                "message": "unused variable `x`",
                "spans": [{
                    "file_name": "src/main.rs",
                    "line_start": 10,
                    "line_end": 10,
                    "column_start": 1,
                    "column_end": 10
                }],
                "children": [],
                "rendered": "help: remove this variable"
            }
        }"#;

        let msg: CompilerMessage = serde_json::from_str(json).unwrap();
        let mut parser = WarningParser::new();
        
        if let Some(AnalysisContext::Warning(warning)) = parser.parse_compiler_message(msg) {
            assert_eq!(warning.file, "src/main.rs");
            assert_eq!(warning.line, 10);
            assert_eq!(warning.category.category_type, CategoryType::Style);
        } else {
            panic!("Expected Warning variant");
        }
    }
} 
