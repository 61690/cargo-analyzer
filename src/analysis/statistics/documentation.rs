use std::collections::HashMap;
use crate::types::Warning;

#[derive(Debug, Default)]
pub struct DocStatistics {
    pub total_issues: usize,
    pub missing_docs: HashMap<String, usize>,
    pub quality_issues: HashMap<String, usize>,
    pub link_issues: usize,
}

impl DocStatistics {
    pub fn update(&mut self, warning: &Warning) {
        self.total_issues += 1;
        
        if warning.message.contains("missing") {
            self.missing_docs
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        
        if warning.message.contains("quality") {
            self.quality_issues
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        
        if warning.message.contains("link") {
            self.link_issues += 1;
        }
    }
} 