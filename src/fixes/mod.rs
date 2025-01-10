pub mod examples;
pub mod templates;
pub mod suggestions;

pub use examples::{FixExample, get_fix_example};
pub use templates::write_fix_template;
pub use suggestions::generate_fix_suggestion;
