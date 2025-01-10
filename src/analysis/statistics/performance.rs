use std::collections::HashMap;
use crate::types::Warning;

#[derive(Debug, Default)]
pub struct PerformanceStatistics {
    pub total_issues: usize,
    pub allocation_patterns: HashMap<String, usize>,
    pub clone_patterns: HashMap<String, usize>,
    pub lock_patterns: HashMap<String, usize>,
}

impl PerformanceStatistics {
    pub fn update(&mut self, warning: &Warning) {
        self.total_issues += 1;
        
        if warning.message.contains("allocation") {
            self.allocation_patterns
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        
        if warning.message.contains("clone") {
            self.clone_patterns
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        
        if warning.message.contains("lock") {
            self.lock_patterns
                .entry(warning.message.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
} 