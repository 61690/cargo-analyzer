use std::collections::HashMap;
use crate::types::{Warning, CategoryType, Priority};
use super::{
    safety::SafetyStatistics,
    performance::PerformanceStatistics,
    style::StyleStatistics,
    documentation::DocStatistics,
};

#[derive(Debug, Default)]
pub struct WarningStatistics {
    pub total_warnings: usize,
    pub total_input_warnings: usize,
    pub files_affected: usize,
    pub by_priority: HashMap<Priority, usize>,
    pub by_category: HashMap<CategoryType, usize>,
    pub by_subcategory: HashMap<String, usize>,
    pub safety_details: SafetyStatistics,
    pub performance_details: PerformanceStatistics,
    pub style_details: StyleStatistics,
    pub doc_details: DocStatistics,
}

impl WarningStatistics {
    pub fn from_warnings(warnings: &[Warning], total_files: usize) -> Self {
        let mut stats = WarningStatistics {
            total_warnings: warnings.len(),
            total_input_warnings: warnings.len(),
            files_affected: total_files,
            by_category: HashMap::new(),
            by_priority: HashMap::new(),
            by_subcategory: HashMap::new(),
            safety_details: SafetyStatistics::default(),
            performance_details: PerformanceStatistics::default(),
            style_details: StyleStatistics::default(),
            doc_details: DocStatistics::default(),
        };

        for warning in warnings {
            *stats.by_category
                .entry(warning.category.category_type.clone())
                .or_insert(0) += 1;

            *stats.by_priority
                .entry(warning.priority)
                .or_insert(0) += 1;

            *stats.by_subcategory
                .entry(warning.category.subcategory.clone())
                .or_insert(0) += 1;
        }

        stats
    }

    pub fn get_detailed_stats(&self) -> (
        &SafetyStatistics,
        &PerformanceStatistics,
        &StyleStatistics,
        &DocStatistics
    ) {
        (
            &self.safety_details,
            &self.performance_details,
            &self.style_details,
            &self.doc_details
        )
    }
} 