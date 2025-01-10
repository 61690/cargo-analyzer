use std::collections::HashMap;
use crate::types::Warning;

#[derive(Debug, Default)]
pub struct StyleStatistics {
    pub total_issues: usize,
    pub naming_issues: HashMap<String, usize>,
    pub unused_patterns: HashMap<String, usize>,
    pub complexity_issues: HashMap<String, usize>,
}

impl StyleStatistics {
    pub fn update(&mut self, warning: &Warning) {
        self.total_issues += 1;
        
        if warning.message.contains("naming") {
            self.naming_issues
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        
        if warning.message.contains("unused") {
            self.unused_patterns
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        
        if warning.message.contains("complex") {
            self.complexity_issues
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
} 