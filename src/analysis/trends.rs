use std::collections::HashMap;
use crate::types::{CategoryType, Priority};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub dates: Vec<String>,
    pub total_warnings: usize,
    pub by_category: HashMap<CategoryType, usize>,
    pub by_priority: HashMap<Priority, usize>,
    pub improvement_rate: f64,
    pub recurring_issues: HashMap<String, usize>,
}

impl Default for TrendAnalysis {
    fn default() -> Self {
        Self {
            dates: Vec::new(),
            total_warnings: 0,
            by_category: HashMap::new(),
            by_priority: HashMap::new(),
            improvement_rate: 0.0,
            recurring_issues: HashMap::new(),
        }
    }
}

impl TrendAnalysis {
    pub fn new(
        total_warnings: usize,
        by_category: HashMap<CategoryType, usize>,
        by_priority: HashMap<Priority, usize>,
        recurring_issues: HashMap<String, usize>,
    ) -> Self {
        Self {
            dates: vec![chrono::Local::now().format("%Y-%m-%d").to_string()],
            total_warnings,
            by_category,
            by_priority,
            improvement_rate: 0.0, // This would need historical data to calculate
            recurring_issues,
        }
    }

    pub fn calculate_improvement_rate(&mut self, historical_warnings: &[usize]) -> f64 {
        if historical_warnings.is_empty() {
            return 0.0;
        }

        let historical_avg = historical_warnings.iter().sum::<usize>() as f64 
            / historical_warnings.len() as f64;
        
        if historical_avg == 0.0 {
            return 0.0;
        }

        let improvement = (historical_avg - self.total_warnings as f64) / historical_avg;
        self.improvement_rate = improvement;
        improvement
    }

    pub fn get_top_issues(&self, limit: usize) -> Vec<(&String, &usize)> {
        let mut issues: Vec<(&String, &usize)> = self.recurring_issues.iter().collect();
        issues.sort_by(|a, b| b.1.cmp(a.1));
        issues.truncate(limit);
        issues
    }

    pub fn get_category_distribution(&self) -> Vec<(CategoryType, f64)> {
        let total: usize = self.by_category.values().sum();
        self.by_category
            .iter()
            .map(|(cat, count)| (*cat, *count as f64 / total as f64 * 100.0))
            .collect()
    }

    pub fn get_priority_distribution(&self) -> Vec<(Priority, f64)> {
        let total: usize = self.by_priority.values().sum();
        self.by_priority
            .iter()
            .map(|(pri, count)| (*pri, *count as f64 / total as f64 * 100.0))
            .collect()
    }
}

pub fn analyze_trends(
    current: &TrendAnalysis,
    historical: &[TrendAnalysis]
) -> Vec<String> {
    let mut insights = Vec::new();

    // Compare with historical data
    if !historical.is_empty() {
        let avg_warnings: f64 = historical.iter()
            .map(|h| h.total_warnings as f64)
            .sum::<f64>() / historical.len() as f64;
        
        if current.total_warnings as f64 > avg_warnings * 1.2 {
            insights.push(format!(
                "Warning count increased by {:.1}% compared to historical average",
                ((current.total_warnings as f64 - avg_warnings) / avg_warnings * 100.0)
            ));
        }
    }

    // Analyze category trends
    let category_dist = current.get_category_distribution();
    for (category, percentage) in category_dist {
        if percentage > 30.0 {
            insights.push(format!(
                "High concentration of {} issues ({:.1}%)",
                category, percentage
            ));
        }
    }

    // Analyze priority trends
    let priority_dist = current.get_priority_distribution();
    for (priority, percentage) in priority_dist {
        if matches!(priority, Priority::Critical | Priority::High) && percentage > 20.0 {
            insights.push(format!(
                "Significant number of {} priority issues ({:.1}%)",
                priority, percentage
            ));
        }
    }

    // Analyze recurring issues
    let top_issues = current.get_top_issues(3);
    for (issue, count) in top_issues {
        insights.push(format!(
            "Frequently occurring issue: {} ({} occurrences)",
            issue, count
        ));
    }

    insights
}
