pub mod color;
pub mod report;
pub mod formatter;
pub mod markdown;
pub mod fix_plan;

pub use color::ColorWriter;
pub use report::{write_trend_analysis, write_colored_section};
pub use formatter::format_warning;
pub use markdown::{MarkdownWriter, generate_markdown_report};
pub use fix_plan::FixPlanGenerator;